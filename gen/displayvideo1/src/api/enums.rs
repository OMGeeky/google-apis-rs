use super::*;



// region ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The minimum visible video duration required (in seconds) in order for an impression to be recorded. You must specify minimum_duration, minimum_quartile or both. If both are specified, an impression meets the metric criteria if either requirement is met (whichever happens first).
pub enum ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum {
    

    /// Value is not specified or is unknown in this version.
    ///
    /// "VIDEO_DURATION_UNSPECIFIED"
    #[serde(rename="VIDEO_DURATION_UNSPECIFIED")]
    VIDEODURATIONUNSPECIFIED,
    

    /// No duration value.
    ///
    /// "VIDEO_DURATION_SECONDS_NONE"
    #[serde(rename="VIDEO_DURATION_SECONDS_NONE")]
    VIDEODURATIONSECONDSNONE,
    

    /// 0 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_0"
    #[serde(rename="VIDEO_DURATION_SECONDS_0")]
    VIDEODURATIONSECONDS0,
    

    /// 1 second.
    ///
    /// "VIDEO_DURATION_SECONDS_1"
    #[serde(rename="VIDEO_DURATION_SECONDS_1")]
    VIDEODURATIONSECONDS1,
    

    /// 2 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_2"
    #[serde(rename="VIDEO_DURATION_SECONDS_2")]
    VIDEODURATIONSECONDS2,
    

    /// 3 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_3"
    #[serde(rename="VIDEO_DURATION_SECONDS_3")]
    VIDEODURATIONSECONDS3,
    

    /// 4 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_4"
    #[serde(rename="VIDEO_DURATION_SECONDS_4")]
    VIDEODURATIONSECONDS4,
    

    /// 5 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_5"
    #[serde(rename="VIDEO_DURATION_SECONDS_5")]
    VIDEODURATIONSECONDS5,
    

    /// 6 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_6"
    #[serde(rename="VIDEO_DURATION_SECONDS_6")]
    VIDEODURATIONSECONDS6,
    

    /// 7 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_7"
    #[serde(rename="VIDEO_DURATION_SECONDS_7")]
    VIDEODURATIONSECONDS7,
    

    /// 8 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_8"
    #[serde(rename="VIDEO_DURATION_SECONDS_8")]
    VIDEODURATIONSECONDS8,
    

    /// 9 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_9"
    #[serde(rename="VIDEO_DURATION_SECONDS_9")]
    VIDEODURATIONSECONDS9,
    

    /// 10 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_10"
    #[serde(rename="VIDEO_DURATION_SECONDS_10")]
    VIDEODURATIONSECONDS10,
    

    /// 11 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_11"
    #[serde(rename="VIDEO_DURATION_SECONDS_11")]
    VIDEODURATIONSECONDS11,
    

    /// 12 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_12"
    #[serde(rename="VIDEO_DURATION_SECONDS_12")]
    VIDEODURATIONSECONDS12,
    

    /// 13 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_13"
    #[serde(rename="VIDEO_DURATION_SECONDS_13")]
    VIDEODURATIONSECONDS13,
    

    /// 14 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_14"
    #[serde(rename="VIDEO_DURATION_SECONDS_14")]
    VIDEODURATIONSECONDS14,
    

    /// 15 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_15"
    #[serde(rename="VIDEO_DURATION_SECONDS_15")]
    VIDEODURATIONSECONDS15,
    

    /// 30 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_30"
    #[serde(rename="VIDEO_DURATION_SECONDS_30")]
    VIDEODURATIONSECONDS30,
    

    /// 45 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_45"
    #[serde(rename="VIDEO_DURATION_SECONDS_45")]
    VIDEODURATIONSECONDS45,
    

    /// 60 seconds.
    ///
    /// "VIDEO_DURATION_SECONDS_60"
    #[serde(rename="VIDEO_DURATION_SECONDS_60")]
    VIDEODURATIONSECONDS60,
}

impl AsRef<str> for ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONUNSPECIFIED => "VIDEO_DURATION_UNSPECIFIED",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDSNONE => "VIDEO_DURATION_SECONDS_NONE",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS0 => "VIDEO_DURATION_SECONDS_0",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS1 => "VIDEO_DURATION_SECONDS_1",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS2 => "VIDEO_DURATION_SECONDS_2",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS3 => "VIDEO_DURATION_SECONDS_3",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS4 => "VIDEO_DURATION_SECONDS_4",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS5 => "VIDEO_DURATION_SECONDS_5",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS6 => "VIDEO_DURATION_SECONDS_6",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS7 => "VIDEO_DURATION_SECONDS_7",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS8 => "VIDEO_DURATION_SECONDS_8",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS9 => "VIDEO_DURATION_SECONDS_9",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS10 => "VIDEO_DURATION_SECONDS_10",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS11 => "VIDEO_DURATION_SECONDS_11",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS12 => "VIDEO_DURATION_SECONDS_12",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS13 => "VIDEO_DURATION_SECONDS_13",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS14 => "VIDEO_DURATION_SECONDS_14",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS15 => "VIDEO_DURATION_SECONDS_15",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS30 => "VIDEO_DURATION_SECONDS_30",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS45 => "VIDEO_DURATION_SECONDS_45",
            ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS60 => "VIDEO_DURATION_SECONDS_60",
        }
    }
}

impl std::convert::TryFrom< &str> for ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIDEO_DURATION_UNSPECIFIED" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONUNSPECIFIED),
           "VIDEO_DURATION_SECONDS_NONE" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDSNONE),
           "VIDEO_DURATION_SECONDS_0" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS0),
           "VIDEO_DURATION_SECONDS_1" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS1),
           "VIDEO_DURATION_SECONDS_2" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS2),
           "VIDEO_DURATION_SECONDS_3" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS3),
           "VIDEO_DURATION_SECONDS_4" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS4),
           "VIDEO_DURATION_SECONDS_5" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS5),
           "VIDEO_DURATION_SECONDS_6" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS6),
           "VIDEO_DURATION_SECONDS_7" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS7),
           "VIDEO_DURATION_SECONDS_8" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS8),
           "VIDEO_DURATION_SECONDS_9" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS9),
           "VIDEO_DURATION_SECONDS_10" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS10),
           "VIDEO_DURATION_SECONDS_11" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS11),
           "VIDEO_DURATION_SECONDS_12" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS12),
           "VIDEO_DURATION_SECONDS_13" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS13),
           "VIDEO_DURATION_SECONDS_14" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS14),
           "VIDEO_DURATION_SECONDS_15" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS15),
           "VIDEO_DURATION_SECONDS_30" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS30),
           "VIDEO_DURATION_SECONDS_45" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS45),
           "VIDEO_DURATION_SECONDS_60" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum::VIDEODURATIONSECONDS60),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The minimum visible video duration required, based on the video quartiles, in order for an impression to be recorded. You must specify minimum_duration, minimum_quartile or both. If both are specified, an impression meets the metric criteria if either requirement is met (whichever happens first).
pub enum ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum {
    

    /// Value is not specified or is unknown in this version.
    ///
    /// "VIDEO_DURATION_QUARTILE_UNSPECIFIED"
    #[serde(rename="VIDEO_DURATION_QUARTILE_UNSPECIFIED")]
    VIDEODURATIONQUARTILEUNSPECIFIED,
    

    /// No quartile value.
    ///
    /// "VIDEO_DURATION_QUARTILE_NONE"
    #[serde(rename="VIDEO_DURATION_QUARTILE_NONE")]
    VIDEODURATIONQUARTILENONE,
    

    /// First quartile.
    ///
    /// "VIDEO_DURATION_QUARTILE_FIRST"
    #[serde(rename="VIDEO_DURATION_QUARTILE_FIRST")]
    VIDEODURATIONQUARTILEFIRST,
    

    /// Second quartile (midpoint).
    ///
    /// "VIDEO_DURATION_QUARTILE_SECOND"
    #[serde(rename="VIDEO_DURATION_QUARTILE_SECOND")]
    VIDEODURATIONQUARTILESECOND,
    

    /// Third quartile.
    ///
    /// "VIDEO_DURATION_QUARTILE_THIRD"
    #[serde(rename="VIDEO_DURATION_QUARTILE_THIRD")]
    VIDEODURATIONQUARTILETHIRD,
    

    /// Fourth quartile (completion).
    ///
    /// "VIDEO_DURATION_QUARTILE_FOURTH"
    #[serde(rename="VIDEO_DURATION_QUARTILE_FOURTH")]
    VIDEODURATIONQUARTILEFOURTH,
}

impl AsRef<str> for ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILEUNSPECIFIED => "VIDEO_DURATION_QUARTILE_UNSPECIFIED",
            ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILENONE => "VIDEO_DURATION_QUARTILE_NONE",
            ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILEFIRST => "VIDEO_DURATION_QUARTILE_FIRST",
            ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILESECOND => "VIDEO_DURATION_QUARTILE_SECOND",
            ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILETHIRD => "VIDEO_DURATION_QUARTILE_THIRD",
            ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILEFOURTH => "VIDEO_DURATION_QUARTILE_FOURTH",
        }
    }
}

impl std::convert::TryFrom< &str> for ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIDEO_DURATION_QUARTILE_UNSPECIFIED" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILEUNSPECIFIED),
           "VIDEO_DURATION_QUARTILE_NONE" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILENONE),
           "VIDEO_DURATION_QUARTILE_FIRST" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILEFIRST),
           "VIDEO_DURATION_QUARTILE_SECOND" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILESECOND),
           "VIDEO_DURATION_QUARTILE_THIRD" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILETHIRD),
           "VIDEO_DURATION_QUARTILE_FOURTH" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum::VIDEODURATIONQUARTILEFOURTH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The minimum percentage of the video ad's pixels visible on the screen in order for an impression to be recorded.
pub enum ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum {
    

    /// Value is not specified or is unknown in this version.
    ///
    /// "VIEWABILITY_PERCENT_UNSPECIFIED"
    #[serde(rename="VIEWABILITY_PERCENT_UNSPECIFIED")]
    VIEWABILITYPERCENTUNSPECIFIED,
    

    /// 0% viewable.
    ///
    /// "VIEWABILITY_PERCENT_0"
    #[serde(rename="VIEWABILITY_PERCENT_0")]
    VIEWABILITYPERCENT0,
    

    /// 25% viewable.
    ///
    /// "VIEWABILITY_PERCENT_25"
    #[serde(rename="VIEWABILITY_PERCENT_25")]
    VIEWABILITYPERCENT25,
    

    /// 50% viewable.
    ///
    /// "VIEWABILITY_PERCENT_50"
    #[serde(rename="VIEWABILITY_PERCENT_50")]
    VIEWABILITYPERCENT50,
    

    /// 75% viewable.
    ///
    /// "VIEWABILITY_PERCENT_75"
    #[serde(rename="VIEWABILITY_PERCENT_75")]
    VIEWABILITYPERCENT75,
    

    /// 100% viewable.
    ///
    /// "VIEWABILITY_PERCENT_100"
    #[serde(rename="VIEWABILITY_PERCENT_100")]
    VIEWABILITYPERCENT100,
}

impl AsRef<str> for ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENTUNSPECIFIED => "VIEWABILITY_PERCENT_UNSPECIFIED",
            ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENT0 => "VIEWABILITY_PERCENT_0",
            ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENT25 => "VIEWABILITY_PERCENT_25",
            ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENT50 => "VIEWABILITY_PERCENT_50",
            ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENT75 => "VIEWABILITY_PERCENT_75",
            ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENT100 => "VIEWABILITY_PERCENT_100",
        }
    }
}

impl std::convert::TryFrom< &str> for ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEWABILITY_PERCENT_UNSPECIFIED" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENTUNSPECIFIED),
           "VIEWABILITY_PERCENT_0" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENT0),
           "VIEWABILITY_PERCENT_25" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENT25),
           "VIEWABILITY_PERCENT_50" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENT50),
           "VIEWABILITY_PERCENT_75" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENT75),
           "VIEWABILITY_PERCENT_100" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum::VIEWABILITYPERCENT100),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The minimum percentage of the video ad's volume required in order for an impression to be recorded.
pub enum ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum {
    

    /// Value is not specified or is unknown in this version.
    ///
    /// "VIDEO_VOLUME_PERCENT_UNSPECIFIED"
    #[serde(rename="VIDEO_VOLUME_PERCENT_UNSPECIFIED")]
    VIDEOVOLUMEPERCENTUNSPECIFIED,
    

    /// 0% volume.
    ///
    /// "VIDEO_VOLUME_PERCENT_0"
    #[serde(rename="VIDEO_VOLUME_PERCENT_0")]
    VIDEOVOLUMEPERCENT0,
    

    /// 10% volume.
    ///
    /// "VIDEO_VOLUME_PERCENT_10"
    #[serde(rename="VIDEO_VOLUME_PERCENT_10")]
    VIDEOVOLUMEPERCENT10,
}

impl AsRef<str> for ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum::VIDEOVOLUMEPERCENTUNSPECIFIED => "VIDEO_VOLUME_PERCENT_UNSPECIFIED",
            ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum::VIDEOVOLUMEPERCENT0 => "VIDEO_VOLUME_PERCENT_0",
            ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum::VIDEOVOLUMEPERCENT10 => "VIDEO_VOLUME_PERCENT_10",
        }
    }
}

impl std::convert::TryFrom< &str> for ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIDEO_VOLUME_PERCENT_UNSPECIFIED" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum::VIDEOVOLUMEPERCENTUNSPECIFIED),
           "VIDEO_VOLUME_PERCENT_0" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum::VIDEOVOLUMEPERCENT0),
           "VIDEO_VOLUME_PERCENT_10" => Ok(ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum::VIDEOVOLUMEPERCENT10),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdlooxExcludedAdlooxCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Adloox's brand safety settings.
pub enum AdlooxExcludedAdlooxCategoriesEnum {
    

    /// This enum is only a placeholder and it doesn't specify any Adloox option.
    ///
    /// "ADLOOX_UNSPECIFIED"
    #[serde(rename="ADLOOX_UNSPECIFIED")]
    ADLOOXUNSPECIFIED,
    

    /// Adult content (hard).
    ///
    /// "ADULT_CONTENT_HARD"
    #[serde(rename="ADULT_CONTENT_HARD")]
    ADULTCONTENTHARD,
    

    /// Adult content (soft).
    ///
    /// "ADULT_CONTENT_SOFT"
    #[serde(rename="ADULT_CONTENT_SOFT")]
    ADULTCONTENTSOFT,
    

    /// Illegal content.
    ///
    /// "ILLEGAL_CONTENT"
    #[serde(rename="ILLEGAL_CONTENT")]
    ILLEGALCONTENT,
    

    /// Borderline content.
    ///
    /// "BORDERLINE_CONTENT"
    #[serde(rename="BORDERLINE_CONTENT")]
    BORDERLINECONTENT,
    

    /// Discriminatory content.
    ///
    /// "DISCRIMINATORY_CONTENT"
    #[serde(rename="DISCRIMINATORY_CONTENT")]
    DISCRIMINATORYCONTENT,
    

    /// Violent content & weapons.
    ///
    /// "VIOLENT_CONTENT_WEAPONS"
    #[serde(rename="VIOLENT_CONTENT_WEAPONS")]
    VIOLENTCONTENTWEAPONS,
    

    /// Low viewability domains.
    ///
    /// "LOW_VIEWABILITY_DOMAINS"
    #[serde(rename="LOW_VIEWABILITY_DOMAINS")]
    LOWVIEWABILITYDOMAINS,
    

    /// Fraud.
    ///
    /// "FRAUD"
    #[serde(rename="FRAUD")]
    FRAUD,
}

impl AsRef<str> for AdlooxExcludedAdlooxCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdlooxExcludedAdlooxCategoriesEnum::ADLOOXUNSPECIFIED => "ADLOOX_UNSPECIFIED",
            AdlooxExcludedAdlooxCategoriesEnum::ADULTCONTENTHARD => "ADULT_CONTENT_HARD",
            AdlooxExcludedAdlooxCategoriesEnum::ADULTCONTENTSOFT => "ADULT_CONTENT_SOFT",
            AdlooxExcludedAdlooxCategoriesEnum::ILLEGALCONTENT => "ILLEGAL_CONTENT",
            AdlooxExcludedAdlooxCategoriesEnum::BORDERLINECONTENT => "BORDERLINE_CONTENT",
            AdlooxExcludedAdlooxCategoriesEnum::DISCRIMINATORYCONTENT => "DISCRIMINATORY_CONTENT",
            AdlooxExcludedAdlooxCategoriesEnum::VIOLENTCONTENTWEAPONS => "VIOLENT_CONTENT_WEAPONS",
            AdlooxExcludedAdlooxCategoriesEnum::LOWVIEWABILITYDOMAINS => "LOW_VIEWABILITY_DOMAINS",
            AdlooxExcludedAdlooxCategoriesEnum::FRAUD => "FRAUD",
        }
    }
}

impl std::convert::TryFrom< &str> for AdlooxExcludedAdlooxCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADLOOX_UNSPECIFIED" => Ok(AdlooxExcludedAdlooxCategoriesEnum::ADLOOXUNSPECIFIED),
           "ADULT_CONTENT_HARD" => Ok(AdlooxExcludedAdlooxCategoriesEnum::ADULTCONTENTHARD),
           "ADULT_CONTENT_SOFT" => Ok(AdlooxExcludedAdlooxCategoriesEnum::ADULTCONTENTSOFT),
           "ILLEGAL_CONTENT" => Ok(AdlooxExcludedAdlooxCategoriesEnum::ILLEGALCONTENT),
           "BORDERLINE_CONTENT" => Ok(AdlooxExcludedAdlooxCategoriesEnum::BORDERLINECONTENT),
           "DISCRIMINATORY_CONTENT" => Ok(AdlooxExcludedAdlooxCategoriesEnum::DISCRIMINATORYCONTENT),
           "VIOLENT_CONTENT_WEAPONS" => Ok(AdlooxExcludedAdlooxCategoriesEnum::VIOLENTCONTENTWEAPONS),
           "LOW_VIEWABILITY_DOMAINS" => Ok(AdlooxExcludedAdlooxCategoriesEnum::LOWVIEWABILITYDOMAINS),
           "FRAUD" => Ok(AdlooxExcludedAdlooxCategoriesEnum::FRAUD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdlooxExcludedAdlooxCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdvertiserEntityStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Controls whether or not insertion orders and line items of the advertiser can spend their budgets and bid on inventory. * Accepted values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_PAUSED` and `ENTITY_STATUS_SCHEDULED_FOR_DELETION`. * If set to `ENTITY_STATUS_SCHEDULED_FOR_DELETION`, the advertiser will be deleted 30 days from when it was first scheduled for deletion.
pub enum AdvertiserEntityStatusEnum {
    

    /// Default value when status is not specified or is unknown in this version.
    ///
    /// "ENTITY_STATUS_UNSPECIFIED"
    #[serde(rename="ENTITY_STATUS_UNSPECIFIED")]
    ENTITYSTATUSUNSPECIFIED,
    

    /// The entity is enabled to bid and spend budget.
    ///
    /// "ENTITY_STATUS_ACTIVE"
    #[serde(rename="ENTITY_STATUS_ACTIVE")]
    ENTITYSTATUSACTIVE,
    

    /// The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved.
    ///
    /// "ENTITY_STATUS_ARCHIVED"
    #[serde(rename="ENTITY_STATUS_ARCHIVED")]
    ENTITYSTATUSARCHIVED,
    

    /// The entity is under draft. Bidding and budget spending are disabled.
    ///
    /// "ENTITY_STATUS_DRAFT"
    #[serde(rename="ENTITY_STATUS_DRAFT")]
    ENTITYSTATUSDRAFT,
    

    /// Bidding and budget spending are paused for the entity.
    ///
    /// "ENTITY_STATUS_PAUSED"
    #[serde(rename="ENTITY_STATUS_PAUSED")]
    ENTITYSTATUSPAUSED,
    

    /// The entity is scheduled for deletion.
    ///
    /// "ENTITY_STATUS_SCHEDULED_FOR_DELETION"
    #[serde(rename="ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    ENTITYSTATUSSCHEDULEDFORDELETION,
}

impl AsRef<str> for AdvertiserEntityStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserEntityStatusEnum::ENTITYSTATUSUNSPECIFIED => "ENTITY_STATUS_UNSPECIFIED",
            AdvertiserEntityStatusEnum::ENTITYSTATUSACTIVE => "ENTITY_STATUS_ACTIVE",
            AdvertiserEntityStatusEnum::ENTITYSTATUSARCHIVED => "ENTITY_STATUS_ARCHIVED",
            AdvertiserEntityStatusEnum::ENTITYSTATUSDRAFT => "ENTITY_STATUS_DRAFT",
            AdvertiserEntityStatusEnum::ENTITYSTATUSPAUSED => "ENTITY_STATUS_PAUSED",
            AdvertiserEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION => "ENTITY_STATUS_SCHEDULED_FOR_DELETION",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserEntityStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_STATUS_UNSPECIFIED" => Ok(AdvertiserEntityStatusEnum::ENTITYSTATUSUNSPECIFIED),
           "ENTITY_STATUS_ACTIVE" => Ok(AdvertiserEntityStatusEnum::ENTITYSTATUSACTIVE),
           "ENTITY_STATUS_ARCHIVED" => Ok(AdvertiserEntityStatusEnum::ENTITYSTATUSARCHIVED),
           "ENTITY_STATUS_DRAFT" => Ok(AdvertiserEntityStatusEnum::ENTITYSTATUSDRAFT),
           "ENTITY_STATUS_PAUSED" => Ok(AdvertiserEntityStatusEnum::ENTITYSTATUSPAUSED),
           "ENTITY_STATUS_SCHEDULED_FOR_DELETION" => Ok(AdvertiserEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserEntityStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AgeRangeAssignedTargetingOptionDetailAgeRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The age range of an audience. We only support targeting a continuous age range of an audience. Thus, the age range represented in this field can be 1) targeted solely, or, 2) part of a larger continuous age range. The reach of a continuous age range targeting can be expanded by also targeting an audience of an unknown age.
pub enum AgeRangeAssignedTargetingOptionDetailAgeRangeEnum {
    

    /// Default value when age range is not specified in this version. This enum is a placeholder for default value and does not represent a real age range option.
    ///
    /// "AGE_RANGE_UNSPECIFIED"
    #[serde(rename="AGE_RANGE_UNSPECIFIED")]
    AGERANGEUNSPECIFIED,
    

    /// The age range of the audience is 18 to 24.
    ///
    /// "AGE_RANGE_18_24"
    #[serde(rename="AGE_RANGE_18_24")]
    AGERANGE1824,
    

    /// The age range of the audience is 25 to 34.
    ///
    /// "AGE_RANGE_25_34"
    #[serde(rename="AGE_RANGE_25_34")]
    AGERANGE2534,
    

    /// The age range of the audience is 35 to 44.
    ///
    /// "AGE_RANGE_35_44"
    #[serde(rename="AGE_RANGE_35_44")]
    AGERANGE3544,
    

    /// The age range of the audience is 45 to 54.
    ///
    /// "AGE_RANGE_45_54"
    #[serde(rename="AGE_RANGE_45_54")]
    AGERANGE4554,
    

    /// The age range of the audience is 55 to 64.
    ///
    /// "AGE_RANGE_55_64"
    #[serde(rename="AGE_RANGE_55_64")]
    AGERANGE5564,
    

    /// The age range of the audience is 65 and up.
    ///
    /// "AGE_RANGE_65_PLUS"
    #[serde(rename="AGE_RANGE_65_PLUS")]
    AGERANGE65PLUS,
    

    /// The age range of the audience is unknown.
    ///
    /// "AGE_RANGE_UNKNOWN"
    #[serde(rename="AGE_RANGE_UNKNOWN")]
    AGERANGEUNKNOWN,
}

impl AsRef<str> for AgeRangeAssignedTargetingOptionDetailAgeRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGEUNSPECIFIED => "AGE_RANGE_UNSPECIFIED",
            AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE1824 => "AGE_RANGE_18_24",
            AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE2534 => "AGE_RANGE_25_34",
            AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE3544 => "AGE_RANGE_35_44",
            AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE4554 => "AGE_RANGE_45_54",
            AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE5564 => "AGE_RANGE_55_64",
            AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE65PLUS => "AGE_RANGE_65_PLUS",
            AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGEUNKNOWN => "AGE_RANGE_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for AgeRangeAssignedTargetingOptionDetailAgeRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AGE_RANGE_UNSPECIFIED" => Ok(AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGEUNSPECIFIED),
           "AGE_RANGE_18_24" => Ok(AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE1824),
           "AGE_RANGE_25_34" => Ok(AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE2534),
           "AGE_RANGE_35_44" => Ok(AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE3544),
           "AGE_RANGE_45_54" => Ok(AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE4554),
           "AGE_RANGE_55_64" => Ok(AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE5564),
           "AGE_RANGE_65_PLUS" => Ok(AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGE65PLUS),
           "AGE_RANGE_UNKNOWN" => Ok(AgeRangeAssignedTargetingOptionDetailAgeRangeEnum::AGERANGEUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AgeRangeAssignedTargetingOptionDetailAgeRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AgeRangeTargetingOptionDetailAgeRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The age range of an audience.
pub enum AgeRangeTargetingOptionDetailAgeRangeEnum {
    

    /// Default value when age range is not specified in this version. This enum is a placeholder for default value and does not represent a real age range option.
    ///
    /// "AGE_RANGE_UNSPECIFIED"
    #[serde(rename="AGE_RANGE_UNSPECIFIED")]
    AGERANGEUNSPECIFIED,
    

    /// The age range of the audience is 18 to 24.
    ///
    /// "AGE_RANGE_18_24"
    #[serde(rename="AGE_RANGE_18_24")]
    AGERANGE1824,
    

    /// The age range of the audience is 25 to 34.
    ///
    /// "AGE_RANGE_25_34"
    #[serde(rename="AGE_RANGE_25_34")]
    AGERANGE2534,
    

    /// The age range of the audience is 35 to 44.
    ///
    /// "AGE_RANGE_35_44"
    #[serde(rename="AGE_RANGE_35_44")]
    AGERANGE3544,
    

    /// The age range of the audience is 45 to 54.
    ///
    /// "AGE_RANGE_45_54"
    #[serde(rename="AGE_RANGE_45_54")]
    AGERANGE4554,
    

    /// The age range of the audience is 55 to 64.
    ///
    /// "AGE_RANGE_55_64"
    #[serde(rename="AGE_RANGE_55_64")]
    AGERANGE5564,
    

    /// The age range of the audience is 65 and up.
    ///
    /// "AGE_RANGE_65_PLUS"
    #[serde(rename="AGE_RANGE_65_PLUS")]
    AGERANGE65PLUS,
    

    /// The age range of the audience is unknown.
    ///
    /// "AGE_RANGE_UNKNOWN"
    #[serde(rename="AGE_RANGE_UNKNOWN")]
    AGERANGEUNKNOWN,
}

impl AsRef<str> for AgeRangeTargetingOptionDetailAgeRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGEUNSPECIFIED => "AGE_RANGE_UNSPECIFIED",
            AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE1824 => "AGE_RANGE_18_24",
            AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE2534 => "AGE_RANGE_25_34",
            AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE3544 => "AGE_RANGE_35_44",
            AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE4554 => "AGE_RANGE_45_54",
            AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE5564 => "AGE_RANGE_55_64",
            AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE65PLUS => "AGE_RANGE_65_PLUS",
            AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGEUNKNOWN => "AGE_RANGE_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for AgeRangeTargetingOptionDetailAgeRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AGE_RANGE_UNSPECIFIED" => Ok(AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGEUNSPECIFIED),
           "AGE_RANGE_18_24" => Ok(AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE1824),
           "AGE_RANGE_25_34" => Ok(AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE2534),
           "AGE_RANGE_35_44" => Ok(AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE3544),
           "AGE_RANGE_45_54" => Ok(AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE4554),
           "AGE_RANGE_55_64" => Ok(AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE5564),
           "AGE_RANGE_65_PLUS" => Ok(AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGE65PLUS),
           "AGE_RANGE_UNKNOWN" => Ok(AgeRangeTargetingOptionDetailAgeRangeEnum::AGERANGEUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AgeRangeTargetingOptionDetailAgeRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppAssignedTargetingOptionDetailAppPlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the platform of the targeted app. If this field is not specified, the app platform will be assumed to be mobile (i.e., Android or iOS), and we will derive the appropriate mobile platform from the app ID.
pub enum AppAssignedTargetingOptionDetailAppPlatformEnum {
    

    /// Default value when app platform is not specified in this version. This enum is a placeholder for default value and does not represent a real platform option.
    ///
    /// "APP_PLATFORM_UNSPECIFIED"
    #[serde(rename="APP_PLATFORM_UNSPECIFIED")]
    APPPLATFORMUNSPECIFIED,
    

    /// The app platform is iOS.
    ///
    /// "APP_PLATFORM_IOS"
    #[serde(rename="APP_PLATFORM_IOS")]
    APPPLATFORMIOS,
    

    /// The app platform is Android.
    ///
    /// "APP_PLATFORM_ANDROID"
    #[serde(rename="APP_PLATFORM_ANDROID")]
    APPPLATFORMANDROID,
    

    /// The app platform is Roku.
    ///
    /// "APP_PLATFORM_ROKU"
    #[serde(rename="APP_PLATFORM_ROKU")]
    APPPLATFORMROKU,
    

    /// The app platform is Amazon FireTV.
    ///
    /// "APP_PLATFORM_AMAZON_FIRETV"
    #[serde(rename="APP_PLATFORM_AMAZON_FIRETV")]
    APPPLATFORMAMAZONFIRETV,
    

    /// The app platform is Playstation.
    ///
    /// "APP_PLATFORM_PLAYSTATION"
    #[serde(rename="APP_PLATFORM_PLAYSTATION")]
    APPPLATFORMPLAYSTATION,
    

    /// The app platform is Apple TV.
    ///
    /// "APP_PLATFORM_APPLE_TV"
    #[serde(rename="APP_PLATFORM_APPLE_TV")]
    APPPLATFORMAPPLETV,
    

    /// The app platform is Xbox.
    ///
    /// "APP_PLATFORM_XBOX"
    #[serde(rename="APP_PLATFORM_XBOX")]
    APPPLATFORMXBOX,
    

    /// The app platform is Samsung TV.
    ///
    /// "APP_PLATFORM_SAMSUNG_TV"
    #[serde(rename="APP_PLATFORM_SAMSUNG_TV")]
    APPPLATFORMSAMSUNGTV,
    

    /// The app platform is Android TV.
    ///
    /// "APP_PLATFORM_ANDROID_TV"
    #[serde(rename="APP_PLATFORM_ANDROID_TV")]
    APPPLATFORMANDROIDTV,
    

    /// The app platform is a CTV platform that is not explicitly listed elsewhere.
    ///
    /// "APP_PLATFORM_GENERIC_CTV"
    #[serde(rename="APP_PLATFORM_GENERIC_CTV")]
    APPPLATFORMGENERICCTV,
    

    /// The app platform is LG TV.
    ///
    /// "APP_PLATFORM_LG_TV"
    #[serde(rename="APP_PLATFORM_LG_TV")]
    APPPLATFORMLGTV,
    

    /// The app platform is VIZIO TV.
    ///
    /// "APP_PLATFORM_VIZIO_TV"
    #[serde(rename="APP_PLATFORM_VIZIO_TV")]
    APPPLATFORMVIZIOTV,
}

impl AsRef<str> for AppAssignedTargetingOptionDetailAppPlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMUNSPECIFIED => "APP_PLATFORM_UNSPECIFIED",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMIOS => "APP_PLATFORM_IOS",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMANDROID => "APP_PLATFORM_ANDROID",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMROKU => "APP_PLATFORM_ROKU",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMAMAZONFIRETV => "APP_PLATFORM_AMAZON_FIRETV",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMPLAYSTATION => "APP_PLATFORM_PLAYSTATION",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMAPPLETV => "APP_PLATFORM_APPLE_TV",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMXBOX => "APP_PLATFORM_XBOX",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMSAMSUNGTV => "APP_PLATFORM_SAMSUNG_TV",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMANDROIDTV => "APP_PLATFORM_ANDROID_TV",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMGENERICCTV => "APP_PLATFORM_GENERIC_CTV",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMLGTV => "APP_PLATFORM_LG_TV",
            AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMVIZIOTV => "APP_PLATFORM_VIZIO_TV",
        }
    }
}

impl std::convert::TryFrom< &str> for AppAssignedTargetingOptionDetailAppPlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_PLATFORM_UNSPECIFIED" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMUNSPECIFIED),
           "APP_PLATFORM_IOS" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMIOS),
           "APP_PLATFORM_ANDROID" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMANDROID),
           "APP_PLATFORM_ROKU" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMROKU),
           "APP_PLATFORM_AMAZON_FIRETV" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMAMAZONFIRETV),
           "APP_PLATFORM_PLAYSTATION" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMPLAYSTATION),
           "APP_PLATFORM_APPLE_TV" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMAPPLETV),
           "APP_PLATFORM_XBOX" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMXBOX),
           "APP_PLATFORM_SAMSUNG_TV" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMSAMSUNGTV),
           "APP_PLATFORM_ANDROID_TV" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMANDROIDTV),
           "APP_PLATFORM_GENERIC_CTV" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMGENERICCTV),
           "APP_PLATFORM_LG_TV" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMLGTV),
           "APP_PLATFORM_VIZIO_TV" => Ok(AppAssignedTargetingOptionDetailAppPlatformEnum::APPPLATFORMVIZIOTV),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppAssignedTargetingOptionDetailAppPlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AssetAssociationRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role of this asset for the creative.
pub enum AssetAssociationRoleEnum {
    

    /// Asset role is not specified or is unknown in this version.
    ///
    /// "ASSET_ROLE_UNSPECIFIED"
    #[serde(rename="ASSET_ROLE_UNSPECIFIED")]
    ASSETROLEUNSPECIFIED,
    

    /// The asset is the main asset of the creative.
    ///
    /// "ASSET_ROLE_MAIN"
    #[serde(rename="ASSET_ROLE_MAIN")]
    ASSETROLEMAIN,
    

    /// The asset is a backup asset of the creative.
    ///
    /// "ASSET_ROLE_BACKUP"
    #[serde(rename="ASSET_ROLE_BACKUP")]
    ASSETROLEBACKUP,
    

    /// The asset is a polite load asset of the creative.
    ///
    /// "ASSET_ROLE_POLITE_LOAD"
    #[serde(rename="ASSET_ROLE_POLITE_LOAD")]
    ASSETROLEPOLITELOAD,
    

    /// Headline of a native creative. The content must be UTF-8 encoded with a length of no more than 25 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`
    ///
    /// "ASSET_ROLE_HEADLINE"
    #[serde(rename="ASSET_ROLE_HEADLINE")]
    ASSETROLEHEADLINE,
    

    /// Long headline of a native creative. The content must be UTF-8 encoded with a length of no more than 50 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`
    ///
    /// "ASSET_ROLE_LONG_HEADLINE"
    #[serde(rename="ASSET_ROLE_LONG_HEADLINE")]
    ASSETROLELONGHEADLINE,
    

    /// Body text of a native creative. The content must be UTF-8 encoded with a length of no more than 90 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`
    ///
    /// "ASSET_ROLE_BODY"
    #[serde(rename="ASSET_ROLE_BODY")]
    ASSETROLEBODY,
    

    /// Long body text of a native creative. The content must be UTF-8 encoded with a length of no more than 150 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`
    ///
    /// "ASSET_ROLE_LONG_BODY"
    #[serde(rename="ASSET_ROLE_LONG_BODY")]
    ASSETROLELONGBODY,
    

    /// A short, friendly version of the landing page URL to show in the creative. This URL gives people an idea of where they'll arrive after they click on the creative. The content must be UTF-8 encoded with a length of no more than 30 characters. For example, if the landing page URL is 'http://www.example.com/page', the caption URL can be 'example.com'. The protocol (http://) is optional, but the URL can't contain spaces or special characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`
    ///
    /// "ASSET_ROLE_CAPTION_URL"
    #[serde(rename="ASSET_ROLE_CAPTION_URL")]
    ASSETROLECAPTIONURL,
    

    /// The text to use on the call-to-action button of a native creative. The content must be UTF-8 encoded with a length of no more than 15 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`
    ///
    /// "ASSET_ROLE_CALL_TO_ACTION"
    #[serde(rename="ASSET_ROLE_CALL_TO_ACTION")]
    ASSETROLECALLTOACTION,
    

    /// The text that identifies the advertiser or brand name. The content must be UTF-8 encoded with a length of no more than 25 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`
    ///
    /// "ASSET_ROLE_ADVERTISER_NAME"
    #[serde(rename="ASSET_ROLE_ADVERTISER_NAME")]
    ASSETROLEADVERTISERNAME,
    

    /// The purchase price of your app in the Google play store or iOS app store (for example, $5.99). Note that this value is not automatically synced with the actual value listed in the store. It will always be the one provided when save the creative. The content must be UTF-8 encoded with a length of no more than 15 characters. Assets of this role are read-only.
    ///
    /// "ASSET_ROLE_PRICE"
    #[serde(rename="ASSET_ROLE_PRICE")]
    ASSETROLEPRICE,
    

    /// The ID of an Android app in the Google play store. You can find this ID in the App’s Google Play Store URL after ‘id’. For example, in https://play.google.com/store/apps/details?id=com.company.appname the identifier is com.company.appname. Assets of this role are read-only.
    ///
    /// "ASSET_ROLE_ANDROID_APP_ID"
    #[serde(rename="ASSET_ROLE_ANDROID_APP_ID")]
    ASSETROLEANDROIDAPPID,
    

    /// The ID of an iOS app in the Apple app store. This ID number can be found in the Apple App Store URL as the string of numbers directly after "id". For example, in https://apps.apple.com/us/app/gmail-email-by-google/id422689480 the ID is 422689480. Assets of this role are read-only.
    ///
    /// "ASSET_ROLE_IOS_APP_ID"
    #[serde(rename="ASSET_ROLE_IOS_APP_ID")]
    ASSETROLEIOSAPPID,
    

    /// The rating of an app in the Google play store or iOS app store. Note that this value is not automatically synced with the actual rating in the store. It will always be the one provided when save the creative. Assets of this role are read-only.
    ///
    /// "ASSET_ROLE_RATING"
    #[serde(rename="ASSET_ROLE_RATING")]
    ASSETROLERATING,
    

    /// The icon of a creative. This role is only supported and required in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE`
    ///
    /// "ASSET_ROLE_ICON"
    #[serde(rename="ASSET_ROLE_ICON")]
    ASSETROLEICON,
    

    /// The cover image of a native video creative. This role is only supported and required in following creative_type: * `CREATIVE_TYPE_VIDEO`
    ///
    /// "ASSET_ROLE_COVER_IMAGE"
    #[serde(rename="ASSET_ROLE_COVER_IMAGE")]
    ASSETROLECOVERIMAGE,
}

impl AsRef<str> for AssetAssociationRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssetAssociationRoleEnum::ASSETROLEUNSPECIFIED => "ASSET_ROLE_UNSPECIFIED",
            AssetAssociationRoleEnum::ASSETROLEMAIN => "ASSET_ROLE_MAIN",
            AssetAssociationRoleEnum::ASSETROLEBACKUP => "ASSET_ROLE_BACKUP",
            AssetAssociationRoleEnum::ASSETROLEPOLITELOAD => "ASSET_ROLE_POLITE_LOAD",
            AssetAssociationRoleEnum::ASSETROLEHEADLINE => "ASSET_ROLE_HEADLINE",
            AssetAssociationRoleEnum::ASSETROLELONGHEADLINE => "ASSET_ROLE_LONG_HEADLINE",
            AssetAssociationRoleEnum::ASSETROLEBODY => "ASSET_ROLE_BODY",
            AssetAssociationRoleEnum::ASSETROLELONGBODY => "ASSET_ROLE_LONG_BODY",
            AssetAssociationRoleEnum::ASSETROLECAPTIONURL => "ASSET_ROLE_CAPTION_URL",
            AssetAssociationRoleEnum::ASSETROLECALLTOACTION => "ASSET_ROLE_CALL_TO_ACTION",
            AssetAssociationRoleEnum::ASSETROLEADVERTISERNAME => "ASSET_ROLE_ADVERTISER_NAME",
            AssetAssociationRoleEnum::ASSETROLEPRICE => "ASSET_ROLE_PRICE",
            AssetAssociationRoleEnum::ASSETROLEANDROIDAPPID => "ASSET_ROLE_ANDROID_APP_ID",
            AssetAssociationRoleEnum::ASSETROLEIOSAPPID => "ASSET_ROLE_IOS_APP_ID",
            AssetAssociationRoleEnum::ASSETROLERATING => "ASSET_ROLE_RATING",
            AssetAssociationRoleEnum::ASSETROLEICON => "ASSET_ROLE_ICON",
            AssetAssociationRoleEnum::ASSETROLECOVERIMAGE => "ASSET_ROLE_COVER_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for AssetAssociationRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSET_ROLE_UNSPECIFIED" => Ok(AssetAssociationRoleEnum::ASSETROLEUNSPECIFIED),
           "ASSET_ROLE_MAIN" => Ok(AssetAssociationRoleEnum::ASSETROLEMAIN),
           "ASSET_ROLE_BACKUP" => Ok(AssetAssociationRoleEnum::ASSETROLEBACKUP),
           "ASSET_ROLE_POLITE_LOAD" => Ok(AssetAssociationRoleEnum::ASSETROLEPOLITELOAD),
           "ASSET_ROLE_HEADLINE" => Ok(AssetAssociationRoleEnum::ASSETROLEHEADLINE),
           "ASSET_ROLE_LONG_HEADLINE" => Ok(AssetAssociationRoleEnum::ASSETROLELONGHEADLINE),
           "ASSET_ROLE_BODY" => Ok(AssetAssociationRoleEnum::ASSETROLEBODY),
           "ASSET_ROLE_LONG_BODY" => Ok(AssetAssociationRoleEnum::ASSETROLELONGBODY),
           "ASSET_ROLE_CAPTION_URL" => Ok(AssetAssociationRoleEnum::ASSETROLECAPTIONURL),
           "ASSET_ROLE_CALL_TO_ACTION" => Ok(AssetAssociationRoleEnum::ASSETROLECALLTOACTION),
           "ASSET_ROLE_ADVERTISER_NAME" => Ok(AssetAssociationRoleEnum::ASSETROLEADVERTISERNAME),
           "ASSET_ROLE_PRICE" => Ok(AssetAssociationRoleEnum::ASSETROLEPRICE),
           "ASSET_ROLE_ANDROID_APP_ID" => Ok(AssetAssociationRoleEnum::ASSETROLEANDROIDAPPID),
           "ASSET_ROLE_IOS_APP_ID" => Ok(AssetAssociationRoleEnum::ASSETROLEIOSAPPID),
           "ASSET_ROLE_RATING" => Ok(AssetAssociationRoleEnum::ASSETROLERATING),
           "ASSET_ROLE_ICON" => Ok(AssetAssociationRoleEnum::ASSETROLEICON),
           "ASSET_ROLE_COVER_IMAGE" => Ok(AssetAssociationRoleEnum::ASSETROLECOVERIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssetAssociationRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AssignedTargetingOptionInheritanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The inheritance status of the assigned targeting option.
pub enum AssignedTargetingOptionInheritanceEnum {
    

    /// The inheritance is unspecified or unknown.
    ///
    /// "INHERITANCE_UNSPECIFIED"
    #[serde(rename="INHERITANCE_UNSPECIFIED")]
    INHERITANCEUNSPECIFIED,
    

    /// The assigned targeting option is not inherited from higher level entity.
    ///
    /// "NOT_INHERITED"
    #[serde(rename="NOT_INHERITED")]
    NOTINHERITED,
    

    /// The assigned targeting option is inherited from partner targeting settings.
    ///
    /// "INHERITED_FROM_PARTNER"
    #[serde(rename="INHERITED_FROM_PARTNER")]
    INHERITEDFROMPARTNER,
    

    /// The assigned targeting option is inherited from advertiser targeting settings.
    ///
    /// "INHERITED_FROM_ADVERTISER"
    #[serde(rename="INHERITED_FROM_ADVERTISER")]
    INHERITEDFROMADVERTISER,
}

impl AsRef<str> for AssignedTargetingOptionInheritanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssignedTargetingOptionInheritanceEnum::INHERITANCEUNSPECIFIED => "INHERITANCE_UNSPECIFIED",
            AssignedTargetingOptionInheritanceEnum::NOTINHERITED => "NOT_INHERITED",
            AssignedTargetingOptionInheritanceEnum::INHERITEDFROMPARTNER => "INHERITED_FROM_PARTNER",
            AssignedTargetingOptionInheritanceEnum::INHERITEDFROMADVERTISER => "INHERITED_FROM_ADVERTISER",
        }
    }
}

impl std::convert::TryFrom< &str> for AssignedTargetingOptionInheritanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INHERITANCE_UNSPECIFIED" => Ok(AssignedTargetingOptionInheritanceEnum::INHERITANCEUNSPECIFIED),
           "NOT_INHERITED" => Ok(AssignedTargetingOptionInheritanceEnum::NOTINHERITED),
           "INHERITED_FROM_PARTNER" => Ok(AssignedTargetingOptionInheritanceEnum::INHERITEDFROMPARTNER),
           "INHERITED_FROM_ADVERTISER" => Ok(AssignedTargetingOptionInheritanceEnum::INHERITEDFROMADVERTISER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssignedTargetingOptionInheritanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AssignedTargetingOptionTargetingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Identifies the type of this assigned targeting option.
pub enum AssignedTargetingOptionTargetingTypeEnum {
    

    /// Default value when type is not specified or is unknown in this version.
    ///
    /// "TARGETING_TYPE_UNSPECIFIED"
    #[serde(rename="TARGETING_TYPE_UNSPECIFIED")]
    TARGETINGTYPEUNSPECIFIED,
    

    /// Target a channel (a custom group of related websites or apps).
    ///
    /// "TARGETING_TYPE_CHANNEL"
    #[serde(rename="TARGETING_TYPE_CHANNEL")]
    TARGETINGTYPECHANNEL,
    

    /// Target an app category (for example, education or puzzle games).
    ///
    /// "TARGETING_TYPE_APP_CATEGORY"
    #[serde(rename="TARGETING_TYPE_APP_CATEGORY")]
    TARGETINGTYPEAPPCATEGORY,
    

    /// Target a specific app (for example, Angry Birds).
    ///
    /// "TARGETING_TYPE_APP"
    #[serde(rename="TARGETING_TYPE_APP")]
    TARGETINGTYPEAPP,
    

    /// Target a specific url (for example, quora.com).
    ///
    /// "TARGETING_TYPE_URL"
    #[serde(rename="TARGETING_TYPE_URL")]
    TARGETINGTYPEURL,
    

    /// Target ads during a chosen time period on a specific day.
    ///
    /// "TARGETING_TYPE_DAY_AND_TIME"
    #[serde(rename="TARGETING_TYPE_DAY_AND_TIME")]
    TARGETINGTYPEDAYANDTIME,
    

    /// Target ads to a specific age range (for example, 18-24).
    ///
    /// "TARGETING_TYPE_AGE_RANGE"
    #[serde(rename="TARGETING_TYPE_AGE_RANGE")]
    TARGETINGTYPEAGERANGE,
    

    /// Target ads to the specified regions on a regional location list.
    ///
    /// "TARGETING_TYPE_REGIONAL_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    TARGETINGTYPEREGIONALLOCATIONLIST,
    

    /// Target ads to the specified points of interest on a proximity location list.
    ///
    /// "TARGETING_TYPE_PROXIMITY_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    TARGETINGTYPEPROXIMITYLOCATIONLIST,
    

    /// Target ads to a specific gender (for example, female or male).
    ///
    /// "TARGETING_TYPE_GENDER"
    #[serde(rename="TARGETING_TYPE_GENDER")]
    TARGETINGTYPEGENDER,
    

    /// Target a specific video player size for video ads.
    ///
    /// "TARGETING_TYPE_VIDEO_PLAYER_SIZE"
    #[serde(rename="TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    TARGETINGTYPEVIDEOPLAYERSIZE,
    

    /// Target user rewarded content for video ads.
    ///
    /// "TARGETING_TYPE_USER_REWARDED_CONTENT"
    #[serde(rename="TARGETING_TYPE_USER_REWARDED_CONTENT")]
    TARGETINGTYPEUSERREWARDEDCONTENT,
    

    /// Target ads to a specific parental status (for example, parent or not a parent).
    ///
    /// "TARGETING_TYPE_PARENTAL_STATUS"
    #[serde(rename="TARGETING_TYPE_PARENTAL_STATUS")]
    TARGETINGTYPEPARENTALSTATUS,
    

    /// Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll).
    ///
    /// "TARGETING_TYPE_CONTENT_INSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    TARGETINGTYPECONTENTINSTREAMPOSITION,
    

    /// Target ads in a specific content outstream position.
    ///
    /// "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    TARGETINGTYPECONTENTOUTSTREAMPOSITION,
    

    /// Target ads to a specific device type (for example, tablet or connected TV).
    ///
    /// "TARGETING_TYPE_DEVICE_TYPE"
    #[serde(rename="TARGETING_TYPE_DEVICE_TYPE")]
    TARGETINGTYPEDEVICETYPE,
    

    /// Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time.
    ///
    /// "TARGETING_TYPE_AUDIENCE_GROUP"
    #[serde(rename="TARGETING_TYPE_AUDIENCE_GROUP")]
    TARGETINGTYPEAUDIENCEGROUP,
    

    /// Target ads to specific web browsers (for example, Chrome).
    ///
    /// "TARGETING_TYPE_BROWSER"
    #[serde(rename="TARGETING_TYPE_BROWSER")]
    TARGETINGTYPEBROWSER,
    

    /// Target ads to a specific household income range (for example, top 10%).
    ///
    /// "TARGETING_TYPE_HOUSEHOLD_INCOME"
    #[serde(rename="TARGETING_TYPE_HOUSEHOLD_INCOME")]
    TARGETINGTYPEHOUSEHOLDINCOME,
    

    /// Target ads in a specific on screen position.
    ///
    /// "TARGETING_TYPE_ON_SCREEN_POSITION"
    #[serde(rename="TARGETING_TYPE_ON_SCREEN_POSITION")]
    TARGETINGTYPEONSCREENPOSITION,
    

    /// Filter web sites through third party verification (for example, IAS or DoubleVerify).
    ///
    /// "TARGETING_TYPE_THIRD_PARTY_VERIFIER"
    #[serde(rename="TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    TARGETINGTYPETHIRDPARTYVERIFIER,
    

    /// Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences).
    ///
    /// "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION,
    

    /// Filter website content by sensitive categories (for example, adult).
    ///
    /// "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    TARGETINGTYPESENSITIVECATEGORYEXCLUSION,
    

    /// Target ads to a specific environment (for example, web or app).
    ///
    /// "TARGETING_TYPE_ENVIRONMENT"
    #[serde(rename="TARGETING_TYPE_ENVIRONMENT")]
    TARGETINGTYPEENVIRONMENT,
    

    /// Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange).
    ///
    /// "TARGETING_TYPE_CARRIER_AND_ISP"
    #[serde(rename="TARGETING_TYPE_CARRIER_AND_ISP")]
    TARGETINGTYPECARRIERANDISP,
    

    /// Target ads to a specific operating system (for example, macOS).
    ///
    /// "TARGETING_TYPE_OPERATING_SYSTEM"
    #[serde(rename="TARGETING_TYPE_OPERATING_SYSTEM")]
    TARGETINGTYPEOPERATINGSYSTEM,
    

    /// Target ads to a specific device make or model (for example, Roku or Samsung).
    ///
    /// "TARGETING_TYPE_DEVICE_MAKE_MODEL"
    #[serde(rename="TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    TARGETINGTYPEDEVICEMAKEMODEL,
    

    /// Target ads to a specific keyword (for example, dog or retriever).
    ///
    /// "TARGETING_TYPE_KEYWORD"
    #[serde(rename="TARGETING_TYPE_KEYWORD")]
    TARGETINGTYPEKEYWORD,
    

    /// Target ads to a specific negative keyword list.
    ///
    /// "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST"
    #[serde(rename="TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    TARGETINGTYPENEGATIVEKEYWORDLIST,
    

    /// Target ads to a specific viewability (for example, 80% viewable).
    ///
    /// "TARGETING_TYPE_VIEWABILITY"
    #[serde(rename="TARGETING_TYPE_VIEWABILITY")]
    TARGETINGTYPEVIEWABILITY,
    

    /// Target ads to a specific content category (for example, arts & entertainment).
    ///
    /// "TARGETING_TYPE_CATEGORY"
    #[serde(rename="TARGETING_TYPE_CATEGORY")]
    TARGETINGTYPECATEGORY,
    

    /// Purchase impressions from specific deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE")]
    TARGETINGTYPEINVENTORYSOURCE,
    

    /// Target ads to a specific language (for example, English or Japanese).
    ///
    /// "TARGETING_TYPE_LANGUAGE"
    #[serde(rename="TARGETING_TYPE_LANGUAGE")]
    TARGETINGTYPELANGUAGE,
    

    /// Target ads to ads.txt authorized sellers. If no targeting option of this type is assigned, the resource uses the "Authorized Direct Sellers and Resellers" option by default.
    ///
    /// "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS"
    #[serde(rename="TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    TARGETINGTYPEAUTHORIZEDSELLERSTATUS,
    

    /// Target ads to a specific regional location (for example, a city or state).
    ///
    /// "TARGETING_TYPE_GEO_REGION"
    #[serde(rename="TARGETING_TYPE_GEO_REGION")]
    TARGETINGTYPEGEOREGION,
    

    /// Purchase impressions from a group of deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE_GROUP"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    TARGETINGTYPEINVENTORYSOURCEGROUP,
    

    /// Purchase impressions from specific exchanges.
    ///
    /// "TARGETING_TYPE_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_EXCHANGE")]
    TARGETINGTYPEEXCHANGE,
    

    /// Purchase impressions from specific sub-exchanges.
    ///
    /// "TARGETING_TYPE_SUB_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_SUB_EXCHANGE")]
    TARGETINGTYPESUBEXCHANGE,
    

    /// Target ads around a specific point of interest, such as a notable building, a street address, or latitude/longitude coordinates.
    ///
    /// "TARGETING_TYPE_POI"
    #[serde(rename="TARGETING_TYPE_POI")]
    TARGETINGTYPEPOI,
    

    /// Target ads around locations of a business chain within a specific geo region.
    ///
    /// "TARGETING_TYPE_BUSINESS_CHAIN"
    #[serde(rename="TARGETING_TYPE_BUSINESS_CHAIN")]
    TARGETINGTYPEBUSINESSCHAIN,
    

    /// Target ads to a specific video content duration.
    ///
    /// "TARGETING_TYPE_CONTENT_DURATION"
    #[serde(rename="TARGETING_TYPE_CONTENT_DURATION")]
    TARGETINGTYPECONTENTDURATION,
    

    /// Target ads to a specific video content stream type.
    ///
    /// "TARGETING_TYPE_CONTENT_STREAM_TYPE"
    #[serde(rename="TARGETING_TYPE_CONTENT_STREAM_TYPE")]
    TARGETINGTYPECONTENTSTREAMTYPE,
    

    /// Target ads to a specific native content position.
    ///
    /// "TARGETING_TYPE_NATIVE_CONTENT_POSITION"
    #[serde(rename="TARGETING_TYPE_NATIVE_CONTENT_POSITION")]
    TARGETINGTYPENATIVECONTENTPOSITION,
    

    /// Target ads in an Open Measurement enabled inventory.
    ///
    /// "TARGETING_TYPE_OMID"
    #[serde(rename="TARGETING_TYPE_OMID")]
    TARGETINGTYPEOMID,
    

    /// Target ads to a specific audio content type.
    ///
    /// "TARGETING_TYPE_AUDIO_CONTENT_TYPE"
    #[serde(rename="TARGETING_TYPE_AUDIO_CONTENT_TYPE")]
    TARGETINGTYPEAUDIOCONTENTTYPE,
    

    /// Target ads to a specific content genre.
    ///
    /// "TARGETING_TYPE_CONTENT_GENRE"
    #[serde(rename="TARGETING_TYPE_CONTENT_GENRE")]
    TARGETINGTYPECONTENTGENRE,
}

impl AsRef<str> for AssignedTargetingOptionTargetingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED => "TARGETING_TYPE_UNSPECIFIED",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECHANNEL => "TARGETING_TYPE_CHANNEL",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY => "TARGETING_TYPE_APP_CATEGORY",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAPP => "TARGETING_TYPE_APP",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEURL => "TARGETING_TYPE_URL",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEDAYANDTIME => "TARGETING_TYPE_DAY_AND_TIME",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAGERANGE => "TARGETING_TYPE_AGE_RANGE",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST => "TARGETING_TYPE_REGIONAL_LOCATION_LIST",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST => "TARGETING_TYPE_PROXIMITY_LOCATION_LIST",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEGENDER => "TARGETING_TYPE_GENDER",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE => "TARGETING_TYPE_VIDEO_PLAYER_SIZE",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT => "TARGETING_TYPE_USER_REWARDED_CONTENT",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS => "TARGETING_TYPE_PARENTAL_STATUS",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION => "TARGETING_TYPE_CONTENT_INSTREAM_POSITION",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION => "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEDEVICETYPE => "TARGETING_TYPE_DEVICE_TYPE",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP => "TARGETING_TYPE_AUDIENCE_GROUP",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEBROWSER => "TARGETING_TYPE_BROWSER",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME => "TARGETING_TYPE_HOUSEHOLD_INCOME",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION => "TARGETING_TYPE_ON_SCREEN_POSITION",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER => "TARGETING_TYPE_THIRD_PARTY_VERIFIER",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION => "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION => "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEENVIRONMENT => "TARGETING_TYPE_ENVIRONMENT",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECARRIERANDISP => "TARGETING_TYPE_CARRIER_AND_ISP",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM => "TARGETING_TYPE_OPERATING_SYSTEM",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL => "TARGETING_TYPE_DEVICE_MAKE_MODEL",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEKEYWORD => "TARGETING_TYPE_KEYWORD",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST => "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEVIEWABILITY => "TARGETING_TYPE_VIEWABILITY",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECATEGORY => "TARGETING_TYPE_CATEGORY",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE => "TARGETING_TYPE_INVENTORY_SOURCE",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPELANGUAGE => "TARGETING_TYPE_LANGUAGE",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS => "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEGEOREGION => "TARGETING_TYPE_GEO_REGION",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP => "TARGETING_TYPE_INVENTORY_SOURCE_GROUP",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEEXCHANGE => "TARGETING_TYPE_EXCHANGE",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE => "TARGETING_TYPE_SUB_EXCHANGE",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEPOI => "TARGETING_TYPE_POI",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN => "TARGETING_TYPE_BUSINESS_CHAIN",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTDURATION => "TARGETING_TYPE_CONTENT_DURATION",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE => "TARGETING_TYPE_CONTENT_STREAM_TYPE",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION => "TARGETING_TYPE_NATIVE_CONTENT_POSITION",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEOMID => "TARGETING_TYPE_OMID",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE => "TARGETING_TYPE_AUDIO_CONTENT_TYPE",
            AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTGENRE => "TARGETING_TYPE_CONTENT_GENRE",
        }
    }
}

impl std::convert::TryFrom< &str> for AssignedTargetingOptionTargetingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_TYPE_UNSPECIFIED" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED),
           "TARGETING_TYPE_CHANNEL" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECHANNEL),
           "TARGETING_TYPE_APP_CATEGORY" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY),
           "TARGETING_TYPE_APP" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAPP),
           "TARGETING_TYPE_URL" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEURL),
           "TARGETING_TYPE_DAY_AND_TIME" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEDAYANDTIME),
           "TARGETING_TYPE_AGE_RANGE" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAGERANGE),
           "TARGETING_TYPE_REGIONAL_LOCATION_LIST" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST),
           "TARGETING_TYPE_PROXIMITY_LOCATION_LIST" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST),
           "TARGETING_TYPE_GENDER" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEGENDER),
           "TARGETING_TYPE_VIDEO_PLAYER_SIZE" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE),
           "TARGETING_TYPE_USER_REWARDED_CONTENT" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT),
           "TARGETING_TYPE_PARENTAL_STATUS" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS),
           "TARGETING_TYPE_CONTENT_INSTREAM_POSITION" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION),
           "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION),
           "TARGETING_TYPE_DEVICE_TYPE" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEDEVICETYPE),
           "TARGETING_TYPE_AUDIENCE_GROUP" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP),
           "TARGETING_TYPE_BROWSER" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEBROWSER),
           "TARGETING_TYPE_HOUSEHOLD_INCOME" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME),
           "TARGETING_TYPE_ON_SCREEN_POSITION" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION),
           "TARGETING_TYPE_THIRD_PARTY_VERIFIER" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER),
           "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION),
           "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION),
           "TARGETING_TYPE_ENVIRONMENT" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEENVIRONMENT),
           "TARGETING_TYPE_CARRIER_AND_ISP" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECARRIERANDISP),
           "TARGETING_TYPE_OPERATING_SYSTEM" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM),
           "TARGETING_TYPE_DEVICE_MAKE_MODEL" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL),
           "TARGETING_TYPE_KEYWORD" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEKEYWORD),
           "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST),
           "TARGETING_TYPE_VIEWABILITY" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEVIEWABILITY),
           "TARGETING_TYPE_CATEGORY" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECATEGORY),
           "TARGETING_TYPE_INVENTORY_SOURCE" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE),
           "TARGETING_TYPE_LANGUAGE" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPELANGUAGE),
           "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS),
           "TARGETING_TYPE_GEO_REGION" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEGEOREGION),
           "TARGETING_TYPE_INVENTORY_SOURCE_GROUP" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP),
           "TARGETING_TYPE_EXCHANGE" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEEXCHANGE),
           "TARGETING_TYPE_SUB_EXCHANGE" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE),
           "TARGETING_TYPE_POI" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEPOI),
           "TARGETING_TYPE_BUSINESS_CHAIN" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN),
           "TARGETING_TYPE_CONTENT_DURATION" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTDURATION),
           "TARGETING_TYPE_CONTENT_STREAM_TYPE" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE),
           "TARGETING_TYPE_NATIVE_CONTENT_POSITION" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION),
           "TARGETING_TYPE_OMID" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEOMID),
           "TARGETING_TYPE_AUDIO_CONTENT_TYPE" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE),
           "TARGETING_TYPE_CONTENT_GENRE" => Ok(AssignedTargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTGENRE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssignedTargetingOptionTargetingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AssignedUserRoleUserRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The user role to assign to a user for the entity.
pub enum AssignedUserRoleUserRoleEnum {
    

    /// Default value when the user role is not specified or is unknown in this version.
    ///
    /// "USER_ROLE_UNSPECIFIED"
    #[serde(rename="USER_ROLE_UNSPECIFIED")]
    USERROLEUNSPECIFIED,
    

    /// The user can manage campaigns, creatives, insertion orders, line items, and reports for the entity. They can view and edit billing information, create or modify users, and enable or disable exchanges. This role can only be assigned for a partner entity.
    ///
    /// "ADMIN"
    #[serde(rename="ADMIN")]
    ADMIN,
    

    /// The user can manage campaigns, creatives, insertion orders, line items, and reports for the entity. They can create and modify other `ADMIN_PARTNER_CLIENT` users and view billing information. They cannot view revenue models, markups, or any other reseller-sensitive fields. This role can only be assigned for a partner entity.
    ///
    /// "ADMIN_PARTNER_CLIENT"
    #[serde(rename="ADMIN_PARTNER_CLIENT")]
    ADMINPARTNERCLIENT,
    

    /// The user can manage campaigns, creatives, insertion orders, line items, and reports for the entity. They cannot create and modify users or view billing information.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// The user can view all campaigns, creatives, insertion orders, line items, and reports for the entity, including all cost data. They can create and modify planning-related features, including plans and inventory.
    ///
    /// "STANDARD_PLANNER"
    #[serde(rename="STANDARD_PLANNER")]
    STANDARDPLANNER,
    

    /// The user can view all campaigns, creatives, insertion orders, line items, and reports for the entity. They can create or modify planning-related features, including plans and inventory. They have no access to cost data and cannot start, accept, or negotiate deals.
    ///
    /// "STANDARD_PLANNER_LIMITED"
    #[serde(rename="STANDARD_PLANNER_LIMITED")]
    STANDARDPLANNERLIMITED,
    

    /// The user can manage campaigns, creatives, insertion orders, line items, and reports for the entity. They cannot create or modify other users or view billing information. They cannot view revenue models, markups, or any other reseller-sensitive fields. This role can only be assigned for an advertiser entity.
    ///
    /// "STANDARD_PARTNER_CLIENT"
    #[serde(rename="STANDARD_PARTNER_CLIENT")]
    STANDARDPARTNERCLIENT,
    

    /// The user can only build reports and view data for the entity.
    ///
    /// "READ_ONLY"
    #[serde(rename="READ_ONLY")]
    READONLY,
    

    /// The user can only create and manage reports.
    ///
    /// "REPORTING_ONLY"
    #[serde(rename="REPORTING_ONLY")]
    REPORTINGONLY,
    

    /// The user can only create and manage the following client-safe reports: General, Audience Performance, Cross-Partner, Keyword, Order ID, Category, and Third-Party Data Provider.
    ///
    /// "LIMITED_REPORTING_ONLY"
    #[serde(rename="LIMITED_REPORTING_ONLY")]
    LIMITEDREPORTINGONLY,
    

    /// The user can view media plan information they need to collaborate, but can't view cost-related data or Marketplace.
    ///
    /// "CREATIVE"
    #[serde(rename="CREATIVE")]
    CREATIVE,
    

    /// The user can view media plan information they need to collaborate, but can't view cost-related data or Marketplace. In addition, they can add other creative admins or creative users to the entity.
    ///
    /// "CREATIVE_ADMIN"
    #[serde(rename="CREATIVE_ADMIN")]
    CREATIVEADMIN,
}

impl AsRef<str> for AssignedUserRoleUserRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssignedUserRoleUserRoleEnum::USERROLEUNSPECIFIED => "USER_ROLE_UNSPECIFIED",
            AssignedUserRoleUserRoleEnum::ADMIN => "ADMIN",
            AssignedUserRoleUserRoleEnum::ADMINPARTNERCLIENT => "ADMIN_PARTNER_CLIENT",
            AssignedUserRoleUserRoleEnum::STANDARD => "STANDARD",
            AssignedUserRoleUserRoleEnum::STANDARDPLANNER => "STANDARD_PLANNER",
            AssignedUserRoleUserRoleEnum::STANDARDPLANNERLIMITED => "STANDARD_PLANNER_LIMITED",
            AssignedUserRoleUserRoleEnum::STANDARDPARTNERCLIENT => "STANDARD_PARTNER_CLIENT",
            AssignedUserRoleUserRoleEnum::READONLY => "READ_ONLY",
            AssignedUserRoleUserRoleEnum::REPORTINGONLY => "REPORTING_ONLY",
            AssignedUserRoleUserRoleEnum::LIMITEDREPORTINGONLY => "LIMITED_REPORTING_ONLY",
            AssignedUserRoleUserRoleEnum::CREATIVE => "CREATIVE",
            AssignedUserRoleUserRoleEnum::CREATIVEADMIN => "CREATIVE_ADMIN",
        }
    }
}

impl std::convert::TryFrom< &str> for AssignedUserRoleUserRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_ROLE_UNSPECIFIED" => Ok(AssignedUserRoleUserRoleEnum::USERROLEUNSPECIFIED),
           "ADMIN" => Ok(AssignedUserRoleUserRoleEnum::ADMIN),
           "ADMIN_PARTNER_CLIENT" => Ok(AssignedUserRoleUserRoleEnum::ADMINPARTNERCLIENT),
           "STANDARD" => Ok(AssignedUserRoleUserRoleEnum::STANDARD),
           "STANDARD_PLANNER" => Ok(AssignedUserRoleUserRoleEnum::STANDARDPLANNER),
           "STANDARD_PLANNER_LIMITED" => Ok(AssignedUserRoleUserRoleEnum::STANDARDPLANNERLIMITED),
           "STANDARD_PARTNER_CLIENT" => Ok(AssignedUserRoleUserRoleEnum::STANDARDPARTNERCLIENT),
           "READ_ONLY" => Ok(AssignedUserRoleUserRoleEnum::READONLY),
           "REPORTING_ONLY" => Ok(AssignedUserRoleUserRoleEnum::REPORTINGONLY),
           "LIMITED_REPORTING_ONLY" => Ok(AssignedUserRoleUserRoleEnum::LIMITEDREPORTINGONLY),
           "CREATIVE" => Ok(AssignedUserRoleUserRoleEnum::CREATIVE),
           "CREATIVE_ADMIN" => Ok(AssignedUserRoleUserRoleEnum::CREATIVEADMIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssignedUserRoleUserRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The audio content type.
pub enum AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum {
    

    /// Audio content type is not specified in this version. This enum is a place holder for a default value and does not represent a real content stream type.
    ///
    /// "AUDIO_CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="AUDIO_CONTENT_TYPE_UNSPECIFIED")]
    AUDIOCONTENTTYPEUNSPECIFIED,
    

    /// The audio content type is unknown.
    ///
    /// "AUDIO_CONTENT_TYPE_UNKNOWN"
    #[serde(rename="AUDIO_CONTENT_TYPE_UNKNOWN")]
    AUDIOCONTENTTYPEUNKNOWN,
    

    /// The audio content type is music.
    ///
    /// "AUDIO_CONTENT_TYPE_MUSIC"
    #[serde(rename="AUDIO_CONTENT_TYPE_MUSIC")]
    AUDIOCONTENTTYPEMUSIC,
    

    /// The audio content type is broadcast.
    ///
    /// "AUDIO_CONTENT_TYPE_BROADCAST"
    #[serde(rename="AUDIO_CONTENT_TYPE_BROADCAST")]
    AUDIOCONTENTTYPEBROADCAST,
    

    /// The audio content type is podcast.
    ///
    /// "AUDIO_CONTENT_TYPE_PODCAST"
    #[serde(rename="AUDIO_CONTENT_TYPE_PODCAST")]
    AUDIOCONTENTTYPEPODCAST,
}

impl AsRef<str> for AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEUNSPECIFIED => "AUDIO_CONTENT_TYPE_UNSPECIFIED",
            AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEUNKNOWN => "AUDIO_CONTENT_TYPE_UNKNOWN",
            AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEMUSIC => "AUDIO_CONTENT_TYPE_MUSIC",
            AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEBROADCAST => "AUDIO_CONTENT_TYPE_BROADCAST",
            AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEPODCAST => "AUDIO_CONTENT_TYPE_PODCAST",
        }
    }
}

impl std::convert::TryFrom< &str> for AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUDIO_CONTENT_TYPE_UNSPECIFIED" => Ok(AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEUNSPECIFIED),
           "AUDIO_CONTENT_TYPE_UNKNOWN" => Ok(AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEUNKNOWN),
           "AUDIO_CONTENT_TYPE_MUSIC" => Ok(AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEMUSIC),
           "AUDIO_CONTENT_TYPE_BROADCAST" => Ok(AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEBROADCAST),
           "AUDIO_CONTENT_TYPE_PODCAST" => Ok(AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEPODCAST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AudioContentTypeAssignedTargetingOptionDetailAudioContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AudioContentTypeTargetingOptionDetailAudioContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The audio content type.
pub enum AudioContentTypeTargetingOptionDetailAudioContentTypeEnum {
    

    /// Audio content type is not specified in this version. This enum is a place holder for a default value and does not represent a real content stream type.
    ///
    /// "AUDIO_CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="AUDIO_CONTENT_TYPE_UNSPECIFIED")]
    AUDIOCONTENTTYPEUNSPECIFIED,
    

    /// The audio content type is unknown.
    ///
    /// "AUDIO_CONTENT_TYPE_UNKNOWN"
    #[serde(rename="AUDIO_CONTENT_TYPE_UNKNOWN")]
    AUDIOCONTENTTYPEUNKNOWN,
    

    /// The audio content type is music.
    ///
    /// "AUDIO_CONTENT_TYPE_MUSIC"
    #[serde(rename="AUDIO_CONTENT_TYPE_MUSIC")]
    AUDIOCONTENTTYPEMUSIC,
    

    /// The audio content type is broadcast.
    ///
    /// "AUDIO_CONTENT_TYPE_BROADCAST"
    #[serde(rename="AUDIO_CONTENT_TYPE_BROADCAST")]
    AUDIOCONTENTTYPEBROADCAST,
    

    /// The audio content type is podcast.
    ///
    /// "AUDIO_CONTENT_TYPE_PODCAST"
    #[serde(rename="AUDIO_CONTENT_TYPE_PODCAST")]
    AUDIOCONTENTTYPEPODCAST,
}

impl AsRef<str> for AudioContentTypeTargetingOptionDetailAudioContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AudioContentTypeTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEUNSPECIFIED => "AUDIO_CONTENT_TYPE_UNSPECIFIED",
            AudioContentTypeTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEUNKNOWN => "AUDIO_CONTENT_TYPE_UNKNOWN",
            AudioContentTypeTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEMUSIC => "AUDIO_CONTENT_TYPE_MUSIC",
            AudioContentTypeTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEBROADCAST => "AUDIO_CONTENT_TYPE_BROADCAST",
            AudioContentTypeTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEPODCAST => "AUDIO_CONTENT_TYPE_PODCAST",
        }
    }
}

impl std::convert::TryFrom< &str> for AudioContentTypeTargetingOptionDetailAudioContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUDIO_CONTENT_TYPE_UNSPECIFIED" => Ok(AudioContentTypeTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEUNSPECIFIED),
           "AUDIO_CONTENT_TYPE_UNKNOWN" => Ok(AudioContentTypeTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEUNKNOWN),
           "AUDIO_CONTENT_TYPE_MUSIC" => Ok(AudioContentTypeTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEMUSIC),
           "AUDIO_CONTENT_TYPE_BROADCAST" => Ok(AudioContentTypeTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEBROADCAST),
           "AUDIO_CONTENT_TYPE_PODCAST" => Ok(AudioContentTypeTargetingOptionDetailAudioContentTypeEnum::AUDIOCONTENTTYPEPODCAST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AudioContentTypeTargetingOptionDetailAudioContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The authorized seller status to target.
pub enum AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum {
    

    /// Default value when authorized seller status is not specified in this version. This enum is a placeholder for the default value, or "Authorized Direct Sellers and Resellers" in the UI.
    ///
    /// "AUTHORIZED_SELLER_STATUS_UNSPECIFIED"
    #[serde(rename="AUTHORIZED_SELLER_STATUS_UNSPECIFIED")]
    AUTHORIZEDSELLERSTATUSUNSPECIFIED,
    

    /// Only authorized sellers that directly own the inventory being monetized, as indicated by a DIRECT declaration in the ads.txt file. This value is equivalent to "Authorized Direct Sellers" in the UI.
    ///
    /// "AUTHORIZED_SELLER_STATUS_AUTHORIZED_DIRECT_SELLERS_ONLY"
    #[serde(rename="AUTHORIZED_SELLER_STATUS_AUTHORIZED_DIRECT_SELLERS_ONLY")]
    AUTHORIZEDSELLERSTATUSAUTHORIZEDDIRECTSELLERSONLY,
    

    /// All authorized sellers, including publishers that have not posted an ads.txt file. Display & Video 360 automatically disallows unauthorized sellers. This value is equivalent to "Authorized and Non-Participating Publishers" in the UI.
    ///
    /// "AUTHORIZED_SELLER_STATUS_AUTHORIZED_AND_NON_PARTICIPATING_PUBLISHERS"
    #[serde(rename="AUTHORIZED_SELLER_STATUS_AUTHORIZED_AND_NON_PARTICIPATING_PUBLISHERS")]
    AUTHORIZEDSELLERSTATUSAUTHORIZEDANDNONPARTICIPATINGPUBLISHERS,
}

impl AsRef<str> for AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSUNSPECIFIED => "AUTHORIZED_SELLER_STATUS_UNSPECIFIED",
            AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSAUTHORIZEDDIRECTSELLERSONLY => "AUTHORIZED_SELLER_STATUS_AUTHORIZED_DIRECT_SELLERS_ONLY",
            AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSAUTHORIZEDANDNONPARTICIPATINGPUBLISHERS => "AUTHORIZED_SELLER_STATUS_AUTHORIZED_AND_NON_PARTICIPATING_PUBLISHERS",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTHORIZED_SELLER_STATUS_UNSPECIFIED" => Ok(AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSUNSPECIFIED),
           "AUTHORIZED_SELLER_STATUS_AUTHORIZED_DIRECT_SELLERS_ONLY" => Ok(AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSAUTHORIZEDDIRECTSELLERSONLY),
           "AUTHORIZED_SELLER_STATUS_AUTHORIZED_AND_NON_PARTICIPATING_PUBLISHERS" => Ok(AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSAUTHORIZEDANDNONPARTICIPATINGPUBLISHERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthorizedSellerStatusAssignedTargetingOptionDetailAuthorizedSellerStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The authorized seller status.
pub enum AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum {
    

    /// Default value when authorized seller status is not specified in this version. This enum is a placeholder for the default value, or "Authorized Direct Sellers and Resellers" in the UI.
    ///
    /// "AUTHORIZED_SELLER_STATUS_UNSPECIFIED"
    #[serde(rename="AUTHORIZED_SELLER_STATUS_UNSPECIFIED")]
    AUTHORIZEDSELLERSTATUSUNSPECIFIED,
    

    /// Only authorized sellers that directly own the inventory being monetized, as indicated by a DIRECT declaration in the ads.txt file. This value is equivalent to "Authorized Direct Sellers" in the UI.
    ///
    /// "AUTHORIZED_SELLER_STATUS_AUTHORIZED_DIRECT_SELLERS_ONLY"
    #[serde(rename="AUTHORIZED_SELLER_STATUS_AUTHORIZED_DIRECT_SELLERS_ONLY")]
    AUTHORIZEDSELLERSTATUSAUTHORIZEDDIRECTSELLERSONLY,
    

    /// All authorized sellers, including publishers that have not posted an ads.txt file. Display & Video 360 automatically disallows unauthorized sellers. This value is equivalent to "Authorized and Non-Participating Publishers" in the UI.
    ///
    /// "AUTHORIZED_SELLER_STATUS_AUTHORIZED_AND_NON_PARTICIPATING_PUBLISHERS"
    #[serde(rename="AUTHORIZED_SELLER_STATUS_AUTHORIZED_AND_NON_PARTICIPATING_PUBLISHERS")]
    AUTHORIZEDSELLERSTATUSAUTHORIZEDANDNONPARTICIPATINGPUBLISHERS,
}

impl AsRef<str> for AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSUNSPECIFIED => "AUTHORIZED_SELLER_STATUS_UNSPECIFIED",
            AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSAUTHORIZEDDIRECTSELLERSONLY => "AUTHORIZED_SELLER_STATUS_AUTHORIZED_DIRECT_SELLERS_ONLY",
            AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSAUTHORIZEDANDNONPARTICIPATINGPUBLISHERS => "AUTHORIZED_SELLER_STATUS_AUTHORIZED_AND_NON_PARTICIPATING_PUBLISHERS",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTHORIZED_SELLER_STATUS_UNSPECIFIED" => Ok(AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSUNSPECIFIED),
           "AUTHORIZED_SELLER_STATUS_AUTHORIZED_DIRECT_SELLERS_ONLY" => Ok(AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSAUTHORIZEDDIRECTSELLERSONLY),
           "AUTHORIZED_SELLER_STATUS_AUTHORIZED_AND_NON_PARTICIPATING_PUBLISHERS" => Ok(AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum::AUTHORIZEDSELLERSTATUSAUTHORIZEDANDNONPARTICIPATINGPUBLISHERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthorizedSellerStatusTargetingOptionDetailAuthorizedSellerStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The unit of distance by which the targeting radius is measured.
pub enum BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "DISTANCE_UNIT_UNSPECIFIED"
    #[serde(rename="DISTANCE_UNIT_UNSPECIFIED")]
    DISTANCEUNITUNSPECIFIED,
    

    /// Miles.
    ///
    /// "DISTANCE_UNIT_MILES"
    #[serde(rename="DISTANCE_UNIT_MILES")]
    DISTANCEUNITMILES,
    

    /// Kilometers.
    ///
    /// "DISTANCE_UNIT_KILOMETERS"
    #[serde(rename="DISTANCE_UNIT_KILOMETERS")]
    DISTANCEUNITKILOMETERS,
}

impl AsRef<str> for BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITUNSPECIFIED => "DISTANCE_UNIT_UNSPECIFIED",
            BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITMILES => "DISTANCE_UNIT_MILES",
            BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITKILOMETERS => "DISTANCE_UNIT_KILOMETERS",
        }
    }
}

impl std::convert::TryFrom< &str> for BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISTANCE_UNIT_UNSPECIFIED" => Ok(BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITUNSPECIFIED),
           "DISTANCE_UNIT_MILES" => Ok(BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITMILES),
           "DISTANCE_UNIT_KILOMETERS" => Ok(BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITKILOMETERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BusinessChainAssignedTargetingOptionDetailProximityRadiusUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BusinessChainTargetingOptionDetailGeoRegionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the geographic region.
pub enum BusinessChainTargetingOptionDetailGeoRegionTypeEnum {
    

    /// The geographic region type is unknown.
    ///
    /// "GEO_REGION_TYPE_UNKNOWN"
    #[serde(rename="GEO_REGION_TYPE_UNKNOWN")]
    GEOREGIONTYPEUNKNOWN,
    

    /// The geographic region type is other.
    ///
    /// "GEO_REGION_TYPE_OTHER"
    #[serde(rename="GEO_REGION_TYPE_OTHER")]
    GEOREGIONTYPEOTHER,
    

    /// The geographic region is a country.
    ///
    /// "GEO_REGION_TYPE_COUNTRY"
    #[serde(rename="GEO_REGION_TYPE_COUNTRY")]
    GEOREGIONTYPECOUNTRY,
    

    /// The geographic region type is region.
    ///
    /// "GEO_REGION_TYPE_REGION"
    #[serde(rename="GEO_REGION_TYPE_REGION")]
    GEOREGIONTYPEREGION,
    

    /// The geographic region is a territory.
    ///
    /// "GEO_REGION_TYPE_TERRITORY"
    #[serde(rename="GEO_REGION_TYPE_TERRITORY")]
    GEOREGIONTYPETERRITORY,
    

    /// The geographic region is a province.
    ///
    /// "GEO_REGION_TYPE_PROVINCE"
    #[serde(rename="GEO_REGION_TYPE_PROVINCE")]
    GEOREGIONTYPEPROVINCE,
    

    /// The geographic region is a state.
    ///
    /// "GEO_REGION_TYPE_STATE"
    #[serde(rename="GEO_REGION_TYPE_STATE")]
    GEOREGIONTYPESTATE,
    

    /// The geographic region is a prefecture.
    ///
    /// "GEO_REGION_TYPE_PREFECTURE"
    #[serde(rename="GEO_REGION_TYPE_PREFECTURE")]
    GEOREGIONTYPEPREFECTURE,
    

    /// The geographic region is a governorate.
    ///
    /// "GEO_REGION_TYPE_GOVERNORATE"
    #[serde(rename="GEO_REGION_TYPE_GOVERNORATE")]
    GEOREGIONTYPEGOVERNORATE,
    

    /// The geographic region is a canton.
    ///
    /// "GEO_REGION_TYPE_CANTON"
    #[serde(rename="GEO_REGION_TYPE_CANTON")]
    GEOREGIONTYPECANTON,
    

    /// The geographic region is a union territory.
    ///
    /// "GEO_REGION_TYPE_UNION_TERRITORY"
    #[serde(rename="GEO_REGION_TYPE_UNION_TERRITORY")]
    GEOREGIONTYPEUNIONTERRITORY,
    

    /// The geographic region is an autonomous community.
    ///
    /// "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY"
    #[serde(rename="GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY")]
    GEOREGIONTYPEAUTONOMOUSCOMMUNITY,
    

    /// The geographic region is a designated market area (DMA) region.
    ///
    /// "GEO_REGION_TYPE_DMA_REGION"
    #[serde(rename="GEO_REGION_TYPE_DMA_REGION")]
    GEOREGIONTYPEDMAREGION,
    

    /// The geographic region type is metro.
    ///
    /// "GEO_REGION_TYPE_METRO"
    #[serde(rename="GEO_REGION_TYPE_METRO")]
    GEOREGIONTYPEMETRO,
    

    /// The geographic region is a congressional district.
    ///
    /// "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT"
    #[serde(rename="GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT")]
    GEOREGIONTYPECONGRESSIONALDISTRICT,
    

    /// The geographic region is a county.
    ///
    /// "GEO_REGION_TYPE_COUNTY"
    #[serde(rename="GEO_REGION_TYPE_COUNTY")]
    GEOREGIONTYPECOUNTY,
    

    /// The geographic region is a municipality.
    ///
    /// "GEO_REGION_TYPE_MUNICIPALITY"
    #[serde(rename="GEO_REGION_TYPE_MUNICIPALITY")]
    GEOREGIONTYPEMUNICIPALITY,
    

    /// The geographic region is a city.
    ///
    /// "GEO_REGION_TYPE_CITY"
    #[serde(rename="GEO_REGION_TYPE_CITY")]
    GEOREGIONTYPECITY,
    

    /// The geographic region targeting type is postal code.
    ///
    /// "GEO_REGION_TYPE_POSTAL_CODE"
    #[serde(rename="GEO_REGION_TYPE_POSTAL_CODE")]
    GEOREGIONTYPEPOSTALCODE,
    

    /// The geographic region targeting type is department.
    ///
    /// "GEO_REGION_TYPE_DEPARTMENT"
    #[serde(rename="GEO_REGION_TYPE_DEPARTMENT")]
    GEOREGIONTYPEDEPARTMENT,
    

    /// The geographic region is an airport.
    ///
    /// "GEO_REGION_TYPE_AIRPORT"
    #[serde(rename="GEO_REGION_TYPE_AIRPORT")]
    GEOREGIONTYPEAIRPORT,
    

    /// The geographic region is a TV region.
    ///
    /// "GEO_REGION_TYPE_TV_REGION"
    #[serde(rename="GEO_REGION_TYPE_TV_REGION")]
    GEOREGIONTYPETVREGION,
    

    /// The geographic region is an okrug.
    ///
    /// "GEO_REGION_TYPE_OKRUG"
    #[serde(rename="GEO_REGION_TYPE_OKRUG")]
    GEOREGIONTYPEOKRUG,
    

    /// The geographic region is a borough.
    ///
    /// "GEO_REGION_TYPE_BOROUGH"
    #[serde(rename="GEO_REGION_TYPE_BOROUGH")]
    GEOREGIONTYPEBOROUGH,
    

    /// The geographic region is a city region.
    ///
    /// "GEO_REGION_TYPE_CITY_REGION"
    #[serde(rename="GEO_REGION_TYPE_CITY_REGION")]
    GEOREGIONTYPECITYREGION,
    

    /// The geographic region is an arrondissement.
    ///
    /// "GEO_REGION_TYPE_ARRONDISSEMENT"
    #[serde(rename="GEO_REGION_TYPE_ARRONDISSEMENT")]
    GEOREGIONTYPEARRONDISSEMENT,
    

    /// The geographic region is a neighborhood.
    ///
    /// "GEO_REGION_TYPE_NEIGHBORHOOD"
    #[serde(rename="GEO_REGION_TYPE_NEIGHBORHOOD")]
    GEOREGIONTYPENEIGHBORHOOD,
    

    /// The geographic region is a university.
    ///
    /// "GEO_REGION_TYPE_UNIVERSITY"
    #[serde(rename="GEO_REGION_TYPE_UNIVERSITY")]
    GEOREGIONTYPEUNIVERSITY,
    

    /// The geographic region is a district.
    ///
    /// "GEO_REGION_TYPE_DISTRICT"
    #[serde(rename="GEO_REGION_TYPE_DISTRICT")]
    GEOREGIONTYPEDISTRICT,
}

impl AsRef<str> for BusinessChainTargetingOptionDetailGeoRegionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNKNOWN => "GEO_REGION_TYPE_UNKNOWN",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOTHER => "GEO_REGION_TYPE_OTHER",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTRY => "GEO_REGION_TYPE_COUNTRY",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEREGION => "GEO_REGION_TYPE_REGION",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETERRITORY => "GEO_REGION_TYPE_TERRITORY",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPROVINCE => "GEO_REGION_TYPE_PROVINCE",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPESTATE => "GEO_REGION_TYPE_STATE",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPREFECTURE => "GEO_REGION_TYPE_PREFECTURE",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEGOVERNORATE => "GEO_REGION_TYPE_GOVERNORATE",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECANTON => "GEO_REGION_TYPE_CANTON",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIONTERRITORY => "GEO_REGION_TYPE_UNION_TERRITORY",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAUTONOMOUSCOMMUNITY => "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDMAREGION => "GEO_REGION_TYPE_DMA_REGION",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMETRO => "GEO_REGION_TYPE_METRO",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECONGRESSIONALDISTRICT => "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTY => "GEO_REGION_TYPE_COUNTY",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMUNICIPALITY => "GEO_REGION_TYPE_MUNICIPALITY",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITY => "GEO_REGION_TYPE_CITY",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPOSTALCODE => "GEO_REGION_TYPE_POSTAL_CODE",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDEPARTMENT => "GEO_REGION_TYPE_DEPARTMENT",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAIRPORT => "GEO_REGION_TYPE_AIRPORT",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETVREGION => "GEO_REGION_TYPE_TV_REGION",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOKRUG => "GEO_REGION_TYPE_OKRUG",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEBOROUGH => "GEO_REGION_TYPE_BOROUGH",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITYREGION => "GEO_REGION_TYPE_CITY_REGION",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEARRONDISSEMENT => "GEO_REGION_TYPE_ARRONDISSEMENT",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPENEIGHBORHOOD => "GEO_REGION_TYPE_NEIGHBORHOOD",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIVERSITY => "GEO_REGION_TYPE_UNIVERSITY",
            BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDISTRICT => "GEO_REGION_TYPE_DISTRICT",
        }
    }
}

impl std::convert::TryFrom< &str> for BusinessChainTargetingOptionDetailGeoRegionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GEO_REGION_TYPE_UNKNOWN" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNKNOWN),
           "GEO_REGION_TYPE_OTHER" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOTHER),
           "GEO_REGION_TYPE_COUNTRY" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTRY),
           "GEO_REGION_TYPE_REGION" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEREGION),
           "GEO_REGION_TYPE_TERRITORY" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETERRITORY),
           "GEO_REGION_TYPE_PROVINCE" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPROVINCE),
           "GEO_REGION_TYPE_STATE" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPESTATE),
           "GEO_REGION_TYPE_PREFECTURE" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPREFECTURE),
           "GEO_REGION_TYPE_GOVERNORATE" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEGOVERNORATE),
           "GEO_REGION_TYPE_CANTON" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECANTON),
           "GEO_REGION_TYPE_UNION_TERRITORY" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIONTERRITORY),
           "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAUTONOMOUSCOMMUNITY),
           "GEO_REGION_TYPE_DMA_REGION" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDMAREGION),
           "GEO_REGION_TYPE_METRO" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMETRO),
           "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECONGRESSIONALDISTRICT),
           "GEO_REGION_TYPE_COUNTY" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTY),
           "GEO_REGION_TYPE_MUNICIPALITY" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMUNICIPALITY),
           "GEO_REGION_TYPE_CITY" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITY),
           "GEO_REGION_TYPE_POSTAL_CODE" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPOSTALCODE),
           "GEO_REGION_TYPE_DEPARTMENT" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDEPARTMENT),
           "GEO_REGION_TYPE_AIRPORT" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAIRPORT),
           "GEO_REGION_TYPE_TV_REGION" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETVREGION),
           "GEO_REGION_TYPE_OKRUG" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOKRUG),
           "GEO_REGION_TYPE_BOROUGH" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEBOROUGH),
           "GEO_REGION_TYPE_CITY_REGION" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITYREGION),
           "GEO_REGION_TYPE_ARRONDISSEMENT" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEARRONDISSEMENT),
           "GEO_REGION_TYPE_NEIGHBORHOOD" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPENEIGHBORHOOD),
           "GEO_REGION_TYPE_UNIVERSITY" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIVERSITY),
           "GEO_REGION_TYPE_DISTRICT" => Ok(BusinessChainTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDISTRICT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BusinessChainTargetingOptionDetailGeoRegionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CampaignEntityStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Controls whether or not the insertion orders under this campaign can spend their budgets and bid on inventory. * Accepted values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_ARCHIVED`, and `ENTITY_STATUS_PAUSED`. * For CreateCampaign method, `ENTITY_STATUS_ARCHIVED` is not allowed.
pub enum CampaignEntityStatusEnum {
    

    /// Default value when status is not specified or is unknown in this version.
    ///
    /// "ENTITY_STATUS_UNSPECIFIED"
    #[serde(rename="ENTITY_STATUS_UNSPECIFIED")]
    ENTITYSTATUSUNSPECIFIED,
    

    /// The entity is enabled to bid and spend budget.
    ///
    /// "ENTITY_STATUS_ACTIVE"
    #[serde(rename="ENTITY_STATUS_ACTIVE")]
    ENTITYSTATUSACTIVE,
    

    /// The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved.
    ///
    /// "ENTITY_STATUS_ARCHIVED"
    #[serde(rename="ENTITY_STATUS_ARCHIVED")]
    ENTITYSTATUSARCHIVED,
    

    /// The entity is under draft. Bidding and budget spending are disabled.
    ///
    /// "ENTITY_STATUS_DRAFT"
    #[serde(rename="ENTITY_STATUS_DRAFT")]
    ENTITYSTATUSDRAFT,
    

    /// Bidding and budget spending are paused for the entity.
    ///
    /// "ENTITY_STATUS_PAUSED"
    #[serde(rename="ENTITY_STATUS_PAUSED")]
    ENTITYSTATUSPAUSED,
    

    /// The entity is scheduled for deletion.
    ///
    /// "ENTITY_STATUS_SCHEDULED_FOR_DELETION"
    #[serde(rename="ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    ENTITYSTATUSSCHEDULEDFORDELETION,
}

impl AsRef<str> for CampaignEntityStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CampaignEntityStatusEnum::ENTITYSTATUSUNSPECIFIED => "ENTITY_STATUS_UNSPECIFIED",
            CampaignEntityStatusEnum::ENTITYSTATUSACTIVE => "ENTITY_STATUS_ACTIVE",
            CampaignEntityStatusEnum::ENTITYSTATUSARCHIVED => "ENTITY_STATUS_ARCHIVED",
            CampaignEntityStatusEnum::ENTITYSTATUSDRAFT => "ENTITY_STATUS_DRAFT",
            CampaignEntityStatusEnum::ENTITYSTATUSPAUSED => "ENTITY_STATUS_PAUSED",
            CampaignEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION => "ENTITY_STATUS_SCHEDULED_FOR_DELETION",
        }
    }
}

impl std::convert::TryFrom< &str> for CampaignEntityStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_STATUS_UNSPECIFIED" => Ok(CampaignEntityStatusEnum::ENTITYSTATUSUNSPECIFIED),
           "ENTITY_STATUS_ACTIVE" => Ok(CampaignEntityStatusEnum::ENTITYSTATUSACTIVE),
           "ENTITY_STATUS_ARCHIVED" => Ok(CampaignEntityStatusEnum::ENTITYSTATUSARCHIVED),
           "ENTITY_STATUS_DRAFT" => Ok(CampaignEntityStatusEnum::ENTITYSTATUSDRAFT),
           "ENTITY_STATUS_PAUSED" => Ok(CampaignEntityStatusEnum::ENTITYSTATUSPAUSED),
           "ENTITY_STATUS_SCHEDULED_FOR_DELETION" => Ok(CampaignEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CampaignEntityStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CampaignBudgetBudgetUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. Specifies whether the budget is measured in currency or impressions.
pub enum CampaignBudgetBudgetUnitEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "BUDGET_UNIT_UNSPECIFIED"
    #[serde(rename="BUDGET_UNIT_UNSPECIFIED")]
    BUDGETUNITUNSPECIFIED,
    

    /// Budgeting in currency amounts.
    ///
    /// "BUDGET_UNIT_CURRENCY"
    #[serde(rename="BUDGET_UNIT_CURRENCY")]
    BUDGETUNITCURRENCY,
    

    /// Budgeting in impression amounts.
    ///
    /// "BUDGET_UNIT_IMPRESSIONS"
    #[serde(rename="BUDGET_UNIT_IMPRESSIONS")]
    BUDGETUNITIMPRESSIONS,
}

impl AsRef<str> for CampaignBudgetBudgetUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CampaignBudgetBudgetUnitEnum::BUDGETUNITUNSPECIFIED => "BUDGET_UNIT_UNSPECIFIED",
            CampaignBudgetBudgetUnitEnum::BUDGETUNITCURRENCY => "BUDGET_UNIT_CURRENCY",
            CampaignBudgetBudgetUnitEnum::BUDGETUNITIMPRESSIONS => "BUDGET_UNIT_IMPRESSIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for CampaignBudgetBudgetUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUDGET_UNIT_UNSPECIFIED" => Ok(CampaignBudgetBudgetUnitEnum::BUDGETUNITUNSPECIFIED),
           "BUDGET_UNIT_CURRENCY" => Ok(CampaignBudgetBudgetUnitEnum::BUDGETUNITCURRENCY),
           "BUDGET_UNIT_IMPRESSIONS" => Ok(CampaignBudgetBudgetUnitEnum::BUDGETUNITIMPRESSIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CampaignBudgetBudgetUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CampaignBudgetExternalBudgetSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The external source of the budget.
pub enum CampaignBudgetExternalBudgetSourceEnum {
    

    /// External budget source value is not specified or unknown in this version.
    ///
    /// "EXTERNAL_BUDGET_SOURCE_UNSPECIFIED"
    #[serde(rename="EXTERNAL_BUDGET_SOURCE_UNSPECIFIED")]
    EXTERNALBUDGETSOURCEUNSPECIFIED,
    

    /// Budget has no external source.
    ///
    /// "EXTERNAL_BUDGET_SOURCE_NONE"
    #[serde(rename="EXTERNAL_BUDGET_SOURCE_NONE")]
    EXTERNALBUDGETSOURCENONE,
    

    /// Budget source is Mediaocean.
    ///
    /// "EXTERNAL_BUDGET_SOURCE_MEDIA_OCEAN"
    #[serde(rename="EXTERNAL_BUDGET_SOURCE_MEDIA_OCEAN")]
    EXTERNALBUDGETSOURCEMEDIAOCEAN,
}

impl AsRef<str> for CampaignBudgetExternalBudgetSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CampaignBudgetExternalBudgetSourceEnum::EXTERNALBUDGETSOURCEUNSPECIFIED => "EXTERNAL_BUDGET_SOURCE_UNSPECIFIED",
            CampaignBudgetExternalBudgetSourceEnum::EXTERNALBUDGETSOURCENONE => "EXTERNAL_BUDGET_SOURCE_NONE",
            CampaignBudgetExternalBudgetSourceEnum::EXTERNALBUDGETSOURCEMEDIAOCEAN => "EXTERNAL_BUDGET_SOURCE_MEDIA_OCEAN",
        }
    }
}

impl std::convert::TryFrom< &str> for CampaignBudgetExternalBudgetSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXTERNAL_BUDGET_SOURCE_UNSPECIFIED" => Ok(CampaignBudgetExternalBudgetSourceEnum::EXTERNALBUDGETSOURCEUNSPECIFIED),
           "EXTERNAL_BUDGET_SOURCE_NONE" => Ok(CampaignBudgetExternalBudgetSourceEnum::EXTERNALBUDGETSOURCENONE),
           "EXTERNAL_BUDGET_SOURCE_MEDIA_OCEAN" => Ok(CampaignBudgetExternalBudgetSourceEnum::EXTERNALBUDGETSOURCEMEDIAOCEAN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CampaignBudgetExternalBudgetSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CampaignGoalCampaignGoalTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the campaign goal.
pub enum CampaignGoalCampaignGoalTypeEnum {
    

    /// Goal value is not specified or unknown in this version.
    ///
    /// "CAMPAIGN_GOAL_TYPE_UNSPECIFIED"
    #[serde(rename="CAMPAIGN_GOAL_TYPE_UNSPECIFIED")]
    CAMPAIGNGOALTYPEUNSPECIFIED,
    

    /// Drive app installs or engagements.
    ///
    /// "CAMPAIGN_GOAL_TYPE_APP_INSTALL"
    #[serde(rename="CAMPAIGN_GOAL_TYPE_APP_INSTALL")]
    CAMPAIGNGOALTYPEAPPINSTALL,
    

    /// Raise awareness of a brand or product.
    ///
    /// "CAMPAIGN_GOAL_TYPE_BRAND_AWARENESS"
    #[serde(rename="CAMPAIGN_GOAL_TYPE_BRAND_AWARENESS")]
    CAMPAIGNGOALTYPEBRANDAWARENESS,
    

    /// Drive offline or in-store sales.
    ///
    /// "CAMPAIGN_GOAL_TYPE_OFFLINE_ACTION"
    #[serde(rename="CAMPAIGN_GOAL_TYPE_OFFLINE_ACTION")]
    CAMPAIGNGOALTYPEOFFLINEACTION,
    

    /// Drive online action or visits.
    ///
    /// "CAMPAIGN_GOAL_TYPE_ONLINE_ACTION"
    #[serde(rename="CAMPAIGN_GOAL_TYPE_ONLINE_ACTION")]
    CAMPAIGNGOALTYPEONLINEACTION,
}

impl AsRef<str> for CampaignGoalCampaignGoalTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CampaignGoalCampaignGoalTypeEnum::CAMPAIGNGOALTYPEUNSPECIFIED => "CAMPAIGN_GOAL_TYPE_UNSPECIFIED",
            CampaignGoalCampaignGoalTypeEnum::CAMPAIGNGOALTYPEAPPINSTALL => "CAMPAIGN_GOAL_TYPE_APP_INSTALL",
            CampaignGoalCampaignGoalTypeEnum::CAMPAIGNGOALTYPEBRANDAWARENESS => "CAMPAIGN_GOAL_TYPE_BRAND_AWARENESS",
            CampaignGoalCampaignGoalTypeEnum::CAMPAIGNGOALTYPEOFFLINEACTION => "CAMPAIGN_GOAL_TYPE_OFFLINE_ACTION",
            CampaignGoalCampaignGoalTypeEnum::CAMPAIGNGOALTYPEONLINEACTION => "CAMPAIGN_GOAL_TYPE_ONLINE_ACTION",
        }
    }
}

impl std::convert::TryFrom< &str> for CampaignGoalCampaignGoalTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CAMPAIGN_GOAL_TYPE_UNSPECIFIED" => Ok(CampaignGoalCampaignGoalTypeEnum::CAMPAIGNGOALTYPEUNSPECIFIED),
           "CAMPAIGN_GOAL_TYPE_APP_INSTALL" => Ok(CampaignGoalCampaignGoalTypeEnum::CAMPAIGNGOALTYPEAPPINSTALL),
           "CAMPAIGN_GOAL_TYPE_BRAND_AWARENESS" => Ok(CampaignGoalCampaignGoalTypeEnum::CAMPAIGNGOALTYPEBRANDAWARENESS),
           "CAMPAIGN_GOAL_TYPE_OFFLINE_ACTION" => Ok(CampaignGoalCampaignGoalTypeEnum::CAMPAIGNGOALTYPEOFFLINEACTION),
           "CAMPAIGN_GOAL_TYPE_ONLINE_ACTION" => Ok(CampaignGoalCampaignGoalTypeEnum::CAMPAIGNGOALTYPEONLINEACTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CampaignGoalCampaignGoalTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CarrierAndIspTargetingOptionDetailTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type indicating if it's carrier or ISP.
pub enum CarrierAndIspTargetingOptionDetailTypeEnum {
    

    /// Default value when type is not specified or is unknown in this version.
    ///
    /// "CARRIER_AND_ISP_TYPE_UNSPECIFIED"
    #[serde(rename="CARRIER_AND_ISP_TYPE_UNSPECIFIED")]
    CARRIERANDISPTYPEUNSPECIFIED,
    

    /// Indicates this targeting resource refers to an ISP.
    ///
    /// "CARRIER_AND_ISP_TYPE_ISP"
    #[serde(rename="CARRIER_AND_ISP_TYPE_ISP")]
    CARRIERANDISPTYPEISP,
    

    /// Indicates this targeting resource refers to a mobile carrier.
    ///
    /// "CARRIER_AND_ISP_TYPE_CARRIER"
    #[serde(rename="CARRIER_AND_ISP_TYPE_CARRIER")]
    CARRIERANDISPTYPECARRIER,
}

impl AsRef<str> for CarrierAndIspTargetingOptionDetailTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CarrierAndIspTargetingOptionDetailTypeEnum::CARRIERANDISPTYPEUNSPECIFIED => "CARRIER_AND_ISP_TYPE_UNSPECIFIED",
            CarrierAndIspTargetingOptionDetailTypeEnum::CARRIERANDISPTYPEISP => "CARRIER_AND_ISP_TYPE_ISP",
            CarrierAndIspTargetingOptionDetailTypeEnum::CARRIERANDISPTYPECARRIER => "CARRIER_AND_ISP_TYPE_CARRIER",
        }
    }
}

impl std::convert::TryFrom< &str> for CarrierAndIspTargetingOptionDetailTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CARRIER_AND_ISP_TYPE_UNSPECIFIED" => Ok(CarrierAndIspTargetingOptionDetailTypeEnum::CARRIERANDISPTYPEUNSPECIFIED),
           "CARRIER_AND_ISP_TYPE_ISP" => Ok(CarrierAndIspTargetingOptionDetailTypeEnum::CARRIERANDISPTYPEISP),
           "CARRIER_AND_ISP_TYPE_CARRIER" => Ok(CarrierAndIspTargetingOptionDetailTypeEnum::CARRIERANDISPTYPECARRIER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CarrierAndIspTargetingOptionDetailTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConsentAdPersonalizationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Represents consent for ad personalization.
pub enum ConsentAdPersonalizationEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "CONSENT_STATUS_UNSPECIFIED"
    #[serde(rename="CONSENT_STATUS_UNSPECIFIED")]
    CONSENTSTATUSUNSPECIFIED,
    

    /// Consent is granted.
    ///
    /// "CONSENT_STATUS_GRANTED"
    #[serde(rename="CONSENT_STATUS_GRANTED")]
    CONSENTSTATUSGRANTED,
    

    /// Consent is denied.
    ///
    /// "CONSENT_STATUS_DENIED"
    #[serde(rename="CONSENT_STATUS_DENIED")]
    CONSENTSTATUSDENIED,
}

impl AsRef<str> for ConsentAdPersonalizationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConsentAdPersonalizationEnum::CONSENTSTATUSUNSPECIFIED => "CONSENT_STATUS_UNSPECIFIED",
            ConsentAdPersonalizationEnum::CONSENTSTATUSGRANTED => "CONSENT_STATUS_GRANTED",
            ConsentAdPersonalizationEnum::CONSENTSTATUSDENIED => "CONSENT_STATUS_DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for ConsentAdPersonalizationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONSENT_STATUS_UNSPECIFIED" => Ok(ConsentAdPersonalizationEnum::CONSENTSTATUSUNSPECIFIED),
           "CONSENT_STATUS_GRANTED" => Ok(ConsentAdPersonalizationEnum::CONSENTSTATUSGRANTED),
           "CONSENT_STATUS_DENIED" => Ok(ConsentAdPersonalizationEnum::CONSENTSTATUSDENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConsentAdPersonalizationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConsentAdUserDataEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Represents consent for ad user data.
pub enum ConsentAdUserDataEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "CONSENT_STATUS_UNSPECIFIED"
    #[serde(rename="CONSENT_STATUS_UNSPECIFIED")]
    CONSENTSTATUSUNSPECIFIED,
    

    /// Consent is granted.
    ///
    /// "CONSENT_STATUS_GRANTED"
    #[serde(rename="CONSENT_STATUS_GRANTED")]
    CONSENTSTATUSGRANTED,
    

    /// Consent is denied.
    ///
    /// "CONSENT_STATUS_DENIED"
    #[serde(rename="CONSENT_STATUS_DENIED")]
    CONSENTSTATUSDENIED,
}

impl AsRef<str> for ConsentAdUserDataEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConsentAdUserDataEnum::CONSENTSTATUSUNSPECIFIED => "CONSENT_STATUS_UNSPECIFIED",
            ConsentAdUserDataEnum::CONSENTSTATUSGRANTED => "CONSENT_STATUS_GRANTED",
            ConsentAdUserDataEnum::CONSENTSTATUSDENIED => "CONSENT_STATUS_DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for ConsentAdUserDataEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONSENT_STATUS_UNSPECIFIED" => Ok(ConsentAdUserDataEnum::CONSENTSTATUSUNSPECIFIED),
           "CONSENT_STATUS_GRANTED" => Ok(ConsentAdUserDataEnum::CONSENTSTATUSGRANTED),
           "CONSENT_STATUS_DENIED" => Ok(ConsentAdUserDataEnum::CONSENTSTATUSDENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConsentAdUserDataEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentDurationAssignedTargetingOptionDetailContentDurationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The content duration.
pub enum ContentDurationAssignedTargetingOptionDetailContentDurationEnum {
    

    /// Content duration is not specified in this version. This enum is a place holder for a default value and does not represent a real content duration.
    ///
    /// "CONTENT_DURATION_UNSPECIFIED"
    #[serde(rename="CONTENT_DURATION_UNSPECIFIED")]
    CONTENTDURATIONUNSPECIFIED,
    

    /// The content duration is unknown.
    ///
    /// "CONTENT_DURATION_UNKNOWN"
    #[serde(rename="CONTENT_DURATION_UNKNOWN")]
    CONTENTDURATIONUNKNOWN,
    

    /// Content is 0-1 minute long.
    ///
    /// "CONTENT_DURATION_0_TO_1_MIN"
    #[serde(rename="CONTENT_DURATION_0_TO_1_MIN")]
    CONTENTDURATION0TO1MIN,
    

    /// Content is 1-5 minutes long.
    ///
    /// "CONTENT_DURATION_1_TO_5_MIN"
    #[serde(rename="CONTENT_DURATION_1_TO_5_MIN")]
    CONTENTDURATION1TO5MIN,
    

    /// Content is 5-15 minutes long.
    ///
    /// "CONTENT_DURATION_5_TO_15_MIN"
    #[serde(rename="CONTENT_DURATION_5_TO_15_MIN")]
    CONTENTDURATION5TO15MIN,
    

    /// Content is 15-30 minutes long.
    ///
    /// "CONTENT_DURATION_15_TO_30_MIN"
    #[serde(rename="CONTENT_DURATION_15_TO_30_MIN")]
    CONTENTDURATION15TO30MIN,
    

    /// Content is 30-60 minutes long.
    ///
    /// "CONTENT_DURATION_30_TO_60_MIN"
    #[serde(rename="CONTENT_DURATION_30_TO_60_MIN")]
    CONTENTDURATION30TO60MIN,
    

    /// Content is over 60 minutes long.
    ///
    /// "CONTENT_DURATION_OVER_60_MIN"
    #[serde(rename="CONTENT_DURATION_OVER_60_MIN")]
    CONTENTDURATIONOVER60MIN,
}

impl AsRef<str> for ContentDurationAssignedTargetingOptionDetailContentDurationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATIONUNSPECIFIED => "CONTENT_DURATION_UNSPECIFIED",
            ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATIONUNKNOWN => "CONTENT_DURATION_UNKNOWN",
            ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATION0TO1MIN => "CONTENT_DURATION_0_TO_1_MIN",
            ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATION1TO5MIN => "CONTENT_DURATION_1_TO_5_MIN",
            ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATION5TO15MIN => "CONTENT_DURATION_5_TO_15_MIN",
            ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATION15TO30MIN => "CONTENT_DURATION_15_TO_30_MIN",
            ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATION30TO60MIN => "CONTENT_DURATION_30_TO_60_MIN",
            ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATIONOVER60MIN => "CONTENT_DURATION_OVER_60_MIN",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentDurationAssignedTargetingOptionDetailContentDurationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_DURATION_UNSPECIFIED" => Ok(ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATIONUNSPECIFIED),
           "CONTENT_DURATION_UNKNOWN" => Ok(ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATIONUNKNOWN),
           "CONTENT_DURATION_0_TO_1_MIN" => Ok(ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATION0TO1MIN),
           "CONTENT_DURATION_1_TO_5_MIN" => Ok(ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATION1TO5MIN),
           "CONTENT_DURATION_5_TO_15_MIN" => Ok(ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATION5TO15MIN),
           "CONTENT_DURATION_15_TO_30_MIN" => Ok(ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATION15TO30MIN),
           "CONTENT_DURATION_30_TO_60_MIN" => Ok(ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATION30TO60MIN),
           "CONTENT_DURATION_OVER_60_MIN" => Ok(ContentDurationAssignedTargetingOptionDetailContentDurationEnum::CONTENTDURATIONOVER60MIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentDurationAssignedTargetingOptionDetailContentDurationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentDurationTargetingOptionDetailContentDurationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The content duration.
pub enum ContentDurationTargetingOptionDetailContentDurationEnum {
    

    /// Content duration is not specified in this version. This enum is a place holder for a default value and does not represent a real content duration.
    ///
    /// "CONTENT_DURATION_UNSPECIFIED"
    #[serde(rename="CONTENT_DURATION_UNSPECIFIED")]
    CONTENTDURATIONUNSPECIFIED,
    

    /// The content duration is unknown.
    ///
    /// "CONTENT_DURATION_UNKNOWN"
    #[serde(rename="CONTENT_DURATION_UNKNOWN")]
    CONTENTDURATIONUNKNOWN,
    

    /// Content is 0-1 minute long.
    ///
    /// "CONTENT_DURATION_0_TO_1_MIN"
    #[serde(rename="CONTENT_DURATION_0_TO_1_MIN")]
    CONTENTDURATION0TO1MIN,
    

    /// Content is 1-5 minutes long.
    ///
    /// "CONTENT_DURATION_1_TO_5_MIN"
    #[serde(rename="CONTENT_DURATION_1_TO_5_MIN")]
    CONTENTDURATION1TO5MIN,
    

    /// Content is 5-15 minutes long.
    ///
    /// "CONTENT_DURATION_5_TO_15_MIN"
    #[serde(rename="CONTENT_DURATION_5_TO_15_MIN")]
    CONTENTDURATION5TO15MIN,
    

    /// Content is 15-30 minutes long.
    ///
    /// "CONTENT_DURATION_15_TO_30_MIN"
    #[serde(rename="CONTENT_DURATION_15_TO_30_MIN")]
    CONTENTDURATION15TO30MIN,
    

    /// Content is 30-60 minutes long.
    ///
    /// "CONTENT_DURATION_30_TO_60_MIN"
    #[serde(rename="CONTENT_DURATION_30_TO_60_MIN")]
    CONTENTDURATION30TO60MIN,
    

    /// Content is over 60 minutes long.
    ///
    /// "CONTENT_DURATION_OVER_60_MIN"
    #[serde(rename="CONTENT_DURATION_OVER_60_MIN")]
    CONTENTDURATIONOVER60MIN,
}

impl AsRef<str> for ContentDurationTargetingOptionDetailContentDurationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATIONUNSPECIFIED => "CONTENT_DURATION_UNSPECIFIED",
            ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATIONUNKNOWN => "CONTENT_DURATION_UNKNOWN",
            ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATION0TO1MIN => "CONTENT_DURATION_0_TO_1_MIN",
            ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATION1TO5MIN => "CONTENT_DURATION_1_TO_5_MIN",
            ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATION5TO15MIN => "CONTENT_DURATION_5_TO_15_MIN",
            ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATION15TO30MIN => "CONTENT_DURATION_15_TO_30_MIN",
            ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATION30TO60MIN => "CONTENT_DURATION_30_TO_60_MIN",
            ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATIONOVER60MIN => "CONTENT_DURATION_OVER_60_MIN",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentDurationTargetingOptionDetailContentDurationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_DURATION_UNSPECIFIED" => Ok(ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATIONUNSPECIFIED),
           "CONTENT_DURATION_UNKNOWN" => Ok(ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATIONUNKNOWN),
           "CONTENT_DURATION_0_TO_1_MIN" => Ok(ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATION0TO1MIN),
           "CONTENT_DURATION_1_TO_5_MIN" => Ok(ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATION1TO5MIN),
           "CONTENT_DURATION_5_TO_15_MIN" => Ok(ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATION5TO15MIN),
           "CONTENT_DURATION_15_TO_30_MIN" => Ok(ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATION15TO30MIN),
           "CONTENT_DURATION_30_TO_60_MIN" => Ok(ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATION30TO60MIN),
           "CONTENT_DURATION_OVER_60_MIN" => Ok(ContentDurationTargetingOptionDetailContentDurationEnum::CONTENTDURATIONOVER60MIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentDurationTargetingOptionDetailContentDurationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`. * `AD_TYPE_AUDIO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_AUDIO_DEFAULT`.
pub enum ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum {
    

    /// Ad type is not specified or is unknown in this version.
    ///
    /// "AD_TYPE_UNSPECIFIED"
    #[serde(rename="AD_TYPE_UNSPECIFIED")]
    ADTYPEUNSPECIFIED,
    

    /// Display creatives, e.g. image and HTML5.
    ///
    /// "AD_TYPE_DISPLAY"
    #[serde(rename="AD_TYPE_DISPLAY")]
    ADTYPEDISPLAY,
    

    /// Video creatives, e.g. video ads that play during streaming content in video players.
    ///
    /// "AD_TYPE_VIDEO"
    #[serde(rename="AD_TYPE_VIDEO")]
    ADTYPEVIDEO,
    

    /// Audio creatives, e.g. audio ads that play during audio content.
    ///
    /// "AD_TYPE_AUDIO"
    #[serde(rename="AD_TYPE_AUDIO")]
    ADTYPEAUDIO,
}

impl AsRef<str> for ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEUNSPECIFIED => "AD_TYPE_UNSPECIFIED",
            ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEDISPLAY => "AD_TYPE_DISPLAY",
            ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEVIDEO => "AD_TYPE_VIDEO",
            ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEAUDIO => "AD_TYPE_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AD_TYPE_UNSPECIFIED" => Ok(ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEUNSPECIFIED),
           "AD_TYPE_DISPLAY" => Ok(ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEDISPLAY),
           "AD_TYPE_VIDEO" => Ok(ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEVIDEO),
           "AD_TYPE_AUDIO" => Ok(ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentInstreamPositionAssignedTargetingOptionDetailAdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The content instream position for video or audio ads.
pub enum ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum {
    

    /// Content instream position is not specified in this version. This enum is a place holder for a default value and does not represent a real in stream ad position.
    ///
    /// "CONTENT_INSTREAM_POSITION_UNSPECIFIED"
    #[serde(rename="CONTENT_INSTREAM_POSITION_UNSPECIFIED")]
    CONTENTINSTREAMPOSITIONUNSPECIFIED,
    

    /// Ads that play before streaming content.
    ///
    /// "CONTENT_INSTREAM_POSITION_PRE_ROLL"
    #[serde(rename="CONTENT_INSTREAM_POSITION_PRE_ROLL")]
    CONTENTINSTREAMPOSITIONPREROLL,
    

    /// Ads that play between the beginning and end of streaming content.
    ///
    /// "CONTENT_INSTREAM_POSITION_MID_ROLL"
    #[serde(rename="CONTENT_INSTREAM_POSITION_MID_ROLL")]
    CONTENTINSTREAMPOSITIONMIDROLL,
    

    /// Ads that play at the end of streaming content.
    ///
    /// "CONTENT_INSTREAM_POSITION_POST_ROLL"
    #[serde(rename="CONTENT_INSTREAM_POSITION_POST_ROLL")]
    CONTENTINSTREAMPOSITIONPOSTROLL,
    

    /// Ads instream position is unknown.
    ///
    /// "CONTENT_INSTREAM_POSITION_UNKNOWN"
    #[serde(rename="CONTENT_INSTREAM_POSITION_UNKNOWN")]
    CONTENTINSTREAMPOSITIONUNKNOWN,
}

impl AsRef<str> for ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONUNSPECIFIED => "CONTENT_INSTREAM_POSITION_UNSPECIFIED",
            ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONPREROLL => "CONTENT_INSTREAM_POSITION_PRE_ROLL",
            ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONMIDROLL => "CONTENT_INSTREAM_POSITION_MID_ROLL",
            ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONPOSTROLL => "CONTENT_INSTREAM_POSITION_POST_ROLL",
            ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONUNKNOWN => "CONTENT_INSTREAM_POSITION_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_INSTREAM_POSITION_UNSPECIFIED" => Ok(ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONUNSPECIFIED),
           "CONTENT_INSTREAM_POSITION_PRE_ROLL" => Ok(ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONPREROLL),
           "CONTENT_INSTREAM_POSITION_MID_ROLL" => Ok(ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONMIDROLL),
           "CONTENT_INSTREAM_POSITION_POST_ROLL" => Ok(ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONPOSTROLL),
           "CONTENT_INSTREAM_POSITION_UNKNOWN" => Ok(ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentInstreamPositionAssignedTargetingOptionDetailContentInstreamPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The content instream position.
pub enum ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum {
    

    /// Content instream position is not specified in this version. This enum is a place holder for a default value and does not represent a real in stream ad position.
    ///
    /// "CONTENT_INSTREAM_POSITION_UNSPECIFIED"
    #[serde(rename="CONTENT_INSTREAM_POSITION_UNSPECIFIED")]
    CONTENTINSTREAMPOSITIONUNSPECIFIED,
    

    /// Ads that play before streaming content.
    ///
    /// "CONTENT_INSTREAM_POSITION_PRE_ROLL"
    #[serde(rename="CONTENT_INSTREAM_POSITION_PRE_ROLL")]
    CONTENTINSTREAMPOSITIONPREROLL,
    

    /// Ads that play between the beginning and end of streaming content.
    ///
    /// "CONTENT_INSTREAM_POSITION_MID_ROLL"
    #[serde(rename="CONTENT_INSTREAM_POSITION_MID_ROLL")]
    CONTENTINSTREAMPOSITIONMIDROLL,
    

    /// Ads that play at the end of streaming content.
    ///
    /// "CONTENT_INSTREAM_POSITION_POST_ROLL"
    #[serde(rename="CONTENT_INSTREAM_POSITION_POST_ROLL")]
    CONTENTINSTREAMPOSITIONPOSTROLL,
    

    /// Ads instream position is unknown.
    ///
    /// "CONTENT_INSTREAM_POSITION_UNKNOWN"
    #[serde(rename="CONTENT_INSTREAM_POSITION_UNKNOWN")]
    CONTENTINSTREAMPOSITIONUNKNOWN,
}

impl AsRef<str> for ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONUNSPECIFIED => "CONTENT_INSTREAM_POSITION_UNSPECIFIED",
            ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONPREROLL => "CONTENT_INSTREAM_POSITION_PRE_ROLL",
            ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONMIDROLL => "CONTENT_INSTREAM_POSITION_MID_ROLL",
            ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONPOSTROLL => "CONTENT_INSTREAM_POSITION_POST_ROLL",
            ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONUNKNOWN => "CONTENT_INSTREAM_POSITION_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_INSTREAM_POSITION_UNSPECIFIED" => Ok(ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONUNSPECIFIED),
           "CONTENT_INSTREAM_POSITION_PRE_ROLL" => Ok(ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONPREROLL),
           "CONTENT_INSTREAM_POSITION_MID_ROLL" => Ok(ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONMIDROLL),
           "CONTENT_INSTREAM_POSITION_POST_ROLL" => Ok(ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONPOSTROLL),
           "CONTENT_INSTREAM_POSITION_UNKNOWN" => Ok(ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum::CONTENTINSTREAMPOSITIONUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentInstreamPositionTargetingOptionDetailContentInstreamPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_DISPLAY`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_DISPLAY_DEFAULT`. * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`.
pub enum ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum {
    

    /// Ad type is not specified or is unknown in this version.
    ///
    /// "AD_TYPE_UNSPECIFIED"
    #[serde(rename="AD_TYPE_UNSPECIFIED")]
    ADTYPEUNSPECIFIED,
    

    /// Display creatives, e.g. image and HTML5.
    ///
    /// "AD_TYPE_DISPLAY"
    #[serde(rename="AD_TYPE_DISPLAY")]
    ADTYPEDISPLAY,
    

    /// Video creatives, e.g. video ads that play during streaming content in video players.
    ///
    /// "AD_TYPE_VIDEO"
    #[serde(rename="AD_TYPE_VIDEO")]
    ADTYPEVIDEO,
    

    /// Audio creatives, e.g. audio ads that play during audio content.
    ///
    /// "AD_TYPE_AUDIO"
    #[serde(rename="AD_TYPE_AUDIO")]
    ADTYPEAUDIO,
}

impl AsRef<str> for ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEUNSPECIFIED => "AD_TYPE_UNSPECIFIED",
            ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEDISPLAY => "AD_TYPE_DISPLAY",
            ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEVIDEO => "AD_TYPE_VIDEO",
            ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEAUDIO => "AD_TYPE_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AD_TYPE_UNSPECIFIED" => Ok(ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEUNSPECIFIED),
           "AD_TYPE_DISPLAY" => Ok(ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEDISPLAY),
           "AD_TYPE_VIDEO" => Ok(ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEVIDEO),
           "AD_TYPE_AUDIO" => Ok(ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentOutstreamPositionAssignedTargetingOptionDetailAdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The content outstream position.
pub enum ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum {
    

    /// Content outstream position is not specified in this version. This enum is a place holder for a default value and does not represent a real content outstream position.
    ///
    /// "CONTENT_OUTSTREAM_POSITION_UNSPECIFIED"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_UNSPECIFIED")]
    CONTENTOUTSTREAMPOSITIONUNSPECIFIED,
    

    /// The ad position is unknown in the content outstream.
    ///
    /// "CONTENT_OUTSTREAM_POSITION_UNKNOWN"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_UNKNOWN")]
    CONTENTOUTSTREAMPOSITIONUNKNOWN,
    

    /// Ads that appear between the paragraphs of your pages.
    ///
    /// "CONTENT_OUTSTREAM_POSITION_IN_ARTICLE"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_IN_ARTICLE")]
    CONTENTOUTSTREAMPOSITIONINARTICLE,
    

    /// Ads that display on the top and the sides of a page.
    ///
    /// "CONTENT_OUTSTREAM_POSITION_IN_BANNER"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_IN_BANNER")]
    CONTENTOUTSTREAMPOSITIONINBANNER,
    

    /// Ads that appear in a scrollable stream of content. A feed is typically editorial (e.g. a list of articles or news) or listings (e.g. a list of products or services).
    ///
    /// "CONTENT_OUTSTREAM_POSITION_IN_FEED"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_IN_FEED")]
    CONTENTOUTSTREAMPOSITIONINFEED,
    

    /// Ads shown before or between content loads.
    ///
    /// "CONTENT_OUTSTREAM_POSITION_INTERSTITIAL"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_INTERSTITIAL")]
    CONTENTOUTSTREAMPOSITIONINTERSTITIAL,
}

impl AsRef<str> for ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONUNSPECIFIED => "CONTENT_OUTSTREAM_POSITION_UNSPECIFIED",
            ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONUNKNOWN => "CONTENT_OUTSTREAM_POSITION_UNKNOWN",
            ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINARTICLE => "CONTENT_OUTSTREAM_POSITION_IN_ARTICLE",
            ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINBANNER => "CONTENT_OUTSTREAM_POSITION_IN_BANNER",
            ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINFEED => "CONTENT_OUTSTREAM_POSITION_IN_FEED",
            ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINTERSTITIAL => "CONTENT_OUTSTREAM_POSITION_INTERSTITIAL",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_OUTSTREAM_POSITION_UNSPECIFIED" => Ok(ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONUNSPECIFIED),
           "CONTENT_OUTSTREAM_POSITION_UNKNOWN" => Ok(ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONUNKNOWN),
           "CONTENT_OUTSTREAM_POSITION_IN_ARTICLE" => Ok(ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINARTICLE),
           "CONTENT_OUTSTREAM_POSITION_IN_BANNER" => Ok(ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINBANNER),
           "CONTENT_OUTSTREAM_POSITION_IN_FEED" => Ok(ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINFEED),
           "CONTENT_OUTSTREAM_POSITION_INTERSTITIAL" => Ok(ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINTERSTITIAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentOutstreamPositionAssignedTargetingOptionDetailContentOutstreamPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The content outstream position.
pub enum ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum {
    

    /// Content outstream position is not specified in this version. This enum is a place holder for a default value and does not represent a real content outstream position.
    ///
    /// "CONTENT_OUTSTREAM_POSITION_UNSPECIFIED"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_UNSPECIFIED")]
    CONTENTOUTSTREAMPOSITIONUNSPECIFIED,
    

    /// The ad position is unknown in the content outstream.
    ///
    /// "CONTENT_OUTSTREAM_POSITION_UNKNOWN"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_UNKNOWN")]
    CONTENTOUTSTREAMPOSITIONUNKNOWN,
    

    /// Ads that appear between the paragraphs of your pages.
    ///
    /// "CONTENT_OUTSTREAM_POSITION_IN_ARTICLE"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_IN_ARTICLE")]
    CONTENTOUTSTREAMPOSITIONINARTICLE,
    

    /// Ads that display on the top and the sides of a page.
    ///
    /// "CONTENT_OUTSTREAM_POSITION_IN_BANNER"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_IN_BANNER")]
    CONTENTOUTSTREAMPOSITIONINBANNER,
    

    /// Ads that appear in a scrollable stream of content. A feed is typically editorial (e.g. a list of articles or news) or listings (e.g. a list of products or services).
    ///
    /// "CONTENT_OUTSTREAM_POSITION_IN_FEED"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_IN_FEED")]
    CONTENTOUTSTREAMPOSITIONINFEED,
    

    /// Ads shown before or between content loads.
    ///
    /// "CONTENT_OUTSTREAM_POSITION_INTERSTITIAL"
    #[serde(rename="CONTENT_OUTSTREAM_POSITION_INTERSTITIAL")]
    CONTENTOUTSTREAMPOSITIONINTERSTITIAL,
}

impl AsRef<str> for ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONUNSPECIFIED => "CONTENT_OUTSTREAM_POSITION_UNSPECIFIED",
            ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONUNKNOWN => "CONTENT_OUTSTREAM_POSITION_UNKNOWN",
            ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINARTICLE => "CONTENT_OUTSTREAM_POSITION_IN_ARTICLE",
            ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINBANNER => "CONTENT_OUTSTREAM_POSITION_IN_BANNER",
            ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINFEED => "CONTENT_OUTSTREAM_POSITION_IN_FEED",
            ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINTERSTITIAL => "CONTENT_OUTSTREAM_POSITION_INTERSTITIAL",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_OUTSTREAM_POSITION_UNSPECIFIED" => Ok(ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONUNSPECIFIED),
           "CONTENT_OUTSTREAM_POSITION_UNKNOWN" => Ok(ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONUNKNOWN),
           "CONTENT_OUTSTREAM_POSITION_IN_ARTICLE" => Ok(ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINARTICLE),
           "CONTENT_OUTSTREAM_POSITION_IN_BANNER" => Ok(ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINBANNER),
           "CONTENT_OUTSTREAM_POSITION_IN_FEED" => Ok(ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINFEED),
           "CONTENT_OUTSTREAM_POSITION_INTERSTITIAL" => Ok(ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum::CONTENTOUTSTREAMPOSITIONINTERSTITIAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentOutstreamPositionTargetingOptionDetailContentOutstreamPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The content stream type.
pub enum ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum {
    

    /// Content stream type is not specified in this version. This enum is a place holder for a default value and does not represent a real content stream type.
    ///
    /// "CONTENT_STREAM_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_STREAM_TYPE_UNSPECIFIED")]
    CONTENTSTREAMTYPEUNSPECIFIED,
    

    /// The content is being live-streamed.
    ///
    /// "CONTENT_LIVE_STREAM"
    #[serde(rename="CONTENT_LIVE_STREAM")]
    CONTENTLIVESTREAM,
    

    /// The content is viewed on-demand.
    ///
    /// "CONTENT_ON_DEMAND"
    #[serde(rename="CONTENT_ON_DEMAND")]
    CONTENTONDEMAND,
}

impl AsRef<str> for ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum::CONTENTSTREAMTYPEUNSPECIFIED => "CONTENT_STREAM_TYPE_UNSPECIFIED",
            ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum::CONTENTLIVESTREAM => "CONTENT_LIVE_STREAM",
            ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum::CONTENTONDEMAND => "CONTENT_ON_DEMAND",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_STREAM_TYPE_UNSPECIFIED" => Ok(ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum::CONTENTSTREAMTYPEUNSPECIFIED),
           "CONTENT_LIVE_STREAM" => Ok(ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum::CONTENTLIVESTREAM),
           "CONTENT_ON_DEMAND" => Ok(ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum::CONTENTONDEMAND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentStreamTypeAssignedTargetingOptionDetailContentStreamTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The content stream type.
pub enum ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum {
    

    /// Content stream type is not specified in this version. This enum is a place holder for a default value and does not represent a real content stream type.
    ///
    /// "CONTENT_STREAM_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_STREAM_TYPE_UNSPECIFIED")]
    CONTENTSTREAMTYPEUNSPECIFIED,
    

    /// The content is being live-streamed.
    ///
    /// "CONTENT_LIVE_STREAM"
    #[serde(rename="CONTENT_LIVE_STREAM")]
    CONTENTLIVESTREAM,
    

    /// The content is viewed on-demand.
    ///
    /// "CONTENT_ON_DEMAND"
    #[serde(rename="CONTENT_ON_DEMAND")]
    CONTENTONDEMAND,
}

impl AsRef<str> for ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum::CONTENTSTREAMTYPEUNSPECIFIED => "CONTENT_STREAM_TYPE_UNSPECIFIED",
            ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum::CONTENTLIVESTREAM => "CONTENT_LIVE_STREAM",
            ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum::CONTENTONDEMAND => "CONTENT_ON_DEMAND",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_STREAM_TYPE_UNSPECIFIED" => Ok(ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum::CONTENTSTREAMTYPEUNSPECIFIED),
           "CONTENT_LIVE_STREAM" => Ok(ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum::CONTENTLIVESTREAM),
           "CONTENT_ON_DEMAND" => Ok(ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum::CONTENTONDEMAND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentStreamTypeTargetingOptionDetailContentStreamTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreateAssignedTargetingOptionsRequestTargetingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Identifies the type of this assigned targeting option.
pub enum CreateAssignedTargetingOptionsRequestTargetingTypeEnum {
    

    /// Default value when type is not specified or is unknown in this version.
    ///
    /// "TARGETING_TYPE_UNSPECIFIED"
    #[serde(rename="TARGETING_TYPE_UNSPECIFIED")]
    TARGETINGTYPEUNSPECIFIED,
    

    /// Target a channel (a custom group of related websites or apps).
    ///
    /// "TARGETING_TYPE_CHANNEL"
    #[serde(rename="TARGETING_TYPE_CHANNEL")]
    TARGETINGTYPECHANNEL,
    

    /// Target an app category (for example, education or puzzle games).
    ///
    /// "TARGETING_TYPE_APP_CATEGORY"
    #[serde(rename="TARGETING_TYPE_APP_CATEGORY")]
    TARGETINGTYPEAPPCATEGORY,
    

    /// Target a specific app (for example, Angry Birds).
    ///
    /// "TARGETING_TYPE_APP"
    #[serde(rename="TARGETING_TYPE_APP")]
    TARGETINGTYPEAPP,
    

    /// Target a specific url (for example, quora.com).
    ///
    /// "TARGETING_TYPE_URL"
    #[serde(rename="TARGETING_TYPE_URL")]
    TARGETINGTYPEURL,
    

    /// Target ads during a chosen time period on a specific day.
    ///
    /// "TARGETING_TYPE_DAY_AND_TIME"
    #[serde(rename="TARGETING_TYPE_DAY_AND_TIME")]
    TARGETINGTYPEDAYANDTIME,
    

    /// Target ads to a specific age range (for example, 18-24).
    ///
    /// "TARGETING_TYPE_AGE_RANGE"
    #[serde(rename="TARGETING_TYPE_AGE_RANGE")]
    TARGETINGTYPEAGERANGE,
    

    /// Target ads to the specified regions on a regional location list.
    ///
    /// "TARGETING_TYPE_REGIONAL_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    TARGETINGTYPEREGIONALLOCATIONLIST,
    

    /// Target ads to the specified points of interest on a proximity location list.
    ///
    /// "TARGETING_TYPE_PROXIMITY_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    TARGETINGTYPEPROXIMITYLOCATIONLIST,
    

    /// Target ads to a specific gender (for example, female or male).
    ///
    /// "TARGETING_TYPE_GENDER"
    #[serde(rename="TARGETING_TYPE_GENDER")]
    TARGETINGTYPEGENDER,
    

    /// Target a specific video player size for video ads.
    ///
    /// "TARGETING_TYPE_VIDEO_PLAYER_SIZE"
    #[serde(rename="TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    TARGETINGTYPEVIDEOPLAYERSIZE,
    

    /// Target user rewarded content for video ads.
    ///
    /// "TARGETING_TYPE_USER_REWARDED_CONTENT"
    #[serde(rename="TARGETING_TYPE_USER_REWARDED_CONTENT")]
    TARGETINGTYPEUSERREWARDEDCONTENT,
    

    /// Target ads to a specific parental status (for example, parent or not a parent).
    ///
    /// "TARGETING_TYPE_PARENTAL_STATUS"
    #[serde(rename="TARGETING_TYPE_PARENTAL_STATUS")]
    TARGETINGTYPEPARENTALSTATUS,
    

    /// Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll).
    ///
    /// "TARGETING_TYPE_CONTENT_INSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    TARGETINGTYPECONTENTINSTREAMPOSITION,
    

    /// Target ads in a specific content outstream position.
    ///
    /// "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    TARGETINGTYPECONTENTOUTSTREAMPOSITION,
    

    /// Target ads to a specific device type (for example, tablet or connected TV).
    ///
    /// "TARGETING_TYPE_DEVICE_TYPE"
    #[serde(rename="TARGETING_TYPE_DEVICE_TYPE")]
    TARGETINGTYPEDEVICETYPE,
    

    /// Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time.
    ///
    /// "TARGETING_TYPE_AUDIENCE_GROUP"
    #[serde(rename="TARGETING_TYPE_AUDIENCE_GROUP")]
    TARGETINGTYPEAUDIENCEGROUP,
    

    /// Target ads to specific web browsers (for example, Chrome).
    ///
    /// "TARGETING_TYPE_BROWSER"
    #[serde(rename="TARGETING_TYPE_BROWSER")]
    TARGETINGTYPEBROWSER,
    

    /// Target ads to a specific household income range (for example, top 10%).
    ///
    /// "TARGETING_TYPE_HOUSEHOLD_INCOME"
    #[serde(rename="TARGETING_TYPE_HOUSEHOLD_INCOME")]
    TARGETINGTYPEHOUSEHOLDINCOME,
    

    /// Target ads in a specific on screen position.
    ///
    /// "TARGETING_TYPE_ON_SCREEN_POSITION"
    #[serde(rename="TARGETING_TYPE_ON_SCREEN_POSITION")]
    TARGETINGTYPEONSCREENPOSITION,
    

    /// Filter web sites through third party verification (for example, IAS or DoubleVerify).
    ///
    /// "TARGETING_TYPE_THIRD_PARTY_VERIFIER"
    #[serde(rename="TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    TARGETINGTYPETHIRDPARTYVERIFIER,
    

    /// Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences).
    ///
    /// "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION,
    

    /// Filter website content by sensitive categories (for example, adult).
    ///
    /// "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    TARGETINGTYPESENSITIVECATEGORYEXCLUSION,
    

    /// Target ads to a specific environment (for example, web or app).
    ///
    /// "TARGETING_TYPE_ENVIRONMENT"
    #[serde(rename="TARGETING_TYPE_ENVIRONMENT")]
    TARGETINGTYPEENVIRONMENT,
    

    /// Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange).
    ///
    /// "TARGETING_TYPE_CARRIER_AND_ISP"
    #[serde(rename="TARGETING_TYPE_CARRIER_AND_ISP")]
    TARGETINGTYPECARRIERANDISP,
    

    /// Target ads to a specific operating system (for example, macOS).
    ///
    /// "TARGETING_TYPE_OPERATING_SYSTEM"
    #[serde(rename="TARGETING_TYPE_OPERATING_SYSTEM")]
    TARGETINGTYPEOPERATINGSYSTEM,
    

    /// Target ads to a specific device make or model (for example, Roku or Samsung).
    ///
    /// "TARGETING_TYPE_DEVICE_MAKE_MODEL"
    #[serde(rename="TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    TARGETINGTYPEDEVICEMAKEMODEL,
    

    /// Target ads to a specific keyword (for example, dog or retriever).
    ///
    /// "TARGETING_TYPE_KEYWORD"
    #[serde(rename="TARGETING_TYPE_KEYWORD")]
    TARGETINGTYPEKEYWORD,
    

    /// Target ads to a specific negative keyword list.
    ///
    /// "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST"
    #[serde(rename="TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    TARGETINGTYPENEGATIVEKEYWORDLIST,
    

    /// Target ads to a specific viewability (for example, 80% viewable).
    ///
    /// "TARGETING_TYPE_VIEWABILITY"
    #[serde(rename="TARGETING_TYPE_VIEWABILITY")]
    TARGETINGTYPEVIEWABILITY,
    

    /// Target ads to a specific content category (for example, arts & entertainment).
    ///
    /// "TARGETING_TYPE_CATEGORY"
    #[serde(rename="TARGETING_TYPE_CATEGORY")]
    TARGETINGTYPECATEGORY,
    

    /// Purchase impressions from specific deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE")]
    TARGETINGTYPEINVENTORYSOURCE,
    

    /// Target ads to a specific language (for example, English or Japanese).
    ///
    /// "TARGETING_TYPE_LANGUAGE"
    #[serde(rename="TARGETING_TYPE_LANGUAGE")]
    TARGETINGTYPELANGUAGE,
    

    /// Target ads to ads.txt authorized sellers. If no targeting option of this type is assigned, the resource uses the "Authorized Direct Sellers and Resellers" option by default.
    ///
    /// "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS"
    #[serde(rename="TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    TARGETINGTYPEAUTHORIZEDSELLERSTATUS,
    

    /// Target ads to a specific regional location (for example, a city or state).
    ///
    /// "TARGETING_TYPE_GEO_REGION"
    #[serde(rename="TARGETING_TYPE_GEO_REGION")]
    TARGETINGTYPEGEOREGION,
    

    /// Purchase impressions from a group of deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE_GROUP"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    TARGETINGTYPEINVENTORYSOURCEGROUP,
    

    /// Purchase impressions from specific exchanges.
    ///
    /// "TARGETING_TYPE_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_EXCHANGE")]
    TARGETINGTYPEEXCHANGE,
    

    /// Purchase impressions from specific sub-exchanges.
    ///
    /// "TARGETING_TYPE_SUB_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_SUB_EXCHANGE")]
    TARGETINGTYPESUBEXCHANGE,
    

    /// Target ads around a specific point of interest, such as a notable building, a street address, or latitude/longitude coordinates.
    ///
    /// "TARGETING_TYPE_POI"
    #[serde(rename="TARGETING_TYPE_POI")]
    TARGETINGTYPEPOI,
    

    /// Target ads around locations of a business chain within a specific geo region.
    ///
    /// "TARGETING_TYPE_BUSINESS_CHAIN"
    #[serde(rename="TARGETING_TYPE_BUSINESS_CHAIN")]
    TARGETINGTYPEBUSINESSCHAIN,
    

    /// Target ads to a specific video content duration.
    ///
    /// "TARGETING_TYPE_CONTENT_DURATION"
    #[serde(rename="TARGETING_TYPE_CONTENT_DURATION")]
    TARGETINGTYPECONTENTDURATION,
    

    /// Target ads to a specific video content stream type.
    ///
    /// "TARGETING_TYPE_CONTENT_STREAM_TYPE"
    #[serde(rename="TARGETING_TYPE_CONTENT_STREAM_TYPE")]
    TARGETINGTYPECONTENTSTREAMTYPE,
    

    /// Target ads to a specific native content position.
    ///
    /// "TARGETING_TYPE_NATIVE_CONTENT_POSITION"
    #[serde(rename="TARGETING_TYPE_NATIVE_CONTENT_POSITION")]
    TARGETINGTYPENATIVECONTENTPOSITION,
    

    /// Target ads in an Open Measurement enabled inventory.
    ///
    /// "TARGETING_TYPE_OMID"
    #[serde(rename="TARGETING_TYPE_OMID")]
    TARGETINGTYPEOMID,
    

    /// Target ads to a specific audio content type.
    ///
    /// "TARGETING_TYPE_AUDIO_CONTENT_TYPE"
    #[serde(rename="TARGETING_TYPE_AUDIO_CONTENT_TYPE")]
    TARGETINGTYPEAUDIOCONTENTTYPE,
    

    /// Target ads to a specific content genre.
    ///
    /// "TARGETING_TYPE_CONTENT_GENRE"
    #[serde(rename="TARGETING_TYPE_CONTENT_GENRE")]
    TARGETINGTYPECONTENTGENRE,
}

impl AsRef<str> for CreateAssignedTargetingOptionsRequestTargetingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED => "TARGETING_TYPE_UNSPECIFIED",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECHANNEL => "TARGETING_TYPE_CHANNEL",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY => "TARGETING_TYPE_APP_CATEGORY",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAPP => "TARGETING_TYPE_APP",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEURL => "TARGETING_TYPE_URL",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDAYANDTIME => "TARGETING_TYPE_DAY_AND_TIME",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAGERANGE => "TARGETING_TYPE_AGE_RANGE",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST => "TARGETING_TYPE_REGIONAL_LOCATION_LIST",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST => "TARGETING_TYPE_PROXIMITY_LOCATION_LIST",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEGENDER => "TARGETING_TYPE_GENDER",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE => "TARGETING_TYPE_VIDEO_PLAYER_SIZE",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT => "TARGETING_TYPE_USER_REWARDED_CONTENT",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS => "TARGETING_TYPE_PARENTAL_STATUS",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION => "TARGETING_TYPE_CONTENT_INSTREAM_POSITION",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION => "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDEVICETYPE => "TARGETING_TYPE_DEVICE_TYPE",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP => "TARGETING_TYPE_AUDIENCE_GROUP",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEBROWSER => "TARGETING_TYPE_BROWSER",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME => "TARGETING_TYPE_HOUSEHOLD_INCOME",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION => "TARGETING_TYPE_ON_SCREEN_POSITION",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER => "TARGETING_TYPE_THIRD_PARTY_VERIFIER",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION => "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION => "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEENVIRONMENT => "TARGETING_TYPE_ENVIRONMENT",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECARRIERANDISP => "TARGETING_TYPE_CARRIER_AND_ISP",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM => "TARGETING_TYPE_OPERATING_SYSTEM",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL => "TARGETING_TYPE_DEVICE_MAKE_MODEL",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEKEYWORD => "TARGETING_TYPE_KEYWORD",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST => "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEVIEWABILITY => "TARGETING_TYPE_VIEWABILITY",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECATEGORY => "TARGETING_TYPE_CATEGORY",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE => "TARGETING_TYPE_INVENTORY_SOURCE",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPELANGUAGE => "TARGETING_TYPE_LANGUAGE",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS => "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEGEOREGION => "TARGETING_TYPE_GEO_REGION",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP => "TARGETING_TYPE_INVENTORY_SOURCE_GROUP",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEEXCHANGE => "TARGETING_TYPE_EXCHANGE",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE => "TARGETING_TYPE_SUB_EXCHANGE",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPOI => "TARGETING_TYPE_POI",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN => "TARGETING_TYPE_BUSINESS_CHAIN",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTDURATION => "TARGETING_TYPE_CONTENT_DURATION",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE => "TARGETING_TYPE_CONTENT_STREAM_TYPE",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION => "TARGETING_TYPE_NATIVE_CONTENT_POSITION",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEOMID => "TARGETING_TYPE_OMID",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE => "TARGETING_TYPE_AUDIO_CONTENT_TYPE",
            CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTGENRE => "TARGETING_TYPE_CONTENT_GENRE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateAssignedTargetingOptionsRequestTargetingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_TYPE_UNSPECIFIED" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED),
           "TARGETING_TYPE_CHANNEL" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECHANNEL),
           "TARGETING_TYPE_APP_CATEGORY" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY),
           "TARGETING_TYPE_APP" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAPP),
           "TARGETING_TYPE_URL" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEURL),
           "TARGETING_TYPE_DAY_AND_TIME" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDAYANDTIME),
           "TARGETING_TYPE_AGE_RANGE" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAGERANGE),
           "TARGETING_TYPE_REGIONAL_LOCATION_LIST" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST),
           "TARGETING_TYPE_PROXIMITY_LOCATION_LIST" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST),
           "TARGETING_TYPE_GENDER" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEGENDER),
           "TARGETING_TYPE_VIDEO_PLAYER_SIZE" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE),
           "TARGETING_TYPE_USER_REWARDED_CONTENT" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT),
           "TARGETING_TYPE_PARENTAL_STATUS" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS),
           "TARGETING_TYPE_CONTENT_INSTREAM_POSITION" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION),
           "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION),
           "TARGETING_TYPE_DEVICE_TYPE" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDEVICETYPE),
           "TARGETING_TYPE_AUDIENCE_GROUP" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP),
           "TARGETING_TYPE_BROWSER" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEBROWSER),
           "TARGETING_TYPE_HOUSEHOLD_INCOME" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME),
           "TARGETING_TYPE_ON_SCREEN_POSITION" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION),
           "TARGETING_TYPE_THIRD_PARTY_VERIFIER" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER),
           "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION),
           "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION),
           "TARGETING_TYPE_ENVIRONMENT" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEENVIRONMENT),
           "TARGETING_TYPE_CARRIER_AND_ISP" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECARRIERANDISP),
           "TARGETING_TYPE_OPERATING_SYSTEM" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM),
           "TARGETING_TYPE_DEVICE_MAKE_MODEL" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL),
           "TARGETING_TYPE_KEYWORD" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEKEYWORD),
           "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST),
           "TARGETING_TYPE_VIEWABILITY" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEVIEWABILITY),
           "TARGETING_TYPE_CATEGORY" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECATEGORY),
           "TARGETING_TYPE_INVENTORY_SOURCE" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE),
           "TARGETING_TYPE_LANGUAGE" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPELANGUAGE),
           "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS),
           "TARGETING_TYPE_GEO_REGION" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEGEOREGION),
           "TARGETING_TYPE_INVENTORY_SOURCE_GROUP" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP),
           "TARGETING_TYPE_EXCHANGE" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEEXCHANGE),
           "TARGETING_TYPE_SUB_EXCHANGE" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE),
           "TARGETING_TYPE_POI" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPOI),
           "TARGETING_TYPE_BUSINESS_CHAIN" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN),
           "TARGETING_TYPE_CONTENT_DURATION" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTDURATION),
           "TARGETING_TYPE_CONTENT_STREAM_TYPE" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE),
           "TARGETING_TYPE_NATIVE_CONTENT_POSITION" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION),
           "TARGETING_TYPE_OMID" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEOMID),
           "TARGETING_TYPE_AUDIO_CONTENT_TYPE" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE),
           "TARGETING_TYPE_CONTENT_GENRE" => Ok(CreateAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTGENRE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateAssignedTargetingOptionsRequestTargetingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreateSdfDownloadTaskRequestVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The SDF version of the downloaded file. If set to `SDF_VERSION_UNSPECIFIED`, this will default to the version specified by the advertiser or partner identified by `root_id`. An advertiser inherits its SDF version from its partner unless configured otherwise.
pub enum CreateSdfDownloadTaskRequestVersionEnum {
    

    /// SDF version value is not specified or is unknown in this version.
    ///
    /// "SDF_VERSION_UNSPECIFIED"
    #[serde(rename="SDF_VERSION_UNSPECIFIED")]
    SDFVERSIONUNSPECIFIED,
    

    /// SDF version 3.1
    ///
    /// "SDF_VERSION_3_1"
    #[serde(rename="SDF_VERSION_3_1")]
    SDFVERSION31,
    

    /// SDF version 4
    ///
    /// "SDF_VERSION_4"
    #[serde(rename="SDF_VERSION_4")]
    SDFVERSION4,
    

    /// SDF version 4.1
    ///
    /// "SDF_VERSION_4_1"
    #[serde(rename="SDF_VERSION_4_1")]
    SDFVERSION41,
    

    /// SDF version 4.2
    ///
    /// "SDF_VERSION_4_2"
    #[serde(rename="SDF_VERSION_4_2")]
    SDFVERSION42,
    

    /// SDF version 5.
    ///
    /// "SDF_VERSION_5"
    #[serde(rename="SDF_VERSION_5")]
    SDFVERSION5,
    

    /// SDF version 5.1
    ///
    /// "SDF_VERSION_5_1"
    #[serde(rename="SDF_VERSION_5_1")]
    SDFVERSION51,
    

    /// SDF version 5.2
    ///
    /// "SDF_VERSION_5_2"
    #[serde(rename="SDF_VERSION_5_2")]
    SDFVERSION52,
    

    /// SDF version 5.3
    ///
    /// "SDF_VERSION_5_3"
    #[serde(rename="SDF_VERSION_5_3")]
    SDFVERSION53,
    

    /// SDF version 5.4
    ///
    /// "SDF_VERSION_5_4"
    #[serde(rename="SDF_VERSION_5_4")]
    SDFVERSION54,
    

    /// SDF version 5.5
    ///
    /// "SDF_VERSION_5_5"
    #[serde(rename="SDF_VERSION_5_5")]
    SDFVERSION55,
    

    /// SDF version 6
    ///
    /// "SDF_VERSION_6"
    #[serde(rename="SDF_VERSION_6")]
    SDFVERSION6,
    

    /// SDF version 7. Read the [v7 migration guide](/display-video/api/structured-data-file/v7-migration-guide) before migrating to this version.
    ///
    /// "SDF_VERSION_7"
    #[serde(rename="SDF_VERSION_7")]
    SDFVERSION7,
}

impl AsRef<str> for CreateSdfDownloadTaskRequestVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSIONUNSPECIFIED => "SDF_VERSION_UNSPECIFIED",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION31 => "SDF_VERSION_3_1",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION4 => "SDF_VERSION_4",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION41 => "SDF_VERSION_4_1",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION42 => "SDF_VERSION_4_2",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION5 => "SDF_VERSION_5",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION51 => "SDF_VERSION_5_1",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION52 => "SDF_VERSION_5_2",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION53 => "SDF_VERSION_5_3",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION54 => "SDF_VERSION_5_4",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION55 => "SDF_VERSION_5_5",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION6 => "SDF_VERSION_6",
            CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION7 => "SDF_VERSION_7",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateSdfDownloadTaskRequestVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SDF_VERSION_UNSPECIFIED" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSIONUNSPECIFIED),
           "SDF_VERSION_3_1" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION31),
           "SDF_VERSION_4" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION4),
           "SDF_VERSION_4_1" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION41),
           "SDF_VERSION_4_2" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION42),
           "SDF_VERSION_5" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION5),
           "SDF_VERSION_5_1" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION51),
           "SDF_VERSION_5_2" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION52),
           "SDF_VERSION_5_3" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION53),
           "SDF_VERSION_5_4" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION54),
           "SDF_VERSION_5_5" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION55),
           "SDF_VERSION_6" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION6),
           "SDF_VERSION_7" => Ok(CreateSdfDownloadTaskRequestVersionEnum::SDFVERSION7),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateSdfDownloadTaskRequestVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeCreativeAttributesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. A list of attributes of the creative that is generated by the system.
pub enum CreativeCreativeAttributesEnum {
    

    /// The creative attribute is not specified or is unknown in this version.
    ///
    /// "CREATIVE_ATTRIBUTE_UNSPECIFIED"
    #[serde(rename="CREATIVE_ATTRIBUTE_UNSPECIFIED")]
    CREATIVEATTRIBUTEUNSPECIFIED,
    

    /// The creative is a VAST creative.
    ///
    /// "CREATIVE_ATTRIBUTE_VAST"
    #[serde(rename="CREATIVE_ATTRIBUTE_VAST")]
    CREATIVEATTRIBUTEVAST,
    

    /// The creative is a linear VPAID creative.
    ///
    /// "CREATIVE_ATTRIBUTE_VPAID_LINEAR"
    #[serde(rename="CREATIVE_ATTRIBUTE_VPAID_LINEAR")]
    CREATIVEATTRIBUTEVPAIDLINEAR,
    

    /// The creative is a non-linear VPAID creative.
    ///
    /// "CREATIVE_ATTRIBUTE_VPAID_NON_LINEAR"
    #[serde(rename="CREATIVE_ATTRIBUTE_VPAID_NON_LINEAR")]
    CREATIVEATTRIBUTEVPAIDNONLINEAR,
}

impl AsRef<str> for CreativeCreativeAttributesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeCreativeAttributesEnum::CREATIVEATTRIBUTEUNSPECIFIED => "CREATIVE_ATTRIBUTE_UNSPECIFIED",
            CreativeCreativeAttributesEnum::CREATIVEATTRIBUTEVAST => "CREATIVE_ATTRIBUTE_VAST",
            CreativeCreativeAttributesEnum::CREATIVEATTRIBUTEVPAIDLINEAR => "CREATIVE_ATTRIBUTE_VPAID_LINEAR",
            CreativeCreativeAttributesEnum::CREATIVEATTRIBUTEVPAIDNONLINEAR => "CREATIVE_ATTRIBUTE_VPAID_NON_LINEAR",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeCreativeAttributesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_ATTRIBUTE_UNSPECIFIED" => Ok(CreativeCreativeAttributesEnum::CREATIVEATTRIBUTEUNSPECIFIED),
           "CREATIVE_ATTRIBUTE_VAST" => Ok(CreativeCreativeAttributesEnum::CREATIVEATTRIBUTEVAST),
           "CREATIVE_ATTRIBUTE_VPAID_LINEAR" => Ok(CreativeCreativeAttributesEnum::CREATIVEATTRIBUTEVPAIDLINEAR),
           "CREATIVE_ATTRIBUTE_VPAID_NON_LINEAR" => Ok(CreativeCreativeAttributesEnum::CREATIVEATTRIBUTEVPAIDNONLINEAR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeCreativeAttributesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeCreativeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The type of the creative.
pub enum CreativeCreativeTypeEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "CREATIVE_TYPE_UNSPECIFIED"
    #[serde(rename="CREATIVE_TYPE_UNSPECIFIED")]
    CREATIVETYPEUNSPECIFIED,
    

    /// Standard display creative. Create and update methods are supported for this creative type if the hosting_source is one of the following: * `HOSTING_SOURCE_HOSTED` * `HOSTING_SOURCE_THIRD_PARTY`
    ///
    /// "CREATIVE_TYPE_STANDARD"
    #[serde(rename="CREATIVE_TYPE_STANDARD")]
    CREATIVETYPESTANDARD,
    

    /// Expandable creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_THIRD_PARTY`
    ///
    /// "CREATIVE_TYPE_EXPANDABLE"
    #[serde(rename="CREATIVE_TYPE_EXPANDABLE")]
    CREATIVETYPEEXPANDABLE,
    

    /// Video creative. Create and update methods are supported for this creative type if the hosting_source is one of the following: * `HOSTING_SOURCE_HOSTED` * `HOSTING_SOURCE_THIRD_PARTY`
    ///
    /// "CREATIVE_TYPE_VIDEO"
    #[serde(rename="CREATIVE_TYPE_VIDEO")]
    CREATIVETYPEVIDEO,
    

    /// Native creative rendered by publishers with assets from advertiser. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`
    ///
    /// "CREATIVE_TYPE_NATIVE"
    #[serde(rename="CREATIVE_TYPE_NATIVE")]
    CREATIVETYPENATIVE,
    

    /// Templated app install mobile creative (banner). Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_TEMPLATED_APP_INSTALL"
    #[serde(rename="CREATIVE_TYPE_TEMPLATED_APP_INSTALL")]
    CREATIVETYPETEMPLATEDAPPINSTALL,
    

    /// Square native creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`
    ///
    /// "CREATIVE_TYPE_NATIVE_SITE_SQUARE"
    #[serde(rename="CREATIVE_TYPE_NATIVE_SITE_SQUARE")]
    CREATIVETYPENATIVESITESQUARE,
    

    /// Interstitial creative including both display and video. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL"
    #[serde(rename="CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL")]
    CREATIVETYPETEMPLATEDAPPINSTALLINTERSTITIAL,
    

    /// Responsive and expandable Lightbox creative. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_LIGHTBOX"
    #[serde(rename="CREATIVE_TYPE_LIGHTBOX")]
    CREATIVETYPELIGHTBOX,
    

    /// Native app install creative. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_NATIVE_APP_INSTALL"
    #[serde(rename="CREATIVE_TYPE_NATIVE_APP_INSTALL")]
    CREATIVETYPENATIVEAPPINSTALL,
    

    /// Square native app install creative. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE"
    #[serde(rename="CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE")]
    CREATIVETYPENATIVEAPPINSTALLSQUARE,
    

    /// Audio creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`
    ///
    /// "CREATIVE_TYPE_AUDIO"
    #[serde(rename="CREATIVE_TYPE_AUDIO")]
    CREATIVETYPEAUDIO,
    

    /// Publisher hosted creative. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_PUBLISHER_HOSTED"
    #[serde(rename="CREATIVE_TYPE_PUBLISHER_HOSTED")]
    CREATIVETYPEPUBLISHERHOSTED,
    

    /// Native video creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`
    ///
    /// "CREATIVE_TYPE_NATIVE_VIDEO"
    #[serde(rename="CREATIVE_TYPE_NATIVE_VIDEO")]
    CREATIVETYPENATIVEVIDEO,
    

    /// Templated app install mobile video creative. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO"
    #[serde(rename="CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO")]
    CREATIVETYPETEMPLATEDAPPINSTALLVIDEO,
}

impl AsRef<str> for CreativeCreativeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeCreativeTypeEnum::CREATIVETYPEUNSPECIFIED => "CREATIVE_TYPE_UNSPECIFIED",
            CreativeCreativeTypeEnum::CREATIVETYPESTANDARD => "CREATIVE_TYPE_STANDARD",
            CreativeCreativeTypeEnum::CREATIVETYPEEXPANDABLE => "CREATIVE_TYPE_EXPANDABLE",
            CreativeCreativeTypeEnum::CREATIVETYPEVIDEO => "CREATIVE_TYPE_VIDEO",
            CreativeCreativeTypeEnum::CREATIVETYPENATIVE => "CREATIVE_TYPE_NATIVE",
            CreativeCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALL => "CREATIVE_TYPE_TEMPLATED_APP_INSTALL",
            CreativeCreativeTypeEnum::CREATIVETYPENATIVESITESQUARE => "CREATIVE_TYPE_NATIVE_SITE_SQUARE",
            CreativeCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALLINTERSTITIAL => "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL",
            CreativeCreativeTypeEnum::CREATIVETYPELIGHTBOX => "CREATIVE_TYPE_LIGHTBOX",
            CreativeCreativeTypeEnum::CREATIVETYPENATIVEAPPINSTALL => "CREATIVE_TYPE_NATIVE_APP_INSTALL",
            CreativeCreativeTypeEnum::CREATIVETYPENATIVEAPPINSTALLSQUARE => "CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE",
            CreativeCreativeTypeEnum::CREATIVETYPEAUDIO => "CREATIVE_TYPE_AUDIO",
            CreativeCreativeTypeEnum::CREATIVETYPEPUBLISHERHOSTED => "CREATIVE_TYPE_PUBLISHER_HOSTED",
            CreativeCreativeTypeEnum::CREATIVETYPENATIVEVIDEO => "CREATIVE_TYPE_NATIVE_VIDEO",
            CreativeCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALLVIDEO => "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeCreativeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_TYPE_UNSPECIFIED" => Ok(CreativeCreativeTypeEnum::CREATIVETYPEUNSPECIFIED),
           "CREATIVE_TYPE_STANDARD" => Ok(CreativeCreativeTypeEnum::CREATIVETYPESTANDARD),
           "CREATIVE_TYPE_EXPANDABLE" => Ok(CreativeCreativeTypeEnum::CREATIVETYPEEXPANDABLE),
           "CREATIVE_TYPE_VIDEO" => Ok(CreativeCreativeTypeEnum::CREATIVETYPEVIDEO),
           "CREATIVE_TYPE_NATIVE" => Ok(CreativeCreativeTypeEnum::CREATIVETYPENATIVE),
           "CREATIVE_TYPE_TEMPLATED_APP_INSTALL" => Ok(CreativeCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALL),
           "CREATIVE_TYPE_NATIVE_SITE_SQUARE" => Ok(CreativeCreativeTypeEnum::CREATIVETYPENATIVESITESQUARE),
           "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL" => Ok(CreativeCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALLINTERSTITIAL),
           "CREATIVE_TYPE_LIGHTBOX" => Ok(CreativeCreativeTypeEnum::CREATIVETYPELIGHTBOX),
           "CREATIVE_TYPE_NATIVE_APP_INSTALL" => Ok(CreativeCreativeTypeEnum::CREATIVETYPENATIVEAPPINSTALL),
           "CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE" => Ok(CreativeCreativeTypeEnum::CREATIVETYPENATIVEAPPINSTALLSQUARE),
           "CREATIVE_TYPE_AUDIO" => Ok(CreativeCreativeTypeEnum::CREATIVETYPEAUDIO),
           "CREATIVE_TYPE_PUBLISHER_HOSTED" => Ok(CreativeCreativeTypeEnum::CREATIVETYPEPUBLISHERHOSTED),
           "CREATIVE_TYPE_NATIVE_VIDEO" => Ok(CreativeCreativeTypeEnum::CREATIVETYPENATIVEVIDEO),
           "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO" => Ok(CreativeCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALLVIDEO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeCreativeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeEntityStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Controls whether or not the creative can serve. Accepted values are: * `ENTITY_STATUS_ACTIVE` * `ENTITY_STATUS_ARCHIVED` * `ENTITY_STATUS_PAUSED`
pub enum CreativeEntityStatusEnum {
    

    /// Default value when status is not specified or is unknown in this version.
    ///
    /// "ENTITY_STATUS_UNSPECIFIED"
    #[serde(rename="ENTITY_STATUS_UNSPECIFIED")]
    ENTITYSTATUSUNSPECIFIED,
    

    /// The entity is enabled to bid and spend budget.
    ///
    /// "ENTITY_STATUS_ACTIVE"
    #[serde(rename="ENTITY_STATUS_ACTIVE")]
    ENTITYSTATUSACTIVE,
    

    /// The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved.
    ///
    /// "ENTITY_STATUS_ARCHIVED"
    #[serde(rename="ENTITY_STATUS_ARCHIVED")]
    ENTITYSTATUSARCHIVED,
    

    /// The entity is under draft. Bidding and budget spending are disabled.
    ///
    /// "ENTITY_STATUS_DRAFT"
    #[serde(rename="ENTITY_STATUS_DRAFT")]
    ENTITYSTATUSDRAFT,
    

    /// Bidding and budget spending are paused for the entity.
    ///
    /// "ENTITY_STATUS_PAUSED"
    #[serde(rename="ENTITY_STATUS_PAUSED")]
    ENTITYSTATUSPAUSED,
    

    /// The entity is scheduled for deletion.
    ///
    /// "ENTITY_STATUS_SCHEDULED_FOR_DELETION"
    #[serde(rename="ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    ENTITYSTATUSSCHEDULEDFORDELETION,
}

impl AsRef<str> for CreativeEntityStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeEntityStatusEnum::ENTITYSTATUSUNSPECIFIED => "ENTITY_STATUS_UNSPECIFIED",
            CreativeEntityStatusEnum::ENTITYSTATUSACTIVE => "ENTITY_STATUS_ACTIVE",
            CreativeEntityStatusEnum::ENTITYSTATUSARCHIVED => "ENTITY_STATUS_ARCHIVED",
            CreativeEntityStatusEnum::ENTITYSTATUSDRAFT => "ENTITY_STATUS_DRAFT",
            CreativeEntityStatusEnum::ENTITYSTATUSPAUSED => "ENTITY_STATUS_PAUSED",
            CreativeEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION => "ENTITY_STATUS_SCHEDULED_FOR_DELETION",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeEntityStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_STATUS_UNSPECIFIED" => Ok(CreativeEntityStatusEnum::ENTITYSTATUSUNSPECIFIED),
           "ENTITY_STATUS_ACTIVE" => Ok(CreativeEntityStatusEnum::ENTITYSTATUSACTIVE),
           "ENTITY_STATUS_ARCHIVED" => Ok(CreativeEntityStatusEnum::ENTITYSTATUSARCHIVED),
           "ENTITY_STATUS_DRAFT" => Ok(CreativeEntityStatusEnum::ENTITYSTATUSDRAFT),
           "ENTITY_STATUS_PAUSED" => Ok(CreativeEntityStatusEnum::ENTITYSTATUSPAUSED),
           "ENTITY_STATUS_SCHEDULED_FOR_DELETION" => Ok(CreativeEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeEntityStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeExpandingDirectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the expanding direction of the creative. Required and only valid for third-party expandable creatives. Third-party expandable creatives are creatives with following hosting source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_EXPANDABLE`
pub enum CreativeExpandingDirectionEnum {
    

    /// The expanding direction is not specified.
    ///
    /// "EXPANDING_DIRECTION_UNSPECIFIED"
    #[serde(rename="EXPANDING_DIRECTION_UNSPECIFIED")]
    EXPANDINGDIRECTIONUNSPECIFIED,
    

    /// Does not expand in any direction.
    ///
    /// "EXPANDING_DIRECTION_NONE"
    #[serde(rename="EXPANDING_DIRECTION_NONE")]
    EXPANDINGDIRECTIONNONE,
    

    /// Expands up.
    ///
    /// "EXPANDING_DIRECTION_UP"
    #[serde(rename="EXPANDING_DIRECTION_UP")]
    EXPANDINGDIRECTIONUP,
    

    /// Expands down.
    ///
    /// "EXPANDING_DIRECTION_DOWN"
    #[serde(rename="EXPANDING_DIRECTION_DOWN")]
    EXPANDINGDIRECTIONDOWN,
    

    /// Expands left.
    ///
    /// "EXPANDING_DIRECTION_LEFT"
    #[serde(rename="EXPANDING_DIRECTION_LEFT")]
    EXPANDINGDIRECTIONLEFT,
    

    /// Expands right.
    ///
    /// "EXPANDING_DIRECTION_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_RIGHT")]
    EXPANDINGDIRECTIONRIGHT,
    

    /// Expands up and to the left side.
    ///
    /// "EXPANDING_DIRECTION_UP_AND_LEFT"
    #[serde(rename="EXPANDING_DIRECTION_UP_AND_LEFT")]
    EXPANDINGDIRECTIONUPANDLEFT,
    

    /// Expands up and to the right side.
    ///
    /// "EXPANDING_DIRECTION_UP_AND_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_UP_AND_RIGHT")]
    EXPANDINGDIRECTIONUPANDRIGHT,
    

    /// Expands down and to the left side.
    ///
    /// "EXPANDING_DIRECTION_DOWN_AND_LEFT"
    #[serde(rename="EXPANDING_DIRECTION_DOWN_AND_LEFT")]
    EXPANDINGDIRECTIONDOWNANDLEFT,
    

    /// Expands down and to the right side.
    ///
    /// "EXPANDING_DIRECTION_DOWN_AND_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_DOWN_AND_RIGHT")]
    EXPANDINGDIRECTIONDOWNANDRIGHT,
    

    /// Expands either up or down.
    ///
    /// "EXPANDING_DIRECTION_UP_OR_DOWN"
    #[serde(rename="EXPANDING_DIRECTION_UP_OR_DOWN")]
    EXPANDINGDIRECTIONUPORDOWN,
    

    /// Expands to either the left or the right side.
    ///
    /// "EXPANDING_DIRECTION_LEFT_OR_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_LEFT_OR_RIGHT")]
    EXPANDINGDIRECTIONLEFTORRIGHT,
    

    /// Can expand in any diagonal direction.
    ///
    /// "EXPANDING_DIRECTION_ANY_DIAGONAL"
    #[serde(rename="EXPANDING_DIRECTION_ANY_DIAGONAL")]
    EXPANDINGDIRECTIONANYDIAGONAL,
}

impl AsRef<str> for CreativeExpandingDirectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONUNSPECIFIED => "EXPANDING_DIRECTION_UNSPECIFIED",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONNONE => "EXPANDING_DIRECTION_NONE",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONUP => "EXPANDING_DIRECTION_UP",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONDOWN => "EXPANDING_DIRECTION_DOWN",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONLEFT => "EXPANDING_DIRECTION_LEFT",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONRIGHT => "EXPANDING_DIRECTION_RIGHT",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONUPANDLEFT => "EXPANDING_DIRECTION_UP_AND_LEFT",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONUPANDRIGHT => "EXPANDING_DIRECTION_UP_AND_RIGHT",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONDOWNANDLEFT => "EXPANDING_DIRECTION_DOWN_AND_LEFT",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONDOWNANDRIGHT => "EXPANDING_DIRECTION_DOWN_AND_RIGHT",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONUPORDOWN => "EXPANDING_DIRECTION_UP_OR_DOWN",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONLEFTORRIGHT => "EXPANDING_DIRECTION_LEFT_OR_RIGHT",
            CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONANYDIAGONAL => "EXPANDING_DIRECTION_ANY_DIAGONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeExpandingDirectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPANDING_DIRECTION_UNSPECIFIED" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONUNSPECIFIED),
           "EXPANDING_DIRECTION_NONE" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONNONE),
           "EXPANDING_DIRECTION_UP" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONUP),
           "EXPANDING_DIRECTION_DOWN" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONDOWN),
           "EXPANDING_DIRECTION_LEFT" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONLEFT),
           "EXPANDING_DIRECTION_RIGHT" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONRIGHT),
           "EXPANDING_DIRECTION_UP_AND_LEFT" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONUPANDLEFT),
           "EXPANDING_DIRECTION_UP_AND_RIGHT" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONUPANDRIGHT),
           "EXPANDING_DIRECTION_DOWN_AND_LEFT" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONDOWNANDLEFT),
           "EXPANDING_DIRECTION_DOWN_AND_RIGHT" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONDOWNANDRIGHT),
           "EXPANDING_DIRECTION_UP_OR_DOWN" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONUPORDOWN),
           "EXPANDING_DIRECTION_LEFT_OR_RIGHT" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONLEFTORRIGHT),
           "EXPANDING_DIRECTION_ANY_DIAGONAL" => Ok(CreativeExpandingDirectionEnum::EXPANDINGDIRECTIONANYDIAGONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeExpandingDirectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeHostingSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates where the creative is hosted.
pub enum CreativeHostingSourceEnum {
    

    /// Hosting source is not specified or is unknown in this version.
    ///
    /// "HOSTING_SOURCE_UNSPECIFIED"
    #[serde(rename="HOSTING_SOURCE_UNSPECIFIED")]
    HOSTINGSOURCEUNSPECIFIED,
    

    /// A creative synced from Campaign Manager 360. Create and update methods are **not** supported for this hosting type.
    ///
    /// "HOSTING_SOURCE_CM"
    #[serde(rename="HOSTING_SOURCE_CM")]
    HOSTINGSOURCECM,
    

    /// A creative hosted by a third-party ad server (3PAS). Create and update methods are supported for this hosting type if the creative_type is one of the following: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_EXPANDABLE` * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_VIDEO`
    ///
    /// "HOSTING_SOURCE_THIRD_PARTY"
    #[serde(rename="HOSTING_SOURCE_THIRD_PARTY")]
    HOSTINGSOURCETHIRDPARTY,
    

    /// A creative created in DV360 and hosted by Campaign Manager 360. Create and update methods are supported for this hosting type if the creative_type is one of the following: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO` * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_VIDEO`
    ///
    /// "HOSTING_SOURCE_HOSTED"
    #[serde(rename="HOSTING_SOURCE_HOSTED")]
    HOSTINGSOURCEHOSTED,
    

    /// A rich media creative created in Studio and hosted by Campaign Manager 360. Create and update methods are **not** supported for this hosting type.
    ///
    /// "HOSTING_SOURCE_RICH_MEDIA"
    #[serde(rename="HOSTING_SOURCE_RICH_MEDIA")]
    HOSTINGSOURCERICHMEDIA,
}

impl AsRef<str> for CreativeHostingSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeHostingSourceEnum::HOSTINGSOURCEUNSPECIFIED => "HOSTING_SOURCE_UNSPECIFIED",
            CreativeHostingSourceEnum::HOSTINGSOURCECM => "HOSTING_SOURCE_CM",
            CreativeHostingSourceEnum::HOSTINGSOURCETHIRDPARTY => "HOSTING_SOURCE_THIRD_PARTY",
            CreativeHostingSourceEnum::HOSTINGSOURCEHOSTED => "HOSTING_SOURCE_HOSTED",
            CreativeHostingSourceEnum::HOSTINGSOURCERICHMEDIA => "HOSTING_SOURCE_RICH_MEDIA",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeHostingSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HOSTING_SOURCE_UNSPECIFIED" => Ok(CreativeHostingSourceEnum::HOSTINGSOURCEUNSPECIFIED),
           "HOSTING_SOURCE_CM" => Ok(CreativeHostingSourceEnum::HOSTINGSOURCECM),
           "HOSTING_SOURCE_THIRD_PARTY" => Ok(CreativeHostingSourceEnum::HOSTINGSOURCETHIRDPARTY),
           "HOSTING_SOURCE_HOSTED" => Ok(CreativeHostingSourceEnum::HOSTINGSOURCEHOSTED),
           "HOSTING_SOURCE_RICH_MEDIA" => Ok(CreativeHostingSourceEnum::HOSTINGSOURCERICHMEDIA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeHostingSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeConfigCreativeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of creative that can be assigned to the inventory source. Only the following types are supported: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_VIDEO`
pub enum CreativeConfigCreativeTypeEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "CREATIVE_TYPE_UNSPECIFIED"
    #[serde(rename="CREATIVE_TYPE_UNSPECIFIED")]
    CREATIVETYPEUNSPECIFIED,
    

    /// Standard display creative. Create and update methods are supported for this creative type if the hosting_source is one of the following: * `HOSTING_SOURCE_HOSTED` * `HOSTING_SOURCE_THIRD_PARTY`
    ///
    /// "CREATIVE_TYPE_STANDARD"
    #[serde(rename="CREATIVE_TYPE_STANDARD")]
    CREATIVETYPESTANDARD,
    

    /// Expandable creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_THIRD_PARTY`
    ///
    /// "CREATIVE_TYPE_EXPANDABLE"
    #[serde(rename="CREATIVE_TYPE_EXPANDABLE")]
    CREATIVETYPEEXPANDABLE,
    

    /// Video creative. Create and update methods are supported for this creative type if the hosting_source is one of the following: * `HOSTING_SOURCE_HOSTED` * `HOSTING_SOURCE_THIRD_PARTY`
    ///
    /// "CREATIVE_TYPE_VIDEO"
    #[serde(rename="CREATIVE_TYPE_VIDEO")]
    CREATIVETYPEVIDEO,
    

    /// Native creative rendered by publishers with assets from advertiser. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`
    ///
    /// "CREATIVE_TYPE_NATIVE"
    #[serde(rename="CREATIVE_TYPE_NATIVE")]
    CREATIVETYPENATIVE,
    

    /// Templated app install mobile creative (banner). Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_TEMPLATED_APP_INSTALL"
    #[serde(rename="CREATIVE_TYPE_TEMPLATED_APP_INSTALL")]
    CREATIVETYPETEMPLATEDAPPINSTALL,
    

    /// Square native creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`
    ///
    /// "CREATIVE_TYPE_NATIVE_SITE_SQUARE"
    #[serde(rename="CREATIVE_TYPE_NATIVE_SITE_SQUARE")]
    CREATIVETYPENATIVESITESQUARE,
    

    /// Interstitial creative including both display and video. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL"
    #[serde(rename="CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL")]
    CREATIVETYPETEMPLATEDAPPINSTALLINTERSTITIAL,
    

    /// Responsive and expandable Lightbox creative. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_LIGHTBOX"
    #[serde(rename="CREATIVE_TYPE_LIGHTBOX")]
    CREATIVETYPELIGHTBOX,
    

    /// Native app install creative. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_NATIVE_APP_INSTALL"
    #[serde(rename="CREATIVE_TYPE_NATIVE_APP_INSTALL")]
    CREATIVETYPENATIVEAPPINSTALL,
    

    /// Square native app install creative. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE"
    #[serde(rename="CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE")]
    CREATIVETYPENATIVEAPPINSTALLSQUARE,
    

    /// Audio creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`
    ///
    /// "CREATIVE_TYPE_AUDIO"
    #[serde(rename="CREATIVE_TYPE_AUDIO")]
    CREATIVETYPEAUDIO,
    

    /// Publisher hosted creative. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_PUBLISHER_HOSTED"
    #[serde(rename="CREATIVE_TYPE_PUBLISHER_HOSTED")]
    CREATIVETYPEPUBLISHERHOSTED,
    

    /// Native video creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`
    ///
    /// "CREATIVE_TYPE_NATIVE_VIDEO"
    #[serde(rename="CREATIVE_TYPE_NATIVE_VIDEO")]
    CREATIVETYPENATIVEVIDEO,
    

    /// Templated app install mobile video creative. Create and update methods are **not** supported for this creative type.
    ///
    /// "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO"
    #[serde(rename="CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO")]
    CREATIVETYPETEMPLATEDAPPINSTALLVIDEO,
}

impl AsRef<str> for CreativeConfigCreativeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeConfigCreativeTypeEnum::CREATIVETYPEUNSPECIFIED => "CREATIVE_TYPE_UNSPECIFIED",
            CreativeConfigCreativeTypeEnum::CREATIVETYPESTANDARD => "CREATIVE_TYPE_STANDARD",
            CreativeConfigCreativeTypeEnum::CREATIVETYPEEXPANDABLE => "CREATIVE_TYPE_EXPANDABLE",
            CreativeConfigCreativeTypeEnum::CREATIVETYPEVIDEO => "CREATIVE_TYPE_VIDEO",
            CreativeConfigCreativeTypeEnum::CREATIVETYPENATIVE => "CREATIVE_TYPE_NATIVE",
            CreativeConfigCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALL => "CREATIVE_TYPE_TEMPLATED_APP_INSTALL",
            CreativeConfigCreativeTypeEnum::CREATIVETYPENATIVESITESQUARE => "CREATIVE_TYPE_NATIVE_SITE_SQUARE",
            CreativeConfigCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALLINTERSTITIAL => "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL",
            CreativeConfigCreativeTypeEnum::CREATIVETYPELIGHTBOX => "CREATIVE_TYPE_LIGHTBOX",
            CreativeConfigCreativeTypeEnum::CREATIVETYPENATIVEAPPINSTALL => "CREATIVE_TYPE_NATIVE_APP_INSTALL",
            CreativeConfigCreativeTypeEnum::CREATIVETYPENATIVEAPPINSTALLSQUARE => "CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE",
            CreativeConfigCreativeTypeEnum::CREATIVETYPEAUDIO => "CREATIVE_TYPE_AUDIO",
            CreativeConfigCreativeTypeEnum::CREATIVETYPEPUBLISHERHOSTED => "CREATIVE_TYPE_PUBLISHER_HOSTED",
            CreativeConfigCreativeTypeEnum::CREATIVETYPENATIVEVIDEO => "CREATIVE_TYPE_NATIVE_VIDEO",
            CreativeConfigCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALLVIDEO => "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeConfigCreativeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_TYPE_UNSPECIFIED" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPEUNSPECIFIED),
           "CREATIVE_TYPE_STANDARD" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPESTANDARD),
           "CREATIVE_TYPE_EXPANDABLE" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPEEXPANDABLE),
           "CREATIVE_TYPE_VIDEO" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPEVIDEO),
           "CREATIVE_TYPE_NATIVE" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPENATIVE),
           "CREATIVE_TYPE_TEMPLATED_APP_INSTALL" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALL),
           "CREATIVE_TYPE_NATIVE_SITE_SQUARE" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPENATIVESITESQUARE),
           "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALLINTERSTITIAL),
           "CREATIVE_TYPE_LIGHTBOX" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPELIGHTBOX),
           "CREATIVE_TYPE_NATIVE_APP_INSTALL" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPENATIVEAPPINSTALL),
           "CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPENATIVEAPPINSTALLSQUARE),
           "CREATIVE_TYPE_AUDIO" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPEAUDIO),
           "CREATIVE_TYPE_PUBLISHER_HOSTED" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPEPUBLISHERHOSTED),
           "CREATIVE_TYPE_NATIVE_VIDEO" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPENATIVEVIDEO),
           "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO" => Ok(CreativeConfigCreativeTypeEnum::CREATIVETYPETEMPLATEDAPPINSTALLVIDEO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeConfigCreativeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The status of custom bidding algorithm.
pub enum CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum {
    

    /// State is not specified or is unknown in this version.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Algorithm is enabled, either recently used, currently used or scheduled to be used. The algorithm is actively scoring impressions.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// Algorithm has not been used recently. Although the algorithm still acts as `ENABLED`, it will eventually be suspended if not used.
    ///
    /// "DORMANT"
    #[serde(rename="DORMANT")]
    DORMANT,
    

    /// Algorithm is susepended from scoring impressions and doesn't have a serving model trained. If the algorithm is assigned to a line item or otherwise updated, it will switch back to the `ENABLED` state and require time to prepare the serving model again.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
}

impl AsRef<str> for CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum::ENABLED => "ENABLED",
            CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum::DORMANT => "DORMANT",
            CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum::SUSPENDED => "SUSPENDED",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum::STATEUNSPECIFIED),
           "ENABLED" => Ok(CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum::ENABLED),
           "DORMANT" => Ok(CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum::DORMANT),
           "SUSPENDED" => Ok(CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum::SUSPENDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomBiddingAlgorithmCustomBiddingAlgorithmStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The type of custom bidding algorithm.
pub enum CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum {
    

    /// Algorithm type is not specified or is unknown in this version.
    ///
    /// "CUSTOM_BIDDING_ALGORITHM_TYPE_UNSPECIFIED"
    #[serde(rename="CUSTOM_BIDDING_ALGORITHM_TYPE_UNSPECIFIED")]
    CUSTOMBIDDINGALGORITHMTYPEUNSPECIFIED,
    

    /// Algorithm generated through customer-uploaded custom bidding script files.
    ///
    /// "SCRIPT_BASED"
    #[serde(rename="SCRIPT_BASED")]
    SCRIPTBASED,
    

    /// Algorithm created through Ads Data Hub product.
    ///
    /// "ADS_DATA_HUB_BASED"
    #[serde(rename="ADS_DATA_HUB_BASED")]
    ADSDATAHUBBASED,
    

    /// Algorithm created through goal builder in DV3 UI.
    ///
    /// "GOAL_BUILDER_BASED"
    #[serde(rename="GOAL_BUILDER_BASED")]
    GOALBUILDERBASED,
}

impl AsRef<str> for CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum::CUSTOMBIDDINGALGORITHMTYPEUNSPECIFIED => "CUSTOM_BIDDING_ALGORITHM_TYPE_UNSPECIFIED",
            CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum::SCRIPTBASED => "SCRIPT_BASED",
            CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum::ADSDATAHUBBASED => "ADS_DATA_HUB_BASED",
            CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum::GOALBUILDERBASED => "GOAL_BUILDER_BASED",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CUSTOM_BIDDING_ALGORITHM_TYPE_UNSPECIFIED" => Ok(CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum::CUSTOMBIDDINGALGORITHMTYPEUNSPECIFIED),
           "SCRIPT_BASED" => Ok(CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum::SCRIPTBASED),
           "ADS_DATA_HUB_BASED" => Ok(CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum::ADSDATAHUBBASED),
           "GOAL_BUILDER_BASED" => Ok(CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum::GOALBUILDERBASED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomBiddingAlgorithmEntityStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether or not the custom bidding algorithm can be used as a bidding strategy. Accepted values are: * `ENTITY_STATUS_ACTIVE` * `ENTITY_STATUS_ARCHIVED`
pub enum CustomBiddingAlgorithmEntityStatusEnum {
    

    /// Default value when status is not specified or is unknown in this version.
    ///
    /// "ENTITY_STATUS_UNSPECIFIED"
    #[serde(rename="ENTITY_STATUS_UNSPECIFIED")]
    ENTITYSTATUSUNSPECIFIED,
    

    /// The entity is enabled to bid and spend budget.
    ///
    /// "ENTITY_STATUS_ACTIVE"
    #[serde(rename="ENTITY_STATUS_ACTIVE")]
    ENTITYSTATUSACTIVE,
    

    /// The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved.
    ///
    /// "ENTITY_STATUS_ARCHIVED"
    #[serde(rename="ENTITY_STATUS_ARCHIVED")]
    ENTITYSTATUSARCHIVED,
    

    /// The entity is under draft. Bidding and budget spending are disabled.
    ///
    /// "ENTITY_STATUS_DRAFT"
    #[serde(rename="ENTITY_STATUS_DRAFT")]
    ENTITYSTATUSDRAFT,
    

    /// Bidding and budget spending are paused for the entity.
    ///
    /// "ENTITY_STATUS_PAUSED"
    #[serde(rename="ENTITY_STATUS_PAUSED")]
    ENTITYSTATUSPAUSED,
    

    /// The entity is scheduled for deletion.
    ///
    /// "ENTITY_STATUS_SCHEDULED_FOR_DELETION"
    #[serde(rename="ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    ENTITYSTATUSSCHEDULEDFORDELETION,
}

impl AsRef<str> for CustomBiddingAlgorithmEntityStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSUNSPECIFIED => "ENTITY_STATUS_UNSPECIFIED",
            CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSACTIVE => "ENTITY_STATUS_ACTIVE",
            CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSARCHIVED => "ENTITY_STATUS_ARCHIVED",
            CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSDRAFT => "ENTITY_STATUS_DRAFT",
            CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSPAUSED => "ENTITY_STATUS_PAUSED",
            CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION => "ENTITY_STATUS_SCHEDULED_FOR_DELETION",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomBiddingAlgorithmEntityStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_STATUS_UNSPECIFIED" => Ok(CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSUNSPECIFIED),
           "ENTITY_STATUS_ACTIVE" => Ok(CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSACTIVE),
           "ENTITY_STATUS_ARCHIVED" => Ok(CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSARCHIVED),
           "ENTITY_STATUS_DRAFT" => Ok(CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSDRAFT),
           "ENTITY_STATUS_PAUSED" => Ok(CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSPAUSED),
           "ENTITY_STATUS_SCHEDULED_FOR_DELETION" => Ok(CustomBiddingAlgorithmEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomBiddingAlgorithmEntityStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomBiddingModelReadinessStateReadinessStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The readiness state of custom bidding model.
pub enum CustomBiddingModelReadinessStateReadinessStateEnum {
    

    /// State is not specified or is unknown in this version.
    ///
    /// "READINESS_STATE_UNSPECIFIED"
    #[serde(rename="READINESS_STATE_UNSPECIFIED")]
    READINESSSTATEUNSPECIFIED,
    

    /// The model is trained and ready for serving.
    ///
    /// "READINESS_STATE_ACTIVE"
    #[serde(rename="READINESS_STATE_ACTIVE")]
    READINESSSTATEACTIVE,
    

    /// There is not enough data to train the serving model.
    ///
    /// "READINESS_STATE_INSUFFICIENT_DATA"
    #[serde(rename="READINESS_STATE_INSUFFICIENT_DATA")]
    READINESSSTATEINSUFFICIENTDATA,
    

    /// The model is training and not ready for serving.
    ///
    /// "READINESS_STATE_TRAINING"
    #[serde(rename="READINESS_STATE_TRAINING")]
    READINESSSTATETRAINING,
    

    /// A valid custom bidding script has not been provided with which to train the model. This state will only be applied to algorithms whose `custom_bidding_algorithm_type` is `SCRIPT_BASED`.
    ///
    /// "READINESS_STATE_NO_VALID_SCRIPT"
    #[serde(rename="READINESS_STATE_NO_VALID_SCRIPT")]
    READINESSSTATENOVALIDSCRIPT,
}

impl AsRef<str> for CustomBiddingModelReadinessStateReadinessStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomBiddingModelReadinessStateReadinessStateEnum::READINESSSTATEUNSPECIFIED => "READINESS_STATE_UNSPECIFIED",
            CustomBiddingModelReadinessStateReadinessStateEnum::READINESSSTATEACTIVE => "READINESS_STATE_ACTIVE",
            CustomBiddingModelReadinessStateReadinessStateEnum::READINESSSTATEINSUFFICIENTDATA => "READINESS_STATE_INSUFFICIENT_DATA",
            CustomBiddingModelReadinessStateReadinessStateEnum::READINESSSTATETRAINING => "READINESS_STATE_TRAINING",
            CustomBiddingModelReadinessStateReadinessStateEnum::READINESSSTATENOVALIDSCRIPT => "READINESS_STATE_NO_VALID_SCRIPT",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomBiddingModelReadinessStateReadinessStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "READINESS_STATE_UNSPECIFIED" => Ok(CustomBiddingModelReadinessStateReadinessStateEnum::READINESSSTATEUNSPECIFIED),
           "READINESS_STATE_ACTIVE" => Ok(CustomBiddingModelReadinessStateReadinessStateEnum::READINESSSTATEACTIVE),
           "READINESS_STATE_INSUFFICIENT_DATA" => Ok(CustomBiddingModelReadinessStateReadinessStateEnum::READINESSSTATEINSUFFICIENTDATA),
           "READINESS_STATE_TRAINING" => Ok(CustomBiddingModelReadinessStateReadinessStateEnum::READINESSSTATETRAINING),
           "READINESS_STATE_NO_VALID_SCRIPT" => Ok(CustomBiddingModelReadinessStateReadinessStateEnum::READINESSSTATENOVALIDSCRIPT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomBiddingModelReadinessStateReadinessStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomBiddingScriptStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the custom bidding script.
pub enum CustomBiddingScriptStateEnum {
    

    /// The script state is not specified or is unknown in this version.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The script has been accepted for scoring impressions.
    ///
    /// "ACCEPTED"
    #[serde(rename="ACCEPTED")]
    ACCEPTED,
    

    /// The script has been rejected by backend pipelines. It may have errors.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// The script is being processed for backend pipelines.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
}

impl AsRef<str> for CustomBiddingScriptStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomBiddingScriptStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CustomBiddingScriptStateEnum::ACCEPTED => "ACCEPTED",
            CustomBiddingScriptStateEnum::REJECTED => "REJECTED",
            CustomBiddingScriptStateEnum::PENDING => "PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomBiddingScriptStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CustomBiddingScriptStateEnum::STATEUNSPECIFIED),
           "ACCEPTED" => Ok(CustomBiddingScriptStateEnum::ACCEPTED),
           "REJECTED" => Ok(CustomBiddingScriptStateEnum::REJECTED),
           "PENDING" => Ok(CustomBiddingScriptStateEnum::PENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomBiddingScriptStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The day of the week for this day and time targeting setting.
pub enum DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum {
    

    /// The day of the week is unspecified.
    ///
    /// "DAY_OF_WEEK_UNSPECIFIED"
    #[serde(rename="DAY_OF_WEEK_UNSPECIFIED")]
    DAYOFWEEKUNSPECIFIED,
    

    /// Monday
    ///
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    

    /// Tuesday
    ///
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    

    /// Wednesday
    ///
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    

    /// Thursday
    ///
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    

    /// Friday
    ///
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    

    /// Saturday
    ///
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    

    /// Sunday
    ///
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::MONDAY => "MONDAY",
            DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::TUESDAY => "TUESDAY",
            DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::WEDNESDAY => "WEDNESDAY",
            DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::THURSDAY => "THURSDAY",
            DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::FRIDAY => "FRIDAY",
            DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::SATURDAY => "SATURDAY",
            DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::MONDAY),
           "TUESDAY" => Ok(DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::TUESDAY),
           "WEDNESDAY" => Ok(DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::WEDNESDAY),
           "THURSDAY" => Ok(DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::THURSDAY),
           "FRIDAY" => Ok(DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::FRIDAY),
           "SATURDAY" => Ok(DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::SATURDAY),
           "SUNDAY" => Ok(DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DayAndTimeAssignedTargetingOptionDetailDayOfWeekEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The mechanism used to determine which timezone to use for this day and time targeting setting.
pub enum DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum {
    

    /// Time zone resolution is either unspecific or unknown.
    ///
    /// "TIME_ZONE_RESOLUTION_UNSPECIFIED"
    #[serde(rename="TIME_ZONE_RESOLUTION_UNSPECIFIED")]
    TIMEZONERESOLUTIONUNSPECIFIED,
    

    /// Times are resolved in the time zone of the user that saw the ad.
    ///
    /// "TIME_ZONE_RESOLUTION_END_USER"
    #[serde(rename="TIME_ZONE_RESOLUTION_END_USER")]
    TIMEZONERESOLUTIONENDUSER,
    

    /// Times are resolved in the time zone of the advertiser that served the ad.
    ///
    /// "TIME_ZONE_RESOLUTION_ADVERTISER"
    #[serde(rename="TIME_ZONE_RESOLUTION_ADVERTISER")]
    TIMEZONERESOLUTIONADVERTISER,
}

impl AsRef<str> for DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum::TIMEZONERESOLUTIONUNSPECIFIED => "TIME_ZONE_RESOLUTION_UNSPECIFIED",
            DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum::TIMEZONERESOLUTIONENDUSER => "TIME_ZONE_RESOLUTION_END_USER",
            DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum::TIMEZONERESOLUTIONADVERTISER => "TIME_ZONE_RESOLUTION_ADVERTISER",
        }
    }
}

impl std::convert::TryFrom< &str> for DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_ZONE_RESOLUTION_UNSPECIFIED" => Ok(DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum::TIMEZONERESOLUTIONUNSPECIFIED),
           "TIME_ZONE_RESOLUTION_END_USER" => Ok(DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum::TIMEZONERESOLUTIONENDUSER),
           "TIME_ZONE_RESOLUTION_ADVERTISER" => Ok(DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum::TIMEZONERESOLUTIONADVERTISER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DayAndTimeAssignedTargetingOptionDetailTimeZoneResolutionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeleteAssignedTargetingOptionsRequestTargetingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Identifies the type of this assigned targeting option.
pub enum DeleteAssignedTargetingOptionsRequestTargetingTypeEnum {
    

    /// Default value when type is not specified or is unknown in this version.
    ///
    /// "TARGETING_TYPE_UNSPECIFIED"
    #[serde(rename="TARGETING_TYPE_UNSPECIFIED")]
    TARGETINGTYPEUNSPECIFIED,
    

    /// Target a channel (a custom group of related websites or apps).
    ///
    /// "TARGETING_TYPE_CHANNEL"
    #[serde(rename="TARGETING_TYPE_CHANNEL")]
    TARGETINGTYPECHANNEL,
    

    /// Target an app category (for example, education or puzzle games).
    ///
    /// "TARGETING_TYPE_APP_CATEGORY"
    #[serde(rename="TARGETING_TYPE_APP_CATEGORY")]
    TARGETINGTYPEAPPCATEGORY,
    

    /// Target a specific app (for example, Angry Birds).
    ///
    /// "TARGETING_TYPE_APP"
    #[serde(rename="TARGETING_TYPE_APP")]
    TARGETINGTYPEAPP,
    

    /// Target a specific url (for example, quora.com).
    ///
    /// "TARGETING_TYPE_URL"
    #[serde(rename="TARGETING_TYPE_URL")]
    TARGETINGTYPEURL,
    

    /// Target ads during a chosen time period on a specific day.
    ///
    /// "TARGETING_TYPE_DAY_AND_TIME"
    #[serde(rename="TARGETING_TYPE_DAY_AND_TIME")]
    TARGETINGTYPEDAYANDTIME,
    

    /// Target ads to a specific age range (for example, 18-24).
    ///
    /// "TARGETING_TYPE_AGE_RANGE"
    #[serde(rename="TARGETING_TYPE_AGE_RANGE")]
    TARGETINGTYPEAGERANGE,
    

    /// Target ads to the specified regions on a regional location list.
    ///
    /// "TARGETING_TYPE_REGIONAL_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    TARGETINGTYPEREGIONALLOCATIONLIST,
    

    /// Target ads to the specified points of interest on a proximity location list.
    ///
    /// "TARGETING_TYPE_PROXIMITY_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    TARGETINGTYPEPROXIMITYLOCATIONLIST,
    

    /// Target ads to a specific gender (for example, female or male).
    ///
    /// "TARGETING_TYPE_GENDER"
    #[serde(rename="TARGETING_TYPE_GENDER")]
    TARGETINGTYPEGENDER,
    

    /// Target a specific video player size for video ads.
    ///
    /// "TARGETING_TYPE_VIDEO_PLAYER_SIZE"
    #[serde(rename="TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    TARGETINGTYPEVIDEOPLAYERSIZE,
    

    /// Target user rewarded content for video ads.
    ///
    /// "TARGETING_TYPE_USER_REWARDED_CONTENT"
    #[serde(rename="TARGETING_TYPE_USER_REWARDED_CONTENT")]
    TARGETINGTYPEUSERREWARDEDCONTENT,
    

    /// Target ads to a specific parental status (for example, parent or not a parent).
    ///
    /// "TARGETING_TYPE_PARENTAL_STATUS"
    #[serde(rename="TARGETING_TYPE_PARENTAL_STATUS")]
    TARGETINGTYPEPARENTALSTATUS,
    

    /// Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll).
    ///
    /// "TARGETING_TYPE_CONTENT_INSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    TARGETINGTYPECONTENTINSTREAMPOSITION,
    

    /// Target ads in a specific content outstream position.
    ///
    /// "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    TARGETINGTYPECONTENTOUTSTREAMPOSITION,
    

    /// Target ads to a specific device type (for example, tablet or connected TV).
    ///
    /// "TARGETING_TYPE_DEVICE_TYPE"
    #[serde(rename="TARGETING_TYPE_DEVICE_TYPE")]
    TARGETINGTYPEDEVICETYPE,
    

    /// Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time.
    ///
    /// "TARGETING_TYPE_AUDIENCE_GROUP"
    #[serde(rename="TARGETING_TYPE_AUDIENCE_GROUP")]
    TARGETINGTYPEAUDIENCEGROUP,
    

    /// Target ads to specific web browsers (for example, Chrome).
    ///
    /// "TARGETING_TYPE_BROWSER"
    #[serde(rename="TARGETING_TYPE_BROWSER")]
    TARGETINGTYPEBROWSER,
    

    /// Target ads to a specific household income range (for example, top 10%).
    ///
    /// "TARGETING_TYPE_HOUSEHOLD_INCOME"
    #[serde(rename="TARGETING_TYPE_HOUSEHOLD_INCOME")]
    TARGETINGTYPEHOUSEHOLDINCOME,
    

    /// Target ads in a specific on screen position.
    ///
    /// "TARGETING_TYPE_ON_SCREEN_POSITION"
    #[serde(rename="TARGETING_TYPE_ON_SCREEN_POSITION")]
    TARGETINGTYPEONSCREENPOSITION,
    

    /// Filter web sites through third party verification (for example, IAS or DoubleVerify).
    ///
    /// "TARGETING_TYPE_THIRD_PARTY_VERIFIER"
    #[serde(rename="TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    TARGETINGTYPETHIRDPARTYVERIFIER,
    

    /// Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences).
    ///
    /// "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION,
    

    /// Filter website content by sensitive categories (for example, adult).
    ///
    /// "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    TARGETINGTYPESENSITIVECATEGORYEXCLUSION,
    

    /// Target ads to a specific environment (for example, web or app).
    ///
    /// "TARGETING_TYPE_ENVIRONMENT"
    #[serde(rename="TARGETING_TYPE_ENVIRONMENT")]
    TARGETINGTYPEENVIRONMENT,
    

    /// Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange).
    ///
    /// "TARGETING_TYPE_CARRIER_AND_ISP"
    #[serde(rename="TARGETING_TYPE_CARRIER_AND_ISP")]
    TARGETINGTYPECARRIERANDISP,
    

    /// Target ads to a specific operating system (for example, macOS).
    ///
    /// "TARGETING_TYPE_OPERATING_SYSTEM"
    #[serde(rename="TARGETING_TYPE_OPERATING_SYSTEM")]
    TARGETINGTYPEOPERATINGSYSTEM,
    

    /// Target ads to a specific device make or model (for example, Roku or Samsung).
    ///
    /// "TARGETING_TYPE_DEVICE_MAKE_MODEL"
    #[serde(rename="TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    TARGETINGTYPEDEVICEMAKEMODEL,
    

    /// Target ads to a specific keyword (for example, dog or retriever).
    ///
    /// "TARGETING_TYPE_KEYWORD"
    #[serde(rename="TARGETING_TYPE_KEYWORD")]
    TARGETINGTYPEKEYWORD,
    

    /// Target ads to a specific negative keyword list.
    ///
    /// "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST"
    #[serde(rename="TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    TARGETINGTYPENEGATIVEKEYWORDLIST,
    

    /// Target ads to a specific viewability (for example, 80% viewable).
    ///
    /// "TARGETING_TYPE_VIEWABILITY"
    #[serde(rename="TARGETING_TYPE_VIEWABILITY")]
    TARGETINGTYPEVIEWABILITY,
    

    /// Target ads to a specific content category (for example, arts & entertainment).
    ///
    /// "TARGETING_TYPE_CATEGORY"
    #[serde(rename="TARGETING_TYPE_CATEGORY")]
    TARGETINGTYPECATEGORY,
    

    /// Purchase impressions from specific deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE")]
    TARGETINGTYPEINVENTORYSOURCE,
    

    /// Target ads to a specific language (for example, English or Japanese).
    ///
    /// "TARGETING_TYPE_LANGUAGE"
    #[serde(rename="TARGETING_TYPE_LANGUAGE")]
    TARGETINGTYPELANGUAGE,
    

    /// Target ads to ads.txt authorized sellers. If no targeting option of this type is assigned, the resource uses the "Authorized Direct Sellers and Resellers" option by default.
    ///
    /// "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS"
    #[serde(rename="TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    TARGETINGTYPEAUTHORIZEDSELLERSTATUS,
    

    /// Target ads to a specific regional location (for example, a city or state).
    ///
    /// "TARGETING_TYPE_GEO_REGION"
    #[serde(rename="TARGETING_TYPE_GEO_REGION")]
    TARGETINGTYPEGEOREGION,
    

    /// Purchase impressions from a group of deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE_GROUP"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    TARGETINGTYPEINVENTORYSOURCEGROUP,
    

    /// Purchase impressions from specific exchanges.
    ///
    /// "TARGETING_TYPE_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_EXCHANGE")]
    TARGETINGTYPEEXCHANGE,
    

    /// Purchase impressions from specific sub-exchanges.
    ///
    /// "TARGETING_TYPE_SUB_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_SUB_EXCHANGE")]
    TARGETINGTYPESUBEXCHANGE,
    

    /// Target ads around a specific point of interest, such as a notable building, a street address, or latitude/longitude coordinates.
    ///
    /// "TARGETING_TYPE_POI"
    #[serde(rename="TARGETING_TYPE_POI")]
    TARGETINGTYPEPOI,
    

    /// Target ads around locations of a business chain within a specific geo region.
    ///
    /// "TARGETING_TYPE_BUSINESS_CHAIN"
    #[serde(rename="TARGETING_TYPE_BUSINESS_CHAIN")]
    TARGETINGTYPEBUSINESSCHAIN,
    

    /// Target ads to a specific video content duration.
    ///
    /// "TARGETING_TYPE_CONTENT_DURATION"
    #[serde(rename="TARGETING_TYPE_CONTENT_DURATION")]
    TARGETINGTYPECONTENTDURATION,
    

    /// Target ads to a specific video content stream type.
    ///
    /// "TARGETING_TYPE_CONTENT_STREAM_TYPE"
    #[serde(rename="TARGETING_TYPE_CONTENT_STREAM_TYPE")]
    TARGETINGTYPECONTENTSTREAMTYPE,
    

    /// Target ads to a specific native content position.
    ///
    /// "TARGETING_TYPE_NATIVE_CONTENT_POSITION"
    #[serde(rename="TARGETING_TYPE_NATIVE_CONTENT_POSITION")]
    TARGETINGTYPENATIVECONTENTPOSITION,
    

    /// Target ads in an Open Measurement enabled inventory.
    ///
    /// "TARGETING_TYPE_OMID"
    #[serde(rename="TARGETING_TYPE_OMID")]
    TARGETINGTYPEOMID,
    

    /// Target ads to a specific audio content type.
    ///
    /// "TARGETING_TYPE_AUDIO_CONTENT_TYPE"
    #[serde(rename="TARGETING_TYPE_AUDIO_CONTENT_TYPE")]
    TARGETINGTYPEAUDIOCONTENTTYPE,
    

    /// Target ads to a specific content genre.
    ///
    /// "TARGETING_TYPE_CONTENT_GENRE"
    #[serde(rename="TARGETING_TYPE_CONTENT_GENRE")]
    TARGETINGTYPECONTENTGENRE,
}

impl AsRef<str> for DeleteAssignedTargetingOptionsRequestTargetingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED => "TARGETING_TYPE_UNSPECIFIED",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECHANNEL => "TARGETING_TYPE_CHANNEL",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY => "TARGETING_TYPE_APP_CATEGORY",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAPP => "TARGETING_TYPE_APP",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEURL => "TARGETING_TYPE_URL",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDAYANDTIME => "TARGETING_TYPE_DAY_AND_TIME",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAGERANGE => "TARGETING_TYPE_AGE_RANGE",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST => "TARGETING_TYPE_REGIONAL_LOCATION_LIST",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST => "TARGETING_TYPE_PROXIMITY_LOCATION_LIST",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEGENDER => "TARGETING_TYPE_GENDER",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE => "TARGETING_TYPE_VIDEO_PLAYER_SIZE",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT => "TARGETING_TYPE_USER_REWARDED_CONTENT",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS => "TARGETING_TYPE_PARENTAL_STATUS",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION => "TARGETING_TYPE_CONTENT_INSTREAM_POSITION",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION => "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDEVICETYPE => "TARGETING_TYPE_DEVICE_TYPE",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP => "TARGETING_TYPE_AUDIENCE_GROUP",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEBROWSER => "TARGETING_TYPE_BROWSER",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME => "TARGETING_TYPE_HOUSEHOLD_INCOME",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION => "TARGETING_TYPE_ON_SCREEN_POSITION",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER => "TARGETING_TYPE_THIRD_PARTY_VERIFIER",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION => "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION => "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEENVIRONMENT => "TARGETING_TYPE_ENVIRONMENT",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECARRIERANDISP => "TARGETING_TYPE_CARRIER_AND_ISP",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM => "TARGETING_TYPE_OPERATING_SYSTEM",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL => "TARGETING_TYPE_DEVICE_MAKE_MODEL",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEKEYWORD => "TARGETING_TYPE_KEYWORD",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST => "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEVIEWABILITY => "TARGETING_TYPE_VIEWABILITY",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECATEGORY => "TARGETING_TYPE_CATEGORY",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE => "TARGETING_TYPE_INVENTORY_SOURCE",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPELANGUAGE => "TARGETING_TYPE_LANGUAGE",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS => "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEGEOREGION => "TARGETING_TYPE_GEO_REGION",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP => "TARGETING_TYPE_INVENTORY_SOURCE_GROUP",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEEXCHANGE => "TARGETING_TYPE_EXCHANGE",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE => "TARGETING_TYPE_SUB_EXCHANGE",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPOI => "TARGETING_TYPE_POI",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN => "TARGETING_TYPE_BUSINESS_CHAIN",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTDURATION => "TARGETING_TYPE_CONTENT_DURATION",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE => "TARGETING_TYPE_CONTENT_STREAM_TYPE",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION => "TARGETING_TYPE_NATIVE_CONTENT_POSITION",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEOMID => "TARGETING_TYPE_OMID",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE => "TARGETING_TYPE_AUDIO_CONTENT_TYPE",
            DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTGENRE => "TARGETING_TYPE_CONTENT_GENRE",
        }
    }
}

impl std::convert::TryFrom< &str> for DeleteAssignedTargetingOptionsRequestTargetingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_TYPE_UNSPECIFIED" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED),
           "TARGETING_TYPE_CHANNEL" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECHANNEL),
           "TARGETING_TYPE_APP_CATEGORY" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY),
           "TARGETING_TYPE_APP" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAPP),
           "TARGETING_TYPE_URL" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEURL),
           "TARGETING_TYPE_DAY_AND_TIME" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDAYANDTIME),
           "TARGETING_TYPE_AGE_RANGE" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAGERANGE),
           "TARGETING_TYPE_REGIONAL_LOCATION_LIST" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST),
           "TARGETING_TYPE_PROXIMITY_LOCATION_LIST" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST),
           "TARGETING_TYPE_GENDER" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEGENDER),
           "TARGETING_TYPE_VIDEO_PLAYER_SIZE" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE),
           "TARGETING_TYPE_USER_REWARDED_CONTENT" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT),
           "TARGETING_TYPE_PARENTAL_STATUS" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS),
           "TARGETING_TYPE_CONTENT_INSTREAM_POSITION" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION),
           "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION),
           "TARGETING_TYPE_DEVICE_TYPE" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDEVICETYPE),
           "TARGETING_TYPE_AUDIENCE_GROUP" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP),
           "TARGETING_TYPE_BROWSER" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEBROWSER),
           "TARGETING_TYPE_HOUSEHOLD_INCOME" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME),
           "TARGETING_TYPE_ON_SCREEN_POSITION" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION),
           "TARGETING_TYPE_THIRD_PARTY_VERIFIER" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER),
           "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION),
           "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION),
           "TARGETING_TYPE_ENVIRONMENT" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEENVIRONMENT),
           "TARGETING_TYPE_CARRIER_AND_ISP" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECARRIERANDISP),
           "TARGETING_TYPE_OPERATING_SYSTEM" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM),
           "TARGETING_TYPE_DEVICE_MAKE_MODEL" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL),
           "TARGETING_TYPE_KEYWORD" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEKEYWORD),
           "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST),
           "TARGETING_TYPE_VIEWABILITY" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEVIEWABILITY),
           "TARGETING_TYPE_CATEGORY" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECATEGORY),
           "TARGETING_TYPE_INVENTORY_SOURCE" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE),
           "TARGETING_TYPE_LANGUAGE" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPELANGUAGE),
           "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS),
           "TARGETING_TYPE_GEO_REGION" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEGEOREGION),
           "TARGETING_TYPE_INVENTORY_SOURCE_GROUP" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP),
           "TARGETING_TYPE_EXCHANGE" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEEXCHANGE),
           "TARGETING_TYPE_SUB_EXCHANGE" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE),
           "TARGETING_TYPE_POI" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEPOI),
           "TARGETING_TYPE_BUSINESS_CHAIN" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN),
           "TARGETING_TYPE_CONTENT_DURATION" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTDURATION),
           "TARGETING_TYPE_CONTENT_STREAM_TYPE" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE),
           "TARGETING_TYPE_NATIVE_CONTENT_POSITION" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION),
           "TARGETING_TYPE_OMID" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEOMID),
           "TARGETING_TYPE_AUDIO_CONTENT_TYPE" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE),
           "TARGETING_TYPE_CONTENT_GENRE" => Ok(DeleteAssignedTargetingOptionsRequestTargetingTypeEnum::TARGETINGTYPECONTENTGENRE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeleteAssignedTargetingOptionsRequestTargetingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The display name of the device type.
pub enum DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum {
    

    /// Default value when device type is not specified in this version. This enum is a placeholder for default value and does not represent a real device type option.
    ///
    /// "DEVICE_TYPE_UNSPECIFIED"
    #[serde(rename="DEVICE_TYPE_UNSPECIFIED")]
    DEVICETYPEUNSPECIFIED,
    

    /// Computer.
    ///
    /// "DEVICE_TYPE_COMPUTER"
    #[serde(rename="DEVICE_TYPE_COMPUTER")]
    DEVICETYPECOMPUTER,
    

    /// Connected TV.
    ///
    /// "DEVICE_TYPE_CONNECTED_TV"
    #[serde(rename="DEVICE_TYPE_CONNECTED_TV")]
    DEVICETYPECONNECTEDTV,
    

    /// Smart phone.
    ///
    /// "DEVICE_TYPE_SMART_PHONE"
    #[serde(rename="DEVICE_TYPE_SMART_PHONE")]
    DEVICETYPESMARTPHONE,
    

    /// Tablet.
    ///
    /// "DEVICE_TYPE_TABLET"
    #[serde(rename="DEVICE_TYPE_TABLET")]
    DEVICETYPETABLET,
}

impl AsRef<str> for DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum::DEVICETYPEUNSPECIFIED => "DEVICE_TYPE_UNSPECIFIED",
            DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum::DEVICETYPECOMPUTER => "DEVICE_TYPE_COMPUTER",
            DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum::DEVICETYPECONNECTEDTV => "DEVICE_TYPE_CONNECTED_TV",
            DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum::DEVICETYPESMARTPHONE => "DEVICE_TYPE_SMART_PHONE",
            DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum::DEVICETYPETABLET => "DEVICE_TYPE_TABLET",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_TYPE_UNSPECIFIED" => Ok(DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum::DEVICETYPEUNSPECIFIED),
           "DEVICE_TYPE_COMPUTER" => Ok(DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum::DEVICETYPECOMPUTER),
           "DEVICE_TYPE_CONNECTED_TV" => Ok(DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum::DEVICETYPECONNECTEDTV),
           "DEVICE_TYPE_SMART_PHONE" => Ok(DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum::DEVICETYPESMARTPHONE),
           "DEVICE_TYPE_TABLET" => Ok(DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum::DEVICETYPETABLET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceTypeAssignedTargetingOptionDetailDeviceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceTypeTargetingOptionDetailDeviceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The device type that is used to be targeted.
pub enum DeviceTypeTargetingOptionDetailDeviceTypeEnum {
    

    /// Default value when device type is not specified in this version. This enum is a placeholder for default value and does not represent a real device type option.
    ///
    /// "DEVICE_TYPE_UNSPECIFIED"
    #[serde(rename="DEVICE_TYPE_UNSPECIFIED")]
    DEVICETYPEUNSPECIFIED,
    

    /// Computer.
    ///
    /// "DEVICE_TYPE_COMPUTER"
    #[serde(rename="DEVICE_TYPE_COMPUTER")]
    DEVICETYPECOMPUTER,
    

    /// Connected TV.
    ///
    /// "DEVICE_TYPE_CONNECTED_TV"
    #[serde(rename="DEVICE_TYPE_CONNECTED_TV")]
    DEVICETYPECONNECTEDTV,
    

    /// Smart phone.
    ///
    /// "DEVICE_TYPE_SMART_PHONE"
    #[serde(rename="DEVICE_TYPE_SMART_PHONE")]
    DEVICETYPESMARTPHONE,
    

    /// Tablet.
    ///
    /// "DEVICE_TYPE_TABLET"
    #[serde(rename="DEVICE_TYPE_TABLET")]
    DEVICETYPETABLET,
}

impl AsRef<str> for DeviceTypeTargetingOptionDetailDeviceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceTypeTargetingOptionDetailDeviceTypeEnum::DEVICETYPEUNSPECIFIED => "DEVICE_TYPE_UNSPECIFIED",
            DeviceTypeTargetingOptionDetailDeviceTypeEnum::DEVICETYPECOMPUTER => "DEVICE_TYPE_COMPUTER",
            DeviceTypeTargetingOptionDetailDeviceTypeEnum::DEVICETYPECONNECTEDTV => "DEVICE_TYPE_CONNECTED_TV",
            DeviceTypeTargetingOptionDetailDeviceTypeEnum::DEVICETYPESMARTPHONE => "DEVICE_TYPE_SMART_PHONE",
            DeviceTypeTargetingOptionDetailDeviceTypeEnum::DEVICETYPETABLET => "DEVICE_TYPE_TABLET",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceTypeTargetingOptionDetailDeviceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_TYPE_UNSPECIFIED" => Ok(DeviceTypeTargetingOptionDetailDeviceTypeEnum::DEVICETYPEUNSPECIFIED),
           "DEVICE_TYPE_COMPUTER" => Ok(DeviceTypeTargetingOptionDetailDeviceTypeEnum::DEVICETYPECOMPUTER),
           "DEVICE_TYPE_CONNECTED_TV" => Ok(DeviceTypeTargetingOptionDetailDeviceTypeEnum::DEVICETYPECONNECTEDTV),
           "DEVICE_TYPE_SMART_PHONE" => Ok(DeviceTypeTargetingOptionDetailDeviceTypeEnum::DEVICETYPESMARTPHONE),
           "DEVICE_TYPE_TABLET" => Ok(DeviceTypeTargetingOptionDetailDeviceTypeEnum::DEVICETYPETABLET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceTypeTargetingOptionDetailDeviceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The display name of the digital content label rating tier.
pub enum DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum {
    

    /// Content label is not specified in this version. This enum is a place holder for a default value and does not represent a real content rating.
    ///
    /// "CONTENT_RATING_TIER_UNSPECIFIED"
    #[serde(rename="CONTENT_RATING_TIER_UNSPECIFIED")]
    CONTENTRATINGTIERUNSPECIFIED,
    

    /// Content that has not been labeled.
    ///
    /// "CONTENT_RATING_TIER_UNRATED"
    #[serde(rename="CONTENT_RATING_TIER_UNRATED")]
    CONTENTRATINGTIERUNRATED,
    

    /// Content suitable for general audiences.
    ///
    /// "CONTENT_RATING_TIER_GENERAL"
    #[serde(rename="CONTENT_RATING_TIER_GENERAL")]
    CONTENTRATINGTIERGENERAL,
    

    /// Content suitable for most audiences with parental guidance.
    ///
    /// "CONTENT_RATING_TIER_PARENTAL_GUIDANCE"
    #[serde(rename="CONTENT_RATING_TIER_PARENTAL_GUIDANCE")]
    CONTENTRATINGTIERPARENTALGUIDANCE,
    

    /// Content suitable for teen and older audiences.
    ///
    /// "CONTENT_RATING_TIER_TEENS"
    #[serde(rename="CONTENT_RATING_TIER_TEENS")]
    CONTENTRATINGTIERTEENS,
    

    /// Content suitable only for mature audiences.
    ///
    /// "CONTENT_RATING_TIER_MATURE"
    #[serde(rename="CONTENT_RATING_TIER_MATURE")]
    CONTENTRATINGTIERMATURE,
}

impl AsRef<str> for DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERUNSPECIFIED => "CONTENT_RATING_TIER_UNSPECIFIED",
            DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERUNRATED => "CONTENT_RATING_TIER_UNRATED",
            DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERGENERAL => "CONTENT_RATING_TIER_GENERAL",
            DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERPARENTALGUIDANCE => "CONTENT_RATING_TIER_PARENTAL_GUIDANCE",
            DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERTEENS => "CONTENT_RATING_TIER_TEENS",
            DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERMATURE => "CONTENT_RATING_TIER_MATURE",
        }
    }
}

impl std::convert::TryFrom< &str> for DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_RATING_TIER_UNSPECIFIED" => Ok(DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERUNSPECIFIED),
           "CONTENT_RATING_TIER_UNRATED" => Ok(DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERUNRATED),
           "CONTENT_RATING_TIER_GENERAL" => Ok(DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERGENERAL),
           "CONTENT_RATING_TIER_PARENTAL_GUIDANCE" => Ok(DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERPARENTALGUIDANCE),
           "CONTENT_RATING_TIER_TEENS" => Ok(DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERTEENS),
           "CONTENT_RATING_TIER_MATURE" => Ok(DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERMATURE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DigitalContentLabelAssignedTargetingOptionDetailContentRatingTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DigitalContentLabelTargetingOptionDetailContentRatingTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. An enum for the content label brand safety tiers.
pub enum DigitalContentLabelTargetingOptionDetailContentRatingTierEnum {
    

    /// Content label is not specified in this version. This enum is a place holder for a default value and does not represent a real content rating.
    ///
    /// "CONTENT_RATING_TIER_UNSPECIFIED"
    #[serde(rename="CONTENT_RATING_TIER_UNSPECIFIED")]
    CONTENTRATINGTIERUNSPECIFIED,
    

    /// Content that has not been labeled.
    ///
    /// "CONTENT_RATING_TIER_UNRATED"
    #[serde(rename="CONTENT_RATING_TIER_UNRATED")]
    CONTENTRATINGTIERUNRATED,
    

    /// Content suitable for general audiences.
    ///
    /// "CONTENT_RATING_TIER_GENERAL"
    #[serde(rename="CONTENT_RATING_TIER_GENERAL")]
    CONTENTRATINGTIERGENERAL,
    

    /// Content suitable for most audiences with parental guidance.
    ///
    /// "CONTENT_RATING_TIER_PARENTAL_GUIDANCE"
    #[serde(rename="CONTENT_RATING_TIER_PARENTAL_GUIDANCE")]
    CONTENTRATINGTIERPARENTALGUIDANCE,
    

    /// Content suitable for teen and older audiences.
    ///
    /// "CONTENT_RATING_TIER_TEENS"
    #[serde(rename="CONTENT_RATING_TIER_TEENS")]
    CONTENTRATINGTIERTEENS,
    

    /// Content suitable only for mature audiences.
    ///
    /// "CONTENT_RATING_TIER_MATURE"
    #[serde(rename="CONTENT_RATING_TIER_MATURE")]
    CONTENTRATINGTIERMATURE,
}

impl AsRef<str> for DigitalContentLabelTargetingOptionDetailContentRatingTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERUNSPECIFIED => "CONTENT_RATING_TIER_UNSPECIFIED",
            DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERUNRATED => "CONTENT_RATING_TIER_UNRATED",
            DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERGENERAL => "CONTENT_RATING_TIER_GENERAL",
            DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERPARENTALGUIDANCE => "CONTENT_RATING_TIER_PARENTAL_GUIDANCE",
            DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERTEENS => "CONTENT_RATING_TIER_TEENS",
            DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERMATURE => "CONTENT_RATING_TIER_MATURE",
        }
    }
}

impl std::convert::TryFrom< &str> for DigitalContentLabelTargetingOptionDetailContentRatingTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_RATING_TIER_UNSPECIFIED" => Ok(DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERUNSPECIFIED),
           "CONTENT_RATING_TIER_UNRATED" => Ok(DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERUNRATED),
           "CONTENT_RATING_TIER_GENERAL" => Ok(DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERGENERAL),
           "CONTENT_RATING_TIER_PARENTAL_GUIDANCE" => Ok(DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERPARENTALGUIDANCE),
           "CONTENT_RATING_TIER_TEENS" => Ok(DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERTEENS),
           "CONTENT_RATING_TIER_MATURE" => Ok(DigitalContentLabelTargetingOptionDetailContentRatingTierEnum::CONTENTRATINGTIERMATURE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DigitalContentLabelTargetingOptionDetailContentRatingTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleVerifyAvoidedAgeRatingsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Avoid bidding on apps with the age rating.
pub enum DoubleVerifyAvoidedAgeRatingsEnum {
    

    /// This enum is only a placeholder and it doesn't specify any age rating options.
    ///
    /// "AGE_RATING_UNSPECIFIED"
    #[serde(rename="AGE_RATING_UNSPECIFIED")]
    AGERATINGUNSPECIFIED,
    

    /// Apps with unknown age rating.
    ///
    /// "APP_AGE_RATE_UNKNOWN"
    #[serde(rename="APP_AGE_RATE_UNKNOWN")]
    APPAGERATEUNKNOWN,
    

    /// Apps rated for Everyone (4+).
    ///
    /// "APP_AGE_RATE_4_PLUS"
    #[serde(rename="APP_AGE_RATE_4_PLUS")]
    APPAGERATE4PLUS,
    

    /// Apps rated for Everyone (9+).
    ///
    /// "APP_AGE_RATE_9_PLUS"
    #[serde(rename="APP_AGE_RATE_9_PLUS")]
    APPAGERATE9PLUS,
    

    /// Apps rated for Teens (12+).
    ///
    /// "APP_AGE_RATE_12_PLUS"
    #[serde(rename="APP_AGE_RATE_12_PLUS")]
    APPAGERATE12PLUS,
    

    /// Apps rated for Mature (17+).
    ///
    /// "APP_AGE_RATE_17_PLUS"
    #[serde(rename="APP_AGE_RATE_17_PLUS")]
    APPAGERATE17PLUS,
    

    /// Apps rated for Adults Only (18+).
    ///
    /// "APP_AGE_RATE_18_PLUS"
    #[serde(rename="APP_AGE_RATE_18_PLUS")]
    APPAGERATE18PLUS,
}

impl AsRef<str> for DoubleVerifyAvoidedAgeRatingsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleVerifyAvoidedAgeRatingsEnum::AGERATINGUNSPECIFIED => "AGE_RATING_UNSPECIFIED",
            DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATEUNKNOWN => "APP_AGE_RATE_UNKNOWN",
            DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATE4PLUS => "APP_AGE_RATE_4_PLUS",
            DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATE9PLUS => "APP_AGE_RATE_9_PLUS",
            DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATE12PLUS => "APP_AGE_RATE_12_PLUS",
            DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATE17PLUS => "APP_AGE_RATE_17_PLUS",
            DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATE18PLUS => "APP_AGE_RATE_18_PLUS",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleVerifyAvoidedAgeRatingsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AGE_RATING_UNSPECIFIED" => Ok(DoubleVerifyAvoidedAgeRatingsEnum::AGERATINGUNSPECIFIED),
           "APP_AGE_RATE_UNKNOWN" => Ok(DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATEUNKNOWN),
           "APP_AGE_RATE_4_PLUS" => Ok(DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATE4PLUS),
           "APP_AGE_RATE_9_PLUS" => Ok(DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATE9PLUS),
           "APP_AGE_RATE_12_PLUS" => Ok(DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATE12PLUS),
           "APP_AGE_RATE_17_PLUS" => Ok(DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATE17PLUS),
           "APP_AGE_RATE_18_PLUS" => Ok(DoubleVerifyAvoidedAgeRatingsEnum::APPAGERATE18PLUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleVerifyAvoidedAgeRatingsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleVerifyAppStarRatingAvoidedStarRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Avoid bidding on apps with the star ratings.
pub enum DoubleVerifyAppStarRatingAvoidedStarRatingEnum {
    

    /// This enum is only a placeholder and it doesn't specify any app star rating options.
    ///
    /// "APP_STAR_RATE_UNSPECIFIED"
    #[serde(rename="APP_STAR_RATE_UNSPECIFIED")]
    APPSTARRATEUNSPECIFIED,
    

    /// Official Apps with rating < 1.5 Stars.
    ///
    /// "APP_STAR_RATE_1_POINT_5_LESS"
    #[serde(rename="APP_STAR_RATE_1_POINT_5_LESS")]
    APPSTARRATE1POINT5LESS,
    

    /// Official Apps with rating < 2 Stars.
    ///
    /// "APP_STAR_RATE_2_LESS"
    #[serde(rename="APP_STAR_RATE_2_LESS")]
    APPSTARRATE2LESS,
    

    /// Official Apps with rating < 2.5 Stars.
    ///
    /// "APP_STAR_RATE_2_POINT_5_LESS"
    #[serde(rename="APP_STAR_RATE_2_POINT_5_LESS")]
    APPSTARRATE2POINT5LESS,
    

    /// Official Apps with rating < 3 Stars.
    ///
    /// "APP_STAR_RATE_3_LESS"
    #[serde(rename="APP_STAR_RATE_3_LESS")]
    APPSTARRATE3LESS,
    

    /// Official Apps with rating < 3.5 Stars.
    ///
    /// "APP_STAR_RATE_3_POINT_5_LESS"
    #[serde(rename="APP_STAR_RATE_3_POINT_5_LESS")]
    APPSTARRATE3POINT5LESS,
    

    /// Official Apps with rating < 4 Stars.
    ///
    /// "APP_STAR_RATE_4_LESS"
    #[serde(rename="APP_STAR_RATE_4_LESS")]
    APPSTARRATE4LESS,
    

    /// Official Apps with rating < 4.5 Stars.
    ///
    /// "APP_STAR_RATE_4_POINT_5_LESS"
    #[serde(rename="APP_STAR_RATE_4_POINT_5_LESS")]
    APPSTARRATE4POINT5LESS,
}

impl AsRef<str> for DoubleVerifyAppStarRatingAvoidedStarRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATEUNSPECIFIED => "APP_STAR_RATE_UNSPECIFIED",
            DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE1POINT5LESS => "APP_STAR_RATE_1_POINT_5_LESS",
            DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE2LESS => "APP_STAR_RATE_2_LESS",
            DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE2POINT5LESS => "APP_STAR_RATE_2_POINT_5_LESS",
            DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE3LESS => "APP_STAR_RATE_3_LESS",
            DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE3POINT5LESS => "APP_STAR_RATE_3_POINT_5_LESS",
            DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE4LESS => "APP_STAR_RATE_4_LESS",
            DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE4POINT5LESS => "APP_STAR_RATE_4_POINT_5_LESS",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleVerifyAppStarRatingAvoidedStarRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_STAR_RATE_UNSPECIFIED" => Ok(DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATEUNSPECIFIED),
           "APP_STAR_RATE_1_POINT_5_LESS" => Ok(DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE1POINT5LESS),
           "APP_STAR_RATE_2_LESS" => Ok(DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE2LESS),
           "APP_STAR_RATE_2_POINT_5_LESS" => Ok(DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE2POINT5LESS),
           "APP_STAR_RATE_3_LESS" => Ok(DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE3LESS),
           "APP_STAR_RATE_3_POINT_5_LESS" => Ok(DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE3POINT5LESS),
           "APP_STAR_RATE_4_LESS" => Ok(DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE4LESS),
           "APP_STAR_RATE_4_POINT_5_LESS" => Ok(DoubleVerifyAppStarRatingAvoidedStarRatingEnum::APPSTARRATE4POINT5LESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleVerifyAppStarRatingAvoidedStarRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Brand safety high severity avoidance categories.
pub enum DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum {
    

    /// This enum is only a placeholder and it doesn't specify any high severity categories.
    ///
    /// "HIGHER_SEVERITY_UNSPECIFIED"
    #[serde(rename="HIGHER_SEVERITY_UNSPECIFIED")]
    HIGHERSEVERITYUNSPECIFIED,
    

    /// Adult Content: Pornography, Mature Topics & Nudity.
    ///
    /// "ADULT_CONTENT_PORNOGRAPHY"
    #[serde(rename="ADULT_CONTENT_PORNOGRAPHY")]
    ADULTCONTENTPORNOGRAPHY,
    

    /// Copyright Infringement.
    ///
    /// "COPYRIGHT_INFRINGEMENT"
    #[serde(rename="COPYRIGHT_INFRINGEMENT")]
    COPYRIGHTINFRINGEMENT,
    

    /// Drugs/Alcohol/Controlled Substances: Substance Abuse.
    ///
    /// "SUBSTANCE_ABUSE"
    #[serde(rename="SUBSTANCE_ABUSE")]
    SUBSTANCEABUSE,
    

    /// Extreme Graphic/Explicit Violence/Weapons.
    ///
    /// "GRAPHIC_VIOLENCE_WEAPONS"
    #[serde(rename="GRAPHIC_VIOLENCE_WEAPONS")]
    GRAPHICVIOLENCEWEAPONS,
    

    /// Hate/Profanity.
    ///
    /// "HATE_PROFANITY"
    #[serde(rename="HATE_PROFANITY")]
    HATEPROFANITY,
    

    /// Illegal Activities: Criminal Skills.
    ///
    /// "CRIMINAL_SKILLS"
    #[serde(rename="CRIMINAL_SKILLS")]
    CRIMINALSKILLS,
    

    /// Incentivized/Malware/Clutter.
    ///
    /// "NUISANCE_INCENTIVIZED_MALWARE_CLUTTER"
    #[serde(rename="NUISANCE_INCENTIVIZED_MALWARE_CLUTTER")]
    NUISANCEINCENTIVIZEDMALWARECLUTTER,
}

impl AsRef<str> for DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::HIGHERSEVERITYUNSPECIFIED => "HIGHER_SEVERITY_UNSPECIFIED",
            DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::ADULTCONTENTPORNOGRAPHY => "ADULT_CONTENT_PORNOGRAPHY",
            DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::COPYRIGHTINFRINGEMENT => "COPYRIGHT_INFRINGEMENT",
            DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::SUBSTANCEABUSE => "SUBSTANCE_ABUSE",
            DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::GRAPHICVIOLENCEWEAPONS => "GRAPHIC_VIOLENCE_WEAPONS",
            DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::HATEPROFANITY => "HATE_PROFANITY",
            DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::CRIMINALSKILLS => "CRIMINAL_SKILLS",
            DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::NUISANCEINCENTIVIZEDMALWARECLUTTER => "NUISANCE_INCENTIVIZED_MALWARE_CLUTTER",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HIGHER_SEVERITY_UNSPECIFIED" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::HIGHERSEVERITYUNSPECIFIED),
           "ADULT_CONTENT_PORNOGRAPHY" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::ADULTCONTENTPORNOGRAPHY),
           "COPYRIGHT_INFRINGEMENT" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::COPYRIGHTINFRINGEMENT),
           "SUBSTANCE_ABUSE" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::SUBSTANCEABUSE),
           "GRAPHIC_VIOLENCE_WEAPONS" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::GRAPHICVIOLENCEWEAPONS),
           "HATE_PROFANITY" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::HATEPROFANITY),
           "CRIMINAL_SKILLS" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::CRIMINALSKILLS),
           "NUISANCE_INCENTIVIZED_MALWARE_CLUTTER" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum::NUISANCEINCENTIVIZEDMALWARECLUTTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleVerifyBrandSafetyCategoryAvoidedHighSeverityCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Brand safety medium severity avoidance categories.
pub enum DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum {
    

    /// This enum is only a placeholder and it doesn't specify any medium severity categories.
    ///
    /// "MEDIUM_SEVERITY_UNSPECIFIED"
    #[serde(rename="MEDIUM_SEVERITY_UNSPECIFIED")]
    MEDIUMSEVERITYUNSPECIFIED,
    

    /// Ad Servers.
    ///
    /// "AD_SERVERS"
    #[serde(rename="AD_SERVERS")]
    ADSERVERS,
    

    /// Adult Content: Swimsuit.
    ///
    /// "ADULT_CONTENT_SWIMSUIT"
    #[serde(rename="ADULT_CONTENT_SWIMSUIT")]
    ADULTCONTENTSWIMSUIT,
    

    /// Controversial Subjects: Alternative Lifestyles.
    ///
    /// "ALTERNATIVE_LIFESTYLES"
    #[serde(rename="ALTERNATIVE_LIFESTYLES")]
    ALTERNATIVELIFESTYLES,
    

    /// Controversial Subjects: Celebrity Gossip.
    ///
    /// "CELEBRITY_GOSSIP"
    #[serde(rename="CELEBRITY_GOSSIP")]
    CELEBRITYGOSSIP,
    

    /// Controversial Subjects: Gambling.
    ///
    /// "GAMBLING"
    #[serde(rename="GAMBLING")]
    GAMBLING,
    

    /// Controversial Subjects: Occult.
    ///
    /// "OCCULT"
    #[serde(rename="OCCULT")]
    OCCULT,
    

    /// Controversial Subjects: Sex Education.
    ///
    /// "SEX_EDUCATION"
    #[serde(rename="SEX_EDUCATION")]
    SEXEDUCATION,
    

    /// Disaster: Aviation.
    ///
    /// "DISASTER_AVIATION"
    #[serde(rename="DISASTER_AVIATION")]
    DISASTERAVIATION,
    

    /// Disaster: Man-made.
    ///
    /// "DISASTER_MAN_MADE"
    #[serde(rename="DISASTER_MAN_MADE")]
    DISASTERMANMADE,
    

    /// Disaster: Natural.
    ///
    /// "DISASTER_NATURAL"
    #[serde(rename="DISASTER_NATURAL")]
    DISASTERNATURAL,
    

    /// Disaster: Terrorist Events.
    ///
    /// "DISASTER_TERRORIST_EVENTS"
    #[serde(rename="DISASTER_TERRORIST_EVENTS")]
    DISASTERTERRORISTEVENTS,
    

    /// Disaster: Vehicle.
    ///
    /// "DISASTER_VEHICLE"
    #[serde(rename="DISASTER_VEHICLE")]
    DISASTERVEHICLE,
    

    /// Drugs/Alcohol/Controlled Substances: Alcohol.
    ///
    /// "ALCOHOL"
    #[serde(rename="ALCOHOL")]
    ALCOHOL,
    

    /// Drugs/Alcohol/Controlled Substances: Smoking.
    ///
    /// "SMOKING"
    #[serde(rename="SMOKING")]
    SMOKING,
    

    /// Negative News: Financial.
    ///
    /// "NEGATIVE_NEWS_FINANCIAL"
    #[serde(rename="NEGATIVE_NEWS_FINANCIAL")]
    NEGATIVENEWSFINANCIAL,
    

    /// Non-Std Content: Non-English.
    ///
    /// "NON_ENGLISH"
    #[serde(rename="NON_ENGLISH")]
    NONENGLISH,
    

    /// Non-Std Content: Parking Page.
    ///
    /// "PARKING_PAGE"
    #[serde(rename="PARKING_PAGE")]
    PARKINGPAGE,
    

    /// Unmoderated UGC: Forums, Images & Video.
    ///
    /// "UNMODERATED_UGC"
    #[serde(rename="UNMODERATED_UGC")]
    UNMODERATEDUGC,
    

    /// Controversial Subjects: Inflammatory Politics and News.
    ///
    /// "INFLAMMATORY_POLITICS_AND_NEWS"
    #[serde(rename="INFLAMMATORY_POLITICS_AND_NEWS")]
    INFLAMMATORYPOLITICSANDNEWS,
    

    /// Negative News: Pharmaceutical.
    ///
    /// "NEGATIVE_NEWS_PHARMACEUTICAL"
    #[serde(rename="NEGATIVE_NEWS_PHARMACEUTICAL")]
    NEGATIVENEWSPHARMACEUTICAL,
}

impl AsRef<str> for DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::MEDIUMSEVERITYUNSPECIFIED => "MEDIUM_SEVERITY_UNSPECIFIED",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::ADSERVERS => "AD_SERVERS",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::ADULTCONTENTSWIMSUIT => "ADULT_CONTENT_SWIMSUIT",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::ALTERNATIVELIFESTYLES => "ALTERNATIVE_LIFESTYLES",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::CELEBRITYGOSSIP => "CELEBRITY_GOSSIP",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::GAMBLING => "GAMBLING",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::OCCULT => "OCCULT",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::SEXEDUCATION => "SEX_EDUCATION",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::DISASTERAVIATION => "DISASTER_AVIATION",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::DISASTERMANMADE => "DISASTER_MAN_MADE",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::DISASTERNATURAL => "DISASTER_NATURAL",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::DISASTERTERRORISTEVENTS => "DISASTER_TERRORIST_EVENTS",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::DISASTERVEHICLE => "DISASTER_VEHICLE",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::ALCOHOL => "ALCOHOL",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::SMOKING => "SMOKING",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::NEGATIVENEWSFINANCIAL => "NEGATIVE_NEWS_FINANCIAL",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::NONENGLISH => "NON_ENGLISH",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::PARKINGPAGE => "PARKING_PAGE",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::UNMODERATEDUGC => "UNMODERATED_UGC",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::INFLAMMATORYPOLITICSANDNEWS => "INFLAMMATORY_POLITICS_AND_NEWS",
            DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::NEGATIVENEWSPHARMACEUTICAL => "NEGATIVE_NEWS_PHARMACEUTICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEDIUM_SEVERITY_UNSPECIFIED" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::MEDIUMSEVERITYUNSPECIFIED),
           "AD_SERVERS" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::ADSERVERS),
           "ADULT_CONTENT_SWIMSUIT" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::ADULTCONTENTSWIMSUIT),
           "ALTERNATIVE_LIFESTYLES" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::ALTERNATIVELIFESTYLES),
           "CELEBRITY_GOSSIP" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::CELEBRITYGOSSIP),
           "GAMBLING" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::GAMBLING),
           "OCCULT" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::OCCULT),
           "SEX_EDUCATION" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::SEXEDUCATION),
           "DISASTER_AVIATION" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::DISASTERAVIATION),
           "DISASTER_MAN_MADE" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::DISASTERMANMADE),
           "DISASTER_NATURAL" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::DISASTERNATURAL),
           "DISASTER_TERRORIST_EVENTS" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::DISASTERTERRORISTEVENTS),
           "DISASTER_VEHICLE" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::DISASTERVEHICLE),
           "ALCOHOL" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::ALCOHOL),
           "SMOKING" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::SMOKING),
           "NEGATIVE_NEWS_FINANCIAL" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::NEGATIVENEWSFINANCIAL),
           "NON_ENGLISH" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::NONENGLISH),
           "PARKING_PAGE" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::PARKINGPAGE),
           "UNMODERATED_UGC" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::UNMODERATEDUGC),
           "INFLAMMATORY_POLITICS_AND_NEWS" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::INFLAMMATORYPOLITICSANDNEWS),
           "NEGATIVE_NEWS_PHARMACEUTICAL" => Ok(DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum::NEGATIVENEWSPHARMACEUTICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleVerifyBrandSafetyCategoryAvoidedMediumSeverityCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleVerifyDisplayViewabilityIabEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Target web and app inventory to maximize IAB viewable rate.
pub enum DoubleVerifyDisplayViewabilityIabEnum {
    

    /// This enum is only a placeholder and it doesn't specify any IAB viewed rate options.
    ///
    /// "IAB_VIEWED_RATE_UNSPECIFIED"
    #[serde(rename="IAB_VIEWED_RATE_UNSPECIFIED")]
    IABVIEWEDRATEUNSPECIFIED,
    

    /// Target web and app inventory to maximize IAB viewable rate 80% or higher.
    ///
    /// "IAB_VIEWED_RATE_80_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWED_RATE_80_PERCENT_HIGHER")]
    IABVIEWEDRATE80PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 75% or higher.
    ///
    /// "IAB_VIEWED_RATE_75_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWED_RATE_75_PERCENT_HIGHER")]
    IABVIEWEDRATE75PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 70% or higher.
    ///
    /// "IAB_VIEWED_RATE_70_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWED_RATE_70_PERCENT_HIGHER")]
    IABVIEWEDRATE70PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 65% or higher.
    ///
    /// "IAB_VIEWED_RATE_65_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWED_RATE_65_PERCENT_HIGHER")]
    IABVIEWEDRATE65PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 60% or higher.
    ///
    /// "IAB_VIEWED_RATE_60_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWED_RATE_60_PERCENT_HIGHER")]
    IABVIEWEDRATE60PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 55% or higher.
    ///
    /// "IAB_VIEWED_RATE_55_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWED_RATE_55_PERCENT_HIGHER")]
    IABVIEWEDRATE55PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 50% or higher.
    ///
    /// "IAB_VIEWED_RATE_50_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWED_RATE_50_PERCENT_HIGHER")]
    IABVIEWEDRATE50PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 40% or higher.
    ///
    /// "IAB_VIEWED_RATE_40_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWED_RATE_40_PERCENT_HIGHER")]
    IABVIEWEDRATE40PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 30% or higher.
    ///
    /// "IAB_VIEWED_RATE_30_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWED_RATE_30_PERCENT_HIGHER")]
    IABVIEWEDRATE30PERCENTHIGHER,
}

impl AsRef<str> for DoubleVerifyDisplayViewabilityIabEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATEUNSPECIFIED => "IAB_VIEWED_RATE_UNSPECIFIED",
            DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE80PERCENTHIGHER => "IAB_VIEWED_RATE_80_PERCENT_HIGHER",
            DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE75PERCENTHIGHER => "IAB_VIEWED_RATE_75_PERCENT_HIGHER",
            DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE70PERCENTHIGHER => "IAB_VIEWED_RATE_70_PERCENT_HIGHER",
            DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE65PERCENTHIGHER => "IAB_VIEWED_RATE_65_PERCENT_HIGHER",
            DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE60PERCENTHIGHER => "IAB_VIEWED_RATE_60_PERCENT_HIGHER",
            DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE55PERCENTHIGHER => "IAB_VIEWED_RATE_55_PERCENT_HIGHER",
            DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE50PERCENTHIGHER => "IAB_VIEWED_RATE_50_PERCENT_HIGHER",
            DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE40PERCENTHIGHER => "IAB_VIEWED_RATE_40_PERCENT_HIGHER",
            DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE30PERCENTHIGHER => "IAB_VIEWED_RATE_30_PERCENT_HIGHER",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleVerifyDisplayViewabilityIabEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IAB_VIEWED_RATE_UNSPECIFIED" => Ok(DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATEUNSPECIFIED),
           "IAB_VIEWED_RATE_80_PERCENT_HIGHER" => Ok(DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE80PERCENTHIGHER),
           "IAB_VIEWED_RATE_75_PERCENT_HIGHER" => Ok(DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE75PERCENTHIGHER),
           "IAB_VIEWED_RATE_70_PERCENT_HIGHER" => Ok(DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE70PERCENTHIGHER),
           "IAB_VIEWED_RATE_65_PERCENT_HIGHER" => Ok(DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE65PERCENTHIGHER),
           "IAB_VIEWED_RATE_60_PERCENT_HIGHER" => Ok(DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE60PERCENTHIGHER),
           "IAB_VIEWED_RATE_55_PERCENT_HIGHER" => Ok(DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE55PERCENTHIGHER),
           "IAB_VIEWED_RATE_50_PERCENT_HIGHER" => Ok(DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE50PERCENTHIGHER),
           "IAB_VIEWED_RATE_40_PERCENT_HIGHER" => Ok(DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE40PERCENTHIGHER),
           "IAB_VIEWED_RATE_30_PERCENT_HIGHER" => Ok(DoubleVerifyDisplayViewabilityIabEnum::IABVIEWEDRATE30PERCENTHIGHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleVerifyDisplayViewabilityIabEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleVerifyDisplayViewabilityViewableDuringEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Target web and app inventory to maximize 100% viewable duration.
pub enum DoubleVerifyDisplayViewabilityViewableDuringEnum {
    

    /// This enum is only a placeholder and it doesn't specify any average view duration options.
    ///
    /// "AVERAGE_VIEW_DURATION_UNSPECIFIED"
    #[serde(rename="AVERAGE_VIEW_DURATION_UNSPECIFIED")]
    AVERAGEVIEWDURATIONUNSPECIFIED,
    

    /// Target web and app inventory to maximize 100% viewable duration 5 seconds or more.
    ///
    /// "AVERAGE_VIEW_DURATION_5_SEC"
    #[serde(rename="AVERAGE_VIEW_DURATION_5_SEC")]
    AVERAGEVIEWDURATION5SEC,
    

    /// Target web and app inventory to maximize 100% viewable duration 10 seconds or more.
    ///
    /// "AVERAGE_VIEW_DURATION_10_SEC"
    #[serde(rename="AVERAGE_VIEW_DURATION_10_SEC")]
    AVERAGEVIEWDURATION10SEC,
    

    /// Target web and app inventory to maximize 100% viewable duration 15 seconds or more.
    ///
    /// "AVERAGE_VIEW_DURATION_15_SEC"
    #[serde(rename="AVERAGE_VIEW_DURATION_15_SEC")]
    AVERAGEVIEWDURATION15SEC,
}

impl AsRef<str> for DoubleVerifyDisplayViewabilityViewableDuringEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleVerifyDisplayViewabilityViewableDuringEnum::AVERAGEVIEWDURATIONUNSPECIFIED => "AVERAGE_VIEW_DURATION_UNSPECIFIED",
            DoubleVerifyDisplayViewabilityViewableDuringEnum::AVERAGEVIEWDURATION5SEC => "AVERAGE_VIEW_DURATION_5_SEC",
            DoubleVerifyDisplayViewabilityViewableDuringEnum::AVERAGEVIEWDURATION10SEC => "AVERAGE_VIEW_DURATION_10_SEC",
            DoubleVerifyDisplayViewabilityViewableDuringEnum::AVERAGEVIEWDURATION15SEC => "AVERAGE_VIEW_DURATION_15_SEC",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleVerifyDisplayViewabilityViewableDuringEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AVERAGE_VIEW_DURATION_UNSPECIFIED" => Ok(DoubleVerifyDisplayViewabilityViewableDuringEnum::AVERAGEVIEWDURATIONUNSPECIFIED),
           "AVERAGE_VIEW_DURATION_5_SEC" => Ok(DoubleVerifyDisplayViewabilityViewableDuringEnum::AVERAGEVIEWDURATION5SEC),
           "AVERAGE_VIEW_DURATION_10_SEC" => Ok(DoubleVerifyDisplayViewabilityViewableDuringEnum::AVERAGEVIEWDURATION10SEC),
           "AVERAGE_VIEW_DURATION_15_SEC" => Ok(DoubleVerifyDisplayViewabilityViewableDuringEnum::AVERAGEVIEWDURATION15SEC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleVerifyDisplayViewabilityViewableDuringEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Avoid Sites and Apps with historical Fraud & IVT.
pub enum DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum {
    

    /// This enum is only a placeholder and it doesn't specify any fraud and invalid traffic options.
    ///
    /// "FRAUD_UNSPECIFIED"
    #[serde(rename="FRAUD_UNSPECIFIED")]
    FRAUDUNSPECIFIED,
    

    /// 100% Fraud & IVT.
    ///
    /// "AD_IMPRESSION_FRAUD_100"
    #[serde(rename="AD_IMPRESSION_FRAUD_100")]
    ADIMPRESSIONFRAUD100,
    

    /// 50% or Higher Fraud & IVT.
    ///
    /// "AD_IMPRESSION_FRAUD_50"
    #[serde(rename="AD_IMPRESSION_FRAUD_50")]
    ADIMPRESSIONFRAUD50,
    

    /// 25% or Higher Fraud & IVT.
    ///
    /// "AD_IMPRESSION_FRAUD_25"
    #[serde(rename="AD_IMPRESSION_FRAUD_25")]
    ADIMPRESSIONFRAUD25,
    

    /// 10% or Higher Fraud & IVT.
    ///
    /// "AD_IMPRESSION_FRAUD_10"
    #[serde(rename="AD_IMPRESSION_FRAUD_10")]
    ADIMPRESSIONFRAUD10,
    

    /// 8% or Higher Fraud & IVT.
    ///
    /// "AD_IMPRESSION_FRAUD_8"
    #[serde(rename="AD_IMPRESSION_FRAUD_8")]
    ADIMPRESSIONFRAUD8,
    

    /// 6% or Higher Fraud & IVT.
    ///
    /// "AD_IMPRESSION_FRAUD_6"
    #[serde(rename="AD_IMPRESSION_FRAUD_6")]
    ADIMPRESSIONFRAUD6,
    

    /// 4% or Higher Fraud & IVT.
    ///
    /// "AD_IMPRESSION_FRAUD_4"
    #[serde(rename="AD_IMPRESSION_FRAUD_4")]
    ADIMPRESSIONFRAUD4,
    

    /// 2% or Higher Fraud & IVT.
    ///
    /// "AD_IMPRESSION_FRAUD_2"
    #[serde(rename="AD_IMPRESSION_FRAUD_2")]
    ADIMPRESSIONFRAUD2,
}

impl AsRef<str> for DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::FRAUDUNSPECIFIED => "FRAUD_UNSPECIFIED",
            DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD100 => "AD_IMPRESSION_FRAUD_100",
            DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD50 => "AD_IMPRESSION_FRAUD_50",
            DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD25 => "AD_IMPRESSION_FRAUD_25",
            DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD10 => "AD_IMPRESSION_FRAUD_10",
            DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD8 => "AD_IMPRESSION_FRAUD_8",
            DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD6 => "AD_IMPRESSION_FRAUD_6",
            DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD4 => "AD_IMPRESSION_FRAUD_4",
            DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD2 => "AD_IMPRESSION_FRAUD_2",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FRAUD_UNSPECIFIED" => Ok(DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::FRAUDUNSPECIFIED),
           "AD_IMPRESSION_FRAUD_100" => Ok(DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD100),
           "AD_IMPRESSION_FRAUD_50" => Ok(DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD50),
           "AD_IMPRESSION_FRAUD_25" => Ok(DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD25),
           "AD_IMPRESSION_FRAUD_10" => Ok(DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD10),
           "AD_IMPRESSION_FRAUD_8" => Ok(DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD8),
           "AD_IMPRESSION_FRAUD_6" => Ok(DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD6),
           "AD_IMPRESSION_FRAUD_4" => Ok(DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD4),
           "AD_IMPRESSION_FRAUD_2" => Ok(DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum::ADIMPRESSIONFRAUD2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleVerifyVideoViewabilityPlayerImpressionRateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Target inventory to maximize impressions with 400x300 or greater player size.
pub enum DoubleVerifyVideoViewabilityPlayerImpressionRateEnum {
    

    /// This enum is only a placeholder and it doesn't specify any impressions options.
    ///
    /// "PLAYER_SIZE_400X300_UNSPECIFIED"
    #[serde(rename="PLAYER_SIZE_400X300_UNSPECIFIED")]
    PLAYERSIZE400X300UNSPECIFIED,
    

    /// Sites with 95%+ of impressions.
    ///
    /// "PLAYER_SIZE_400X300_95"
    #[serde(rename="PLAYER_SIZE_400X300_95")]
    PLAYERSIZE400X30095,
    

    /// Sites with 70%+ of impressions.
    ///
    /// "PLAYER_SIZE_400X300_70"
    #[serde(rename="PLAYER_SIZE_400X300_70")]
    PLAYERSIZE400X30070,
    

    /// Sites with 25%+ of impressions.
    ///
    /// "PLAYER_SIZE_400X300_25"
    #[serde(rename="PLAYER_SIZE_400X300_25")]
    PLAYERSIZE400X30025,
    

    /// Sites with 5%+ of impressions.
    ///
    /// "PLAYER_SIZE_400X300_5"
    #[serde(rename="PLAYER_SIZE_400X300_5")]
    PLAYERSIZE400X3005,
}

impl AsRef<str> for DoubleVerifyVideoViewabilityPlayerImpressionRateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleVerifyVideoViewabilityPlayerImpressionRateEnum::PLAYERSIZE400X300UNSPECIFIED => "PLAYER_SIZE_400X300_UNSPECIFIED",
            DoubleVerifyVideoViewabilityPlayerImpressionRateEnum::PLAYERSIZE400X30095 => "PLAYER_SIZE_400X300_95",
            DoubleVerifyVideoViewabilityPlayerImpressionRateEnum::PLAYERSIZE400X30070 => "PLAYER_SIZE_400X300_70",
            DoubleVerifyVideoViewabilityPlayerImpressionRateEnum::PLAYERSIZE400X30025 => "PLAYER_SIZE_400X300_25",
            DoubleVerifyVideoViewabilityPlayerImpressionRateEnum::PLAYERSIZE400X3005 => "PLAYER_SIZE_400X300_5",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleVerifyVideoViewabilityPlayerImpressionRateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLAYER_SIZE_400X300_UNSPECIFIED" => Ok(DoubleVerifyVideoViewabilityPlayerImpressionRateEnum::PLAYERSIZE400X300UNSPECIFIED),
           "PLAYER_SIZE_400X300_95" => Ok(DoubleVerifyVideoViewabilityPlayerImpressionRateEnum::PLAYERSIZE400X30095),
           "PLAYER_SIZE_400X300_70" => Ok(DoubleVerifyVideoViewabilityPlayerImpressionRateEnum::PLAYERSIZE400X30070),
           "PLAYER_SIZE_400X300_25" => Ok(DoubleVerifyVideoViewabilityPlayerImpressionRateEnum::PLAYERSIZE400X30025),
           "PLAYER_SIZE_400X300_5" => Ok(DoubleVerifyVideoViewabilityPlayerImpressionRateEnum::PLAYERSIZE400X3005),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleVerifyVideoViewabilityPlayerImpressionRateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleVerifyVideoViewabilityVideoIabEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Target web inventory to maximize IAB viewable rate.
pub enum DoubleVerifyVideoViewabilityVideoIabEnum {
    

    /// This enum is only a placeholder and it doesn't specify any video IAB viewable rate options.
    ///
    /// "VIDEO_IAB_UNSPECIFIED"
    #[serde(rename="VIDEO_IAB_UNSPECIFIED")]
    VIDEOIABUNSPECIFIED,
    

    /// Target web and app inventory to maximize IAB viewable rate 80% or higher.
    ///
    /// "IAB_VIEWABILITY_80_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWABILITY_80_PERCENT_HIGHER")]
    IABVIEWABILITY80PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 75% or higher.
    ///
    /// "IAB_VIEWABILITY_75_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWABILITY_75_PERCENT_HIGHER")]
    IABVIEWABILITY75PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 70% or higher.
    ///
    /// "IAB_VIEWABILITY_70_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWABILITY_70_PERCENT_HIGHER")]
    IABVIEWABILITY70PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 65% or higher.
    ///
    /// "IAB_VIEWABILITY_65_PERCENT_HIHGER"
    #[serde(rename="IAB_VIEWABILITY_65_PERCENT_HIHGER")]
    IABVIEWABILITY65PERCENTHIHGER,
    

    /// Target web and app inventory to maximize IAB viewable rate 60% or higher.
    ///
    /// "IAB_VIEWABILITY_60_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWABILITY_60_PERCENT_HIGHER")]
    IABVIEWABILITY60PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 55% or higher.
    ///
    /// "IAB_VIEWABILITY_55_PERCENT_HIHGER"
    #[serde(rename="IAB_VIEWABILITY_55_PERCENT_HIHGER")]
    IABVIEWABILITY55PERCENTHIHGER,
    

    /// Target web and app inventory to maximize IAB viewable rate 50% or higher.
    ///
    /// "IAB_VIEWABILITY_50_PERCENT_HIGHER"
    #[serde(rename="IAB_VIEWABILITY_50_PERCENT_HIGHER")]
    IABVIEWABILITY50PERCENTHIGHER,
    

    /// Target web and app inventory to maximize IAB viewable rate 40% or higher.
    ///
    /// "IAB_VIEWABILITY_40_PERCENT_HIHGER"
    #[serde(rename="IAB_VIEWABILITY_40_PERCENT_HIHGER")]
    IABVIEWABILITY40PERCENTHIHGER,
    

    /// Target web and app inventory to maximize IAB viewable rate 30% or higher.
    ///
    /// "IAB_VIEWABILITY_30_PERCENT_HIHGER"
    #[serde(rename="IAB_VIEWABILITY_30_PERCENT_HIHGER")]
    IABVIEWABILITY30PERCENTHIHGER,
}

impl AsRef<str> for DoubleVerifyVideoViewabilityVideoIabEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleVerifyVideoViewabilityVideoIabEnum::VIDEOIABUNSPECIFIED => "VIDEO_IAB_UNSPECIFIED",
            DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY80PERCENTHIGHER => "IAB_VIEWABILITY_80_PERCENT_HIGHER",
            DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY75PERCENTHIGHER => "IAB_VIEWABILITY_75_PERCENT_HIGHER",
            DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY70PERCENTHIGHER => "IAB_VIEWABILITY_70_PERCENT_HIGHER",
            DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY65PERCENTHIHGER => "IAB_VIEWABILITY_65_PERCENT_HIHGER",
            DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY60PERCENTHIGHER => "IAB_VIEWABILITY_60_PERCENT_HIGHER",
            DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY55PERCENTHIHGER => "IAB_VIEWABILITY_55_PERCENT_HIHGER",
            DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY50PERCENTHIGHER => "IAB_VIEWABILITY_50_PERCENT_HIGHER",
            DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY40PERCENTHIHGER => "IAB_VIEWABILITY_40_PERCENT_HIHGER",
            DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY30PERCENTHIHGER => "IAB_VIEWABILITY_30_PERCENT_HIHGER",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleVerifyVideoViewabilityVideoIabEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIDEO_IAB_UNSPECIFIED" => Ok(DoubleVerifyVideoViewabilityVideoIabEnum::VIDEOIABUNSPECIFIED),
           "IAB_VIEWABILITY_80_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY80PERCENTHIGHER),
           "IAB_VIEWABILITY_75_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY75PERCENTHIGHER),
           "IAB_VIEWABILITY_70_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY70PERCENTHIGHER),
           "IAB_VIEWABILITY_65_PERCENT_HIHGER" => Ok(DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY65PERCENTHIHGER),
           "IAB_VIEWABILITY_60_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY60PERCENTHIGHER),
           "IAB_VIEWABILITY_55_PERCENT_HIHGER" => Ok(DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY55PERCENTHIHGER),
           "IAB_VIEWABILITY_50_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY50PERCENTHIGHER),
           "IAB_VIEWABILITY_40_PERCENT_HIHGER" => Ok(DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY40PERCENTHIHGER),
           "IAB_VIEWABILITY_30_PERCENT_HIHGER" => Ok(DoubleVerifyVideoViewabilityVideoIabEnum::IABVIEWABILITY30PERCENTHIHGER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleVerifyVideoViewabilityVideoIabEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleVerifyVideoViewabilityVideoViewableRateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Target web inventory to maximize fully viewable rate.
pub enum DoubleVerifyVideoViewabilityVideoViewableRateEnum {
    

    /// This enum is only a placeholder and it doesn't specify any video viewable rate options.
    ///
    /// "VIDEO_VIEWABLE_RATE_UNSPECIFIED"
    #[serde(rename="VIDEO_VIEWABLE_RATE_UNSPECIFIED")]
    VIDEOVIEWABLERATEUNSPECIFIED,
    

    /// Target web inventory to maximize fully viewable rate 40% or higher.
    ///
    /// "VIEWED_PERFORMANCE_40_PERCENT_HIGHER"
    #[serde(rename="VIEWED_PERFORMANCE_40_PERCENT_HIGHER")]
    VIEWEDPERFORMANCE40PERCENTHIGHER,
    

    /// Target web inventory to maximize fully viewable rate 35% or higher.
    ///
    /// "VIEWED_PERFORMANCE_35_PERCENT_HIGHER"
    #[serde(rename="VIEWED_PERFORMANCE_35_PERCENT_HIGHER")]
    VIEWEDPERFORMANCE35PERCENTHIGHER,
    

    /// Target web inventory to maximize fully viewable rate 30% or higher.
    ///
    /// "VIEWED_PERFORMANCE_30_PERCENT_HIGHER"
    #[serde(rename="VIEWED_PERFORMANCE_30_PERCENT_HIGHER")]
    VIEWEDPERFORMANCE30PERCENTHIGHER,
    

    /// Target web inventory to maximize fully viewable rate 25% or higher.
    ///
    /// "VIEWED_PERFORMANCE_25_PERCENT_HIGHER"
    #[serde(rename="VIEWED_PERFORMANCE_25_PERCENT_HIGHER")]
    VIEWEDPERFORMANCE25PERCENTHIGHER,
    

    /// Target web inventory to maximize fully viewable rate 20% or higher.
    ///
    /// "VIEWED_PERFORMANCE_20_PERCENT_HIGHER"
    #[serde(rename="VIEWED_PERFORMANCE_20_PERCENT_HIGHER")]
    VIEWEDPERFORMANCE20PERCENTHIGHER,
    

    /// Target web inventory to maximize fully viewable rate 10% or higher.
    ///
    /// "VIEWED_PERFORMANCE_10_PERCENT_HIGHER"
    #[serde(rename="VIEWED_PERFORMANCE_10_PERCENT_HIGHER")]
    VIEWEDPERFORMANCE10PERCENTHIGHER,
}

impl AsRef<str> for DoubleVerifyVideoViewabilityVideoViewableRateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIDEOVIEWABLERATEUNSPECIFIED => "VIDEO_VIEWABLE_RATE_UNSPECIFIED",
            DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE40PERCENTHIGHER => "VIEWED_PERFORMANCE_40_PERCENT_HIGHER",
            DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE35PERCENTHIGHER => "VIEWED_PERFORMANCE_35_PERCENT_HIGHER",
            DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE30PERCENTHIGHER => "VIEWED_PERFORMANCE_30_PERCENT_HIGHER",
            DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE25PERCENTHIGHER => "VIEWED_PERFORMANCE_25_PERCENT_HIGHER",
            DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE20PERCENTHIGHER => "VIEWED_PERFORMANCE_20_PERCENT_HIGHER",
            DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE10PERCENTHIGHER => "VIEWED_PERFORMANCE_10_PERCENT_HIGHER",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleVerifyVideoViewabilityVideoViewableRateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIDEO_VIEWABLE_RATE_UNSPECIFIED" => Ok(DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIDEOVIEWABLERATEUNSPECIFIED),
           "VIEWED_PERFORMANCE_40_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE40PERCENTHIGHER),
           "VIEWED_PERFORMANCE_35_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE35PERCENTHIGHER),
           "VIEWED_PERFORMANCE_30_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE30PERCENTHIGHER),
           "VIEWED_PERFORMANCE_25_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE25PERCENTHIGHER),
           "VIEWED_PERFORMANCE_20_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE20PERCENTHIGHER),
           "VIEWED_PERFORMANCE_10_PERCENT_HIGHER" => Ok(DoubleVerifyVideoViewabilityVideoViewableRateEnum::VIEWEDPERFORMANCE10PERCENTHIGHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleVerifyVideoViewabilityVideoViewableRateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnvironmentAssignedTargetingOptionDetailEnvironmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The serving environment.
pub enum EnvironmentAssignedTargetingOptionDetailEnvironmentEnum {
    

    /// Default value when environment is not specified in this version. This enum is a placeholder for default value and does not represent a real environment option.
    ///
    /// "ENVIRONMENT_UNSPECIFIED"
    #[serde(rename="ENVIRONMENT_UNSPECIFIED")]
    ENVIRONMENTUNSPECIFIED,
    

    /// Target inventory displayed in browsers. This includes inventory that was designed for the device it was viewed on, such as mobile websites viewed on a mobile device. ENVIRONMENT_WEB_NOT_OPTIMIZED, if targeted, should be deleted prior to the deletion of this targeting option.
    ///
    /// "ENVIRONMENT_WEB_OPTIMIZED"
    #[serde(rename="ENVIRONMENT_WEB_OPTIMIZED")]
    ENVIRONMENTWEBOPTIMIZED,
    

    /// Target inventory displayed in browsers. This includes inventory that was not designed for the device but viewed on it, such as websites optimized for desktop but viewed on a mobile device. ENVIRONMENT_WEB_OPTIMIZED should be targeted prior to the addition of this targeting option.
    ///
    /// "ENVIRONMENT_WEB_NOT_OPTIMIZED"
    #[serde(rename="ENVIRONMENT_WEB_NOT_OPTIMIZED")]
    ENVIRONMENTWEBNOTOPTIMIZED,
    

    /// Target inventory displayed in apps.
    ///
    /// "ENVIRONMENT_APP"
    #[serde(rename="ENVIRONMENT_APP")]
    ENVIRONMENTAPP,
}

impl AsRef<str> for EnvironmentAssignedTargetingOptionDetailEnvironmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnvironmentAssignedTargetingOptionDetailEnvironmentEnum::ENVIRONMENTUNSPECIFIED => "ENVIRONMENT_UNSPECIFIED",
            EnvironmentAssignedTargetingOptionDetailEnvironmentEnum::ENVIRONMENTWEBOPTIMIZED => "ENVIRONMENT_WEB_OPTIMIZED",
            EnvironmentAssignedTargetingOptionDetailEnvironmentEnum::ENVIRONMENTWEBNOTOPTIMIZED => "ENVIRONMENT_WEB_NOT_OPTIMIZED",
            EnvironmentAssignedTargetingOptionDetailEnvironmentEnum::ENVIRONMENTAPP => "ENVIRONMENT_APP",
        }
    }
}

impl std::convert::TryFrom< &str> for EnvironmentAssignedTargetingOptionDetailEnvironmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVIRONMENT_UNSPECIFIED" => Ok(EnvironmentAssignedTargetingOptionDetailEnvironmentEnum::ENVIRONMENTUNSPECIFIED),
           "ENVIRONMENT_WEB_OPTIMIZED" => Ok(EnvironmentAssignedTargetingOptionDetailEnvironmentEnum::ENVIRONMENTWEBOPTIMIZED),
           "ENVIRONMENT_WEB_NOT_OPTIMIZED" => Ok(EnvironmentAssignedTargetingOptionDetailEnvironmentEnum::ENVIRONMENTWEBNOTOPTIMIZED),
           "ENVIRONMENT_APP" => Ok(EnvironmentAssignedTargetingOptionDetailEnvironmentEnum::ENVIRONMENTAPP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnvironmentAssignedTargetingOptionDetailEnvironmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnvironmentTargetingOptionDetailEnvironmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The serving environment.
pub enum EnvironmentTargetingOptionDetailEnvironmentEnum {
    

    /// Default value when environment is not specified in this version. This enum is a placeholder for default value and does not represent a real environment option.
    ///
    /// "ENVIRONMENT_UNSPECIFIED"
    #[serde(rename="ENVIRONMENT_UNSPECIFIED")]
    ENVIRONMENTUNSPECIFIED,
    

    /// Target inventory displayed in browsers. This includes inventory that was designed for the device it was viewed on, such as mobile websites viewed on a mobile device. ENVIRONMENT_WEB_NOT_OPTIMIZED, if targeted, should be deleted prior to the deletion of this targeting option.
    ///
    /// "ENVIRONMENT_WEB_OPTIMIZED"
    #[serde(rename="ENVIRONMENT_WEB_OPTIMIZED")]
    ENVIRONMENTWEBOPTIMIZED,
    

    /// Target inventory displayed in browsers. This includes inventory that was not designed for the device but viewed on it, such as websites optimized for desktop but viewed on a mobile device. ENVIRONMENT_WEB_OPTIMIZED should be targeted prior to the addition of this targeting option.
    ///
    /// "ENVIRONMENT_WEB_NOT_OPTIMIZED"
    #[serde(rename="ENVIRONMENT_WEB_NOT_OPTIMIZED")]
    ENVIRONMENTWEBNOTOPTIMIZED,
    

    /// Target inventory displayed in apps.
    ///
    /// "ENVIRONMENT_APP"
    #[serde(rename="ENVIRONMENT_APP")]
    ENVIRONMENTAPP,
}

impl AsRef<str> for EnvironmentTargetingOptionDetailEnvironmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnvironmentTargetingOptionDetailEnvironmentEnum::ENVIRONMENTUNSPECIFIED => "ENVIRONMENT_UNSPECIFIED",
            EnvironmentTargetingOptionDetailEnvironmentEnum::ENVIRONMENTWEBOPTIMIZED => "ENVIRONMENT_WEB_OPTIMIZED",
            EnvironmentTargetingOptionDetailEnvironmentEnum::ENVIRONMENTWEBNOTOPTIMIZED => "ENVIRONMENT_WEB_NOT_OPTIMIZED",
            EnvironmentTargetingOptionDetailEnvironmentEnum::ENVIRONMENTAPP => "ENVIRONMENT_APP",
        }
    }
}

impl std::convert::TryFrom< &str> for EnvironmentTargetingOptionDetailEnvironmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVIRONMENT_UNSPECIFIED" => Ok(EnvironmentTargetingOptionDetailEnvironmentEnum::ENVIRONMENTUNSPECIFIED),
           "ENVIRONMENT_WEB_OPTIMIZED" => Ok(EnvironmentTargetingOptionDetailEnvironmentEnum::ENVIRONMENTWEBOPTIMIZED),
           "ENVIRONMENT_WEB_NOT_OPTIMIZED" => Ok(EnvironmentTargetingOptionDetailEnvironmentEnum::ENVIRONMENTWEBNOTOPTIMIZED),
           "ENVIRONMENT_APP" => Ok(EnvironmentTargetingOptionDetailEnvironmentEnum::ENVIRONMENTAPP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnvironmentTargetingOptionDetailEnvironmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExchangeConfigEnabledExchangeExchangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The enabled exchange.
pub enum ExchangeConfigEnabledExchangeExchangeEnum {
    

    /// Exchange is not specified or is unknown in this version.
    ///
    /// "EXCHANGE_UNSPECIFIED"
    #[serde(rename="EXCHANGE_UNSPECIFIED")]
    EXCHANGEUNSPECIFIED,
    

    /// Google Ad Manager.
    ///
    /// "EXCHANGE_GOOGLE_AD_MANAGER"
    #[serde(rename="EXCHANGE_GOOGLE_AD_MANAGER")]
    EXCHANGEGOOGLEADMANAGER,
    

    /// AppNexus.
    ///
    /// "EXCHANGE_APPNEXUS"
    #[serde(rename="EXCHANGE_APPNEXUS")]
    EXCHANGEAPPNEXUS,
    

    /// BrightRoll Exchange for Video from Yahoo!.
    ///
    /// "EXCHANGE_BRIGHTROLL"
    #[serde(rename="EXCHANGE_BRIGHTROLL")]
    EXCHANGEBRIGHTROLL,
    

    /// Adform.
    ///
    /// "EXCHANGE_ADFORM"
    #[serde(rename="EXCHANGE_ADFORM")]
    EXCHANGEADFORM,
    

    /// Admeta.
    ///
    /// "EXCHANGE_ADMETA"
    #[serde(rename="EXCHANGE_ADMETA")]
    EXCHANGEADMETA,
    

    /// Admixer.
    ///
    /// "EXCHANGE_ADMIXER"
    #[serde(rename="EXCHANGE_ADMIXER")]
    EXCHANGEADMIXER,
    

    /// AdsMogo.
    ///
    /// "EXCHANGE_ADSMOGO"
    #[serde(rename="EXCHANGE_ADSMOGO")]
    EXCHANGEADSMOGO,
    

    /// AdsWizz.
    ///
    /// "EXCHANGE_ADSWIZZ"
    #[serde(rename="EXCHANGE_ADSWIZZ")]
    EXCHANGEADSWIZZ,
    

    /// BidSwitch.
    ///
    /// "EXCHANGE_BIDSWITCH"
    #[serde(rename="EXCHANGE_BIDSWITCH")]
    EXCHANGEBIDSWITCH,
    

    /// BrightRoll Exchange for Display from Yahoo!.
    ///
    /// "EXCHANGE_BRIGHTROLL_DISPLAY"
    #[serde(rename="EXCHANGE_BRIGHTROLL_DISPLAY")]
    EXCHANGEBRIGHTROLLDISPLAY,
    

    /// Cadreon.
    ///
    /// "EXCHANGE_CADREON"
    #[serde(rename="EXCHANGE_CADREON")]
    EXCHANGECADREON,
    

    /// Dailymotion.
    ///
    /// "EXCHANGE_DAILYMOTION"
    #[serde(rename="EXCHANGE_DAILYMOTION")]
    EXCHANGEDAILYMOTION,
    

    /// Five.
    ///
    /// "EXCHANGE_FIVE"
    #[serde(rename="EXCHANGE_FIVE")]
    EXCHANGEFIVE,
    

    /// Fluct.
    ///
    /// "EXCHANGE_FLUCT"
    #[serde(rename="EXCHANGE_FLUCT")]
    EXCHANGEFLUCT,
    

    /// FreeWheel SSP.
    ///
    /// "EXCHANGE_FREEWHEEL"
    #[serde(rename="EXCHANGE_FREEWHEEL")]
    EXCHANGEFREEWHEEL,
    

    /// Geniee.
    ///
    /// "EXCHANGE_GENIEE"
    #[serde(rename="EXCHANGE_GENIEE")]
    EXCHANGEGENIEE,
    

    /// GumGum.
    ///
    /// "EXCHANGE_GUMGUM"
    #[serde(rename="EXCHANGE_GUMGUM")]
    EXCHANGEGUMGUM,
    

    /// i-mobile.
    ///
    /// "EXCHANGE_IMOBILE"
    #[serde(rename="EXCHANGE_IMOBILE")]
    EXCHANGEIMOBILE,
    

    /// iBILLBOARD.
    ///
    /// "EXCHANGE_IBILLBOARD"
    #[serde(rename="EXCHANGE_IBILLBOARD")]
    EXCHANGEIBILLBOARD,
    

    /// Improve Digital.
    ///
    /// "EXCHANGE_IMPROVE_DIGITAL"
    #[serde(rename="EXCHANGE_IMPROVE_DIGITAL")]
    EXCHANGEIMPROVEDIGITAL,
    

    /// Index Exchange.
    ///
    /// "EXCHANGE_INDEX"
    #[serde(rename="EXCHANGE_INDEX")]
    EXCHANGEINDEX,
    

    /// Kargo.
    ///
    /// "EXCHANGE_KARGO"
    #[serde(rename="EXCHANGE_KARGO")]
    EXCHANGEKARGO,
    

    /// MicroAd.
    ///
    /// "EXCHANGE_MICROAD"
    #[serde(rename="EXCHANGE_MICROAD")]
    EXCHANGEMICROAD,
    

    /// MoPub.
    ///
    /// "EXCHANGE_MOPUB"
    #[serde(rename="EXCHANGE_MOPUB")]
    EXCHANGEMOPUB,
    

    /// Nend.
    ///
    /// "EXCHANGE_NEND"
    #[serde(rename="EXCHANGE_NEND")]
    EXCHANGENEND,
    

    /// ONE by AOL: Display Market Place.
    ///
    /// "EXCHANGE_ONE_BY_AOL_DISPLAY"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_DISPLAY")]
    EXCHANGEONEBYAOLDISPLAY,
    

    /// ONE by AOL: Mobile.
    ///
    /// "EXCHANGE_ONE_BY_AOL_MOBILE"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_MOBILE")]
    EXCHANGEONEBYAOLMOBILE,
    

    /// ONE by AOL: Video.
    ///
    /// "EXCHANGE_ONE_BY_AOL_VIDEO"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_VIDEO")]
    EXCHANGEONEBYAOLVIDEO,
    

    /// Ooyala.
    ///
    /// "EXCHANGE_OOYALA"
    #[serde(rename="EXCHANGE_OOYALA")]
    EXCHANGEOOYALA,
    

    /// OpenX.
    ///
    /// "EXCHANGE_OPENX"
    #[serde(rename="EXCHANGE_OPENX")]
    EXCHANGEOPENX,
    

    /// Permodo.
    ///
    /// "EXCHANGE_PERMODO"
    #[serde(rename="EXCHANGE_PERMODO")]
    EXCHANGEPERMODO,
    

    /// Platform One.
    ///
    /// "EXCHANGE_PLATFORMONE"
    #[serde(rename="EXCHANGE_PLATFORMONE")]
    EXCHANGEPLATFORMONE,
    

    /// PlatformId.
    ///
    /// "EXCHANGE_PLATFORMID"
    #[serde(rename="EXCHANGE_PLATFORMID")]
    EXCHANGEPLATFORMID,
    

    /// PubMatic.
    ///
    /// "EXCHANGE_PUBMATIC"
    #[serde(rename="EXCHANGE_PUBMATIC")]
    EXCHANGEPUBMATIC,
    

    /// PulsePoint.
    ///
    /// "EXCHANGE_PULSEPOINT"
    #[serde(rename="EXCHANGE_PULSEPOINT")]
    EXCHANGEPULSEPOINT,
    

    /// RevenueMax.
    ///
    /// "EXCHANGE_REVENUEMAX"
    #[serde(rename="EXCHANGE_REVENUEMAX")]
    EXCHANGEREVENUEMAX,
    

    /// Rubicon.
    ///
    /// "EXCHANGE_RUBICON"
    #[serde(rename="EXCHANGE_RUBICON")]
    EXCHANGERUBICON,
    

    /// SmartClip.
    ///
    /// "EXCHANGE_SMARTCLIP"
    #[serde(rename="EXCHANGE_SMARTCLIP")]
    EXCHANGESMARTCLIP,
    

    /// SmartRTB+.
    ///
    /// "EXCHANGE_SMARTRTB"
    #[serde(rename="EXCHANGE_SMARTRTB")]
    EXCHANGESMARTRTB,
    

    /// SmartstreamTv.
    ///
    /// "EXCHANGE_SMARTSTREAMTV"
    #[serde(rename="EXCHANGE_SMARTSTREAMTV")]
    EXCHANGESMARTSTREAMTV,
    

    /// Sovrn.
    ///
    /// "EXCHANGE_SOVRN"
    #[serde(rename="EXCHANGE_SOVRN")]
    EXCHANGESOVRN,
    

    /// SpotXchange.
    ///
    /// "EXCHANGE_SPOTXCHANGE"
    #[serde(rename="EXCHANGE_SPOTXCHANGE")]
    EXCHANGESPOTXCHANGE,
    

    /// Ströer SSP.
    ///
    /// "EXCHANGE_STROER"
    #[serde(rename="EXCHANGE_STROER")]
    EXCHANGESTROER,
    

    /// TeadsTv.
    ///
    /// "EXCHANGE_TEADSTV"
    #[serde(rename="EXCHANGE_TEADSTV")]
    EXCHANGETEADSTV,
    

    /// Telaria.
    ///
    /// "EXCHANGE_TELARIA"
    #[serde(rename="EXCHANGE_TELARIA")]
    EXCHANGETELARIA,
    

    /// TVN.
    ///
    /// "EXCHANGE_TVN"
    #[serde(rename="EXCHANGE_TVN")]
    EXCHANGETVN,
    

    /// United.
    ///
    /// "EXCHANGE_UNITED"
    #[serde(rename="EXCHANGE_UNITED")]
    EXCHANGEUNITED,
    

    /// Yieldlab.
    ///
    /// "EXCHANGE_YIELDLAB"
    #[serde(rename="EXCHANGE_YIELDLAB")]
    EXCHANGEYIELDLAB,
    

    /// Yieldmo.
    ///
    /// "EXCHANGE_YIELDMO"
    #[serde(rename="EXCHANGE_YIELDMO")]
    EXCHANGEYIELDMO,
    

    /// UnrulyX.
    ///
    /// "EXCHANGE_UNRULYX"
    #[serde(rename="EXCHANGE_UNRULYX")]
    EXCHANGEUNRULYX,
    

    /// Open8.
    ///
    /// "EXCHANGE_OPEN8"
    #[serde(rename="EXCHANGE_OPEN8")]
    EXCHANGEOPEN8,
    

    /// Triton.
    ///
    /// "EXCHANGE_TRITON"
    #[serde(rename="EXCHANGE_TRITON")]
    EXCHANGETRITON,
    

    /// TripleLift.
    ///
    /// "EXCHANGE_TRIPLELIFT"
    #[serde(rename="EXCHANGE_TRIPLELIFT")]
    EXCHANGETRIPLELIFT,
    

    /// Taboola.
    ///
    /// "EXCHANGE_TABOOLA"
    #[serde(rename="EXCHANGE_TABOOLA")]
    EXCHANGETABOOLA,
    

    /// InMobi.
    ///
    /// "EXCHANGE_INMOBI"
    #[serde(rename="EXCHANGE_INMOBI")]
    EXCHANGEINMOBI,
    

    /// Smaato.
    ///
    /// "EXCHANGE_SMAATO"
    #[serde(rename="EXCHANGE_SMAATO")]
    EXCHANGESMAATO,
    

    /// Aja.
    ///
    /// "EXCHANGE_AJA"
    #[serde(rename="EXCHANGE_AJA")]
    EXCHANGEAJA,
    

    /// Supership.
    ///
    /// "EXCHANGE_SUPERSHIP"
    #[serde(rename="EXCHANGE_SUPERSHIP")]
    EXCHANGESUPERSHIP,
    

    /// Nexstar Digital.
    ///
    /// "EXCHANGE_NEXSTAR_DIGITAL"
    #[serde(rename="EXCHANGE_NEXSTAR_DIGITAL")]
    EXCHANGENEXSTARDIGITAL,
    

    /// Waze.
    ///
    /// "EXCHANGE_WAZE"
    #[serde(rename="EXCHANGE_WAZE")]
    EXCHANGEWAZE,
    

    /// SoundCast.
    ///
    /// "EXCHANGE_SOUNDCAST"
    #[serde(rename="EXCHANGE_SOUNDCAST")]
    EXCHANGESOUNDCAST,
    

    /// Sharethrough.
    ///
    /// "EXCHANGE_SHARETHROUGH"
    #[serde(rename="EXCHANGE_SHARETHROUGH")]
    EXCHANGESHARETHROUGH,
    

    /// Fyber.
    ///
    /// "EXCHANGE_FYBER"
    #[serde(rename="EXCHANGE_FYBER")]
    EXCHANGEFYBER,
    

    /// Red For Publishers.
    ///
    /// "EXCHANGE_RED_FOR_PUBLISHERS"
    #[serde(rename="EXCHANGE_RED_FOR_PUBLISHERS")]
    EXCHANGEREDFORPUBLISHERS,
    

    /// Media.net.
    ///
    /// "EXCHANGE_MEDIANET"
    #[serde(rename="EXCHANGE_MEDIANET")]
    EXCHANGEMEDIANET,
    

    /// Tapjoy.
    ///
    /// "EXCHANGE_TAPJOY"
    #[serde(rename="EXCHANGE_TAPJOY")]
    EXCHANGETAPJOY,
    

    /// Vistar.
    ///
    /// "EXCHANGE_VISTAR"
    #[serde(rename="EXCHANGE_VISTAR")]
    EXCHANGEVISTAR,
    

    /// DAX.
    ///
    /// "EXCHANGE_DAX"
    #[serde(rename="EXCHANGE_DAX")]
    EXCHANGEDAX,
    

    /// JCD.
    ///
    /// "EXCHANGE_JCD"
    #[serde(rename="EXCHANGE_JCD")]
    EXCHANGEJCD,
    

    /// Place Exchange.
    ///
    /// "EXCHANGE_PLACE_EXCHANGE"
    #[serde(rename="EXCHANGE_PLACE_EXCHANGE")]
    EXCHANGEPLACEEXCHANGE,
    

    /// AppLovin.
    ///
    /// "EXCHANGE_APPLOVIN"
    #[serde(rename="EXCHANGE_APPLOVIN")]
    EXCHANGEAPPLOVIN,
    

    /// Connatix.
    ///
    /// "EXCHANGE_CONNATIX"
    #[serde(rename="EXCHANGE_CONNATIX")]
    EXCHANGECONNATIX,
    

    /// Reset Digital.
    ///
    /// "EXCHANGE_RESET_DIGITAL"
    #[serde(rename="EXCHANGE_RESET_DIGITAL")]
    EXCHANGERESETDIGITAL,
    

    /// Hivestack.
    ///
    /// "EXCHANGE_HIVESTACK"
    #[serde(rename="EXCHANGE_HIVESTACK")]
    EXCHANGEHIVESTACK,
    

    /// Drax.
    ///
    /// "EXCHANGE_DRAX"
    #[serde(rename="EXCHANGE_DRAX")]
    EXCHANGEDRAX,
    

    /// AppLovin MAX.
    ///
    /// "EXCHANGE_APPLOVIN_GBID"
    #[serde(rename="EXCHANGE_APPLOVIN_GBID")]
    EXCHANGEAPPLOVINGBID,
    

    /// DT Fairbid.
    ///
    /// "EXCHANGE_FYBER_GBID"
    #[serde(rename="EXCHANGE_FYBER_GBID")]
    EXCHANGEFYBERGBID,
    

    /// Unity LevelPlay.
    ///
    /// "EXCHANGE_UNITY_GBID"
    #[serde(rename="EXCHANGE_UNITY_GBID")]
    EXCHANGEUNITYGBID,
    

    /// Chartboost Mediation.
    ///
    /// "EXCHANGE_CHARTBOOST_GBID"
    #[serde(rename="EXCHANGE_CHARTBOOST_GBID")]
    EXCHANGECHARTBOOSTGBID,
    

    /// AdMost.
    ///
    /// "EXCHANGE_ADMOST_GBID"
    #[serde(rename="EXCHANGE_ADMOST_GBID")]
    EXCHANGEADMOSTGBID,
    

    /// TopOn.
    ///
    /// "EXCHANGE_TOPON_GBID"
    #[serde(rename="EXCHANGE_TOPON_GBID")]
    EXCHANGETOPONGBID,
}

impl AsRef<str> for ExchangeConfigEnabledExchangeExchangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEUNSPECIFIED => "EXCHANGE_UNSPECIFIED",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEGOOGLEADMANAGER => "EXCHANGE_GOOGLE_AD_MANAGER",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEAPPNEXUS => "EXCHANGE_APPNEXUS",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEBRIGHTROLL => "EXCHANGE_BRIGHTROLL",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADFORM => "EXCHANGE_ADFORM",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADMETA => "EXCHANGE_ADMETA",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADMIXER => "EXCHANGE_ADMIXER",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADSMOGO => "EXCHANGE_ADSMOGO",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADSWIZZ => "EXCHANGE_ADSWIZZ",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEBIDSWITCH => "EXCHANGE_BIDSWITCH",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEBRIGHTROLLDISPLAY => "EXCHANGE_BRIGHTROLL_DISPLAY",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGECADREON => "EXCHANGE_CADREON",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEDAILYMOTION => "EXCHANGE_DAILYMOTION",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEFIVE => "EXCHANGE_FIVE",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEFLUCT => "EXCHANGE_FLUCT",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEFREEWHEEL => "EXCHANGE_FREEWHEEL",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEGENIEE => "EXCHANGE_GENIEE",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEGUMGUM => "EXCHANGE_GUMGUM",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEIMOBILE => "EXCHANGE_IMOBILE",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEIBILLBOARD => "EXCHANGE_IBILLBOARD",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEIMPROVEDIGITAL => "EXCHANGE_IMPROVE_DIGITAL",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEINDEX => "EXCHANGE_INDEX",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEKARGO => "EXCHANGE_KARGO",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEMICROAD => "EXCHANGE_MICROAD",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEMOPUB => "EXCHANGE_MOPUB",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGENEND => "EXCHANGE_NEND",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEONEBYAOLDISPLAY => "EXCHANGE_ONE_BY_AOL_DISPLAY",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEONEBYAOLMOBILE => "EXCHANGE_ONE_BY_AOL_MOBILE",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEONEBYAOLVIDEO => "EXCHANGE_ONE_BY_AOL_VIDEO",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEOOYALA => "EXCHANGE_OOYALA",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEOPENX => "EXCHANGE_OPENX",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPERMODO => "EXCHANGE_PERMODO",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPLATFORMONE => "EXCHANGE_PLATFORMONE",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPLATFORMID => "EXCHANGE_PLATFORMID",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPUBMATIC => "EXCHANGE_PUBMATIC",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPULSEPOINT => "EXCHANGE_PULSEPOINT",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEREVENUEMAX => "EXCHANGE_REVENUEMAX",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGERUBICON => "EXCHANGE_RUBICON",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESMARTCLIP => "EXCHANGE_SMARTCLIP",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESMARTRTB => "EXCHANGE_SMARTRTB",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESMARTSTREAMTV => "EXCHANGE_SMARTSTREAMTV",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESOVRN => "EXCHANGE_SOVRN",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESPOTXCHANGE => "EXCHANGE_SPOTXCHANGE",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESTROER => "EXCHANGE_STROER",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETEADSTV => "EXCHANGE_TEADSTV",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETELARIA => "EXCHANGE_TELARIA",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETVN => "EXCHANGE_TVN",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEUNITED => "EXCHANGE_UNITED",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEYIELDLAB => "EXCHANGE_YIELDLAB",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEYIELDMO => "EXCHANGE_YIELDMO",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEUNRULYX => "EXCHANGE_UNRULYX",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEOPEN8 => "EXCHANGE_OPEN8",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETRITON => "EXCHANGE_TRITON",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETRIPLELIFT => "EXCHANGE_TRIPLELIFT",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETABOOLA => "EXCHANGE_TABOOLA",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEINMOBI => "EXCHANGE_INMOBI",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESMAATO => "EXCHANGE_SMAATO",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEAJA => "EXCHANGE_AJA",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESUPERSHIP => "EXCHANGE_SUPERSHIP",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGENEXSTARDIGITAL => "EXCHANGE_NEXSTAR_DIGITAL",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEWAZE => "EXCHANGE_WAZE",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESOUNDCAST => "EXCHANGE_SOUNDCAST",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESHARETHROUGH => "EXCHANGE_SHARETHROUGH",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEFYBER => "EXCHANGE_FYBER",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEREDFORPUBLISHERS => "EXCHANGE_RED_FOR_PUBLISHERS",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEMEDIANET => "EXCHANGE_MEDIANET",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETAPJOY => "EXCHANGE_TAPJOY",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEVISTAR => "EXCHANGE_VISTAR",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEDAX => "EXCHANGE_DAX",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEJCD => "EXCHANGE_JCD",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPLACEEXCHANGE => "EXCHANGE_PLACE_EXCHANGE",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEAPPLOVIN => "EXCHANGE_APPLOVIN",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGECONNATIX => "EXCHANGE_CONNATIX",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGERESETDIGITAL => "EXCHANGE_RESET_DIGITAL",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEHIVESTACK => "EXCHANGE_HIVESTACK",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEDRAX => "EXCHANGE_DRAX",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEAPPLOVINGBID => "EXCHANGE_APPLOVIN_GBID",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEFYBERGBID => "EXCHANGE_FYBER_GBID",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEUNITYGBID => "EXCHANGE_UNITY_GBID",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGECHARTBOOSTGBID => "EXCHANGE_CHARTBOOST_GBID",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADMOSTGBID => "EXCHANGE_ADMOST_GBID",
            ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETOPONGBID => "EXCHANGE_TOPON_GBID",
        }
    }
}

impl std::convert::TryFrom< &str> for ExchangeConfigEnabledExchangeExchangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCHANGE_UNSPECIFIED" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEUNSPECIFIED),
           "EXCHANGE_GOOGLE_AD_MANAGER" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEGOOGLEADMANAGER),
           "EXCHANGE_APPNEXUS" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEAPPNEXUS),
           "EXCHANGE_BRIGHTROLL" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEBRIGHTROLL),
           "EXCHANGE_ADFORM" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADFORM),
           "EXCHANGE_ADMETA" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADMETA),
           "EXCHANGE_ADMIXER" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADMIXER),
           "EXCHANGE_ADSMOGO" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADSMOGO),
           "EXCHANGE_ADSWIZZ" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADSWIZZ),
           "EXCHANGE_BIDSWITCH" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEBIDSWITCH),
           "EXCHANGE_BRIGHTROLL_DISPLAY" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEBRIGHTROLLDISPLAY),
           "EXCHANGE_CADREON" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGECADREON),
           "EXCHANGE_DAILYMOTION" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEDAILYMOTION),
           "EXCHANGE_FIVE" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEFIVE),
           "EXCHANGE_FLUCT" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEFLUCT),
           "EXCHANGE_FREEWHEEL" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEFREEWHEEL),
           "EXCHANGE_GENIEE" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEGENIEE),
           "EXCHANGE_GUMGUM" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEGUMGUM),
           "EXCHANGE_IMOBILE" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEIMOBILE),
           "EXCHANGE_IBILLBOARD" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEIBILLBOARD),
           "EXCHANGE_IMPROVE_DIGITAL" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEIMPROVEDIGITAL),
           "EXCHANGE_INDEX" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEINDEX),
           "EXCHANGE_KARGO" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEKARGO),
           "EXCHANGE_MICROAD" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEMICROAD),
           "EXCHANGE_MOPUB" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEMOPUB),
           "EXCHANGE_NEND" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGENEND),
           "EXCHANGE_ONE_BY_AOL_DISPLAY" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEONEBYAOLDISPLAY),
           "EXCHANGE_ONE_BY_AOL_MOBILE" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEONEBYAOLMOBILE),
           "EXCHANGE_ONE_BY_AOL_VIDEO" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEONEBYAOLVIDEO),
           "EXCHANGE_OOYALA" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEOOYALA),
           "EXCHANGE_OPENX" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEOPENX),
           "EXCHANGE_PERMODO" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPERMODO),
           "EXCHANGE_PLATFORMONE" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPLATFORMONE),
           "EXCHANGE_PLATFORMID" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPLATFORMID),
           "EXCHANGE_PUBMATIC" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPUBMATIC),
           "EXCHANGE_PULSEPOINT" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPULSEPOINT),
           "EXCHANGE_REVENUEMAX" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEREVENUEMAX),
           "EXCHANGE_RUBICON" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGERUBICON),
           "EXCHANGE_SMARTCLIP" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESMARTCLIP),
           "EXCHANGE_SMARTRTB" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESMARTRTB),
           "EXCHANGE_SMARTSTREAMTV" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESMARTSTREAMTV),
           "EXCHANGE_SOVRN" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESOVRN),
           "EXCHANGE_SPOTXCHANGE" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESPOTXCHANGE),
           "EXCHANGE_STROER" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESTROER),
           "EXCHANGE_TEADSTV" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETEADSTV),
           "EXCHANGE_TELARIA" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETELARIA),
           "EXCHANGE_TVN" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETVN),
           "EXCHANGE_UNITED" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEUNITED),
           "EXCHANGE_YIELDLAB" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEYIELDLAB),
           "EXCHANGE_YIELDMO" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEYIELDMO),
           "EXCHANGE_UNRULYX" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEUNRULYX),
           "EXCHANGE_OPEN8" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEOPEN8),
           "EXCHANGE_TRITON" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETRITON),
           "EXCHANGE_TRIPLELIFT" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETRIPLELIFT),
           "EXCHANGE_TABOOLA" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETABOOLA),
           "EXCHANGE_INMOBI" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEINMOBI),
           "EXCHANGE_SMAATO" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESMAATO),
           "EXCHANGE_AJA" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEAJA),
           "EXCHANGE_SUPERSHIP" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESUPERSHIP),
           "EXCHANGE_NEXSTAR_DIGITAL" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGENEXSTARDIGITAL),
           "EXCHANGE_WAZE" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEWAZE),
           "EXCHANGE_SOUNDCAST" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESOUNDCAST),
           "EXCHANGE_SHARETHROUGH" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGESHARETHROUGH),
           "EXCHANGE_FYBER" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEFYBER),
           "EXCHANGE_RED_FOR_PUBLISHERS" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEREDFORPUBLISHERS),
           "EXCHANGE_MEDIANET" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEMEDIANET),
           "EXCHANGE_TAPJOY" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETAPJOY),
           "EXCHANGE_VISTAR" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEVISTAR),
           "EXCHANGE_DAX" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEDAX),
           "EXCHANGE_JCD" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEJCD),
           "EXCHANGE_PLACE_EXCHANGE" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEPLACEEXCHANGE),
           "EXCHANGE_APPLOVIN" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEAPPLOVIN),
           "EXCHANGE_CONNATIX" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGECONNATIX),
           "EXCHANGE_RESET_DIGITAL" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGERESETDIGITAL),
           "EXCHANGE_HIVESTACK" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEHIVESTACK),
           "EXCHANGE_DRAX" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEDRAX),
           "EXCHANGE_APPLOVIN_GBID" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEAPPLOVINGBID),
           "EXCHANGE_FYBER_GBID" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEFYBERGBID),
           "EXCHANGE_UNITY_GBID" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEUNITYGBID),
           "EXCHANGE_CHARTBOOST_GBID" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGECHARTBOOSTGBID),
           "EXCHANGE_ADMOST_GBID" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGEADMOSTGBID),
           "EXCHANGE_TOPON_GBID" => Ok(ExchangeConfigEnabledExchangeExchangeEnum::EXCHANGETOPONGBID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExchangeConfigEnabledExchangeExchangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExchangeReviewStatusExchangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The exchange reviewing the creative.
pub enum ExchangeReviewStatusExchangeEnum {
    

    /// Exchange is not specified or is unknown in this version.
    ///
    /// "EXCHANGE_UNSPECIFIED"
    #[serde(rename="EXCHANGE_UNSPECIFIED")]
    EXCHANGEUNSPECIFIED,
    

    /// Google Ad Manager.
    ///
    /// "EXCHANGE_GOOGLE_AD_MANAGER"
    #[serde(rename="EXCHANGE_GOOGLE_AD_MANAGER")]
    EXCHANGEGOOGLEADMANAGER,
    

    /// AppNexus.
    ///
    /// "EXCHANGE_APPNEXUS"
    #[serde(rename="EXCHANGE_APPNEXUS")]
    EXCHANGEAPPNEXUS,
    

    /// BrightRoll Exchange for Video from Yahoo!.
    ///
    /// "EXCHANGE_BRIGHTROLL"
    #[serde(rename="EXCHANGE_BRIGHTROLL")]
    EXCHANGEBRIGHTROLL,
    

    /// Adform.
    ///
    /// "EXCHANGE_ADFORM"
    #[serde(rename="EXCHANGE_ADFORM")]
    EXCHANGEADFORM,
    

    /// Admeta.
    ///
    /// "EXCHANGE_ADMETA"
    #[serde(rename="EXCHANGE_ADMETA")]
    EXCHANGEADMETA,
    

    /// Admixer.
    ///
    /// "EXCHANGE_ADMIXER"
    #[serde(rename="EXCHANGE_ADMIXER")]
    EXCHANGEADMIXER,
    

    /// AdsMogo.
    ///
    /// "EXCHANGE_ADSMOGO"
    #[serde(rename="EXCHANGE_ADSMOGO")]
    EXCHANGEADSMOGO,
    

    /// AdsWizz.
    ///
    /// "EXCHANGE_ADSWIZZ"
    #[serde(rename="EXCHANGE_ADSWIZZ")]
    EXCHANGEADSWIZZ,
    

    /// BidSwitch.
    ///
    /// "EXCHANGE_BIDSWITCH"
    #[serde(rename="EXCHANGE_BIDSWITCH")]
    EXCHANGEBIDSWITCH,
    

    /// BrightRoll Exchange for Display from Yahoo!.
    ///
    /// "EXCHANGE_BRIGHTROLL_DISPLAY"
    #[serde(rename="EXCHANGE_BRIGHTROLL_DISPLAY")]
    EXCHANGEBRIGHTROLLDISPLAY,
    

    /// Cadreon.
    ///
    /// "EXCHANGE_CADREON"
    #[serde(rename="EXCHANGE_CADREON")]
    EXCHANGECADREON,
    

    /// Dailymotion.
    ///
    /// "EXCHANGE_DAILYMOTION"
    #[serde(rename="EXCHANGE_DAILYMOTION")]
    EXCHANGEDAILYMOTION,
    

    /// Five.
    ///
    /// "EXCHANGE_FIVE"
    #[serde(rename="EXCHANGE_FIVE")]
    EXCHANGEFIVE,
    

    /// Fluct.
    ///
    /// "EXCHANGE_FLUCT"
    #[serde(rename="EXCHANGE_FLUCT")]
    EXCHANGEFLUCT,
    

    /// FreeWheel SSP.
    ///
    /// "EXCHANGE_FREEWHEEL"
    #[serde(rename="EXCHANGE_FREEWHEEL")]
    EXCHANGEFREEWHEEL,
    

    /// Geniee.
    ///
    /// "EXCHANGE_GENIEE"
    #[serde(rename="EXCHANGE_GENIEE")]
    EXCHANGEGENIEE,
    

    /// GumGum.
    ///
    /// "EXCHANGE_GUMGUM"
    #[serde(rename="EXCHANGE_GUMGUM")]
    EXCHANGEGUMGUM,
    

    /// i-mobile.
    ///
    /// "EXCHANGE_IMOBILE"
    #[serde(rename="EXCHANGE_IMOBILE")]
    EXCHANGEIMOBILE,
    

    /// iBILLBOARD.
    ///
    /// "EXCHANGE_IBILLBOARD"
    #[serde(rename="EXCHANGE_IBILLBOARD")]
    EXCHANGEIBILLBOARD,
    

    /// Improve Digital.
    ///
    /// "EXCHANGE_IMPROVE_DIGITAL"
    #[serde(rename="EXCHANGE_IMPROVE_DIGITAL")]
    EXCHANGEIMPROVEDIGITAL,
    

    /// Index Exchange.
    ///
    /// "EXCHANGE_INDEX"
    #[serde(rename="EXCHANGE_INDEX")]
    EXCHANGEINDEX,
    

    /// Kargo.
    ///
    /// "EXCHANGE_KARGO"
    #[serde(rename="EXCHANGE_KARGO")]
    EXCHANGEKARGO,
    

    /// MicroAd.
    ///
    /// "EXCHANGE_MICROAD"
    #[serde(rename="EXCHANGE_MICROAD")]
    EXCHANGEMICROAD,
    

    /// MoPub.
    ///
    /// "EXCHANGE_MOPUB"
    #[serde(rename="EXCHANGE_MOPUB")]
    EXCHANGEMOPUB,
    

    /// Nend.
    ///
    /// "EXCHANGE_NEND"
    #[serde(rename="EXCHANGE_NEND")]
    EXCHANGENEND,
    

    /// ONE by AOL: Display Market Place.
    ///
    /// "EXCHANGE_ONE_BY_AOL_DISPLAY"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_DISPLAY")]
    EXCHANGEONEBYAOLDISPLAY,
    

    /// ONE by AOL: Mobile.
    ///
    /// "EXCHANGE_ONE_BY_AOL_MOBILE"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_MOBILE")]
    EXCHANGEONEBYAOLMOBILE,
    

    /// ONE by AOL: Video.
    ///
    /// "EXCHANGE_ONE_BY_AOL_VIDEO"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_VIDEO")]
    EXCHANGEONEBYAOLVIDEO,
    

    /// Ooyala.
    ///
    /// "EXCHANGE_OOYALA"
    #[serde(rename="EXCHANGE_OOYALA")]
    EXCHANGEOOYALA,
    

    /// OpenX.
    ///
    /// "EXCHANGE_OPENX"
    #[serde(rename="EXCHANGE_OPENX")]
    EXCHANGEOPENX,
    

    /// Permodo.
    ///
    /// "EXCHANGE_PERMODO"
    #[serde(rename="EXCHANGE_PERMODO")]
    EXCHANGEPERMODO,
    

    /// Platform One.
    ///
    /// "EXCHANGE_PLATFORMONE"
    #[serde(rename="EXCHANGE_PLATFORMONE")]
    EXCHANGEPLATFORMONE,
    

    /// PlatformId.
    ///
    /// "EXCHANGE_PLATFORMID"
    #[serde(rename="EXCHANGE_PLATFORMID")]
    EXCHANGEPLATFORMID,
    

    /// PubMatic.
    ///
    /// "EXCHANGE_PUBMATIC"
    #[serde(rename="EXCHANGE_PUBMATIC")]
    EXCHANGEPUBMATIC,
    

    /// PulsePoint.
    ///
    /// "EXCHANGE_PULSEPOINT"
    #[serde(rename="EXCHANGE_PULSEPOINT")]
    EXCHANGEPULSEPOINT,
    

    /// RevenueMax.
    ///
    /// "EXCHANGE_REVENUEMAX"
    #[serde(rename="EXCHANGE_REVENUEMAX")]
    EXCHANGEREVENUEMAX,
    

    /// Rubicon.
    ///
    /// "EXCHANGE_RUBICON"
    #[serde(rename="EXCHANGE_RUBICON")]
    EXCHANGERUBICON,
    

    /// SmartClip.
    ///
    /// "EXCHANGE_SMARTCLIP"
    #[serde(rename="EXCHANGE_SMARTCLIP")]
    EXCHANGESMARTCLIP,
    

    /// SmartRTB+.
    ///
    /// "EXCHANGE_SMARTRTB"
    #[serde(rename="EXCHANGE_SMARTRTB")]
    EXCHANGESMARTRTB,
    

    /// SmartstreamTv.
    ///
    /// "EXCHANGE_SMARTSTREAMTV"
    #[serde(rename="EXCHANGE_SMARTSTREAMTV")]
    EXCHANGESMARTSTREAMTV,
    

    /// Sovrn.
    ///
    /// "EXCHANGE_SOVRN"
    #[serde(rename="EXCHANGE_SOVRN")]
    EXCHANGESOVRN,
    

    /// SpotXchange.
    ///
    /// "EXCHANGE_SPOTXCHANGE"
    #[serde(rename="EXCHANGE_SPOTXCHANGE")]
    EXCHANGESPOTXCHANGE,
    

    /// Ströer SSP.
    ///
    /// "EXCHANGE_STROER"
    #[serde(rename="EXCHANGE_STROER")]
    EXCHANGESTROER,
    

    /// TeadsTv.
    ///
    /// "EXCHANGE_TEADSTV"
    #[serde(rename="EXCHANGE_TEADSTV")]
    EXCHANGETEADSTV,
    

    /// Telaria.
    ///
    /// "EXCHANGE_TELARIA"
    #[serde(rename="EXCHANGE_TELARIA")]
    EXCHANGETELARIA,
    

    /// TVN.
    ///
    /// "EXCHANGE_TVN"
    #[serde(rename="EXCHANGE_TVN")]
    EXCHANGETVN,
    

    /// United.
    ///
    /// "EXCHANGE_UNITED"
    #[serde(rename="EXCHANGE_UNITED")]
    EXCHANGEUNITED,
    

    /// Yieldlab.
    ///
    /// "EXCHANGE_YIELDLAB"
    #[serde(rename="EXCHANGE_YIELDLAB")]
    EXCHANGEYIELDLAB,
    

    /// Yieldmo.
    ///
    /// "EXCHANGE_YIELDMO"
    #[serde(rename="EXCHANGE_YIELDMO")]
    EXCHANGEYIELDMO,
    

    /// UnrulyX.
    ///
    /// "EXCHANGE_UNRULYX"
    #[serde(rename="EXCHANGE_UNRULYX")]
    EXCHANGEUNRULYX,
    

    /// Open8.
    ///
    /// "EXCHANGE_OPEN8"
    #[serde(rename="EXCHANGE_OPEN8")]
    EXCHANGEOPEN8,
    

    /// Triton.
    ///
    /// "EXCHANGE_TRITON"
    #[serde(rename="EXCHANGE_TRITON")]
    EXCHANGETRITON,
    

    /// TripleLift.
    ///
    /// "EXCHANGE_TRIPLELIFT"
    #[serde(rename="EXCHANGE_TRIPLELIFT")]
    EXCHANGETRIPLELIFT,
    

    /// Taboola.
    ///
    /// "EXCHANGE_TABOOLA"
    #[serde(rename="EXCHANGE_TABOOLA")]
    EXCHANGETABOOLA,
    

    /// InMobi.
    ///
    /// "EXCHANGE_INMOBI"
    #[serde(rename="EXCHANGE_INMOBI")]
    EXCHANGEINMOBI,
    

    /// Smaato.
    ///
    /// "EXCHANGE_SMAATO"
    #[serde(rename="EXCHANGE_SMAATO")]
    EXCHANGESMAATO,
    

    /// Aja.
    ///
    /// "EXCHANGE_AJA"
    #[serde(rename="EXCHANGE_AJA")]
    EXCHANGEAJA,
    

    /// Supership.
    ///
    /// "EXCHANGE_SUPERSHIP"
    #[serde(rename="EXCHANGE_SUPERSHIP")]
    EXCHANGESUPERSHIP,
    

    /// Nexstar Digital.
    ///
    /// "EXCHANGE_NEXSTAR_DIGITAL"
    #[serde(rename="EXCHANGE_NEXSTAR_DIGITAL")]
    EXCHANGENEXSTARDIGITAL,
    

    /// Waze.
    ///
    /// "EXCHANGE_WAZE"
    #[serde(rename="EXCHANGE_WAZE")]
    EXCHANGEWAZE,
    

    /// SoundCast.
    ///
    /// "EXCHANGE_SOUNDCAST"
    #[serde(rename="EXCHANGE_SOUNDCAST")]
    EXCHANGESOUNDCAST,
    

    /// Sharethrough.
    ///
    /// "EXCHANGE_SHARETHROUGH"
    #[serde(rename="EXCHANGE_SHARETHROUGH")]
    EXCHANGESHARETHROUGH,
    

    /// Fyber.
    ///
    /// "EXCHANGE_FYBER"
    #[serde(rename="EXCHANGE_FYBER")]
    EXCHANGEFYBER,
    

    /// Red For Publishers.
    ///
    /// "EXCHANGE_RED_FOR_PUBLISHERS"
    #[serde(rename="EXCHANGE_RED_FOR_PUBLISHERS")]
    EXCHANGEREDFORPUBLISHERS,
    

    /// Media.net.
    ///
    /// "EXCHANGE_MEDIANET"
    #[serde(rename="EXCHANGE_MEDIANET")]
    EXCHANGEMEDIANET,
    

    /// Tapjoy.
    ///
    /// "EXCHANGE_TAPJOY"
    #[serde(rename="EXCHANGE_TAPJOY")]
    EXCHANGETAPJOY,
    

    /// Vistar.
    ///
    /// "EXCHANGE_VISTAR"
    #[serde(rename="EXCHANGE_VISTAR")]
    EXCHANGEVISTAR,
    

    /// DAX.
    ///
    /// "EXCHANGE_DAX"
    #[serde(rename="EXCHANGE_DAX")]
    EXCHANGEDAX,
    

    /// JCD.
    ///
    /// "EXCHANGE_JCD"
    #[serde(rename="EXCHANGE_JCD")]
    EXCHANGEJCD,
    

    /// Place Exchange.
    ///
    /// "EXCHANGE_PLACE_EXCHANGE"
    #[serde(rename="EXCHANGE_PLACE_EXCHANGE")]
    EXCHANGEPLACEEXCHANGE,
    

    /// AppLovin.
    ///
    /// "EXCHANGE_APPLOVIN"
    #[serde(rename="EXCHANGE_APPLOVIN")]
    EXCHANGEAPPLOVIN,
    

    /// Connatix.
    ///
    /// "EXCHANGE_CONNATIX"
    #[serde(rename="EXCHANGE_CONNATIX")]
    EXCHANGECONNATIX,
    

    /// Reset Digital.
    ///
    /// "EXCHANGE_RESET_DIGITAL"
    #[serde(rename="EXCHANGE_RESET_DIGITAL")]
    EXCHANGERESETDIGITAL,
    

    /// Hivestack.
    ///
    /// "EXCHANGE_HIVESTACK"
    #[serde(rename="EXCHANGE_HIVESTACK")]
    EXCHANGEHIVESTACK,
    

    /// Drax.
    ///
    /// "EXCHANGE_DRAX"
    #[serde(rename="EXCHANGE_DRAX")]
    EXCHANGEDRAX,
    

    /// AppLovin MAX.
    ///
    /// "EXCHANGE_APPLOVIN_GBID"
    #[serde(rename="EXCHANGE_APPLOVIN_GBID")]
    EXCHANGEAPPLOVINGBID,
    

    /// DT Fairbid.
    ///
    /// "EXCHANGE_FYBER_GBID"
    #[serde(rename="EXCHANGE_FYBER_GBID")]
    EXCHANGEFYBERGBID,
    

    /// Unity LevelPlay.
    ///
    /// "EXCHANGE_UNITY_GBID"
    #[serde(rename="EXCHANGE_UNITY_GBID")]
    EXCHANGEUNITYGBID,
    

    /// Chartboost Mediation.
    ///
    /// "EXCHANGE_CHARTBOOST_GBID"
    #[serde(rename="EXCHANGE_CHARTBOOST_GBID")]
    EXCHANGECHARTBOOSTGBID,
    

    /// AdMost.
    ///
    /// "EXCHANGE_ADMOST_GBID"
    #[serde(rename="EXCHANGE_ADMOST_GBID")]
    EXCHANGEADMOSTGBID,
    

    /// TopOn.
    ///
    /// "EXCHANGE_TOPON_GBID"
    #[serde(rename="EXCHANGE_TOPON_GBID")]
    EXCHANGETOPONGBID,
}

impl AsRef<str> for ExchangeReviewStatusExchangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExchangeReviewStatusExchangeEnum::EXCHANGEUNSPECIFIED => "EXCHANGE_UNSPECIFIED",
            ExchangeReviewStatusExchangeEnum::EXCHANGEGOOGLEADMANAGER => "EXCHANGE_GOOGLE_AD_MANAGER",
            ExchangeReviewStatusExchangeEnum::EXCHANGEAPPNEXUS => "EXCHANGE_APPNEXUS",
            ExchangeReviewStatusExchangeEnum::EXCHANGEBRIGHTROLL => "EXCHANGE_BRIGHTROLL",
            ExchangeReviewStatusExchangeEnum::EXCHANGEADFORM => "EXCHANGE_ADFORM",
            ExchangeReviewStatusExchangeEnum::EXCHANGEADMETA => "EXCHANGE_ADMETA",
            ExchangeReviewStatusExchangeEnum::EXCHANGEADMIXER => "EXCHANGE_ADMIXER",
            ExchangeReviewStatusExchangeEnum::EXCHANGEADSMOGO => "EXCHANGE_ADSMOGO",
            ExchangeReviewStatusExchangeEnum::EXCHANGEADSWIZZ => "EXCHANGE_ADSWIZZ",
            ExchangeReviewStatusExchangeEnum::EXCHANGEBIDSWITCH => "EXCHANGE_BIDSWITCH",
            ExchangeReviewStatusExchangeEnum::EXCHANGEBRIGHTROLLDISPLAY => "EXCHANGE_BRIGHTROLL_DISPLAY",
            ExchangeReviewStatusExchangeEnum::EXCHANGECADREON => "EXCHANGE_CADREON",
            ExchangeReviewStatusExchangeEnum::EXCHANGEDAILYMOTION => "EXCHANGE_DAILYMOTION",
            ExchangeReviewStatusExchangeEnum::EXCHANGEFIVE => "EXCHANGE_FIVE",
            ExchangeReviewStatusExchangeEnum::EXCHANGEFLUCT => "EXCHANGE_FLUCT",
            ExchangeReviewStatusExchangeEnum::EXCHANGEFREEWHEEL => "EXCHANGE_FREEWHEEL",
            ExchangeReviewStatusExchangeEnum::EXCHANGEGENIEE => "EXCHANGE_GENIEE",
            ExchangeReviewStatusExchangeEnum::EXCHANGEGUMGUM => "EXCHANGE_GUMGUM",
            ExchangeReviewStatusExchangeEnum::EXCHANGEIMOBILE => "EXCHANGE_IMOBILE",
            ExchangeReviewStatusExchangeEnum::EXCHANGEIBILLBOARD => "EXCHANGE_IBILLBOARD",
            ExchangeReviewStatusExchangeEnum::EXCHANGEIMPROVEDIGITAL => "EXCHANGE_IMPROVE_DIGITAL",
            ExchangeReviewStatusExchangeEnum::EXCHANGEINDEX => "EXCHANGE_INDEX",
            ExchangeReviewStatusExchangeEnum::EXCHANGEKARGO => "EXCHANGE_KARGO",
            ExchangeReviewStatusExchangeEnum::EXCHANGEMICROAD => "EXCHANGE_MICROAD",
            ExchangeReviewStatusExchangeEnum::EXCHANGEMOPUB => "EXCHANGE_MOPUB",
            ExchangeReviewStatusExchangeEnum::EXCHANGENEND => "EXCHANGE_NEND",
            ExchangeReviewStatusExchangeEnum::EXCHANGEONEBYAOLDISPLAY => "EXCHANGE_ONE_BY_AOL_DISPLAY",
            ExchangeReviewStatusExchangeEnum::EXCHANGEONEBYAOLMOBILE => "EXCHANGE_ONE_BY_AOL_MOBILE",
            ExchangeReviewStatusExchangeEnum::EXCHANGEONEBYAOLVIDEO => "EXCHANGE_ONE_BY_AOL_VIDEO",
            ExchangeReviewStatusExchangeEnum::EXCHANGEOOYALA => "EXCHANGE_OOYALA",
            ExchangeReviewStatusExchangeEnum::EXCHANGEOPENX => "EXCHANGE_OPENX",
            ExchangeReviewStatusExchangeEnum::EXCHANGEPERMODO => "EXCHANGE_PERMODO",
            ExchangeReviewStatusExchangeEnum::EXCHANGEPLATFORMONE => "EXCHANGE_PLATFORMONE",
            ExchangeReviewStatusExchangeEnum::EXCHANGEPLATFORMID => "EXCHANGE_PLATFORMID",
            ExchangeReviewStatusExchangeEnum::EXCHANGEPUBMATIC => "EXCHANGE_PUBMATIC",
            ExchangeReviewStatusExchangeEnum::EXCHANGEPULSEPOINT => "EXCHANGE_PULSEPOINT",
            ExchangeReviewStatusExchangeEnum::EXCHANGEREVENUEMAX => "EXCHANGE_REVENUEMAX",
            ExchangeReviewStatusExchangeEnum::EXCHANGERUBICON => "EXCHANGE_RUBICON",
            ExchangeReviewStatusExchangeEnum::EXCHANGESMARTCLIP => "EXCHANGE_SMARTCLIP",
            ExchangeReviewStatusExchangeEnum::EXCHANGESMARTRTB => "EXCHANGE_SMARTRTB",
            ExchangeReviewStatusExchangeEnum::EXCHANGESMARTSTREAMTV => "EXCHANGE_SMARTSTREAMTV",
            ExchangeReviewStatusExchangeEnum::EXCHANGESOVRN => "EXCHANGE_SOVRN",
            ExchangeReviewStatusExchangeEnum::EXCHANGESPOTXCHANGE => "EXCHANGE_SPOTXCHANGE",
            ExchangeReviewStatusExchangeEnum::EXCHANGESTROER => "EXCHANGE_STROER",
            ExchangeReviewStatusExchangeEnum::EXCHANGETEADSTV => "EXCHANGE_TEADSTV",
            ExchangeReviewStatusExchangeEnum::EXCHANGETELARIA => "EXCHANGE_TELARIA",
            ExchangeReviewStatusExchangeEnum::EXCHANGETVN => "EXCHANGE_TVN",
            ExchangeReviewStatusExchangeEnum::EXCHANGEUNITED => "EXCHANGE_UNITED",
            ExchangeReviewStatusExchangeEnum::EXCHANGEYIELDLAB => "EXCHANGE_YIELDLAB",
            ExchangeReviewStatusExchangeEnum::EXCHANGEYIELDMO => "EXCHANGE_YIELDMO",
            ExchangeReviewStatusExchangeEnum::EXCHANGEUNRULYX => "EXCHANGE_UNRULYX",
            ExchangeReviewStatusExchangeEnum::EXCHANGEOPEN8 => "EXCHANGE_OPEN8",
            ExchangeReviewStatusExchangeEnum::EXCHANGETRITON => "EXCHANGE_TRITON",
            ExchangeReviewStatusExchangeEnum::EXCHANGETRIPLELIFT => "EXCHANGE_TRIPLELIFT",
            ExchangeReviewStatusExchangeEnum::EXCHANGETABOOLA => "EXCHANGE_TABOOLA",
            ExchangeReviewStatusExchangeEnum::EXCHANGEINMOBI => "EXCHANGE_INMOBI",
            ExchangeReviewStatusExchangeEnum::EXCHANGESMAATO => "EXCHANGE_SMAATO",
            ExchangeReviewStatusExchangeEnum::EXCHANGEAJA => "EXCHANGE_AJA",
            ExchangeReviewStatusExchangeEnum::EXCHANGESUPERSHIP => "EXCHANGE_SUPERSHIP",
            ExchangeReviewStatusExchangeEnum::EXCHANGENEXSTARDIGITAL => "EXCHANGE_NEXSTAR_DIGITAL",
            ExchangeReviewStatusExchangeEnum::EXCHANGEWAZE => "EXCHANGE_WAZE",
            ExchangeReviewStatusExchangeEnum::EXCHANGESOUNDCAST => "EXCHANGE_SOUNDCAST",
            ExchangeReviewStatusExchangeEnum::EXCHANGESHARETHROUGH => "EXCHANGE_SHARETHROUGH",
            ExchangeReviewStatusExchangeEnum::EXCHANGEFYBER => "EXCHANGE_FYBER",
            ExchangeReviewStatusExchangeEnum::EXCHANGEREDFORPUBLISHERS => "EXCHANGE_RED_FOR_PUBLISHERS",
            ExchangeReviewStatusExchangeEnum::EXCHANGEMEDIANET => "EXCHANGE_MEDIANET",
            ExchangeReviewStatusExchangeEnum::EXCHANGETAPJOY => "EXCHANGE_TAPJOY",
            ExchangeReviewStatusExchangeEnum::EXCHANGEVISTAR => "EXCHANGE_VISTAR",
            ExchangeReviewStatusExchangeEnum::EXCHANGEDAX => "EXCHANGE_DAX",
            ExchangeReviewStatusExchangeEnum::EXCHANGEJCD => "EXCHANGE_JCD",
            ExchangeReviewStatusExchangeEnum::EXCHANGEPLACEEXCHANGE => "EXCHANGE_PLACE_EXCHANGE",
            ExchangeReviewStatusExchangeEnum::EXCHANGEAPPLOVIN => "EXCHANGE_APPLOVIN",
            ExchangeReviewStatusExchangeEnum::EXCHANGECONNATIX => "EXCHANGE_CONNATIX",
            ExchangeReviewStatusExchangeEnum::EXCHANGERESETDIGITAL => "EXCHANGE_RESET_DIGITAL",
            ExchangeReviewStatusExchangeEnum::EXCHANGEHIVESTACK => "EXCHANGE_HIVESTACK",
            ExchangeReviewStatusExchangeEnum::EXCHANGEDRAX => "EXCHANGE_DRAX",
            ExchangeReviewStatusExchangeEnum::EXCHANGEAPPLOVINGBID => "EXCHANGE_APPLOVIN_GBID",
            ExchangeReviewStatusExchangeEnum::EXCHANGEFYBERGBID => "EXCHANGE_FYBER_GBID",
            ExchangeReviewStatusExchangeEnum::EXCHANGEUNITYGBID => "EXCHANGE_UNITY_GBID",
            ExchangeReviewStatusExchangeEnum::EXCHANGECHARTBOOSTGBID => "EXCHANGE_CHARTBOOST_GBID",
            ExchangeReviewStatusExchangeEnum::EXCHANGEADMOSTGBID => "EXCHANGE_ADMOST_GBID",
            ExchangeReviewStatusExchangeEnum::EXCHANGETOPONGBID => "EXCHANGE_TOPON_GBID",
        }
    }
}

impl std::convert::TryFrom< &str> for ExchangeReviewStatusExchangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCHANGE_UNSPECIFIED" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEUNSPECIFIED),
           "EXCHANGE_GOOGLE_AD_MANAGER" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEGOOGLEADMANAGER),
           "EXCHANGE_APPNEXUS" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEAPPNEXUS),
           "EXCHANGE_BRIGHTROLL" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEBRIGHTROLL),
           "EXCHANGE_ADFORM" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEADFORM),
           "EXCHANGE_ADMETA" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEADMETA),
           "EXCHANGE_ADMIXER" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEADMIXER),
           "EXCHANGE_ADSMOGO" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEADSMOGO),
           "EXCHANGE_ADSWIZZ" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEADSWIZZ),
           "EXCHANGE_BIDSWITCH" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEBIDSWITCH),
           "EXCHANGE_BRIGHTROLL_DISPLAY" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEBRIGHTROLLDISPLAY),
           "EXCHANGE_CADREON" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGECADREON),
           "EXCHANGE_DAILYMOTION" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEDAILYMOTION),
           "EXCHANGE_FIVE" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEFIVE),
           "EXCHANGE_FLUCT" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEFLUCT),
           "EXCHANGE_FREEWHEEL" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEFREEWHEEL),
           "EXCHANGE_GENIEE" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEGENIEE),
           "EXCHANGE_GUMGUM" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEGUMGUM),
           "EXCHANGE_IMOBILE" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEIMOBILE),
           "EXCHANGE_IBILLBOARD" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEIBILLBOARD),
           "EXCHANGE_IMPROVE_DIGITAL" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEIMPROVEDIGITAL),
           "EXCHANGE_INDEX" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEINDEX),
           "EXCHANGE_KARGO" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEKARGO),
           "EXCHANGE_MICROAD" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEMICROAD),
           "EXCHANGE_MOPUB" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEMOPUB),
           "EXCHANGE_NEND" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGENEND),
           "EXCHANGE_ONE_BY_AOL_DISPLAY" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEONEBYAOLDISPLAY),
           "EXCHANGE_ONE_BY_AOL_MOBILE" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEONEBYAOLMOBILE),
           "EXCHANGE_ONE_BY_AOL_VIDEO" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEONEBYAOLVIDEO),
           "EXCHANGE_OOYALA" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEOOYALA),
           "EXCHANGE_OPENX" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEOPENX),
           "EXCHANGE_PERMODO" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEPERMODO),
           "EXCHANGE_PLATFORMONE" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEPLATFORMONE),
           "EXCHANGE_PLATFORMID" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEPLATFORMID),
           "EXCHANGE_PUBMATIC" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEPUBMATIC),
           "EXCHANGE_PULSEPOINT" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEPULSEPOINT),
           "EXCHANGE_REVENUEMAX" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEREVENUEMAX),
           "EXCHANGE_RUBICON" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGERUBICON),
           "EXCHANGE_SMARTCLIP" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGESMARTCLIP),
           "EXCHANGE_SMARTRTB" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGESMARTRTB),
           "EXCHANGE_SMARTSTREAMTV" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGESMARTSTREAMTV),
           "EXCHANGE_SOVRN" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGESOVRN),
           "EXCHANGE_SPOTXCHANGE" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGESPOTXCHANGE),
           "EXCHANGE_STROER" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGESTROER),
           "EXCHANGE_TEADSTV" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGETEADSTV),
           "EXCHANGE_TELARIA" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGETELARIA),
           "EXCHANGE_TVN" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGETVN),
           "EXCHANGE_UNITED" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEUNITED),
           "EXCHANGE_YIELDLAB" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEYIELDLAB),
           "EXCHANGE_YIELDMO" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEYIELDMO),
           "EXCHANGE_UNRULYX" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEUNRULYX),
           "EXCHANGE_OPEN8" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEOPEN8),
           "EXCHANGE_TRITON" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGETRITON),
           "EXCHANGE_TRIPLELIFT" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGETRIPLELIFT),
           "EXCHANGE_TABOOLA" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGETABOOLA),
           "EXCHANGE_INMOBI" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEINMOBI),
           "EXCHANGE_SMAATO" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGESMAATO),
           "EXCHANGE_AJA" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEAJA),
           "EXCHANGE_SUPERSHIP" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGESUPERSHIP),
           "EXCHANGE_NEXSTAR_DIGITAL" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGENEXSTARDIGITAL),
           "EXCHANGE_WAZE" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEWAZE),
           "EXCHANGE_SOUNDCAST" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGESOUNDCAST),
           "EXCHANGE_SHARETHROUGH" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGESHARETHROUGH),
           "EXCHANGE_FYBER" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEFYBER),
           "EXCHANGE_RED_FOR_PUBLISHERS" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEREDFORPUBLISHERS),
           "EXCHANGE_MEDIANET" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEMEDIANET),
           "EXCHANGE_TAPJOY" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGETAPJOY),
           "EXCHANGE_VISTAR" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEVISTAR),
           "EXCHANGE_DAX" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEDAX),
           "EXCHANGE_JCD" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEJCD),
           "EXCHANGE_PLACE_EXCHANGE" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEPLACEEXCHANGE),
           "EXCHANGE_APPLOVIN" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEAPPLOVIN),
           "EXCHANGE_CONNATIX" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGECONNATIX),
           "EXCHANGE_RESET_DIGITAL" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGERESETDIGITAL),
           "EXCHANGE_HIVESTACK" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEHIVESTACK),
           "EXCHANGE_DRAX" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEDRAX),
           "EXCHANGE_APPLOVIN_GBID" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEAPPLOVINGBID),
           "EXCHANGE_FYBER_GBID" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEFYBERGBID),
           "EXCHANGE_UNITY_GBID" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEUNITYGBID),
           "EXCHANGE_CHARTBOOST_GBID" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGECHARTBOOSTGBID),
           "EXCHANGE_ADMOST_GBID" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGEADMOSTGBID),
           "EXCHANGE_TOPON_GBID" => Ok(ExchangeReviewStatusExchangeEnum::EXCHANGETOPONGBID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExchangeReviewStatusExchangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExchangeReviewStatusStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the exchange review.
pub enum ExchangeReviewStatusStatusEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "REVIEW_STATUS_UNSPECIFIED"
    #[serde(rename="REVIEW_STATUS_UNSPECIFIED")]
    REVIEWSTATUSUNSPECIFIED,
    

    /// The creative is approved.
    ///
    /// "REVIEW_STATUS_APPROVED"
    #[serde(rename="REVIEW_STATUS_APPROVED")]
    REVIEWSTATUSAPPROVED,
    

    /// The creative is rejected.
    ///
    /// "REVIEW_STATUS_REJECTED"
    #[serde(rename="REVIEW_STATUS_REJECTED")]
    REVIEWSTATUSREJECTED,
    

    /// The creative is pending review.
    ///
    /// "REVIEW_STATUS_PENDING"
    #[serde(rename="REVIEW_STATUS_PENDING")]
    REVIEWSTATUSPENDING,
}

impl AsRef<str> for ExchangeReviewStatusStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExchangeReviewStatusStatusEnum::REVIEWSTATUSUNSPECIFIED => "REVIEW_STATUS_UNSPECIFIED",
            ExchangeReviewStatusStatusEnum::REVIEWSTATUSAPPROVED => "REVIEW_STATUS_APPROVED",
            ExchangeReviewStatusStatusEnum::REVIEWSTATUSREJECTED => "REVIEW_STATUS_REJECTED",
            ExchangeReviewStatusStatusEnum::REVIEWSTATUSPENDING => "REVIEW_STATUS_PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for ExchangeReviewStatusStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVIEW_STATUS_UNSPECIFIED" => Ok(ExchangeReviewStatusStatusEnum::REVIEWSTATUSUNSPECIFIED),
           "REVIEW_STATUS_APPROVED" => Ok(ExchangeReviewStatusStatusEnum::REVIEWSTATUSAPPROVED),
           "REVIEW_STATUS_REJECTED" => Ok(ExchangeReviewStatusStatusEnum::REVIEWSTATUSREJECTED),
           "REVIEW_STATUS_PENDING" => Ok(ExchangeReviewStatusStatusEnum::REVIEWSTATUSPENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExchangeReviewStatusStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExchangeTargetingOptionDetailExchangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of exchange.
pub enum ExchangeTargetingOptionDetailExchangeEnum {
    

    /// Exchange is not specified or is unknown in this version.
    ///
    /// "EXCHANGE_UNSPECIFIED"
    #[serde(rename="EXCHANGE_UNSPECIFIED")]
    EXCHANGEUNSPECIFIED,
    

    /// Google Ad Manager.
    ///
    /// "EXCHANGE_GOOGLE_AD_MANAGER"
    #[serde(rename="EXCHANGE_GOOGLE_AD_MANAGER")]
    EXCHANGEGOOGLEADMANAGER,
    

    /// AppNexus.
    ///
    /// "EXCHANGE_APPNEXUS"
    #[serde(rename="EXCHANGE_APPNEXUS")]
    EXCHANGEAPPNEXUS,
    

    /// BrightRoll Exchange for Video from Yahoo!.
    ///
    /// "EXCHANGE_BRIGHTROLL"
    #[serde(rename="EXCHANGE_BRIGHTROLL")]
    EXCHANGEBRIGHTROLL,
    

    /// Adform.
    ///
    /// "EXCHANGE_ADFORM"
    #[serde(rename="EXCHANGE_ADFORM")]
    EXCHANGEADFORM,
    

    /// Admeta.
    ///
    /// "EXCHANGE_ADMETA"
    #[serde(rename="EXCHANGE_ADMETA")]
    EXCHANGEADMETA,
    

    /// Admixer.
    ///
    /// "EXCHANGE_ADMIXER"
    #[serde(rename="EXCHANGE_ADMIXER")]
    EXCHANGEADMIXER,
    

    /// AdsMogo.
    ///
    /// "EXCHANGE_ADSMOGO"
    #[serde(rename="EXCHANGE_ADSMOGO")]
    EXCHANGEADSMOGO,
    

    /// AdsWizz.
    ///
    /// "EXCHANGE_ADSWIZZ"
    #[serde(rename="EXCHANGE_ADSWIZZ")]
    EXCHANGEADSWIZZ,
    

    /// BidSwitch.
    ///
    /// "EXCHANGE_BIDSWITCH"
    #[serde(rename="EXCHANGE_BIDSWITCH")]
    EXCHANGEBIDSWITCH,
    

    /// BrightRoll Exchange for Display from Yahoo!.
    ///
    /// "EXCHANGE_BRIGHTROLL_DISPLAY"
    #[serde(rename="EXCHANGE_BRIGHTROLL_DISPLAY")]
    EXCHANGEBRIGHTROLLDISPLAY,
    

    /// Cadreon.
    ///
    /// "EXCHANGE_CADREON"
    #[serde(rename="EXCHANGE_CADREON")]
    EXCHANGECADREON,
    

    /// Dailymotion.
    ///
    /// "EXCHANGE_DAILYMOTION"
    #[serde(rename="EXCHANGE_DAILYMOTION")]
    EXCHANGEDAILYMOTION,
    

    /// Five.
    ///
    /// "EXCHANGE_FIVE"
    #[serde(rename="EXCHANGE_FIVE")]
    EXCHANGEFIVE,
    

    /// Fluct.
    ///
    /// "EXCHANGE_FLUCT"
    #[serde(rename="EXCHANGE_FLUCT")]
    EXCHANGEFLUCT,
    

    /// FreeWheel SSP.
    ///
    /// "EXCHANGE_FREEWHEEL"
    #[serde(rename="EXCHANGE_FREEWHEEL")]
    EXCHANGEFREEWHEEL,
    

    /// Geniee.
    ///
    /// "EXCHANGE_GENIEE"
    #[serde(rename="EXCHANGE_GENIEE")]
    EXCHANGEGENIEE,
    

    /// GumGum.
    ///
    /// "EXCHANGE_GUMGUM"
    #[serde(rename="EXCHANGE_GUMGUM")]
    EXCHANGEGUMGUM,
    

    /// i-mobile.
    ///
    /// "EXCHANGE_IMOBILE"
    #[serde(rename="EXCHANGE_IMOBILE")]
    EXCHANGEIMOBILE,
    

    /// iBILLBOARD.
    ///
    /// "EXCHANGE_IBILLBOARD"
    #[serde(rename="EXCHANGE_IBILLBOARD")]
    EXCHANGEIBILLBOARD,
    

    /// Improve Digital.
    ///
    /// "EXCHANGE_IMPROVE_DIGITAL"
    #[serde(rename="EXCHANGE_IMPROVE_DIGITAL")]
    EXCHANGEIMPROVEDIGITAL,
    

    /// Index Exchange.
    ///
    /// "EXCHANGE_INDEX"
    #[serde(rename="EXCHANGE_INDEX")]
    EXCHANGEINDEX,
    

    /// Kargo.
    ///
    /// "EXCHANGE_KARGO"
    #[serde(rename="EXCHANGE_KARGO")]
    EXCHANGEKARGO,
    

    /// MicroAd.
    ///
    /// "EXCHANGE_MICROAD"
    #[serde(rename="EXCHANGE_MICROAD")]
    EXCHANGEMICROAD,
    

    /// MoPub.
    ///
    /// "EXCHANGE_MOPUB"
    #[serde(rename="EXCHANGE_MOPUB")]
    EXCHANGEMOPUB,
    

    /// Nend.
    ///
    /// "EXCHANGE_NEND"
    #[serde(rename="EXCHANGE_NEND")]
    EXCHANGENEND,
    

    /// ONE by AOL: Display Market Place.
    ///
    /// "EXCHANGE_ONE_BY_AOL_DISPLAY"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_DISPLAY")]
    EXCHANGEONEBYAOLDISPLAY,
    

    /// ONE by AOL: Mobile.
    ///
    /// "EXCHANGE_ONE_BY_AOL_MOBILE"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_MOBILE")]
    EXCHANGEONEBYAOLMOBILE,
    

    /// ONE by AOL: Video.
    ///
    /// "EXCHANGE_ONE_BY_AOL_VIDEO"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_VIDEO")]
    EXCHANGEONEBYAOLVIDEO,
    

    /// Ooyala.
    ///
    /// "EXCHANGE_OOYALA"
    #[serde(rename="EXCHANGE_OOYALA")]
    EXCHANGEOOYALA,
    

    /// OpenX.
    ///
    /// "EXCHANGE_OPENX"
    #[serde(rename="EXCHANGE_OPENX")]
    EXCHANGEOPENX,
    

    /// Permodo.
    ///
    /// "EXCHANGE_PERMODO"
    #[serde(rename="EXCHANGE_PERMODO")]
    EXCHANGEPERMODO,
    

    /// Platform One.
    ///
    /// "EXCHANGE_PLATFORMONE"
    #[serde(rename="EXCHANGE_PLATFORMONE")]
    EXCHANGEPLATFORMONE,
    

    /// PlatformId.
    ///
    /// "EXCHANGE_PLATFORMID"
    #[serde(rename="EXCHANGE_PLATFORMID")]
    EXCHANGEPLATFORMID,
    

    /// PubMatic.
    ///
    /// "EXCHANGE_PUBMATIC"
    #[serde(rename="EXCHANGE_PUBMATIC")]
    EXCHANGEPUBMATIC,
    

    /// PulsePoint.
    ///
    /// "EXCHANGE_PULSEPOINT"
    #[serde(rename="EXCHANGE_PULSEPOINT")]
    EXCHANGEPULSEPOINT,
    

    /// RevenueMax.
    ///
    /// "EXCHANGE_REVENUEMAX"
    #[serde(rename="EXCHANGE_REVENUEMAX")]
    EXCHANGEREVENUEMAX,
    

    /// Rubicon.
    ///
    /// "EXCHANGE_RUBICON"
    #[serde(rename="EXCHANGE_RUBICON")]
    EXCHANGERUBICON,
    

    /// SmartClip.
    ///
    /// "EXCHANGE_SMARTCLIP"
    #[serde(rename="EXCHANGE_SMARTCLIP")]
    EXCHANGESMARTCLIP,
    

    /// SmartRTB+.
    ///
    /// "EXCHANGE_SMARTRTB"
    #[serde(rename="EXCHANGE_SMARTRTB")]
    EXCHANGESMARTRTB,
    

    /// SmartstreamTv.
    ///
    /// "EXCHANGE_SMARTSTREAMTV"
    #[serde(rename="EXCHANGE_SMARTSTREAMTV")]
    EXCHANGESMARTSTREAMTV,
    

    /// Sovrn.
    ///
    /// "EXCHANGE_SOVRN"
    #[serde(rename="EXCHANGE_SOVRN")]
    EXCHANGESOVRN,
    

    /// SpotXchange.
    ///
    /// "EXCHANGE_SPOTXCHANGE"
    #[serde(rename="EXCHANGE_SPOTXCHANGE")]
    EXCHANGESPOTXCHANGE,
    

    /// Ströer SSP.
    ///
    /// "EXCHANGE_STROER"
    #[serde(rename="EXCHANGE_STROER")]
    EXCHANGESTROER,
    

    /// TeadsTv.
    ///
    /// "EXCHANGE_TEADSTV"
    #[serde(rename="EXCHANGE_TEADSTV")]
    EXCHANGETEADSTV,
    

    /// Telaria.
    ///
    /// "EXCHANGE_TELARIA"
    #[serde(rename="EXCHANGE_TELARIA")]
    EXCHANGETELARIA,
    

    /// TVN.
    ///
    /// "EXCHANGE_TVN"
    #[serde(rename="EXCHANGE_TVN")]
    EXCHANGETVN,
    

    /// United.
    ///
    /// "EXCHANGE_UNITED"
    #[serde(rename="EXCHANGE_UNITED")]
    EXCHANGEUNITED,
    

    /// Yieldlab.
    ///
    /// "EXCHANGE_YIELDLAB"
    #[serde(rename="EXCHANGE_YIELDLAB")]
    EXCHANGEYIELDLAB,
    

    /// Yieldmo.
    ///
    /// "EXCHANGE_YIELDMO"
    #[serde(rename="EXCHANGE_YIELDMO")]
    EXCHANGEYIELDMO,
    

    /// UnrulyX.
    ///
    /// "EXCHANGE_UNRULYX"
    #[serde(rename="EXCHANGE_UNRULYX")]
    EXCHANGEUNRULYX,
    

    /// Open8.
    ///
    /// "EXCHANGE_OPEN8"
    #[serde(rename="EXCHANGE_OPEN8")]
    EXCHANGEOPEN8,
    

    /// Triton.
    ///
    /// "EXCHANGE_TRITON"
    #[serde(rename="EXCHANGE_TRITON")]
    EXCHANGETRITON,
    

    /// TripleLift.
    ///
    /// "EXCHANGE_TRIPLELIFT"
    #[serde(rename="EXCHANGE_TRIPLELIFT")]
    EXCHANGETRIPLELIFT,
    

    /// Taboola.
    ///
    /// "EXCHANGE_TABOOLA"
    #[serde(rename="EXCHANGE_TABOOLA")]
    EXCHANGETABOOLA,
    

    /// InMobi.
    ///
    /// "EXCHANGE_INMOBI"
    #[serde(rename="EXCHANGE_INMOBI")]
    EXCHANGEINMOBI,
    

    /// Smaato.
    ///
    /// "EXCHANGE_SMAATO"
    #[serde(rename="EXCHANGE_SMAATO")]
    EXCHANGESMAATO,
    

    /// Aja.
    ///
    /// "EXCHANGE_AJA"
    #[serde(rename="EXCHANGE_AJA")]
    EXCHANGEAJA,
    

    /// Supership.
    ///
    /// "EXCHANGE_SUPERSHIP"
    #[serde(rename="EXCHANGE_SUPERSHIP")]
    EXCHANGESUPERSHIP,
    

    /// Nexstar Digital.
    ///
    /// "EXCHANGE_NEXSTAR_DIGITAL"
    #[serde(rename="EXCHANGE_NEXSTAR_DIGITAL")]
    EXCHANGENEXSTARDIGITAL,
    

    /// Waze.
    ///
    /// "EXCHANGE_WAZE"
    #[serde(rename="EXCHANGE_WAZE")]
    EXCHANGEWAZE,
    

    /// SoundCast.
    ///
    /// "EXCHANGE_SOUNDCAST"
    #[serde(rename="EXCHANGE_SOUNDCAST")]
    EXCHANGESOUNDCAST,
    

    /// Sharethrough.
    ///
    /// "EXCHANGE_SHARETHROUGH"
    #[serde(rename="EXCHANGE_SHARETHROUGH")]
    EXCHANGESHARETHROUGH,
    

    /// Fyber.
    ///
    /// "EXCHANGE_FYBER"
    #[serde(rename="EXCHANGE_FYBER")]
    EXCHANGEFYBER,
    

    /// Red For Publishers.
    ///
    /// "EXCHANGE_RED_FOR_PUBLISHERS"
    #[serde(rename="EXCHANGE_RED_FOR_PUBLISHERS")]
    EXCHANGEREDFORPUBLISHERS,
    

    /// Media.net.
    ///
    /// "EXCHANGE_MEDIANET"
    #[serde(rename="EXCHANGE_MEDIANET")]
    EXCHANGEMEDIANET,
    

    /// Tapjoy.
    ///
    /// "EXCHANGE_TAPJOY"
    #[serde(rename="EXCHANGE_TAPJOY")]
    EXCHANGETAPJOY,
    

    /// Vistar.
    ///
    /// "EXCHANGE_VISTAR"
    #[serde(rename="EXCHANGE_VISTAR")]
    EXCHANGEVISTAR,
    

    /// DAX.
    ///
    /// "EXCHANGE_DAX"
    #[serde(rename="EXCHANGE_DAX")]
    EXCHANGEDAX,
    

    /// JCD.
    ///
    /// "EXCHANGE_JCD"
    #[serde(rename="EXCHANGE_JCD")]
    EXCHANGEJCD,
    

    /// Place Exchange.
    ///
    /// "EXCHANGE_PLACE_EXCHANGE"
    #[serde(rename="EXCHANGE_PLACE_EXCHANGE")]
    EXCHANGEPLACEEXCHANGE,
    

    /// AppLovin.
    ///
    /// "EXCHANGE_APPLOVIN"
    #[serde(rename="EXCHANGE_APPLOVIN")]
    EXCHANGEAPPLOVIN,
    

    /// Connatix.
    ///
    /// "EXCHANGE_CONNATIX"
    #[serde(rename="EXCHANGE_CONNATIX")]
    EXCHANGECONNATIX,
    

    /// Reset Digital.
    ///
    /// "EXCHANGE_RESET_DIGITAL"
    #[serde(rename="EXCHANGE_RESET_DIGITAL")]
    EXCHANGERESETDIGITAL,
    

    /// Hivestack.
    ///
    /// "EXCHANGE_HIVESTACK"
    #[serde(rename="EXCHANGE_HIVESTACK")]
    EXCHANGEHIVESTACK,
    

    /// Drax.
    ///
    /// "EXCHANGE_DRAX"
    #[serde(rename="EXCHANGE_DRAX")]
    EXCHANGEDRAX,
    

    /// AppLovin MAX.
    ///
    /// "EXCHANGE_APPLOVIN_GBID"
    #[serde(rename="EXCHANGE_APPLOVIN_GBID")]
    EXCHANGEAPPLOVINGBID,
    

    /// DT Fairbid.
    ///
    /// "EXCHANGE_FYBER_GBID"
    #[serde(rename="EXCHANGE_FYBER_GBID")]
    EXCHANGEFYBERGBID,
    

    /// Unity LevelPlay.
    ///
    /// "EXCHANGE_UNITY_GBID"
    #[serde(rename="EXCHANGE_UNITY_GBID")]
    EXCHANGEUNITYGBID,
    

    /// Chartboost Mediation.
    ///
    /// "EXCHANGE_CHARTBOOST_GBID"
    #[serde(rename="EXCHANGE_CHARTBOOST_GBID")]
    EXCHANGECHARTBOOSTGBID,
    

    /// AdMost.
    ///
    /// "EXCHANGE_ADMOST_GBID"
    #[serde(rename="EXCHANGE_ADMOST_GBID")]
    EXCHANGEADMOSTGBID,
    

    /// TopOn.
    ///
    /// "EXCHANGE_TOPON_GBID"
    #[serde(rename="EXCHANGE_TOPON_GBID")]
    EXCHANGETOPONGBID,
}

impl AsRef<str> for ExchangeTargetingOptionDetailExchangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEUNSPECIFIED => "EXCHANGE_UNSPECIFIED",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEGOOGLEADMANAGER => "EXCHANGE_GOOGLE_AD_MANAGER",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEAPPNEXUS => "EXCHANGE_APPNEXUS",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEBRIGHTROLL => "EXCHANGE_BRIGHTROLL",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADFORM => "EXCHANGE_ADFORM",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADMETA => "EXCHANGE_ADMETA",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADMIXER => "EXCHANGE_ADMIXER",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADSMOGO => "EXCHANGE_ADSMOGO",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADSWIZZ => "EXCHANGE_ADSWIZZ",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEBIDSWITCH => "EXCHANGE_BIDSWITCH",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEBRIGHTROLLDISPLAY => "EXCHANGE_BRIGHTROLL_DISPLAY",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGECADREON => "EXCHANGE_CADREON",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEDAILYMOTION => "EXCHANGE_DAILYMOTION",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEFIVE => "EXCHANGE_FIVE",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEFLUCT => "EXCHANGE_FLUCT",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEFREEWHEEL => "EXCHANGE_FREEWHEEL",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEGENIEE => "EXCHANGE_GENIEE",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEGUMGUM => "EXCHANGE_GUMGUM",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEIMOBILE => "EXCHANGE_IMOBILE",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEIBILLBOARD => "EXCHANGE_IBILLBOARD",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEIMPROVEDIGITAL => "EXCHANGE_IMPROVE_DIGITAL",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEINDEX => "EXCHANGE_INDEX",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEKARGO => "EXCHANGE_KARGO",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEMICROAD => "EXCHANGE_MICROAD",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEMOPUB => "EXCHANGE_MOPUB",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGENEND => "EXCHANGE_NEND",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEONEBYAOLDISPLAY => "EXCHANGE_ONE_BY_AOL_DISPLAY",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEONEBYAOLMOBILE => "EXCHANGE_ONE_BY_AOL_MOBILE",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEONEBYAOLVIDEO => "EXCHANGE_ONE_BY_AOL_VIDEO",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEOOYALA => "EXCHANGE_OOYALA",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEOPENX => "EXCHANGE_OPENX",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPERMODO => "EXCHANGE_PERMODO",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPLATFORMONE => "EXCHANGE_PLATFORMONE",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPLATFORMID => "EXCHANGE_PLATFORMID",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPUBMATIC => "EXCHANGE_PUBMATIC",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPULSEPOINT => "EXCHANGE_PULSEPOINT",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEREVENUEMAX => "EXCHANGE_REVENUEMAX",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGERUBICON => "EXCHANGE_RUBICON",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESMARTCLIP => "EXCHANGE_SMARTCLIP",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESMARTRTB => "EXCHANGE_SMARTRTB",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESMARTSTREAMTV => "EXCHANGE_SMARTSTREAMTV",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESOVRN => "EXCHANGE_SOVRN",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESPOTXCHANGE => "EXCHANGE_SPOTXCHANGE",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESTROER => "EXCHANGE_STROER",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETEADSTV => "EXCHANGE_TEADSTV",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETELARIA => "EXCHANGE_TELARIA",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETVN => "EXCHANGE_TVN",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEUNITED => "EXCHANGE_UNITED",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEYIELDLAB => "EXCHANGE_YIELDLAB",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEYIELDMO => "EXCHANGE_YIELDMO",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEUNRULYX => "EXCHANGE_UNRULYX",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEOPEN8 => "EXCHANGE_OPEN8",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETRITON => "EXCHANGE_TRITON",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETRIPLELIFT => "EXCHANGE_TRIPLELIFT",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETABOOLA => "EXCHANGE_TABOOLA",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEINMOBI => "EXCHANGE_INMOBI",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESMAATO => "EXCHANGE_SMAATO",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEAJA => "EXCHANGE_AJA",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESUPERSHIP => "EXCHANGE_SUPERSHIP",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGENEXSTARDIGITAL => "EXCHANGE_NEXSTAR_DIGITAL",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEWAZE => "EXCHANGE_WAZE",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESOUNDCAST => "EXCHANGE_SOUNDCAST",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESHARETHROUGH => "EXCHANGE_SHARETHROUGH",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEFYBER => "EXCHANGE_FYBER",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEREDFORPUBLISHERS => "EXCHANGE_RED_FOR_PUBLISHERS",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEMEDIANET => "EXCHANGE_MEDIANET",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETAPJOY => "EXCHANGE_TAPJOY",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEVISTAR => "EXCHANGE_VISTAR",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEDAX => "EXCHANGE_DAX",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEJCD => "EXCHANGE_JCD",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPLACEEXCHANGE => "EXCHANGE_PLACE_EXCHANGE",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEAPPLOVIN => "EXCHANGE_APPLOVIN",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGECONNATIX => "EXCHANGE_CONNATIX",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGERESETDIGITAL => "EXCHANGE_RESET_DIGITAL",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEHIVESTACK => "EXCHANGE_HIVESTACK",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEDRAX => "EXCHANGE_DRAX",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEAPPLOVINGBID => "EXCHANGE_APPLOVIN_GBID",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEFYBERGBID => "EXCHANGE_FYBER_GBID",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEUNITYGBID => "EXCHANGE_UNITY_GBID",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGECHARTBOOSTGBID => "EXCHANGE_CHARTBOOST_GBID",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADMOSTGBID => "EXCHANGE_ADMOST_GBID",
            ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETOPONGBID => "EXCHANGE_TOPON_GBID",
        }
    }
}

impl std::convert::TryFrom< &str> for ExchangeTargetingOptionDetailExchangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCHANGE_UNSPECIFIED" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEUNSPECIFIED),
           "EXCHANGE_GOOGLE_AD_MANAGER" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEGOOGLEADMANAGER),
           "EXCHANGE_APPNEXUS" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEAPPNEXUS),
           "EXCHANGE_BRIGHTROLL" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEBRIGHTROLL),
           "EXCHANGE_ADFORM" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADFORM),
           "EXCHANGE_ADMETA" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADMETA),
           "EXCHANGE_ADMIXER" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADMIXER),
           "EXCHANGE_ADSMOGO" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADSMOGO),
           "EXCHANGE_ADSWIZZ" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADSWIZZ),
           "EXCHANGE_BIDSWITCH" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEBIDSWITCH),
           "EXCHANGE_BRIGHTROLL_DISPLAY" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEBRIGHTROLLDISPLAY),
           "EXCHANGE_CADREON" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGECADREON),
           "EXCHANGE_DAILYMOTION" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEDAILYMOTION),
           "EXCHANGE_FIVE" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEFIVE),
           "EXCHANGE_FLUCT" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEFLUCT),
           "EXCHANGE_FREEWHEEL" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEFREEWHEEL),
           "EXCHANGE_GENIEE" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEGENIEE),
           "EXCHANGE_GUMGUM" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEGUMGUM),
           "EXCHANGE_IMOBILE" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEIMOBILE),
           "EXCHANGE_IBILLBOARD" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEIBILLBOARD),
           "EXCHANGE_IMPROVE_DIGITAL" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEIMPROVEDIGITAL),
           "EXCHANGE_INDEX" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEINDEX),
           "EXCHANGE_KARGO" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEKARGO),
           "EXCHANGE_MICROAD" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEMICROAD),
           "EXCHANGE_MOPUB" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEMOPUB),
           "EXCHANGE_NEND" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGENEND),
           "EXCHANGE_ONE_BY_AOL_DISPLAY" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEONEBYAOLDISPLAY),
           "EXCHANGE_ONE_BY_AOL_MOBILE" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEONEBYAOLMOBILE),
           "EXCHANGE_ONE_BY_AOL_VIDEO" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEONEBYAOLVIDEO),
           "EXCHANGE_OOYALA" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEOOYALA),
           "EXCHANGE_OPENX" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEOPENX),
           "EXCHANGE_PERMODO" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPERMODO),
           "EXCHANGE_PLATFORMONE" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPLATFORMONE),
           "EXCHANGE_PLATFORMID" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPLATFORMID),
           "EXCHANGE_PUBMATIC" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPUBMATIC),
           "EXCHANGE_PULSEPOINT" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPULSEPOINT),
           "EXCHANGE_REVENUEMAX" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEREVENUEMAX),
           "EXCHANGE_RUBICON" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGERUBICON),
           "EXCHANGE_SMARTCLIP" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESMARTCLIP),
           "EXCHANGE_SMARTRTB" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESMARTRTB),
           "EXCHANGE_SMARTSTREAMTV" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESMARTSTREAMTV),
           "EXCHANGE_SOVRN" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESOVRN),
           "EXCHANGE_SPOTXCHANGE" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESPOTXCHANGE),
           "EXCHANGE_STROER" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESTROER),
           "EXCHANGE_TEADSTV" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETEADSTV),
           "EXCHANGE_TELARIA" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETELARIA),
           "EXCHANGE_TVN" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETVN),
           "EXCHANGE_UNITED" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEUNITED),
           "EXCHANGE_YIELDLAB" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEYIELDLAB),
           "EXCHANGE_YIELDMO" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEYIELDMO),
           "EXCHANGE_UNRULYX" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEUNRULYX),
           "EXCHANGE_OPEN8" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEOPEN8),
           "EXCHANGE_TRITON" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETRITON),
           "EXCHANGE_TRIPLELIFT" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETRIPLELIFT),
           "EXCHANGE_TABOOLA" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETABOOLA),
           "EXCHANGE_INMOBI" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEINMOBI),
           "EXCHANGE_SMAATO" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESMAATO),
           "EXCHANGE_AJA" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEAJA),
           "EXCHANGE_SUPERSHIP" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESUPERSHIP),
           "EXCHANGE_NEXSTAR_DIGITAL" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGENEXSTARDIGITAL),
           "EXCHANGE_WAZE" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEWAZE),
           "EXCHANGE_SOUNDCAST" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESOUNDCAST),
           "EXCHANGE_SHARETHROUGH" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGESHARETHROUGH),
           "EXCHANGE_FYBER" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEFYBER),
           "EXCHANGE_RED_FOR_PUBLISHERS" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEREDFORPUBLISHERS),
           "EXCHANGE_MEDIANET" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEMEDIANET),
           "EXCHANGE_TAPJOY" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETAPJOY),
           "EXCHANGE_VISTAR" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEVISTAR),
           "EXCHANGE_DAX" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEDAX),
           "EXCHANGE_JCD" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEJCD),
           "EXCHANGE_PLACE_EXCHANGE" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEPLACEEXCHANGE),
           "EXCHANGE_APPLOVIN" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEAPPLOVIN),
           "EXCHANGE_CONNATIX" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGECONNATIX),
           "EXCHANGE_RESET_DIGITAL" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGERESETDIGITAL),
           "EXCHANGE_HIVESTACK" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEHIVESTACK),
           "EXCHANGE_DRAX" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEDRAX),
           "EXCHANGE_APPLOVIN_GBID" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEAPPLOVINGBID),
           "EXCHANGE_FYBER_GBID" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEFYBERGBID),
           "EXCHANGE_UNITY_GBID" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEUNITYGBID),
           "EXCHANGE_CHARTBOOST_GBID" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGECHARTBOOSTGBID),
           "EXCHANGE_ADMOST_GBID" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGEADMOSTGBID),
           "EXCHANGE_TOPON_GBID" => Ok(ExchangeTargetingOptionDetailExchangeEnum::EXCHANGETOPONGBID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExchangeTargetingOptionDetailExchangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExitEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the exit event.
pub enum ExitEventTypeEnum {
    

    /// Exit event type is not specified or is unknown in this version.
    ///
    /// "EXIT_EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="EXIT_EVENT_TYPE_UNSPECIFIED")]
    EXITEVENTTYPEUNSPECIFIED,
    

    /// The exit event is the default one.
    ///
    /// "EXIT_EVENT_TYPE_DEFAULT"
    #[serde(rename="EXIT_EVENT_TYPE_DEFAULT")]
    EXITEVENTTYPEDEFAULT,
    

    /// The exit event is a backup exit event. There could be multiple backup exit events in a creative.
    ///
    /// "EXIT_EVENT_TYPE_BACKUP"
    #[serde(rename="EXIT_EVENT_TYPE_BACKUP")]
    EXITEVENTTYPEBACKUP,
}

impl AsRef<str> for ExitEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExitEventTypeEnum::EXITEVENTTYPEUNSPECIFIED => "EXIT_EVENT_TYPE_UNSPECIFIED",
            ExitEventTypeEnum::EXITEVENTTYPEDEFAULT => "EXIT_EVENT_TYPE_DEFAULT",
            ExitEventTypeEnum::EXITEVENTTYPEBACKUP => "EXIT_EVENT_TYPE_BACKUP",
        }
    }
}

impl std::convert::TryFrom< &str> for ExitEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXIT_EVENT_TYPE_UNSPECIFIED" => Ok(ExitEventTypeEnum::EXITEVENTTYPEUNSPECIFIED),
           "EXIT_EVENT_TYPE_DEFAULT" => Ok(ExitEventTypeEnum::EXITEVENTTYPEDEFAULT),
           "EXIT_EVENT_TYPE_BACKUP" => Ok(ExitEventTypeEnum::EXITEVENTTYPEBACKUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExitEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirstAndThirdPartyAudienceAudienceSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The source of the audience.
pub enum FirstAndThirdPartyAudienceAudienceSourceEnum {
    

    /// Default value when audience source is not specified or is unknown.
    ///
    /// "AUDIENCE_SOURCE_UNSPECIFIED"
    #[serde(rename="AUDIENCE_SOURCE_UNSPECIFIED")]
    AUDIENCESOURCEUNSPECIFIED,
    

    /// Originated from Display & Video 360.
    ///
    /// "DISPLAY_VIDEO_360"
    #[serde(rename="DISPLAY_VIDEO_360")]
    DISPLAYVIDEO360,
    

    /// Originated from Campaign Manager 360.
    ///
    /// "CAMPAIGN_MANAGER"
    #[serde(rename="CAMPAIGN_MANAGER")]
    CAMPAIGNMANAGER,
    

    /// Originated from Google Ad Manager.
    ///
    /// "AD_MANAGER"
    #[serde(rename="AD_MANAGER")]
    ADMANAGER,
    

    /// Originated from Search Ads 360.
    ///
    /// "SEARCH_ADS_360"
    #[serde(rename="SEARCH_ADS_360")]
    SEARCHADS360,
    

    /// Originated from Youtube.
    ///
    /// "YOUTUBE"
    #[serde(rename="YOUTUBE")]
    YOUTUBE,
    

    /// Originated from Ads Data Hub.
    ///
    /// "ADS_DATA_HUB"
    #[serde(rename="ADS_DATA_HUB")]
    ADSDATAHUB,
}

impl AsRef<str> for FirstAndThirdPartyAudienceAudienceSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirstAndThirdPartyAudienceAudienceSourceEnum::AUDIENCESOURCEUNSPECIFIED => "AUDIENCE_SOURCE_UNSPECIFIED",
            FirstAndThirdPartyAudienceAudienceSourceEnum::DISPLAYVIDEO360 => "DISPLAY_VIDEO_360",
            FirstAndThirdPartyAudienceAudienceSourceEnum::CAMPAIGNMANAGER => "CAMPAIGN_MANAGER",
            FirstAndThirdPartyAudienceAudienceSourceEnum::ADMANAGER => "AD_MANAGER",
            FirstAndThirdPartyAudienceAudienceSourceEnum::SEARCHADS360 => "SEARCH_ADS_360",
            FirstAndThirdPartyAudienceAudienceSourceEnum::YOUTUBE => "YOUTUBE",
            FirstAndThirdPartyAudienceAudienceSourceEnum::ADSDATAHUB => "ADS_DATA_HUB",
        }
    }
}

impl std::convert::TryFrom< &str> for FirstAndThirdPartyAudienceAudienceSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUDIENCE_SOURCE_UNSPECIFIED" => Ok(FirstAndThirdPartyAudienceAudienceSourceEnum::AUDIENCESOURCEUNSPECIFIED),
           "DISPLAY_VIDEO_360" => Ok(FirstAndThirdPartyAudienceAudienceSourceEnum::DISPLAYVIDEO360),
           "CAMPAIGN_MANAGER" => Ok(FirstAndThirdPartyAudienceAudienceSourceEnum::CAMPAIGNMANAGER),
           "AD_MANAGER" => Ok(FirstAndThirdPartyAudienceAudienceSourceEnum::ADMANAGER),
           "SEARCH_ADS_360" => Ok(FirstAndThirdPartyAudienceAudienceSourceEnum::SEARCHADS360),
           "YOUTUBE" => Ok(FirstAndThirdPartyAudienceAudienceSourceEnum::YOUTUBE),
           "ADS_DATA_HUB" => Ok(FirstAndThirdPartyAudienceAudienceSourceEnum::ADSDATAHUB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirstAndThirdPartyAudienceAudienceSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirstAndThirdPartyAudienceAudienceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the audience.
pub enum FirstAndThirdPartyAudienceAudienceTypeEnum {
    

    /// Default value when type is not specified or is unknown.
    ///
    /// "AUDIENCE_TYPE_UNSPECIFIED"
    #[serde(rename="AUDIENCE_TYPE_UNSPECIFIED")]
    AUDIENCETYPEUNSPECIFIED,
    

    /// Audience was generated through matching customers to known contact information.
    ///
    /// "CUSTOMER_MATCH_CONTACT_INFO"
    #[serde(rename="CUSTOMER_MATCH_CONTACT_INFO")]
    CUSTOMERMATCHCONTACTINFO,
    

    /// Audience was generated through matching customers to known Mobile device IDs.
    ///
    /// "CUSTOMER_MATCH_DEVICE_ID"
    #[serde(rename="CUSTOMER_MATCH_DEVICE_ID")]
    CUSTOMERMATCHDEVICEID,
    

    /// Audience was generated through matching customers to known User IDs.
    ///
    /// "CUSTOMER_MATCH_USER_ID"
    #[serde(rename="CUSTOMER_MATCH_USER_ID")]
    CUSTOMERMATCHUSERID,
    

    /// Audience was created based on campaign activity.
    ///
    /// "ACTIVITY_BASED"
    #[serde(rename="ACTIVITY_BASED")]
    ACTIVITYBASED,
    

    /// Audience was created based on excluding the number of impressions they were served.
    ///
    /// "FREQUENCY_CAP"
    #[serde(rename="FREQUENCY_CAP")]
    FREQUENCYCAP,
    

    /// Audience was created based on custom variables attached to pixel.
    ///
    /// "TAG_BASED"
    #[serde(rename="TAG_BASED")]
    TAGBASED,
    

    /// Audience was created based on past interactions with videos, YouTube ads, or YouTube channel.
    ///
    /// "YOUTUBE_USERS"
    #[serde(rename="YOUTUBE_USERS")]
    YOUTUBEUSERS,
    

    /// Subtype of third party audience type.
    ///
    /// "LICENSED"
    #[serde(rename="LICENSED")]
    LICENSED,
}

impl AsRef<str> for FirstAndThirdPartyAudienceAudienceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirstAndThirdPartyAudienceAudienceTypeEnum::AUDIENCETYPEUNSPECIFIED => "AUDIENCE_TYPE_UNSPECIFIED",
            FirstAndThirdPartyAudienceAudienceTypeEnum::CUSTOMERMATCHCONTACTINFO => "CUSTOMER_MATCH_CONTACT_INFO",
            FirstAndThirdPartyAudienceAudienceTypeEnum::CUSTOMERMATCHDEVICEID => "CUSTOMER_MATCH_DEVICE_ID",
            FirstAndThirdPartyAudienceAudienceTypeEnum::CUSTOMERMATCHUSERID => "CUSTOMER_MATCH_USER_ID",
            FirstAndThirdPartyAudienceAudienceTypeEnum::ACTIVITYBASED => "ACTIVITY_BASED",
            FirstAndThirdPartyAudienceAudienceTypeEnum::FREQUENCYCAP => "FREQUENCY_CAP",
            FirstAndThirdPartyAudienceAudienceTypeEnum::TAGBASED => "TAG_BASED",
            FirstAndThirdPartyAudienceAudienceTypeEnum::YOUTUBEUSERS => "YOUTUBE_USERS",
            FirstAndThirdPartyAudienceAudienceTypeEnum::LICENSED => "LICENSED",
        }
    }
}

impl std::convert::TryFrom< &str> for FirstAndThirdPartyAudienceAudienceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUDIENCE_TYPE_UNSPECIFIED" => Ok(FirstAndThirdPartyAudienceAudienceTypeEnum::AUDIENCETYPEUNSPECIFIED),
           "CUSTOMER_MATCH_CONTACT_INFO" => Ok(FirstAndThirdPartyAudienceAudienceTypeEnum::CUSTOMERMATCHCONTACTINFO),
           "CUSTOMER_MATCH_DEVICE_ID" => Ok(FirstAndThirdPartyAudienceAudienceTypeEnum::CUSTOMERMATCHDEVICEID),
           "CUSTOMER_MATCH_USER_ID" => Ok(FirstAndThirdPartyAudienceAudienceTypeEnum::CUSTOMERMATCHUSERID),
           "ACTIVITY_BASED" => Ok(FirstAndThirdPartyAudienceAudienceTypeEnum::ACTIVITYBASED),
           "FREQUENCY_CAP" => Ok(FirstAndThirdPartyAudienceAudienceTypeEnum::FREQUENCYCAP),
           "TAG_BASED" => Ok(FirstAndThirdPartyAudienceAudienceTypeEnum::TAGBASED),
           "YOUTUBE_USERS" => Ok(FirstAndThirdPartyAudienceAudienceTypeEnum::YOUTUBEUSERS),
           "LICENSED" => Ok(FirstAndThirdPartyAudienceAudienceTypeEnum::LICENSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirstAndThirdPartyAudienceAudienceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the audience is a first or third party audience.
pub enum FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum {
    

    /// Default value when type is not specified or is unknown.
    ///
    /// "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_UNSPECIFIED"
    #[serde(rename="FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_UNSPECIFIED")]
    FIRSTANDTHIRDPARTYAUDIENCETYPEUNSPECIFIED,
    

    /// Audience that is created via usage of client data.
    ///
    /// "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_FIRST_PARTY"
    #[serde(rename="FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_FIRST_PARTY")]
    FIRSTANDTHIRDPARTYAUDIENCETYPEFIRSTPARTY,
    

    /// Audience that is provided by Third Party data providers.
    ///
    /// "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_THIRD_PARTY"
    #[serde(rename="FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_THIRD_PARTY")]
    FIRSTANDTHIRDPARTYAUDIENCETYPETHIRDPARTY,
}

impl AsRef<str> for FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum::FIRSTANDTHIRDPARTYAUDIENCETYPEUNSPECIFIED => "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_UNSPECIFIED",
            FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum::FIRSTANDTHIRDPARTYAUDIENCETYPEFIRSTPARTY => "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_FIRST_PARTY",
            FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum::FIRSTANDTHIRDPARTYAUDIENCETYPETHIRDPARTY => "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_THIRD_PARTY",
        }
    }
}

impl std::convert::TryFrom< &str> for FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_UNSPECIFIED" => Ok(FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum::FIRSTANDTHIRDPARTYAUDIENCETYPEUNSPECIFIED),
           "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_FIRST_PARTY" => Ok(FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum::FIRSTANDTHIRDPARTYAUDIENCETYPEFIRSTPARTY),
           "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_THIRD_PARTY" => Ok(FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum::FIRSTANDTHIRDPARTYAUDIENCETYPETHIRDPARTY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirstAndThirdPartyAudienceTargetingSettingRecencyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The recency of the first and third party audience targeting setting. Only applicable to first party audiences, otherwise will be ignored. For more info, refer to https://support.google.com/displayvideo/answer/2949947#recency When unspecified, no recency limit will be used.
pub enum FirstAndThirdPartyAudienceTargetingSettingRecencyEnum {
    

    /// No limit of recency.
    ///
    /// "RECENCY_NO_LIMIT"
    #[serde(rename="RECENCY_NO_LIMIT")]
    RECENCYNOLIMIT,
    

    /// Recency is 1 minute.
    ///
    /// "RECENCY_1_MINUTE"
    #[serde(rename="RECENCY_1_MINUTE")]
    RECENCY1MINUTE,
    

    /// Recency is 5 minutes.
    ///
    /// "RECENCY_5_MINUTES"
    #[serde(rename="RECENCY_5_MINUTES")]
    RECENCY5MINUTES,
    

    /// Recency is 10 minutes.
    ///
    /// "RECENCY_10_MINUTES"
    #[serde(rename="RECENCY_10_MINUTES")]
    RECENCY10MINUTES,
    

    /// Recency is 15 minutes.
    ///
    /// "RECENCY_15_MINUTES"
    #[serde(rename="RECENCY_15_MINUTES")]
    RECENCY15MINUTES,
    

    /// Recency is 30 minutes.
    ///
    /// "RECENCY_30_MINUTES"
    #[serde(rename="RECENCY_30_MINUTES")]
    RECENCY30MINUTES,
    

    /// Recency is 1 hour.
    ///
    /// "RECENCY_1_HOUR"
    #[serde(rename="RECENCY_1_HOUR")]
    RECENCY1HOUR,
    

    /// Recency is 2 hours.
    ///
    /// "RECENCY_2_HOURS"
    #[serde(rename="RECENCY_2_HOURS")]
    RECENCY2HOURS,
    

    /// Recency is 3 hours.
    ///
    /// "RECENCY_3_HOURS"
    #[serde(rename="RECENCY_3_HOURS")]
    RECENCY3HOURS,
    

    /// Recency is 6 hours.
    ///
    /// "RECENCY_6_HOURS"
    #[serde(rename="RECENCY_6_HOURS")]
    RECENCY6HOURS,
    

    /// Recency is 12 hours.
    ///
    /// "RECENCY_12_HOURS"
    #[serde(rename="RECENCY_12_HOURS")]
    RECENCY12HOURS,
    

    /// Recency is 1 day.
    ///
    /// "RECENCY_1_DAY"
    #[serde(rename="RECENCY_1_DAY")]
    RECENCY1DAY,
    

    /// Recency is 2 days.
    ///
    /// "RECENCY_2_DAYS"
    #[serde(rename="RECENCY_2_DAYS")]
    RECENCY2DAYS,
    

    /// Recency is 3 days.
    ///
    /// "RECENCY_3_DAYS"
    #[serde(rename="RECENCY_3_DAYS")]
    RECENCY3DAYS,
    

    /// Recency is 5 days.
    ///
    /// "RECENCY_5_DAYS"
    #[serde(rename="RECENCY_5_DAYS")]
    RECENCY5DAYS,
    

    /// Recency is 7 days.
    ///
    /// "RECENCY_7_DAYS"
    #[serde(rename="RECENCY_7_DAYS")]
    RECENCY7DAYS,
    

    /// Recency is 10 days.
    ///
    /// "RECENCY_10_DAYS"
    #[serde(rename="RECENCY_10_DAYS")]
    RECENCY10DAYS,
    

    /// Recency is 14 days.
    ///
    /// "RECENCY_14_DAYS"
    #[serde(rename="RECENCY_14_DAYS")]
    RECENCY14DAYS,
    

    /// Recency is 15 days.
    ///
    /// "RECENCY_15_DAYS"
    #[serde(rename="RECENCY_15_DAYS")]
    RECENCY15DAYS,
    

    /// Recency is 21 days.
    ///
    /// "RECENCY_21_DAYS"
    #[serde(rename="RECENCY_21_DAYS")]
    RECENCY21DAYS,
    

    /// Recency is 28 days.
    ///
    /// "RECENCY_28_DAYS"
    #[serde(rename="RECENCY_28_DAYS")]
    RECENCY28DAYS,
    

    /// Recency is 30 days.
    ///
    /// "RECENCY_30_DAYS"
    #[serde(rename="RECENCY_30_DAYS")]
    RECENCY30DAYS,
    

    /// Recency is 40 days.
    ///
    /// "RECENCY_40_DAYS"
    #[serde(rename="RECENCY_40_DAYS")]
    RECENCY40DAYS,
    

    /// Recency is 45 days.
    ///
    /// "RECENCY_45_DAYS"
    #[serde(rename="RECENCY_45_DAYS")]
    RECENCY45DAYS,
    

    /// Recency is 60 days.
    ///
    /// "RECENCY_60_DAYS"
    #[serde(rename="RECENCY_60_DAYS")]
    RECENCY60DAYS,
    

    /// Recency is 90 days.
    ///
    /// "RECENCY_90_DAYS"
    #[serde(rename="RECENCY_90_DAYS")]
    RECENCY90DAYS,
    

    /// Recency is 120 days.
    ///
    /// "RECENCY_120_DAYS"
    #[serde(rename="RECENCY_120_DAYS")]
    RECENCY120DAYS,
    

    /// Recency is 180 days.
    ///
    /// "RECENCY_180_DAYS"
    #[serde(rename="RECENCY_180_DAYS")]
    RECENCY180DAYS,
    

    /// Recency is 270 days.
    ///
    /// "RECENCY_270_DAYS"
    #[serde(rename="RECENCY_270_DAYS")]
    RECENCY270DAYS,
    

    /// Recency is 365 days.
    ///
    /// "RECENCY_365_DAYS"
    #[serde(rename="RECENCY_365_DAYS")]
    RECENCY365DAYS,
}

impl AsRef<str> for FirstAndThirdPartyAudienceTargetingSettingRecencyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCYNOLIMIT => "RECENCY_NO_LIMIT",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY1MINUTE => "RECENCY_1_MINUTE",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY5MINUTES => "RECENCY_5_MINUTES",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY10MINUTES => "RECENCY_10_MINUTES",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY15MINUTES => "RECENCY_15_MINUTES",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY30MINUTES => "RECENCY_30_MINUTES",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY1HOUR => "RECENCY_1_HOUR",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY2HOURS => "RECENCY_2_HOURS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY3HOURS => "RECENCY_3_HOURS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY6HOURS => "RECENCY_6_HOURS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY12HOURS => "RECENCY_12_HOURS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY1DAY => "RECENCY_1_DAY",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY2DAYS => "RECENCY_2_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY3DAYS => "RECENCY_3_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY5DAYS => "RECENCY_5_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY7DAYS => "RECENCY_7_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY10DAYS => "RECENCY_10_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY14DAYS => "RECENCY_14_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY15DAYS => "RECENCY_15_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY21DAYS => "RECENCY_21_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY28DAYS => "RECENCY_28_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY30DAYS => "RECENCY_30_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY40DAYS => "RECENCY_40_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY45DAYS => "RECENCY_45_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY60DAYS => "RECENCY_60_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY90DAYS => "RECENCY_90_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY120DAYS => "RECENCY_120_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY180DAYS => "RECENCY_180_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY270DAYS => "RECENCY_270_DAYS",
            FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY365DAYS => "RECENCY_365_DAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for FirstAndThirdPartyAudienceTargetingSettingRecencyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECENCY_NO_LIMIT" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCYNOLIMIT),
           "RECENCY_1_MINUTE" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY1MINUTE),
           "RECENCY_5_MINUTES" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY5MINUTES),
           "RECENCY_10_MINUTES" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY10MINUTES),
           "RECENCY_15_MINUTES" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY15MINUTES),
           "RECENCY_30_MINUTES" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY30MINUTES),
           "RECENCY_1_HOUR" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY1HOUR),
           "RECENCY_2_HOURS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY2HOURS),
           "RECENCY_3_HOURS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY3HOURS),
           "RECENCY_6_HOURS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY6HOURS),
           "RECENCY_12_HOURS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY12HOURS),
           "RECENCY_1_DAY" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY1DAY),
           "RECENCY_2_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY2DAYS),
           "RECENCY_3_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY3DAYS),
           "RECENCY_5_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY5DAYS),
           "RECENCY_7_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY7DAYS),
           "RECENCY_10_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY10DAYS),
           "RECENCY_14_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY14DAYS),
           "RECENCY_15_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY15DAYS),
           "RECENCY_21_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY21DAYS),
           "RECENCY_28_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY28DAYS),
           "RECENCY_30_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY30DAYS),
           "RECENCY_40_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY40DAYS),
           "RECENCY_45_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY45DAYS),
           "RECENCY_60_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY60DAYS),
           "RECENCY_90_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY90DAYS),
           "RECENCY_120_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY120DAYS),
           "RECENCY_180_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY180DAYS),
           "RECENCY_270_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY270DAYS),
           "RECENCY_365_DAYS" => Ok(FirstAndThirdPartyAudienceTargetingSettingRecencyEnum::RECENCY365DAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirstAndThirdPartyAudienceTargetingSettingRecencyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightGroupWebTagTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The web tag type enabled for the Floodlight group.
pub enum FloodlightGroupWebTagTypeEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "WEB_TAG_TYPE_UNSPECIFIED"
    #[serde(rename="WEB_TAG_TYPE_UNSPECIFIED")]
    WEBTAGTYPEUNSPECIFIED,
    

    /// No tag type.
    ///
    /// "WEB_TAG_TYPE_NONE"
    #[serde(rename="WEB_TAG_TYPE_NONE")]
    WEBTAGTYPENONE,
    

    /// Image tag.
    ///
    /// "WEB_TAG_TYPE_IMAGE"
    #[serde(rename="WEB_TAG_TYPE_IMAGE")]
    WEBTAGTYPEIMAGE,
    

    /// Dynamic tag.
    ///
    /// "WEB_TAG_TYPE_DYNAMIC"
    #[serde(rename="WEB_TAG_TYPE_DYNAMIC")]
    WEBTAGTYPEDYNAMIC,
}

impl AsRef<str> for FloodlightGroupWebTagTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightGroupWebTagTypeEnum::WEBTAGTYPEUNSPECIFIED => "WEB_TAG_TYPE_UNSPECIFIED",
            FloodlightGroupWebTagTypeEnum::WEBTAGTYPENONE => "WEB_TAG_TYPE_NONE",
            FloodlightGroupWebTagTypeEnum::WEBTAGTYPEIMAGE => "WEB_TAG_TYPE_IMAGE",
            FloodlightGroupWebTagTypeEnum::WEBTAGTYPEDYNAMIC => "WEB_TAG_TYPE_DYNAMIC",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightGroupWebTagTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEB_TAG_TYPE_UNSPECIFIED" => Ok(FloodlightGroupWebTagTypeEnum::WEBTAGTYPEUNSPECIFIED),
           "WEB_TAG_TYPE_NONE" => Ok(FloodlightGroupWebTagTypeEnum::WEBTAGTYPENONE),
           "WEB_TAG_TYPE_IMAGE" => Ok(FloodlightGroupWebTagTypeEnum::WEBTAGTYPEIMAGE),
           "WEB_TAG_TYPE_DYNAMIC" => Ok(FloodlightGroupWebTagTypeEnum::WEBTAGTYPEDYNAMIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightGroupWebTagTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FrequencyCapTimeUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The time unit in which the frequency cap will be applied. Required when unlimited is `false`.
pub enum FrequencyCapTimeUnitEnum {
    

    /// Time unit value is not specified or is unknown in this version.
    ///
    /// "TIME_UNIT_UNSPECIFIED"
    #[serde(rename="TIME_UNIT_UNSPECIFIED")]
    TIMEUNITUNSPECIFIED,
    

    /// The frequency cap will be applied to the whole life time of the line item.
    ///
    /// "TIME_UNIT_LIFETIME"
    #[serde(rename="TIME_UNIT_LIFETIME")]
    TIMEUNITLIFETIME,
    

    /// The frequency cap will be applied to a number of months.
    ///
    /// "TIME_UNIT_MONTHS"
    #[serde(rename="TIME_UNIT_MONTHS")]
    TIMEUNITMONTHS,
    

    /// The frequency cap will be applied to a number of weeks.
    ///
    /// "TIME_UNIT_WEEKS"
    #[serde(rename="TIME_UNIT_WEEKS")]
    TIMEUNITWEEKS,
    

    /// The frequency cap will be applied to a number of days.
    ///
    /// "TIME_UNIT_DAYS"
    #[serde(rename="TIME_UNIT_DAYS")]
    TIMEUNITDAYS,
    

    /// The frequency cap will be applied to a number of hours.
    ///
    /// "TIME_UNIT_HOURS"
    #[serde(rename="TIME_UNIT_HOURS")]
    TIMEUNITHOURS,
    

    /// The frequency cap will be applied to a number of minutes.
    ///
    /// "TIME_UNIT_MINUTES"
    #[serde(rename="TIME_UNIT_MINUTES")]
    TIMEUNITMINUTES,
}

impl AsRef<str> for FrequencyCapTimeUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FrequencyCapTimeUnitEnum::TIMEUNITUNSPECIFIED => "TIME_UNIT_UNSPECIFIED",
            FrequencyCapTimeUnitEnum::TIMEUNITLIFETIME => "TIME_UNIT_LIFETIME",
            FrequencyCapTimeUnitEnum::TIMEUNITMONTHS => "TIME_UNIT_MONTHS",
            FrequencyCapTimeUnitEnum::TIMEUNITWEEKS => "TIME_UNIT_WEEKS",
            FrequencyCapTimeUnitEnum::TIMEUNITDAYS => "TIME_UNIT_DAYS",
            FrequencyCapTimeUnitEnum::TIMEUNITHOURS => "TIME_UNIT_HOURS",
            FrequencyCapTimeUnitEnum::TIMEUNITMINUTES => "TIME_UNIT_MINUTES",
        }
    }
}

impl std::convert::TryFrom< &str> for FrequencyCapTimeUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_UNIT_UNSPECIFIED" => Ok(FrequencyCapTimeUnitEnum::TIMEUNITUNSPECIFIED),
           "TIME_UNIT_LIFETIME" => Ok(FrequencyCapTimeUnitEnum::TIMEUNITLIFETIME),
           "TIME_UNIT_MONTHS" => Ok(FrequencyCapTimeUnitEnum::TIMEUNITMONTHS),
           "TIME_UNIT_WEEKS" => Ok(FrequencyCapTimeUnitEnum::TIMEUNITWEEKS),
           "TIME_UNIT_DAYS" => Ok(FrequencyCapTimeUnitEnum::TIMEUNITDAYS),
           "TIME_UNIT_HOURS" => Ok(FrequencyCapTimeUnitEnum::TIMEUNITHOURS),
           "TIME_UNIT_MINUTES" => Ok(FrequencyCapTimeUnitEnum::TIMEUNITMINUTES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FrequencyCapTimeUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GenderAssignedTargetingOptionDetailGenderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The gender of the audience.
pub enum GenderAssignedTargetingOptionDetailGenderEnum {
    

    /// Default value when gender is not specified in this version. This enum is a place holder for default value and does not represent a real gender option.
    ///
    /// "GENDER_UNSPECIFIED"
    #[serde(rename="GENDER_UNSPECIFIED")]
    GENDERUNSPECIFIED,
    

    /// The audience gender is male.
    ///
    /// "GENDER_MALE"
    #[serde(rename="GENDER_MALE")]
    GENDERMALE,
    

    /// The audience gender is female.
    ///
    /// "GENDER_FEMALE"
    #[serde(rename="GENDER_FEMALE")]
    GENDERFEMALE,
    

    /// The audience gender is unknown.
    ///
    /// "GENDER_UNKNOWN"
    #[serde(rename="GENDER_UNKNOWN")]
    GENDERUNKNOWN,
}

impl AsRef<str> for GenderAssignedTargetingOptionDetailGenderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GenderAssignedTargetingOptionDetailGenderEnum::GENDERUNSPECIFIED => "GENDER_UNSPECIFIED",
            GenderAssignedTargetingOptionDetailGenderEnum::GENDERMALE => "GENDER_MALE",
            GenderAssignedTargetingOptionDetailGenderEnum::GENDERFEMALE => "GENDER_FEMALE",
            GenderAssignedTargetingOptionDetailGenderEnum::GENDERUNKNOWN => "GENDER_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for GenderAssignedTargetingOptionDetailGenderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GENDER_UNSPECIFIED" => Ok(GenderAssignedTargetingOptionDetailGenderEnum::GENDERUNSPECIFIED),
           "GENDER_MALE" => Ok(GenderAssignedTargetingOptionDetailGenderEnum::GENDERMALE),
           "GENDER_FEMALE" => Ok(GenderAssignedTargetingOptionDetailGenderEnum::GENDERFEMALE),
           "GENDER_UNKNOWN" => Ok(GenderAssignedTargetingOptionDetailGenderEnum::GENDERUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GenderAssignedTargetingOptionDetailGenderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GenderTargetingOptionDetailGenderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The gender of an audience.
pub enum GenderTargetingOptionDetailGenderEnum {
    

    /// Default value when gender is not specified in this version. This enum is a place holder for default value and does not represent a real gender option.
    ///
    /// "GENDER_UNSPECIFIED"
    #[serde(rename="GENDER_UNSPECIFIED")]
    GENDERUNSPECIFIED,
    

    /// The audience gender is male.
    ///
    /// "GENDER_MALE"
    #[serde(rename="GENDER_MALE")]
    GENDERMALE,
    

    /// The audience gender is female.
    ///
    /// "GENDER_FEMALE"
    #[serde(rename="GENDER_FEMALE")]
    GENDERFEMALE,
    

    /// The audience gender is unknown.
    ///
    /// "GENDER_UNKNOWN"
    #[serde(rename="GENDER_UNKNOWN")]
    GENDERUNKNOWN,
}

impl AsRef<str> for GenderTargetingOptionDetailGenderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GenderTargetingOptionDetailGenderEnum::GENDERUNSPECIFIED => "GENDER_UNSPECIFIED",
            GenderTargetingOptionDetailGenderEnum::GENDERMALE => "GENDER_MALE",
            GenderTargetingOptionDetailGenderEnum::GENDERFEMALE => "GENDER_FEMALE",
            GenderTargetingOptionDetailGenderEnum::GENDERUNKNOWN => "GENDER_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for GenderTargetingOptionDetailGenderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GENDER_UNSPECIFIED" => Ok(GenderTargetingOptionDetailGenderEnum::GENDERUNSPECIFIED),
           "GENDER_MALE" => Ok(GenderTargetingOptionDetailGenderEnum::GENDERMALE),
           "GENDER_FEMALE" => Ok(GenderTargetingOptionDetailGenderEnum::GENDERFEMALE),
           "GENDER_UNKNOWN" => Ok(GenderTargetingOptionDetailGenderEnum::GENDERUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GenderTargetingOptionDetailGenderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GenerateDefaultLineItemRequestLineItemTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the line item.
pub enum GenerateDefaultLineItemRequestLineItemTypeEnum {
    

    /// Type value is not specified or is unknown in this version. Line items of this type and their targeting cannot be created or updated using the API.
    ///
    /// "LINE_ITEM_TYPE_UNSPECIFIED"
    #[serde(rename="LINE_ITEM_TYPE_UNSPECIFIED")]
    LINEITEMTYPEUNSPECIFIED,
    

    /// Image, HTML5, native, or rich media ads.
    ///
    /// "LINE_ITEM_TYPE_DISPLAY_DEFAULT"
    #[serde(rename="LINE_ITEM_TYPE_DISPLAY_DEFAULT")]
    LINEITEMTYPEDISPLAYDEFAULT,
    

    /// Display ads that drive installs of an app.
    ///
    /// "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL"
    #[serde(rename="LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL")]
    LINEITEMTYPEDISPLAYMOBILEAPPINSTALL,
    

    /// Video ads sold on a CPM basis for a variety of environments.
    ///
    /// "LINE_ITEM_TYPE_VIDEO_DEFAULT"
    #[serde(rename="LINE_ITEM_TYPE_VIDEO_DEFAULT")]
    LINEITEMTYPEVIDEODEFAULT,
    

    /// Video ads that drive installs of an app.
    ///
    /// "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL"
    #[serde(rename="LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL")]
    LINEITEMTYPEVIDEOMOBILEAPPINSTALL,
    

    /// Display ads served on mobile app inventory. Line items of this type and their targeting cannot be created or updated using the API.
    ///
    /// "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INVENTORY"
    #[serde(rename="LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INVENTORY")]
    LINEITEMTYPEDISPLAYMOBILEAPPINVENTORY,
    

    /// Video ads served on mobile app inventory. Line items of this type and their targeting cannot be created or updated using the API.
    ///
    /// "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INVENTORY"
    #[serde(rename="LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INVENTORY")]
    LINEITEMTYPEVIDEOMOBILEAPPINVENTORY,
    

    /// RTB Audio ads sold for a variety of environments.
    ///
    /// "LINE_ITEM_TYPE_AUDIO_DEFAULT"
    #[serde(rename="LINE_ITEM_TYPE_AUDIO_DEFAULT")]
    LINEITEMTYPEAUDIODEFAULT,
    

    /// Over-the-top ads present in OTT insertion orders. This type is only applicable to line items with an insertion order of insertion_order_type `OVER_THE_TOP`.
    ///
    /// "LINE_ITEM_TYPE_VIDEO_OVER_THE_TOP"
    #[serde(rename="LINE_ITEM_TYPE_VIDEO_OVER_THE_TOP")]
    LINEITEMTYPEVIDEOOVERTHETOP,
    

    /// Display ads served on digital-out-of-home inventory. Line items of this type and their targeting cannot be created or updated using the API.
    ///
    /// "LINE_ITEM_TYPE_DISPLAY_OUT_OF_HOME"
    #[serde(rename="LINE_ITEM_TYPE_DISPLAY_OUT_OF_HOME")]
    LINEITEMTYPEDISPLAYOUTOFHOME,
    

    /// Video ads served on digital-out-of-home inventory. Line items of this type and their targeting cannot be created or updated using the API.
    ///
    /// "LINE_ITEM_TYPE_VIDEO_OUT_OF_HOME"
    #[serde(rename="LINE_ITEM_TYPE_VIDEO_OUT_OF_HOME")]
    LINEITEMTYPEVIDEOOUTOFHOME,
}

impl AsRef<str> for GenerateDefaultLineItemRequestLineItemTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEUNSPECIFIED => "LINE_ITEM_TYPE_UNSPECIFIED",
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEDISPLAYDEFAULT => "LINE_ITEM_TYPE_DISPLAY_DEFAULT",
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEDISPLAYMOBILEAPPINSTALL => "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL",
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEVIDEODEFAULT => "LINE_ITEM_TYPE_VIDEO_DEFAULT",
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEVIDEOMOBILEAPPINSTALL => "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL",
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEDISPLAYMOBILEAPPINVENTORY => "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INVENTORY",
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEVIDEOMOBILEAPPINVENTORY => "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INVENTORY",
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEAUDIODEFAULT => "LINE_ITEM_TYPE_AUDIO_DEFAULT",
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEVIDEOOVERTHETOP => "LINE_ITEM_TYPE_VIDEO_OVER_THE_TOP",
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEDISPLAYOUTOFHOME => "LINE_ITEM_TYPE_DISPLAY_OUT_OF_HOME",
            GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEVIDEOOUTOFHOME => "LINE_ITEM_TYPE_VIDEO_OUT_OF_HOME",
        }
    }
}

impl std::convert::TryFrom< &str> for GenerateDefaultLineItemRequestLineItemTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINE_ITEM_TYPE_UNSPECIFIED" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEUNSPECIFIED),
           "LINE_ITEM_TYPE_DISPLAY_DEFAULT" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEDISPLAYDEFAULT),
           "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEDISPLAYMOBILEAPPINSTALL),
           "LINE_ITEM_TYPE_VIDEO_DEFAULT" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEVIDEODEFAULT),
           "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEVIDEOMOBILEAPPINSTALL),
           "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INVENTORY" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEDISPLAYMOBILEAPPINVENTORY),
           "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INVENTORY" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEVIDEOMOBILEAPPINVENTORY),
           "LINE_ITEM_TYPE_AUDIO_DEFAULT" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEAUDIODEFAULT),
           "LINE_ITEM_TYPE_VIDEO_OVER_THE_TOP" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEVIDEOOVERTHETOP),
           "LINE_ITEM_TYPE_DISPLAY_OUT_OF_HOME" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEDISPLAYOUTOFHOME),
           "LINE_ITEM_TYPE_VIDEO_OUT_OF_HOME" => Ok(GenerateDefaultLineItemRequestLineItemTypeEnum::LINEITEMTYPEVIDEOOUTOFHOME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GenerateDefaultLineItemRequestLineItemTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of geographic region targeting.
pub enum GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum {
    

    /// The geographic region type is unknown.
    ///
    /// "GEO_REGION_TYPE_UNKNOWN"
    #[serde(rename="GEO_REGION_TYPE_UNKNOWN")]
    GEOREGIONTYPEUNKNOWN,
    

    /// The geographic region type is other.
    ///
    /// "GEO_REGION_TYPE_OTHER"
    #[serde(rename="GEO_REGION_TYPE_OTHER")]
    GEOREGIONTYPEOTHER,
    

    /// The geographic region is a country.
    ///
    /// "GEO_REGION_TYPE_COUNTRY"
    #[serde(rename="GEO_REGION_TYPE_COUNTRY")]
    GEOREGIONTYPECOUNTRY,
    

    /// The geographic region type is region.
    ///
    /// "GEO_REGION_TYPE_REGION"
    #[serde(rename="GEO_REGION_TYPE_REGION")]
    GEOREGIONTYPEREGION,
    

    /// The geographic region is a territory.
    ///
    /// "GEO_REGION_TYPE_TERRITORY"
    #[serde(rename="GEO_REGION_TYPE_TERRITORY")]
    GEOREGIONTYPETERRITORY,
    

    /// The geographic region is a province.
    ///
    /// "GEO_REGION_TYPE_PROVINCE"
    #[serde(rename="GEO_REGION_TYPE_PROVINCE")]
    GEOREGIONTYPEPROVINCE,
    

    /// The geographic region is a state.
    ///
    /// "GEO_REGION_TYPE_STATE"
    #[serde(rename="GEO_REGION_TYPE_STATE")]
    GEOREGIONTYPESTATE,
    

    /// The geographic region is a prefecture.
    ///
    /// "GEO_REGION_TYPE_PREFECTURE"
    #[serde(rename="GEO_REGION_TYPE_PREFECTURE")]
    GEOREGIONTYPEPREFECTURE,
    

    /// The geographic region is a governorate.
    ///
    /// "GEO_REGION_TYPE_GOVERNORATE"
    #[serde(rename="GEO_REGION_TYPE_GOVERNORATE")]
    GEOREGIONTYPEGOVERNORATE,
    

    /// The geographic region is a canton.
    ///
    /// "GEO_REGION_TYPE_CANTON"
    #[serde(rename="GEO_REGION_TYPE_CANTON")]
    GEOREGIONTYPECANTON,
    

    /// The geographic region is a union territory.
    ///
    /// "GEO_REGION_TYPE_UNION_TERRITORY"
    #[serde(rename="GEO_REGION_TYPE_UNION_TERRITORY")]
    GEOREGIONTYPEUNIONTERRITORY,
    

    /// The geographic region is an autonomous community.
    ///
    /// "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY"
    #[serde(rename="GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY")]
    GEOREGIONTYPEAUTONOMOUSCOMMUNITY,
    

    /// The geographic region is a designated market area (DMA) region.
    ///
    /// "GEO_REGION_TYPE_DMA_REGION"
    #[serde(rename="GEO_REGION_TYPE_DMA_REGION")]
    GEOREGIONTYPEDMAREGION,
    

    /// The geographic region type is metro.
    ///
    /// "GEO_REGION_TYPE_METRO"
    #[serde(rename="GEO_REGION_TYPE_METRO")]
    GEOREGIONTYPEMETRO,
    

    /// The geographic region is a congressional district.
    ///
    /// "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT"
    #[serde(rename="GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT")]
    GEOREGIONTYPECONGRESSIONALDISTRICT,
    

    /// The geographic region is a county.
    ///
    /// "GEO_REGION_TYPE_COUNTY"
    #[serde(rename="GEO_REGION_TYPE_COUNTY")]
    GEOREGIONTYPECOUNTY,
    

    /// The geographic region is a municipality.
    ///
    /// "GEO_REGION_TYPE_MUNICIPALITY"
    #[serde(rename="GEO_REGION_TYPE_MUNICIPALITY")]
    GEOREGIONTYPEMUNICIPALITY,
    

    /// The geographic region is a city.
    ///
    /// "GEO_REGION_TYPE_CITY"
    #[serde(rename="GEO_REGION_TYPE_CITY")]
    GEOREGIONTYPECITY,
    

    /// The geographic region targeting type is postal code.
    ///
    /// "GEO_REGION_TYPE_POSTAL_CODE"
    #[serde(rename="GEO_REGION_TYPE_POSTAL_CODE")]
    GEOREGIONTYPEPOSTALCODE,
    

    /// The geographic region targeting type is department.
    ///
    /// "GEO_REGION_TYPE_DEPARTMENT"
    #[serde(rename="GEO_REGION_TYPE_DEPARTMENT")]
    GEOREGIONTYPEDEPARTMENT,
    

    /// The geographic region is an airport.
    ///
    /// "GEO_REGION_TYPE_AIRPORT"
    #[serde(rename="GEO_REGION_TYPE_AIRPORT")]
    GEOREGIONTYPEAIRPORT,
    

    /// The geographic region is a TV region.
    ///
    /// "GEO_REGION_TYPE_TV_REGION"
    #[serde(rename="GEO_REGION_TYPE_TV_REGION")]
    GEOREGIONTYPETVREGION,
    

    /// The geographic region is an okrug.
    ///
    /// "GEO_REGION_TYPE_OKRUG"
    #[serde(rename="GEO_REGION_TYPE_OKRUG")]
    GEOREGIONTYPEOKRUG,
    

    /// The geographic region is a borough.
    ///
    /// "GEO_REGION_TYPE_BOROUGH"
    #[serde(rename="GEO_REGION_TYPE_BOROUGH")]
    GEOREGIONTYPEBOROUGH,
    

    /// The geographic region is a city region.
    ///
    /// "GEO_REGION_TYPE_CITY_REGION"
    #[serde(rename="GEO_REGION_TYPE_CITY_REGION")]
    GEOREGIONTYPECITYREGION,
    

    /// The geographic region is an arrondissement.
    ///
    /// "GEO_REGION_TYPE_ARRONDISSEMENT"
    #[serde(rename="GEO_REGION_TYPE_ARRONDISSEMENT")]
    GEOREGIONTYPEARRONDISSEMENT,
    

    /// The geographic region is a neighborhood.
    ///
    /// "GEO_REGION_TYPE_NEIGHBORHOOD"
    #[serde(rename="GEO_REGION_TYPE_NEIGHBORHOOD")]
    GEOREGIONTYPENEIGHBORHOOD,
    

    /// The geographic region is a university.
    ///
    /// "GEO_REGION_TYPE_UNIVERSITY"
    #[serde(rename="GEO_REGION_TYPE_UNIVERSITY")]
    GEOREGIONTYPEUNIVERSITY,
    

    /// The geographic region is a district.
    ///
    /// "GEO_REGION_TYPE_DISTRICT"
    #[serde(rename="GEO_REGION_TYPE_DISTRICT")]
    GEOREGIONTYPEDISTRICT,
}

impl AsRef<str> for GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNKNOWN => "GEO_REGION_TYPE_UNKNOWN",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOTHER => "GEO_REGION_TYPE_OTHER",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTRY => "GEO_REGION_TYPE_COUNTRY",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEREGION => "GEO_REGION_TYPE_REGION",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETERRITORY => "GEO_REGION_TYPE_TERRITORY",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPROVINCE => "GEO_REGION_TYPE_PROVINCE",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPESTATE => "GEO_REGION_TYPE_STATE",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPREFECTURE => "GEO_REGION_TYPE_PREFECTURE",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEGOVERNORATE => "GEO_REGION_TYPE_GOVERNORATE",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECANTON => "GEO_REGION_TYPE_CANTON",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIONTERRITORY => "GEO_REGION_TYPE_UNION_TERRITORY",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAUTONOMOUSCOMMUNITY => "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDMAREGION => "GEO_REGION_TYPE_DMA_REGION",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMETRO => "GEO_REGION_TYPE_METRO",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECONGRESSIONALDISTRICT => "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTY => "GEO_REGION_TYPE_COUNTY",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMUNICIPALITY => "GEO_REGION_TYPE_MUNICIPALITY",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITY => "GEO_REGION_TYPE_CITY",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPOSTALCODE => "GEO_REGION_TYPE_POSTAL_CODE",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDEPARTMENT => "GEO_REGION_TYPE_DEPARTMENT",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAIRPORT => "GEO_REGION_TYPE_AIRPORT",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETVREGION => "GEO_REGION_TYPE_TV_REGION",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOKRUG => "GEO_REGION_TYPE_OKRUG",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEBOROUGH => "GEO_REGION_TYPE_BOROUGH",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITYREGION => "GEO_REGION_TYPE_CITY_REGION",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEARRONDISSEMENT => "GEO_REGION_TYPE_ARRONDISSEMENT",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPENEIGHBORHOOD => "GEO_REGION_TYPE_NEIGHBORHOOD",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIVERSITY => "GEO_REGION_TYPE_UNIVERSITY",
            GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDISTRICT => "GEO_REGION_TYPE_DISTRICT",
        }
    }
}

impl std::convert::TryFrom< &str> for GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GEO_REGION_TYPE_UNKNOWN" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNKNOWN),
           "GEO_REGION_TYPE_OTHER" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOTHER),
           "GEO_REGION_TYPE_COUNTRY" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTRY),
           "GEO_REGION_TYPE_REGION" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEREGION),
           "GEO_REGION_TYPE_TERRITORY" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETERRITORY),
           "GEO_REGION_TYPE_PROVINCE" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPROVINCE),
           "GEO_REGION_TYPE_STATE" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPESTATE),
           "GEO_REGION_TYPE_PREFECTURE" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPREFECTURE),
           "GEO_REGION_TYPE_GOVERNORATE" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEGOVERNORATE),
           "GEO_REGION_TYPE_CANTON" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECANTON),
           "GEO_REGION_TYPE_UNION_TERRITORY" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIONTERRITORY),
           "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAUTONOMOUSCOMMUNITY),
           "GEO_REGION_TYPE_DMA_REGION" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDMAREGION),
           "GEO_REGION_TYPE_METRO" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMETRO),
           "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECONGRESSIONALDISTRICT),
           "GEO_REGION_TYPE_COUNTY" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTY),
           "GEO_REGION_TYPE_MUNICIPALITY" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMUNICIPALITY),
           "GEO_REGION_TYPE_CITY" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITY),
           "GEO_REGION_TYPE_POSTAL_CODE" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPOSTALCODE),
           "GEO_REGION_TYPE_DEPARTMENT" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDEPARTMENT),
           "GEO_REGION_TYPE_AIRPORT" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAIRPORT),
           "GEO_REGION_TYPE_TV_REGION" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETVREGION),
           "GEO_REGION_TYPE_OKRUG" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOKRUG),
           "GEO_REGION_TYPE_BOROUGH" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEBOROUGH),
           "GEO_REGION_TYPE_CITY_REGION" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITYREGION),
           "GEO_REGION_TYPE_ARRONDISSEMENT" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEARRONDISSEMENT),
           "GEO_REGION_TYPE_NEIGHBORHOOD" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPENEIGHBORHOOD),
           "GEO_REGION_TYPE_UNIVERSITY" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIVERSITY),
           "GEO_REGION_TYPE_DISTRICT" => Ok(GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDISTRICT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GeoRegionAssignedTargetingOptionDetailGeoRegionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GeoRegionTargetingOptionDetailGeoRegionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of geographic region targeting.
pub enum GeoRegionTargetingOptionDetailGeoRegionTypeEnum {
    

    /// The geographic region type is unknown.
    ///
    /// "GEO_REGION_TYPE_UNKNOWN"
    #[serde(rename="GEO_REGION_TYPE_UNKNOWN")]
    GEOREGIONTYPEUNKNOWN,
    

    /// The geographic region type is other.
    ///
    /// "GEO_REGION_TYPE_OTHER"
    #[serde(rename="GEO_REGION_TYPE_OTHER")]
    GEOREGIONTYPEOTHER,
    

    /// The geographic region is a country.
    ///
    /// "GEO_REGION_TYPE_COUNTRY"
    #[serde(rename="GEO_REGION_TYPE_COUNTRY")]
    GEOREGIONTYPECOUNTRY,
    

    /// The geographic region type is region.
    ///
    /// "GEO_REGION_TYPE_REGION"
    #[serde(rename="GEO_REGION_TYPE_REGION")]
    GEOREGIONTYPEREGION,
    

    /// The geographic region is a territory.
    ///
    /// "GEO_REGION_TYPE_TERRITORY"
    #[serde(rename="GEO_REGION_TYPE_TERRITORY")]
    GEOREGIONTYPETERRITORY,
    

    /// The geographic region is a province.
    ///
    /// "GEO_REGION_TYPE_PROVINCE"
    #[serde(rename="GEO_REGION_TYPE_PROVINCE")]
    GEOREGIONTYPEPROVINCE,
    

    /// The geographic region is a state.
    ///
    /// "GEO_REGION_TYPE_STATE"
    #[serde(rename="GEO_REGION_TYPE_STATE")]
    GEOREGIONTYPESTATE,
    

    /// The geographic region is a prefecture.
    ///
    /// "GEO_REGION_TYPE_PREFECTURE"
    #[serde(rename="GEO_REGION_TYPE_PREFECTURE")]
    GEOREGIONTYPEPREFECTURE,
    

    /// The geographic region is a governorate.
    ///
    /// "GEO_REGION_TYPE_GOVERNORATE"
    #[serde(rename="GEO_REGION_TYPE_GOVERNORATE")]
    GEOREGIONTYPEGOVERNORATE,
    

    /// The geographic region is a canton.
    ///
    /// "GEO_REGION_TYPE_CANTON"
    #[serde(rename="GEO_REGION_TYPE_CANTON")]
    GEOREGIONTYPECANTON,
    

    /// The geographic region is a union territory.
    ///
    /// "GEO_REGION_TYPE_UNION_TERRITORY"
    #[serde(rename="GEO_REGION_TYPE_UNION_TERRITORY")]
    GEOREGIONTYPEUNIONTERRITORY,
    

    /// The geographic region is an autonomous community.
    ///
    /// "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY"
    #[serde(rename="GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY")]
    GEOREGIONTYPEAUTONOMOUSCOMMUNITY,
    

    /// The geographic region is a designated market area (DMA) region.
    ///
    /// "GEO_REGION_TYPE_DMA_REGION"
    #[serde(rename="GEO_REGION_TYPE_DMA_REGION")]
    GEOREGIONTYPEDMAREGION,
    

    /// The geographic region type is metro.
    ///
    /// "GEO_REGION_TYPE_METRO"
    #[serde(rename="GEO_REGION_TYPE_METRO")]
    GEOREGIONTYPEMETRO,
    

    /// The geographic region is a congressional district.
    ///
    /// "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT"
    #[serde(rename="GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT")]
    GEOREGIONTYPECONGRESSIONALDISTRICT,
    

    /// The geographic region is a county.
    ///
    /// "GEO_REGION_TYPE_COUNTY"
    #[serde(rename="GEO_REGION_TYPE_COUNTY")]
    GEOREGIONTYPECOUNTY,
    

    /// The geographic region is a municipality.
    ///
    /// "GEO_REGION_TYPE_MUNICIPALITY"
    #[serde(rename="GEO_REGION_TYPE_MUNICIPALITY")]
    GEOREGIONTYPEMUNICIPALITY,
    

    /// The geographic region is a city.
    ///
    /// "GEO_REGION_TYPE_CITY"
    #[serde(rename="GEO_REGION_TYPE_CITY")]
    GEOREGIONTYPECITY,
    

    /// The geographic region targeting type is postal code.
    ///
    /// "GEO_REGION_TYPE_POSTAL_CODE"
    #[serde(rename="GEO_REGION_TYPE_POSTAL_CODE")]
    GEOREGIONTYPEPOSTALCODE,
    

    /// The geographic region targeting type is department.
    ///
    /// "GEO_REGION_TYPE_DEPARTMENT"
    #[serde(rename="GEO_REGION_TYPE_DEPARTMENT")]
    GEOREGIONTYPEDEPARTMENT,
    

    /// The geographic region is an airport.
    ///
    /// "GEO_REGION_TYPE_AIRPORT"
    #[serde(rename="GEO_REGION_TYPE_AIRPORT")]
    GEOREGIONTYPEAIRPORT,
    

    /// The geographic region is a TV region.
    ///
    /// "GEO_REGION_TYPE_TV_REGION"
    #[serde(rename="GEO_REGION_TYPE_TV_REGION")]
    GEOREGIONTYPETVREGION,
    

    /// The geographic region is an okrug.
    ///
    /// "GEO_REGION_TYPE_OKRUG"
    #[serde(rename="GEO_REGION_TYPE_OKRUG")]
    GEOREGIONTYPEOKRUG,
    

    /// The geographic region is a borough.
    ///
    /// "GEO_REGION_TYPE_BOROUGH"
    #[serde(rename="GEO_REGION_TYPE_BOROUGH")]
    GEOREGIONTYPEBOROUGH,
    

    /// The geographic region is a city region.
    ///
    /// "GEO_REGION_TYPE_CITY_REGION"
    #[serde(rename="GEO_REGION_TYPE_CITY_REGION")]
    GEOREGIONTYPECITYREGION,
    

    /// The geographic region is an arrondissement.
    ///
    /// "GEO_REGION_TYPE_ARRONDISSEMENT"
    #[serde(rename="GEO_REGION_TYPE_ARRONDISSEMENT")]
    GEOREGIONTYPEARRONDISSEMENT,
    

    /// The geographic region is a neighborhood.
    ///
    /// "GEO_REGION_TYPE_NEIGHBORHOOD"
    #[serde(rename="GEO_REGION_TYPE_NEIGHBORHOOD")]
    GEOREGIONTYPENEIGHBORHOOD,
    

    /// The geographic region is a university.
    ///
    /// "GEO_REGION_TYPE_UNIVERSITY"
    #[serde(rename="GEO_REGION_TYPE_UNIVERSITY")]
    GEOREGIONTYPEUNIVERSITY,
    

    /// The geographic region is a district.
    ///
    /// "GEO_REGION_TYPE_DISTRICT"
    #[serde(rename="GEO_REGION_TYPE_DISTRICT")]
    GEOREGIONTYPEDISTRICT,
}

impl AsRef<str> for GeoRegionTargetingOptionDetailGeoRegionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNKNOWN => "GEO_REGION_TYPE_UNKNOWN",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOTHER => "GEO_REGION_TYPE_OTHER",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTRY => "GEO_REGION_TYPE_COUNTRY",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEREGION => "GEO_REGION_TYPE_REGION",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETERRITORY => "GEO_REGION_TYPE_TERRITORY",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPROVINCE => "GEO_REGION_TYPE_PROVINCE",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPESTATE => "GEO_REGION_TYPE_STATE",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPREFECTURE => "GEO_REGION_TYPE_PREFECTURE",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEGOVERNORATE => "GEO_REGION_TYPE_GOVERNORATE",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECANTON => "GEO_REGION_TYPE_CANTON",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIONTERRITORY => "GEO_REGION_TYPE_UNION_TERRITORY",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAUTONOMOUSCOMMUNITY => "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDMAREGION => "GEO_REGION_TYPE_DMA_REGION",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMETRO => "GEO_REGION_TYPE_METRO",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECONGRESSIONALDISTRICT => "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTY => "GEO_REGION_TYPE_COUNTY",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMUNICIPALITY => "GEO_REGION_TYPE_MUNICIPALITY",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITY => "GEO_REGION_TYPE_CITY",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPOSTALCODE => "GEO_REGION_TYPE_POSTAL_CODE",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDEPARTMENT => "GEO_REGION_TYPE_DEPARTMENT",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAIRPORT => "GEO_REGION_TYPE_AIRPORT",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETVREGION => "GEO_REGION_TYPE_TV_REGION",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOKRUG => "GEO_REGION_TYPE_OKRUG",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEBOROUGH => "GEO_REGION_TYPE_BOROUGH",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITYREGION => "GEO_REGION_TYPE_CITY_REGION",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEARRONDISSEMENT => "GEO_REGION_TYPE_ARRONDISSEMENT",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPENEIGHBORHOOD => "GEO_REGION_TYPE_NEIGHBORHOOD",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIVERSITY => "GEO_REGION_TYPE_UNIVERSITY",
            GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDISTRICT => "GEO_REGION_TYPE_DISTRICT",
        }
    }
}

impl std::convert::TryFrom< &str> for GeoRegionTargetingOptionDetailGeoRegionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GEO_REGION_TYPE_UNKNOWN" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNKNOWN),
           "GEO_REGION_TYPE_OTHER" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOTHER),
           "GEO_REGION_TYPE_COUNTRY" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTRY),
           "GEO_REGION_TYPE_REGION" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEREGION),
           "GEO_REGION_TYPE_TERRITORY" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETERRITORY),
           "GEO_REGION_TYPE_PROVINCE" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPROVINCE),
           "GEO_REGION_TYPE_STATE" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPESTATE),
           "GEO_REGION_TYPE_PREFECTURE" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPREFECTURE),
           "GEO_REGION_TYPE_GOVERNORATE" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEGOVERNORATE),
           "GEO_REGION_TYPE_CANTON" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECANTON),
           "GEO_REGION_TYPE_UNION_TERRITORY" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIONTERRITORY),
           "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAUTONOMOUSCOMMUNITY),
           "GEO_REGION_TYPE_DMA_REGION" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDMAREGION),
           "GEO_REGION_TYPE_METRO" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMETRO),
           "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECONGRESSIONALDISTRICT),
           "GEO_REGION_TYPE_COUNTY" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECOUNTY),
           "GEO_REGION_TYPE_MUNICIPALITY" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEMUNICIPALITY),
           "GEO_REGION_TYPE_CITY" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITY),
           "GEO_REGION_TYPE_POSTAL_CODE" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEPOSTALCODE),
           "GEO_REGION_TYPE_DEPARTMENT" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDEPARTMENT),
           "GEO_REGION_TYPE_AIRPORT" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEAIRPORT),
           "GEO_REGION_TYPE_TV_REGION" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPETVREGION),
           "GEO_REGION_TYPE_OKRUG" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEOKRUG),
           "GEO_REGION_TYPE_BOROUGH" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEBOROUGH),
           "GEO_REGION_TYPE_CITY_REGION" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPECITYREGION),
           "GEO_REGION_TYPE_ARRONDISSEMENT" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEARRONDISSEMENT),
           "GEO_REGION_TYPE_NEIGHBORHOOD" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPENEIGHBORHOOD),
           "GEO_REGION_TYPE_UNIVERSITY" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEUNIVERSITY),
           "GEO_REGION_TYPE_DISTRICT" => Ok(GeoRegionTargetingOptionDetailGeoRegionTypeEnum::GEOREGIONTYPEDISTRICT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GeoRegionTargetingOptionDetailGeoRegionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAudienceGoogleAudienceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of Google audience. .
pub enum GoogleAudienceGoogleAudienceTypeEnum {
    

    /// Default value when type is not specified or is unknown.
    ///
    /// "GOOGLE_AUDIENCE_TYPE_UNSPECIFIED"
    #[serde(rename="GOOGLE_AUDIENCE_TYPE_UNSPECIFIED")]
    GOOGLEAUDIENCETYPEUNSPECIFIED,
    

    /// Affinity type Google audience.
    ///
    /// "GOOGLE_AUDIENCE_TYPE_AFFINITY"
    #[serde(rename="GOOGLE_AUDIENCE_TYPE_AFFINITY")]
    GOOGLEAUDIENCETYPEAFFINITY,
    

    /// In-Market type Google audience.
    ///
    /// "GOOGLE_AUDIENCE_TYPE_IN_MARKET"
    #[serde(rename="GOOGLE_AUDIENCE_TYPE_IN_MARKET")]
    GOOGLEAUDIENCETYPEINMARKET,
    

    /// Installed-Apps type Google audience.
    ///
    /// "GOOGLE_AUDIENCE_TYPE_INSTALLED_APPS"
    #[serde(rename="GOOGLE_AUDIENCE_TYPE_INSTALLED_APPS")]
    GOOGLEAUDIENCETYPEINSTALLEDAPPS,
    

    /// New-Mobile-Devices type Google audience.
    ///
    /// "GOOGLE_AUDIENCE_TYPE_NEW_MOBILE_DEVICES"
    #[serde(rename="GOOGLE_AUDIENCE_TYPE_NEW_MOBILE_DEVICES")]
    GOOGLEAUDIENCETYPENEWMOBILEDEVICES,
    

    /// Life-Event type Google audience.
    ///
    /// "GOOGLE_AUDIENCE_TYPE_LIFE_EVENT"
    #[serde(rename="GOOGLE_AUDIENCE_TYPE_LIFE_EVENT")]
    GOOGLEAUDIENCETYPELIFEEVENT,
    

    /// Extended-Demographic type Google audience.
    ///
    /// "GOOGLE_AUDIENCE_TYPE_EXTENDED_DEMOGRAPHIC"
    #[serde(rename="GOOGLE_AUDIENCE_TYPE_EXTENDED_DEMOGRAPHIC")]
    GOOGLEAUDIENCETYPEEXTENDEDDEMOGRAPHIC,
}

impl AsRef<str> for GoogleAudienceGoogleAudienceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPEUNSPECIFIED => "GOOGLE_AUDIENCE_TYPE_UNSPECIFIED",
            GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPEAFFINITY => "GOOGLE_AUDIENCE_TYPE_AFFINITY",
            GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPEINMARKET => "GOOGLE_AUDIENCE_TYPE_IN_MARKET",
            GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPEINSTALLEDAPPS => "GOOGLE_AUDIENCE_TYPE_INSTALLED_APPS",
            GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPENEWMOBILEDEVICES => "GOOGLE_AUDIENCE_TYPE_NEW_MOBILE_DEVICES",
            GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPELIFEEVENT => "GOOGLE_AUDIENCE_TYPE_LIFE_EVENT",
            GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPEEXTENDEDDEMOGRAPHIC => "GOOGLE_AUDIENCE_TYPE_EXTENDED_DEMOGRAPHIC",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAudienceGoogleAudienceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GOOGLE_AUDIENCE_TYPE_UNSPECIFIED" => Ok(GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPEUNSPECIFIED),
           "GOOGLE_AUDIENCE_TYPE_AFFINITY" => Ok(GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPEAFFINITY),
           "GOOGLE_AUDIENCE_TYPE_IN_MARKET" => Ok(GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPEINMARKET),
           "GOOGLE_AUDIENCE_TYPE_INSTALLED_APPS" => Ok(GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPEINSTALLEDAPPS),
           "GOOGLE_AUDIENCE_TYPE_NEW_MOBILE_DEVICES" => Ok(GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPENEWMOBILEDEVICES),
           "GOOGLE_AUDIENCE_TYPE_LIFE_EVENT" => Ok(GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPELIFEEVENT),
           "GOOGLE_AUDIENCE_TYPE_EXTENDED_DEMOGRAPHIC" => Ok(GoogleAudienceGoogleAudienceTypeEnum::GOOGLEAUDIENCETYPEEXTENDEDDEMOGRAPHIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAudienceGoogleAudienceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuaranteedOrderExchangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The exchange where the guaranteed order originated.
pub enum GuaranteedOrderExchangeEnum {
    

    /// Exchange is not specified or is unknown in this version.
    ///
    /// "EXCHANGE_UNSPECIFIED"
    #[serde(rename="EXCHANGE_UNSPECIFIED")]
    EXCHANGEUNSPECIFIED,
    

    /// Google Ad Manager.
    ///
    /// "EXCHANGE_GOOGLE_AD_MANAGER"
    #[serde(rename="EXCHANGE_GOOGLE_AD_MANAGER")]
    EXCHANGEGOOGLEADMANAGER,
    

    /// AppNexus.
    ///
    /// "EXCHANGE_APPNEXUS"
    #[serde(rename="EXCHANGE_APPNEXUS")]
    EXCHANGEAPPNEXUS,
    

    /// BrightRoll Exchange for Video from Yahoo!.
    ///
    /// "EXCHANGE_BRIGHTROLL"
    #[serde(rename="EXCHANGE_BRIGHTROLL")]
    EXCHANGEBRIGHTROLL,
    

    /// Adform.
    ///
    /// "EXCHANGE_ADFORM"
    #[serde(rename="EXCHANGE_ADFORM")]
    EXCHANGEADFORM,
    

    /// Admeta.
    ///
    /// "EXCHANGE_ADMETA"
    #[serde(rename="EXCHANGE_ADMETA")]
    EXCHANGEADMETA,
    

    /// Admixer.
    ///
    /// "EXCHANGE_ADMIXER"
    #[serde(rename="EXCHANGE_ADMIXER")]
    EXCHANGEADMIXER,
    

    /// AdsMogo.
    ///
    /// "EXCHANGE_ADSMOGO"
    #[serde(rename="EXCHANGE_ADSMOGO")]
    EXCHANGEADSMOGO,
    

    /// AdsWizz.
    ///
    /// "EXCHANGE_ADSWIZZ"
    #[serde(rename="EXCHANGE_ADSWIZZ")]
    EXCHANGEADSWIZZ,
    

    /// BidSwitch.
    ///
    /// "EXCHANGE_BIDSWITCH"
    #[serde(rename="EXCHANGE_BIDSWITCH")]
    EXCHANGEBIDSWITCH,
    

    /// BrightRoll Exchange for Display from Yahoo!.
    ///
    /// "EXCHANGE_BRIGHTROLL_DISPLAY"
    #[serde(rename="EXCHANGE_BRIGHTROLL_DISPLAY")]
    EXCHANGEBRIGHTROLLDISPLAY,
    

    /// Cadreon.
    ///
    /// "EXCHANGE_CADREON"
    #[serde(rename="EXCHANGE_CADREON")]
    EXCHANGECADREON,
    

    /// Dailymotion.
    ///
    /// "EXCHANGE_DAILYMOTION"
    #[serde(rename="EXCHANGE_DAILYMOTION")]
    EXCHANGEDAILYMOTION,
    

    /// Five.
    ///
    /// "EXCHANGE_FIVE"
    #[serde(rename="EXCHANGE_FIVE")]
    EXCHANGEFIVE,
    

    /// Fluct.
    ///
    /// "EXCHANGE_FLUCT"
    #[serde(rename="EXCHANGE_FLUCT")]
    EXCHANGEFLUCT,
    

    /// FreeWheel SSP.
    ///
    /// "EXCHANGE_FREEWHEEL"
    #[serde(rename="EXCHANGE_FREEWHEEL")]
    EXCHANGEFREEWHEEL,
    

    /// Geniee.
    ///
    /// "EXCHANGE_GENIEE"
    #[serde(rename="EXCHANGE_GENIEE")]
    EXCHANGEGENIEE,
    

    /// GumGum.
    ///
    /// "EXCHANGE_GUMGUM"
    #[serde(rename="EXCHANGE_GUMGUM")]
    EXCHANGEGUMGUM,
    

    /// i-mobile.
    ///
    /// "EXCHANGE_IMOBILE"
    #[serde(rename="EXCHANGE_IMOBILE")]
    EXCHANGEIMOBILE,
    

    /// iBILLBOARD.
    ///
    /// "EXCHANGE_IBILLBOARD"
    #[serde(rename="EXCHANGE_IBILLBOARD")]
    EXCHANGEIBILLBOARD,
    

    /// Improve Digital.
    ///
    /// "EXCHANGE_IMPROVE_DIGITAL"
    #[serde(rename="EXCHANGE_IMPROVE_DIGITAL")]
    EXCHANGEIMPROVEDIGITAL,
    

    /// Index Exchange.
    ///
    /// "EXCHANGE_INDEX"
    #[serde(rename="EXCHANGE_INDEX")]
    EXCHANGEINDEX,
    

    /// Kargo.
    ///
    /// "EXCHANGE_KARGO"
    #[serde(rename="EXCHANGE_KARGO")]
    EXCHANGEKARGO,
    

    /// MicroAd.
    ///
    /// "EXCHANGE_MICROAD"
    #[serde(rename="EXCHANGE_MICROAD")]
    EXCHANGEMICROAD,
    

    /// MoPub.
    ///
    /// "EXCHANGE_MOPUB"
    #[serde(rename="EXCHANGE_MOPUB")]
    EXCHANGEMOPUB,
    

    /// Nend.
    ///
    /// "EXCHANGE_NEND"
    #[serde(rename="EXCHANGE_NEND")]
    EXCHANGENEND,
    

    /// ONE by AOL: Display Market Place.
    ///
    /// "EXCHANGE_ONE_BY_AOL_DISPLAY"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_DISPLAY")]
    EXCHANGEONEBYAOLDISPLAY,
    

    /// ONE by AOL: Mobile.
    ///
    /// "EXCHANGE_ONE_BY_AOL_MOBILE"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_MOBILE")]
    EXCHANGEONEBYAOLMOBILE,
    

    /// ONE by AOL: Video.
    ///
    /// "EXCHANGE_ONE_BY_AOL_VIDEO"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_VIDEO")]
    EXCHANGEONEBYAOLVIDEO,
    

    /// Ooyala.
    ///
    /// "EXCHANGE_OOYALA"
    #[serde(rename="EXCHANGE_OOYALA")]
    EXCHANGEOOYALA,
    

    /// OpenX.
    ///
    /// "EXCHANGE_OPENX"
    #[serde(rename="EXCHANGE_OPENX")]
    EXCHANGEOPENX,
    

    /// Permodo.
    ///
    /// "EXCHANGE_PERMODO"
    #[serde(rename="EXCHANGE_PERMODO")]
    EXCHANGEPERMODO,
    

    /// Platform One.
    ///
    /// "EXCHANGE_PLATFORMONE"
    #[serde(rename="EXCHANGE_PLATFORMONE")]
    EXCHANGEPLATFORMONE,
    

    /// PlatformId.
    ///
    /// "EXCHANGE_PLATFORMID"
    #[serde(rename="EXCHANGE_PLATFORMID")]
    EXCHANGEPLATFORMID,
    

    /// PubMatic.
    ///
    /// "EXCHANGE_PUBMATIC"
    #[serde(rename="EXCHANGE_PUBMATIC")]
    EXCHANGEPUBMATIC,
    

    /// PulsePoint.
    ///
    /// "EXCHANGE_PULSEPOINT"
    #[serde(rename="EXCHANGE_PULSEPOINT")]
    EXCHANGEPULSEPOINT,
    

    /// RevenueMax.
    ///
    /// "EXCHANGE_REVENUEMAX"
    #[serde(rename="EXCHANGE_REVENUEMAX")]
    EXCHANGEREVENUEMAX,
    

    /// Rubicon.
    ///
    /// "EXCHANGE_RUBICON"
    #[serde(rename="EXCHANGE_RUBICON")]
    EXCHANGERUBICON,
    

    /// SmartClip.
    ///
    /// "EXCHANGE_SMARTCLIP"
    #[serde(rename="EXCHANGE_SMARTCLIP")]
    EXCHANGESMARTCLIP,
    

    /// SmartRTB+.
    ///
    /// "EXCHANGE_SMARTRTB"
    #[serde(rename="EXCHANGE_SMARTRTB")]
    EXCHANGESMARTRTB,
    

    /// SmartstreamTv.
    ///
    /// "EXCHANGE_SMARTSTREAMTV"
    #[serde(rename="EXCHANGE_SMARTSTREAMTV")]
    EXCHANGESMARTSTREAMTV,
    

    /// Sovrn.
    ///
    /// "EXCHANGE_SOVRN"
    #[serde(rename="EXCHANGE_SOVRN")]
    EXCHANGESOVRN,
    

    /// SpotXchange.
    ///
    /// "EXCHANGE_SPOTXCHANGE"
    #[serde(rename="EXCHANGE_SPOTXCHANGE")]
    EXCHANGESPOTXCHANGE,
    

    /// Ströer SSP.
    ///
    /// "EXCHANGE_STROER"
    #[serde(rename="EXCHANGE_STROER")]
    EXCHANGESTROER,
    

    /// TeadsTv.
    ///
    /// "EXCHANGE_TEADSTV"
    #[serde(rename="EXCHANGE_TEADSTV")]
    EXCHANGETEADSTV,
    

    /// Telaria.
    ///
    /// "EXCHANGE_TELARIA"
    #[serde(rename="EXCHANGE_TELARIA")]
    EXCHANGETELARIA,
    

    /// TVN.
    ///
    /// "EXCHANGE_TVN"
    #[serde(rename="EXCHANGE_TVN")]
    EXCHANGETVN,
    

    /// United.
    ///
    /// "EXCHANGE_UNITED"
    #[serde(rename="EXCHANGE_UNITED")]
    EXCHANGEUNITED,
    

    /// Yieldlab.
    ///
    /// "EXCHANGE_YIELDLAB"
    #[serde(rename="EXCHANGE_YIELDLAB")]
    EXCHANGEYIELDLAB,
    

    /// Yieldmo.
    ///
    /// "EXCHANGE_YIELDMO"
    #[serde(rename="EXCHANGE_YIELDMO")]
    EXCHANGEYIELDMO,
    

    /// UnrulyX.
    ///
    /// "EXCHANGE_UNRULYX"
    #[serde(rename="EXCHANGE_UNRULYX")]
    EXCHANGEUNRULYX,
    

    /// Open8.
    ///
    /// "EXCHANGE_OPEN8"
    #[serde(rename="EXCHANGE_OPEN8")]
    EXCHANGEOPEN8,
    

    /// Triton.
    ///
    /// "EXCHANGE_TRITON"
    #[serde(rename="EXCHANGE_TRITON")]
    EXCHANGETRITON,
    

    /// TripleLift.
    ///
    /// "EXCHANGE_TRIPLELIFT"
    #[serde(rename="EXCHANGE_TRIPLELIFT")]
    EXCHANGETRIPLELIFT,
    

    /// Taboola.
    ///
    /// "EXCHANGE_TABOOLA"
    #[serde(rename="EXCHANGE_TABOOLA")]
    EXCHANGETABOOLA,
    

    /// InMobi.
    ///
    /// "EXCHANGE_INMOBI"
    #[serde(rename="EXCHANGE_INMOBI")]
    EXCHANGEINMOBI,
    

    /// Smaato.
    ///
    /// "EXCHANGE_SMAATO"
    #[serde(rename="EXCHANGE_SMAATO")]
    EXCHANGESMAATO,
    

    /// Aja.
    ///
    /// "EXCHANGE_AJA"
    #[serde(rename="EXCHANGE_AJA")]
    EXCHANGEAJA,
    

    /// Supership.
    ///
    /// "EXCHANGE_SUPERSHIP"
    #[serde(rename="EXCHANGE_SUPERSHIP")]
    EXCHANGESUPERSHIP,
    

    /// Nexstar Digital.
    ///
    /// "EXCHANGE_NEXSTAR_DIGITAL"
    #[serde(rename="EXCHANGE_NEXSTAR_DIGITAL")]
    EXCHANGENEXSTARDIGITAL,
    

    /// Waze.
    ///
    /// "EXCHANGE_WAZE"
    #[serde(rename="EXCHANGE_WAZE")]
    EXCHANGEWAZE,
    

    /// SoundCast.
    ///
    /// "EXCHANGE_SOUNDCAST"
    #[serde(rename="EXCHANGE_SOUNDCAST")]
    EXCHANGESOUNDCAST,
    

    /// Sharethrough.
    ///
    /// "EXCHANGE_SHARETHROUGH"
    #[serde(rename="EXCHANGE_SHARETHROUGH")]
    EXCHANGESHARETHROUGH,
    

    /// Fyber.
    ///
    /// "EXCHANGE_FYBER"
    #[serde(rename="EXCHANGE_FYBER")]
    EXCHANGEFYBER,
    

    /// Red For Publishers.
    ///
    /// "EXCHANGE_RED_FOR_PUBLISHERS"
    #[serde(rename="EXCHANGE_RED_FOR_PUBLISHERS")]
    EXCHANGEREDFORPUBLISHERS,
    

    /// Media.net.
    ///
    /// "EXCHANGE_MEDIANET"
    #[serde(rename="EXCHANGE_MEDIANET")]
    EXCHANGEMEDIANET,
    

    /// Tapjoy.
    ///
    /// "EXCHANGE_TAPJOY"
    #[serde(rename="EXCHANGE_TAPJOY")]
    EXCHANGETAPJOY,
    

    /// Vistar.
    ///
    /// "EXCHANGE_VISTAR"
    #[serde(rename="EXCHANGE_VISTAR")]
    EXCHANGEVISTAR,
    

    /// DAX.
    ///
    /// "EXCHANGE_DAX"
    #[serde(rename="EXCHANGE_DAX")]
    EXCHANGEDAX,
    

    /// JCD.
    ///
    /// "EXCHANGE_JCD"
    #[serde(rename="EXCHANGE_JCD")]
    EXCHANGEJCD,
    

    /// Place Exchange.
    ///
    /// "EXCHANGE_PLACE_EXCHANGE"
    #[serde(rename="EXCHANGE_PLACE_EXCHANGE")]
    EXCHANGEPLACEEXCHANGE,
    

    /// AppLovin.
    ///
    /// "EXCHANGE_APPLOVIN"
    #[serde(rename="EXCHANGE_APPLOVIN")]
    EXCHANGEAPPLOVIN,
    

    /// Connatix.
    ///
    /// "EXCHANGE_CONNATIX"
    #[serde(rename="EXCHANGE_CONNATIX")]
    EXCHANGECONNATIX,
    

    /// Reset Digital.
    ///
    /// "EXCHANGE_RESET_DIGITAL"
    #[serde(rename="EXCHANGE_RESET_DIGITAL")]
    EXCHANGERESETDIGITAL,
    

    /// Hivestack.
    ///
    /// "EXCHANGE_HIVESTACK"
    #[serde(rename="EXCHANGE_HIVESTACK")]
    EXCHANGEHIVESTACK,
    

    /// Drax.
    ///
    /// "EXCHANGE_DRAX"
    #[serde(rename="EXCHANGE_DRAX")]
    EXCHANGEDRAX,
    

    /// AppLovin MAX.
    ///
    /// "EXCHANGE_APPLOVIN_GBID"
    #[serde(rename="EXCHANGE_APPLOVIN_GBID")]
    EXCHANGEAPPLOVINGBID,
    

    /// DT Fairbid.
    ///
    /// "EXCHANGE_FYBER_GBID"
    #[serde(rename="EXCHANGE_FYBER_GBID")]
    EXCHANGEFYBERGBID,
    

    /// Unity LevelPlay.
    ///
    /// "EXCHANGE_UNITY_GBID"
    #[serde(rename="EXCHANGE_UNITY_GBID")]
    EXCHANGEUNITYGBID,
    

    /// Chartboost Mediation.
    ///
    /// "EXCHANGE_CHARTBOOST_GBID"
    #[serde(rename="EXCHANGE_CHARTBOOST_GBID")]
    EXCHANGECHARTBOOSTGBID,
    

    /// AdMost.
    ///
    /// "EXCHANGE_ADMOST_GBID"
    #[serde(rename="EXCHANGE_ADMOST_GBID")]
    EXCHANGEADMOSTGBID,
    

    /// TopOn.
    ///
    /// "EXCHANGE_TOPON_GBID"
    #[serde(rename="EXCHANGE_TOPON_GBID")]
    EXCHANGETOPONGBID,
}

impl AsRef<str> for GuaranteedOrderExchangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuaranteedOrderExchangeEnum::EXCHANGEUNSPECIFIED => "EXCHANGE_UNSPECIFIED",
            GuaranteedOrderExchangeEnum::EXCHANGEGOOGLEADMANAGER => "EXCHANGE_GOOGLE_AD_MANAGER",
            GuaranteedOrderExchangeEnum::EXCHANGEAPPNEXUS => "EXCHANGE_APPNEXUS",
            GuaranteedOrderExchangeEnum::EXCHANGEBRIGHTROLL => "EXCHANGE_BRIGHTROLL",
            GuaranteedOrderExchangeEnum::EXCHANGEADFORM => "EXCHANGE_ADFORM",
            GuaranteedOrderExchangeEnum::EXCHANGEADMETA => "EXCHANGE_ADMETA",
            GuaranteedOrderExchangeEnum::EXCHANGEADMIXER => "EXCHANGE_ADMIXER",
            GuaranteedOrderExchangeEnum::EXCHANGEADSMOGO => "EXCHANGE_ADSMOGO",
            GuaranteedOrderExchangeEnum::EXCHANGEADSWIZZ => "EXCHANGE_ADSWIZZ",
            GuaranteedOrderExchangeEnum::EXCHANGEBIDSWITCH => "EXCHANGE_BIDSWITCH",
            GuaranteedOrderExchangeEnum::EXCHANGEBRIGHTROLLDISPLAY => "EXCHANGE_BRIGHTROLL_DISPLAY",
            GuaranteedOrderExchangeEnum::EXCHANGECADREON => "EXCHANGE_CADREON",
            GuaranteedOrderExchangeEnum::EXCHANGEDAILYMOTION => "EXCHANGE_DAILYMOTION",
            GuaranteedOrderExchangeEnum::EXCHANGEFIVE => "EXCHANGE_FIVE",
            GuaranteedOrderExchangeEnum::EXCHANGEFLUCT => "EXCHANGE_FLUCT",
            GuaranteedOrderExchangeEnum::EXCHANGEFREEWHEEL => "EXCHANGE_FREEWHEEL",
            GuaranteedOrderExchangeEnum::EXCHANGEGENIEE => "EXCHANGE_GENIEE",
            GuaranteedOrderExchangeEnum::EXCHANGEGUMGUM => "EXCHANGE_GUMGUM",
            GuaranteedOrderExchangeEnum::EXCHANGEIMOBILE => "EXCHANGE_IMOBILE",
            GuaranteedOrderExchangeEnum::EXCHANGEIBILLBOARD => "EXCHANGE_IBILLBOARD",
            GuaranteedOrderExchangeEnum::EXCHANGEIMPROVEDIGITAL => "EXCHANGE_IMPROVE_DIGITAL",
            GuaranteedOrderExchangeEnum::EXCHANGEINDEX => "EXCHANGE_INDEX",
            GuaranteedOrderExchangeEnum::EXCHANGEKARGO => "EXCHANGE_KARGO",
            GuaranteedOrderExchangeEnum::EXCHANGEMICROAD => "EXCHANGE_MICROAD",
            GuaranteedOrderExchangeEnum::EXCHANGEMOPUB => "EXCHANGE_MOPUB",
            GuaranteedOrderExchangeEnum::EXCHANGENEND => "EXCHANGE_NEND",
            GuaranteedOrderExchangeEnum::EXCHANGEONEBYAOLDISPLAY => "EXCHANGE_ONE_BY_AOL_DISPLAY",
            GuaranteedOrderExchangeEnum::EXCHANGEONEBYAOLMOBILE => "EXCHANGE_ONE_BY_AOL_MOBILE",
            GuaranteedOrderExchangeEnum::EXCHANGEONEBYAOLVIDEO => "EXCHANGE_ONE_BY_AOL_VIDEO",
            GuaranteedOrderExchangeEnum::EXCHANGEOOYALA => "EXCHANGE_OOYALA",
            GuaranteedOrderExchangeEnum::EXCHANGEOPENX => "EXCHANGE_OPENX",
            GuaranteedOrderExchangeEnum::EXCHANGEPERMODO => "EXCHANGE_PERMODO",
            GuaranteedOrderExchangeEnum::EXCHANGEPLATFORMONE => "EXCHANGE_PLATFORMONE",
            GuaranteedOrderExchangeEnum::EXCHANGEPLATFORMID => "EXCHANGE_PLATFORMID",
            GuaranteedOrderExchangeEnum::EXCHANGEPUBMATIC => "EXCHANGE_PUBMATIC",
            GuaranteedOrderExchangeEnum::EXCHANGEPULSEPOINT => "EXCHANGE_PULSEPOINT",
            GuaranteedOrderExchangeEnum::EXCHANGEREVENUEMAX => "EXCHANGE_REVENUEMAX",
            GuaranteedOrderExchangeEnum::EXCHANGERUBICON => "EXCHANGE_RUBICON",
            GuaranteedOrderExchangeEnum::EXCHANGESMARTCLIP => "EXCHANGE_SMARTCLIP",
            GuaranteedOrderExchangeEnum::EXCHANGESMARTRTB => "EXCHANGE_SMARTRTB",
            GuaranteedOrderExchangeEnum::EXCHANGESMARTSTREAMTV => "EXCHANGE_SMARTSTREAMTV",
            GuaranteedOrderExchangeEnum::EXCHANGESOVRN => "EXCHANGE_SOVRN",
            GuaranteedOrderExchangeEnum::EXCHANGESPOTXCHANGE => "EXCHANGE_SPOTXCHANGE",
            GuaranteedOrderExchangeEnum::EXCHANGESTROER => "EXCHANGE_STROER",
            GuaranteedOrderExchangeEnum::EXCHANGETEADSTV => "EXCHANGE_TEADSTV",
            GuaranteedOrderExchangeEnum::EXCHANGETELARIA => "EXCHANGE_TELARIA",
            GuaranteedOrderExchangeEnum::EXCHANGETVN => "EXCHANGE_TVN",
            GuaranteedOrderExchangeEnum::EXCHANGEUNITED => "EXCHANGE_UNITED",
            GuaranteedOrderExchangeEnum::EXCHANGEYIELDLAB => "EXCHANGE_YIELDLAB",
            GuaranteedOrderExchangeEnum::EXCHANGEYIELDMO => "EXCHANGE_YIELDMO",
            GuaranteedOrderExchangeEnum::EXCHANGEUNRULYX => "EXCHANGE_UNRULYX",
            GuaranteedOrderExchangeEnum::EXCHANGEOPEN8 => "EXCHANGE_OPEN8",
            GuaranteedOrderExchangeEnum::EXCHANGETRITON => "EXCHANGE_TRITON",
            GuaranteedOrderExchangeEnum::EXCHANGETRIPLELIFT => "EXCHANGE_TRIPLELIFT",
            GuaranteedOrderExchangeEnum::EXCHANGETABOOLA => "EXCHANGE_TABOOLA",
            GuaranteedOrderExchangeEnum::EXCHANGEINMOBI => "EXCHANGE_INMOBI",
            GuaranteedOrderExchangeEnum::EXCHANGESMAATO => "EXCHANGE_SMAATO",
            GuaranteedOrderExchangeEnum::EXCHANGEAJA => "EXCHANGE_AJA",
            GuaranteedOrderExchangeEnum::EXCHANGESUPERSHIP => "EXCHANGE_SUPERSHIP",
            GuaranteedOrderExchangeEnum::EXCHANGENEXSTARDIGITAL => "EXCHANGE_NEXSTAR_DIGITAL",
            GuaranteedOrderExchangeEnum::EXCHANGEWAZE => "EXCHANGE_WAZE",
            GuaranteedOrderExchangeEnum::EXCHANGESOUNDCAST => "EXCHANGE_SOUNDCAST",
            GuaranteedOrderExchangeEnum::EXCHANGESHARETHROUGH => "EXCHANGE_SHARETHROUGH",
            GuaranteedOrderExchangeEnum::EXCHANGEFYBER => "EXCHANGE_FYBER",
            GuaranteedOrderExchangeEnum::EXCHANGEREDFORPUBLISHERS => "EXCHANGE_RED_FOR_PUBLISHERS",
            GuaranteedOrderExchangeEnum::EXCHANGEMEDIANET => "EXCHANGE_MEDIANET",
            GuaranteedOrderExchangeEnum::EXCHANGETAPJOY => "EXCHANGE_TAPJOY",
            GuaranteedOrderExchangeEnum::EXCHANGEVISTAR => "EXCHANGE_VISTAR",
            GuaranteedOrderExchangeEnum::EXCHANGEDAX => "EXCHANGE_DAX",
            GuaranteedOrderExchangeEnum::EXCHANGEJCD => "EXCHANGE_JCD",
            GuaranteedOrderExchangeEnum::EXCHANGEPLACEEXCHANGE => "EXCHANGE_PLACE_EXCHANGE",
            GuaranteedOrderExchangeEnum::EXCHANGEAPPLOVIN => "EXCHANGE_APPLOVIN",
            GuaranteedOrderExchangeEnum::EXCHANGECONNATIX => "EXCHANGE_CONNATIX",
            GuaranteedOrderExchangeEnum::EXCHANGERESETDIGITAL => "EXCHANGE_RESET_DIGITAL",
            GuaranteedOrderExchangeEnum::EXCHANGEHIVESTACK => "EXCHANGE_HIVESTACK",
            GuaranteedOrderExchangeEnum::EXCHANGEDRAX => "EXCHANGE_DRAX",
            GuaranteedOrderExchangeEnum::EXCHANGEAPPLOVINGBID => "EXCHANGE_APPLOVIN_GBID",
            GuaranteedOrderExchangeEnum::EXCHANGEFYBERGBID => "EXCHANGE_FYBER_GBID",
            GuaranteedOrderExchangeEnum::EXCHANGEUNITYGBID => "EXCHANGE_UNITY_GBID",
            GuaranteedOrderExchangeEnum::EXCHANGECHARTBOOSTGBID => "EXCHANGE_CHARTBOOST_GBID",
            GuaranteedOrderExchangeEnum::EXCHANGEADMOSTGBID => "EXCHANGE_ADMOST_GBID",
            GuaranteedOrderExchangeEnum::EXCHANGETOPONGBID => "EXCHANGE_TOPON_GBID",
        }
    }
}

impl std::convert::TryFrom< &str> for GuaranteedOrderExchangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCHANGE_UNSPECIFIED" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEUNSPECIFIED),
           "EXCHANGE_GOOGLE_AD_MANAGER" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEGOOGLEADMANAGER),
           "EXCHANGE_APPNEXUS" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEAPPNEXUS),
           "EXCHANGE_BRIGHTROLL" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEBRIGHTROLL),
           "EXCHANGE_ADFORM" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEADFORM),
           "EXCHANGE_ADMETA" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEADMETA),
           "EXCHANGE_ADMIXER" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEADMIXER),
           "EXCHANGE_ADSMOGO" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEADSMOGO),
           "EXCHANGE_ADSWIZZ" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEADSWIZZ),
           "EXCHANGE_BIDSWITCH" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEBIDSWITCH),
           "EXCHANGE_BRIGHTROLL_DISPLAY" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEBRIGHTROLLDISPLAY),
           "EXCHANGE_CADREON" => Ok(GuaranteedOrderExchangeEnum::EXCHANGECADREON),
           "EXCHANGE_DAILYMOTION" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEDAILYMOTION),
           "EXCHANGE_FIVE" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEFIVE),
           "EXCHANGE_FLUCT" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEFLUCT),
           "EXCHANGE_FREEWHEEL" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEFREEWHEEL),
           "EXCHANGE_GENIEE" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEGENIEE),
           "EXCHANGE_GUMGUM" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEGUMGUM),
           "EXCHANGE_IMOBILE" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEIMOBILE),
           "EXCHANGE_IBILLBOARD" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEIBILLBOARD),
           "EXCHANGE_IMPROVE_DIGITAL" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEIMPROVEDIGITAL),
           "EXCHANGE_INDEX" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEINDEX),
           "EXCHANGE_KARGO" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEKARGO),
           "EXCHANGE_MICROAD" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEMICROAD),
           "EXCHANGE_MOPUB" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEMOPUB),
           "EXCHANGE_NEND" => Ok(GuaranteedOrderExchangeEnum::EXCHANGENEND),
           "EXCHANGE_ONE_BY_AOL_DISPLAY" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEONEBYAOLDISPLAY),
           "EXCHANGE_ONE_BY_AOL_MOBILE" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEONEBYAOLMOBILE),
           "EXCHANGE_ONE_BY_AOL_VIDEO" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEONEBYAOLVIDEO),
           "EXCHANGE_OOYALA" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEOOYALA),
           "EXCHANGE_OPENX" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEOPENX),
           "EXCHANGE_PERMODO" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEPERMODO),
           "EXCHANGE_PLATFORMONE" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEPLATFORMONE),
           "EXCHANGE_PLATFORMID" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEPLATFORMID),
           "EXCHANGE_PUBMATIC" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEPUBMATIC),
           "EXCHANGE_PULSEPOINT" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEPULSEPOINT),
           "EXCHANGE_REVENUEMAX" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEREVENUEMAX),
           "EXCHANGE_RUBICON" => Ok(GuaranteedOrderExchangeEnum::EXCHANGERUBICON),
           "EXCHANGE_SMARTCLIP" => Ok(GuaranteedOrderExchangeEnum::EXCHANGESMARTCLIP),
           "EXCHANGE_SMARTRTB" => Ok(GuaranteedOrderExchangeEnum::EXCHANGESMARTRTB),
           "EXCHANGE_SMARTSTREAMTV" => Ok(GuaranteedOrderExchangeEnum::EXCHANGESMARTSTREAMTV),
           "EXCHANGE_SOVRN" => Ok(GuaranteedOrderExchangeEnum::EXCHANGESOVRN),
           "EXCHANGE_SPOTXCHANGE" => Ok(GuaranteedOrderExchangeEnum::EXCHANGESPOTXCHANGE),
           "EXCHANGE_STROER" => Ok(GuaranteedOrderExchangeEnum::EXCHANGESTROER),
           "EXCHANGE_TEADSTV" => Ok(GuaranteedOrderExchangeEnum::EXCHANGETEADSTV),
           "EXCHANGE_TELARIA" => Ok(GuaranteedOrderExchangeEnum::EXCHANGETELARIA),
           "EXCHANGE_TVN" => Ok(GuaranteedOrderExchangeEnum::EXCHANGETVN),
           "EXCHANGE_UNITED" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEUNITED),
           "EXCHANGE_YIELDLAB" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEYIELDLAB),
           "EXCHANGE_YIELDMO" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEYIELDMO),
           "EXCHANGE_UNRULYX" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEUNRULYX),
           "EXCHANGE_OPEN8" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEOPEN8),
           "EXCHANGE_TRITON" => Ok(GuaranteedOrderExchangeEnum::EXCHANGETRITON),
           "EXCHANGE_TRIPLELIFT" => Ok(GuaranteedOrderExchangeEnum::EXCHANGETRIPLELIFT),
           "EXCHANGE_TABOOLA" => Ok(GuaranteedOrderExchangeEnum::EXCHANGETABOOLA),
           "EXCHANGE_INMOBI" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEINMOBI),
           "EXCHANGE_SMAATO" => Ok(GuaranteedOrderExchangeEnum::EXCHANGESMAATO),
           "EXCHANGE_AJA" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEAJA),
           "EXCHANGE_SUPERSHIP" => Ok(GuaranteedOrderExchangeEnum::EXCHANGESUPERSHIP),
           "EXCHANGE_NEXSTAR_DIGITAL" => Ok(GuaranteedOrderExchangeEnum::EXCHANGENEXSTARDIGITAL),
           "EXCHANGE_WAZE" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEWAZE),
           "EXCHANGE_SOUNDCAST" => Ok(GuaranteedOrderExchangeEnum::EXCHANGESOUNDCAST),
           "EXCHANGE_SHARETHROUGH" => Ok(GuaranteedOrderExchangeEnum::EXCHANGESHARETHROUGH),
           "EXCHANGE_FYBER" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEFYBER),
           "EXCHANGE_RED_FOR_PUBLISHERS" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEREDFORPUBLISHERS),
           "EXCHANGE_MEDIANET" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEMEDIANET),
           "EXCHANGE_TAPJOY" => Ok(GuaranteedOrderExchangeEnum::EXCHANGETAPJOY),
           "EXCHANGE_VISTAR" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEVISTAR),
           "EXCHANGE_DAX" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEDAX),
           "EXCHANGE_JCD" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEJCD),
           "EXCHANGE_PLACE_EXCHANGE" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEPLACEEXCHANGE),
           "EXCHANGE_APPLOVIN" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEAPPLOVIN),
           "EXCHANGE_CONNATIX" => Ok(GuaranteedOrderExchangeEnum::EXCHANGECONNATIX),
           "EXCHANGE_RESET_DIGITAL" => Ok(GuaranteedOrderExchangeEnum::EXCHANGERESETDIGITAL),
           "EXCHANGE_HIVESTACK" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEHIVESTACK),
           "EXCHANGE_DRAX" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEDRAX),
           "EXCHANGE_APPLOVIN_GBID" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEAPPLOVINGBID),
           "EXCHANGE_FYBER_GBID" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEFYBERGBID),
           "EXCHANGE_UNITY_GBID" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEUNITYGBID),
           "EXCHANGE_CHARTBOOST_GBID" => Ok(GuaranteedOrderExchangeEnum::EXCHANGECHARTBOOSTGBID),
           "EXCHANGE_ADMOST_GBID" => Ok(GuaranteedOrderExchangeEnum::EXCHANGEADMOSTGBID),
           "EXCHANGE_TOPON_GBID" => Ok(GuaranteedOrderExchangeEnum::EXCHANGETOPONGBID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuaranteedOrderExchangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuaranteedOrderStatusConfigStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The configuration status of the guaranteed order. Acceptable values are `PENDING` and `COMPLETED`. A guaranteed order must be configured (fill in the required fields, choose creatives, and select a default campaign) before it can serve. Currently the configuration action can only be performed via UI.
pub enum GuaranteedOrderStatusConfigStatusEnum {
    

    /// The approval status is not specified or is unknown in this version.
    ///
    /// "GUARANTEED_ORDER_CONFIG_STATUS_UNSPECIFIED"
    #[serde(rename="GUARANTEED_ORDER_CONFIG_STATUS_UNSPECIFIED")]
    GUARANTEEDORDERCONFIGSTATUSUNSPECIFIED,
    

    /// The beginning state of a guaranteed order. The guaranteed order in this state needs to be configured before it can serve.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The state after the buyer configures a guaranteed order.
    ///
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
}

impl AsRef<str> for GuaranteedOrderStatusConfigStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuaranteedOrderStatusConfigStatusEnum::GUARANTEEDORDERCONFIGSTATUSUNSPECIFIED => "GUARANTEED_ORDER_CONFIG_STATUS_UNSPECIFIED",
            GuaranteedOrderStatusConfigStatusEnum::PENDING => "PENDING",
            GuaranteedOrderStatusConfigStatusEnum::COMPLETED => "COMPLETED",
        }
    }
}

impl std::convert::TryFrom< &str> for GuaranteedOrderStatusConfigStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GUARANTEED_ORDER_CONFIG_STATUS_UNSPECIFIED" => Ok(GuaranteedOrderStatusConfigStatusEnum::GUARANTEEDORDERCONFIGSTATUSUNSPECIFIED),
           "PENDING" => Ok(GuaranteedOrderStatusConfigStatusEnum::PENDING),
           "COMPLETED" => Ok(GuaranteedOrderStatusConfigStatusEnum::COMPLETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuaranteedOrderStatusConfigStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuaranteedOrderStatusEntityStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether or not the guaranteed order is servable. Acceptable values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_ARCHIVED`, and `ENTITY_STATUS_PAUSED`. Default value is `ENTITY_STATUS_ACTIVE`.
pub enum GuaranteedOrderStatusEntityStatusEnum {
    

    /// Default value when status is not specified or is unknown in this version.
    ///
    /// "ENTITY_STATUS_UNSPECIFIED"
    #[serde(rename="ENTITY_STATUS_UNSPECIFIED")]
    ENTITYSTATUSUNSPECIFIED,
    

    /// The entity is enabled to bid and spend budget.
    ///
    /// "ENTITY_STATUS_ACTIVE"
    #[serde(rename="ENTITY_STATUS_ACTIVE")]
    ENTITYSTATUSACTIVE,
    

    /// The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved.
    ///
    /// "ENTITY_STATUS_ARCHIVED"
    #[serde(rename="ENTITY_STATUS_ARCHIVED")]
    ENTITYSTATUSARCHIVED,
    

    /// The entity is under draft. Bidding and budget spending are disabled.
    ///
    /// "ENTITY_STATUS_DRAFT"
    #[serde(rename="ENTITY_STATUS_DRAFT")]
    ENTITYSTATUSDRAFT,
    

    /// Bidding and budget spending are paused for the entity.
    ///
    /// "ENTITY_STATUS_PAUSED"
    #[serde(rename="ENTITY_STATUS_PAUSED")]
    ENTITYSTATUSPAUSED,
    

    /// The entity is scheduled for deletion.
    ///
    /// "ENTITY_STATUS_SCHEDULED_FOR_DELETION"
    #[serde(rename="ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    ENTITYSTATUSSCHEDULEDFORDELETION,
}

impl AsRef<str> for GuaranteedOrderStatusEntityStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSUNSPECIFIED => "ENTITY_STATUS_UNSPECIFIED",
            GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSACTIVE => "ENTITY_STATUS_ACTIVE",
            GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSARCHIVED => "ENTITY_STATUS_ARCHIVED",
            GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSDRAFT => "ENTITY_STATUS_DRAFT",
            GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSPAUSED => "ENTITY_STATUS_PAUSED",
            GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION => "ENTITY_STATUS_SCHEDULED_FOR_DELETION",
        }
    }
}

impl std::convert::TryFrom< &str> for GuaranteedOrderStatusEntityStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_STATUS_UNSPECIFIED" => Ok(GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSUNSPECIFIED),
           "ENTITY_STATUS_ACTIVE" => Ok(GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSACTIVE),
           "ENTITY_STATUS_ARCHIVED" => Ok(GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSARCHIVED),
           "ENTITY_STATUS_DRAFT" => Ok(GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSDRAFT),
           "ENTITY_STATUS_PAUSED" => Ok(GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSPAUSED),
           "ENTITY_STATUS_SCHEDULED_FOR_DELETION" => Ok(GuaranteedOrderStatusEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuaranteedOrderStatusEntityStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The household income of the audience.
pub enum HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum {
    

    /// Default value when household income is not specified in this version. This enum is a placeholder for default value and does not represent a real household income option.
    ///
    /// "HOUSEHOLD_INCOME_UNSPECIFIED"
    #[serde(rename="HOUSEHOLD_INCOME_UNSPECIFIED")]
    HOUSEHOLDINCOMEUNSPECIFIED,
    

    /// The household income of the audience is unknown.
    ///
    /// "HOUSEHOLD_INCOME_UNKNOWN"
    #[serde(rename="HOUSEHOLD_INCOME_UNKNOWN")]
    HOUSEHOLDINCOMEUNKNOWN,
    

    /// The audience is in the lower 50% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_LOWER_50_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_LOWER_50_PERCENT")]
    HOUSEHOLDINCOMELOWER50PERCENT,
    

    /// The audience is in the top 41-50% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_TOP_41_TO_50_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_TOP_41_TO_50_PERCENT")]
    HOUSEHOLDINCOMETOP41TO50PERCENT,
    

    /// The audience is in the top 31-40% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_TOP_31_TO_40_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_TOP_31_TO_40_PERCENT")]
    HOUSEHOLDINCOMETOP31TO40PERCENT,
    

    /// The audience is in the top 21-30% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_TOP_21_TO_30_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_TOP_21_TO_30_PERCENT")]
    HOUSEHOLDINCOMETOP21TO30PERCENT,
    

    /// The audience is in the top 11-20% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_TOP_11_TO_20_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_TOP_11_TO_20_PERCENT")]
    HOUSEHOLDINCOMETOP11TO20PERCENT,
    

    /// The audience is in the top 10% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_TOP_10_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_TOP_10_PERCENT")]
    HOUSEHOLDINCOMETOP10PERCENT,
}

impl AsRef<str> for HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMEUNSPECIFIED => "HOUSEHOLD_INCOME_UNSPECIFIED",
            HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMEUNKNOWN => "HOUSEHOLD_INCOME_UNKNOWN",
            HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMELOWER50PERCENT => "HOUSEHOLD_INCOME_LOWER_50_PERCENT",
            HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP41TO50PERCENT => "HOUSEHOLD_INCOME_TOP_41_TO_50_PERCENT",
            HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP31TO40PERCENT => "HOUSEHOLD_INCOME_TOP_31_TO_40_PERCENT",
            HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP21TO30PERCENT => "HOUSEHOLD_INCOME_TOP_21_TO_30_PERCENT",
            HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP11TO20PERCENT => "HOUSEHOLD_INCOME_TOP_11_TO_20_PERCENT",
            HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP10PERCENT => "HOUSEHOLD_INCOME_TOP_10_PERCENT",
        }
    }
}

impl std::convert::TryFrom< &str> for HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HOUSEHOLD_INCOME_UNSPECIFIED" => Ok(HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMEUNSPECIFIED),
           "HOUSEHOLD_INCOME_UNKNOWN" => Ok(HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMEUNKNOWN),
           "HOUSEHOLD_INCOME_LOWER_50_PERCENT" => Ok(HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMELOWER50PERCENT),
           "HOUSEHOLD_INCOME_TOP_41_TO_50_PERCENT" => Ok(HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP41TO50PERCENT),
           "HOUSEHOLD_INCOME_TOP_31_TO_40_PERCENT" => Ok(HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP31TO40PERCENT),
           "HOUSEHOLD_INCOME_TOP_21_TO_30_PERCENT" => Ok(HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP21TO30PERCENT),
           "HOUSEHOLD_INCOME_TOP_11_TO_20_PERCENT" => Ok(HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP11TO20PERCENT),
           "HOUSEHOLD_INCOME_TOP_10_PERCENT" => Ok(HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP10PERCENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HouseholdIncomeAssignedTargetingOptionDetailHouseholdIncomeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The household income of an audience.
pub enum HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum {
    

    /// Default value when household income is not specified in this version. This enum is a placeholder for default value and does not represent a real household income option.
    ///
    /// "HOUSEHOLD_INCOME_UNSPECIFIED"
    #[serde(rename="HOUSEHOLD_INCOME_UNSPECIFIED")]
    HOUSEHOLDINCOMEUNSPECIFIED,
    

    /// The household income of the audience is unknown.
    ///
    /// "HOUSEHOLD_INCOME_UNKNOWN"
    #[serde(rename="HOUSEHOLD_INCOME_UNKNOWN")]
    HOUSEHOLDINCOMEUNKNOWN,
    

    /// The audience is in the lower 50% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_LOWER_50_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_LOWER_50_PERCENT")]
    HOUSEHOLDINCOMELOWER50PERCENT,
    

    /// The audience is in the top 41-50% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_TOP_41_TO_50_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_TOP_41_TO_50_PERCENT")]
    HOUSEHOLDINCOMETOP41TO50PERCENT,
    

    /// The audience is in the top 31-40% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_TOP_31_TO_40_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_TOP_31_TO_40_PERCENT")]
    HOUSEHOLDINCOMETOP31TO40PERCENT,
    

    /// The audience is in the top 21-30% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_TOP_21_TO_30_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_TOP_21_TO_30_PERCENT")]
    HOUSEHOLDINCOMETOP21TO30PERCENT,
    

    /// The audience is in the top 11-20% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_TOP_11_TO_20_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_TOP_11_TO_20_PERCENT")]
    HOUSEHOLDINCOMETOP11TO20PERCENT,
    

    /// The audience is in the top 10% of U.S. household incomes.
    ///
    /// "HOUSEHOLD_INCOME_TOP_10_PERCENT"
    #[serde(rename="HOUSEHOLD_INCOME_TOP_10_PERCENT")]
    HOUSEHOLDINCOMETOP10PERCENT,
}

impl AsRef<str> for HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMEUNSPECIFIED => "HOUSEHOLD_INCOME_UNSPECIFIED",
            HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMEUNKNOWN => "HOUSEHOLD_INCOME_UNKNOWN",
            HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMELOWER50PERCENT => "HOUSEHOLD_INCOME_LOWER_50_PERCENT",
            HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP41TO50PERCENT => "HOUSEHOLD_INCOME_TOP_41_TO_50_PERCENT",
            HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP31TO40PERCENT => "HOUSEHOLD_INCOME_TOP_31_TO_40_PERCENT",
            HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP21TO30PERCENT => "HOUSEHOLD_INCOME_TOP_21_TO_30_PERCENT",
            HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP11TO20PERCENT => "HOUSEHOLD_INCOME_TOP_11_TO_20_PERCENT",
            HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP10PERCENT => "HOUSEHOLD_INCOME_TOP_10_PERCENT",
        }
    }
}

impl std::convert::TryFrom< &str> for HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HOUSEHOLD_INCOME_UNSPECIFIED" => Ok(HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMEUNSPECIFIED),
           "HOUSEHOLD_INCOME_UNKNOWN" => Ok(HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMEUNKNOWN),
           "HOUSEHOLD_INCOME_LOWER_50_PERCENT" => Ok(HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMELOWER50PERCENT),
           "HOUSEHOLD_INCOME_TOP_41_TO_50_PERCENT" => Ok(HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP41TO50PERCENT),
           "HOUSEHOLD_INCOME_TOP_31_TO_40_PERCENT" => Ok(HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP31TO40PERCENT),
           "HOUSEHOLD_INCOME_TOP_21_TO_30_PERCENT" => Ok(HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP21TO30PERCENT),
           "HOUSEHOLD_INCOME_TOP_11_TO_20_PERCENT" => Ok(HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP11TO20PERCENT),
           "HOUSEHOLD_INCOME_TOP_10_PERCENT" => Ok(HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum::HOUSEHOLDINCOMETOP10PERCENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HouseholdIncomeTargetingOptionDetailHouseholdIncomeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InsertionOrderBillableOutcomeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The billable outcome of the insertion order. Outcome based buying is deprecated. `BILLABLE_OUTCOME_PAY_PER_IMPRESSION` is the only valid value.
pub enum InsertionOrderBillableOutcomeEnum {
    

    /// Unspecified billable outcome.
    ///
    /// "BILLABLE_OUTCOME_UNSPECIFIED"
    #[serde(rename="BILLABLE_OUTCOME_UNSPECIFIED")]
    BILLABLEOUTCOMEUNSPECIFIED,
    

    /// Pay per impressions.
    ///
    /// "BILLABLE_OUTCOME_PAY_PER_IMPRESSION"
    #[serde(rename="BILLABLE_OUTCOME_PAY_PER_IMPRESSION")]
    BILLABLEOUTCOMEPAYPERIMPRESSION,
    

    /// Pay per click.
    ///
    /// "BILLABLE_OUTCOME_PAY_PER_CLICK"
    #[serde(rename="BILLABLE_OUTCOME_PAY_PER_CLICK")]
    BILLABLEOUTCOMEPAYPERCLICK,
    

    /// Pay per active view.
    ///
    /// "BILLABLE_OUTCOME_PAY_PER_VIEWABLE_IMPRESSION"
    #[serde(rename="BILLABLE_OUTCOME_PAY_PER_VIEWABLE_IMPRESSION")]
    BILLABLEOUTCOMEPAYPERVIEWABLEIMPRESSION,
}

impl AsRef<str> for InsertionOrderBillableOutcomeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InsertionOrderBillableOutcomeEnum::BILLABLEOUTCOMEUNSPECIFIED => "BILLABLE_OUTCOME_UNSPECIFIED",
            InsertionOrderBillableOutcomeEnum::BILLABLEOUTCOMEPAYPERIMPRESSION => "BILLABLE_OUTCOME_PAY_PER_IMPRESSION",
            InsertionOrderBillableOutcomeEnum::BILLABLEOUTCOMEPAYPERCLICK => "BILLABLE_OUTCOME_PAY_PER_CLICK",
            InsertionOrderBillableOutcomeEnum::BILLABLEOUTCOMEPAYPERVIEWABLEIMPRESSION => "BILLABLE_OUTCOME_PAY_PER_VIEWABLE_IMPRESSION",
        }
    }
}

impl std::convert::TryFrom< &str> for InsertionOrderBillableOutcomeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BILLABLE_OUTCOME_UNSPECIFIED" => Ok(InsertionOrderBillableOutcomeEnum::BILLABLEOUTCOMEUNSPECIFIED),
           "BILLABLE_OUTCOME_PAY_PER_IMPRESSION" => Ok(InsertionOrderBillableOutcomeEnum::BILLABLEOUTCOMEPAYPERIMPRESSION),
           "BILLABLE_OUTCOME_PAY_PER_CLICK" => Ok(InsertionOrderBillableOutcomeEnum::BILLABLEOUTCOMEPAYPERCLICK),
           "BILLABLE_OUTCOME_PAY_PER_VIEWABLE_IMPRESSION" => Ok(InsertionOrderBillableOutcomeEnum::BILLABLEOUTCOMEPAYPERVIEWABLEIMPRESSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InsertionOrderBillableOutcomeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InsertionOrderEntityStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Controls whether or not the insertion order can spend its budget and bid on inventory. * For CreateInsertionOrder method, only `ENTITY_STATUS_DRAFT` is allowed. To activate an insertion order, use UpdateInsertionOrder method and update the status to `ENTITY_STATUS_ACTIVE` after creation. * An insertion order cannot be changed back to `ENTITY_STATUS_DRAFT` status from any other status. * An insertion order cannot be set to `ENTITY_STATUS_ACTIVE` if its parent campaign is not active.
pub enum InsertionOrderEntityStatusEnum {
    

    /// Default value when status is not specified or is unknown in this version.
    ///
    /// "ENTITY_STATUS_UNSPECIFIED"
    #[serde(rename="ENTITY_STATUS_UNSPECIFIED")]
    ENTITYSTATUSUNSPECIFIED,
    

    /// The entity is enabled to bid and spend budget.
    ///
    /// "ENTITY_STATUS_ACTIVE"
    #[serde(rename="ENTITY_STATUS_ACTIVE")]
    ENTITYSTATUSACTIVE,
    

    /// The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved.
    ///
    /// "ENTITY_STATUS_ARCHIVED"
    #[serde(rename="ENTITY_STATUS_ARCHIVED")]
    ENTITYSTATUSARCHIVED,
    

    /// The entity is under draft. Bidding and budget spending are disabled.
    ///
    /// "ENTITY_STATUS_DRAFT"
    #[serde(rename="ENTITY_STATUS_DRAFT")]
    ENTITYSTATUSDRAFT,
    

    /// Bidding and budget spending are paused for the entity.
    ///
    /// "ENTITY_STATUS_PAUSED"
    #[serde(rename="ENTITY_STATUS_PAUSED")]
    ENTITYSTATUSPAUSED,
    

    /// The entity is scheduled for deletion.
    ///
    /// "ENTITY_STATUS_SCHEDULED_FOR_DELETION"
    #[serde(rename="ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    ENTITYSTATUSSCHEDULEDFORDELETION,
}

impl AsRef<str> for InsertionOrderEntityStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InsertionOrderEntityStatusEnum::ENTITYSTATUSUNSPECIFIED => "ENTITY_STATUS_UNSPECIFIED",
            InsertionOrderEntityStatusEnum::ENTITYSTATUSACTIVE => "ENTITY_STATUS_ACTIVE",
            InsertionOrderEntityStatusEnum::ENTITYSTATUSARCHIVED => "ENTITY_STATUS_ARCHIVED",
            InsertionOrderEntityStatusEnum::ENTITYSTATUSDRAFT => "ENTITY_STATUS_DRAFT",
            InsertionOrderEntityStatusEnum::ENTITYSTATUSPAUSED => "ENTITY_STATUS_PAUSED",
            InsertionOrderEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION => "ENTITY_STATUS_SCHEDULED_FOR_DELETION",
        }
    }
}

impl std::convert::TryFrom< &str> for InsertionOrderEntityStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_STATUS_UNSPECIFIED" => Ok(InsertionOrderEntityStatusEnum::ENTITYSTATUSUNSPECIFIED),
           "ENTITY_STATUS_ACTIVE" => Ok(InsertionOrderEntityStatusEnum::ENTITYSTATUSACTIVE),
           "ENTITY_STATUS_ARCHIVED" => Ok(InsertionOrderEntityStatusEnum::ENTITYSTATUSARCHIVED),
           "ENTITY_STATUS_DRAFT" => Ok(InsertionOrderEntityStatusEnum::ENTITYSTATUSDRAFT),
           "ENTITY_STATUS_PAUSED" => Ok(InsertionOrderEntityStatusEnum::ENTITYSTATUSPAUSED),
           "ENTITY_STATUS_SCHEDULED_FOR_DELETION" => Ok(InsertionOrderEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InsertionOrderEntityStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InsertionOrderInsertionOrderTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of insertion order. If this field is unspecified in creation, the value defaults to `RTB`.
pub enum InsertionOrderInsertionOrderTypeEnum {
    

    /// Insertion order type is not specified or is unknown.
    ///
    /// "INSERTION_ORDER_TYPE_UNSPECIFIED"
    #[serde(rename="INSERTION_ORDER_TYPE_UNSPECIFIED")]
    INSERTIONORDERTYPEUNSPECIFIED,
    

    /// Real-time bidding.
    ///
    /// "RTB"
    #[serde(rename="RTB")]
    RTB,
    

    /// Over-the-top.
    ///
    /// "OVER_THE_TOP"
    #[serde(rename="OVER_THE_TOP")]
    OVERTHETOP,
}

impl AsRef<str> for InsertionOrderInsertionOrderTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InsertionOrderInsertionOrderTypeEnum::INSERTIONORDERTYPEUNSPECIFIED => "INSERTION_ORDER_TYPE_UNSPECIFIED",
            InsertionOrderInsertionOrderTypeEnum::RTB => "RTB",
            InsertionOrderInsertionOrderTypeEnum::OVERTHETOP => "OVER_THE_TOP",
        }
    }
}

impl std::convert::TryFrom< &str> for InsertionOrderInsertionOrderTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSERTION_ORDER_TYPE_UNSPECIFIED" => Ok(InsertionOrderInsertionOrderTypeEnum::INSERTIONORDERTYPEUNSPECIFIED),
           "RTB" => Ok(InsertionOrderInsertionOrderTypeEnum::RTB),
           "OVER_THE_TOP" => Ok(InsertionOrderInsertionOrderTypeEnum::OVERTHETOP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InsertionOrderInsertionOrderTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InsertionOrderReservationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The reservation type of the insertion order.
pub enum InsertionOrderReservationTypeEnum {
    

    /// Reservation type value is not specified or is unknown in this version.
    ///
    /// "RESERVATION_TYPE_UNSPECIFIED"
    #[serde(rename="RESERVATION_TYPE_UNSPECIFIED")]
    RESERVATIONTYPEUNSPECIFIED,
    

    /// Not created through a guaranteed inventory source.
    ///
    /// "RESERVATION_TYPE_NOT_GUARANTEED"
    #[serde(rename="RESERVATION_TYPE_NOT_GUARANTEED")]
    RESERVATIONTYPENOTGUARANTEED,
    

    /// Created through a programmatic guaranteed inventory source.
    ///
    /// "RESERVATION_TYPE_PROGRAMMATIC_GUARANTEED"
    #[serde(rename="RESERVATION_TYPE_PROGRAMMATIC_GUARANTEED")]
    RESERVATIONTYPEPROGRAMMATICGUARANTEED,
    

    /// Created through a tag guaranteed inventory source.
    ///
    /// "RESERVATION_TYPE_TAG_GUARANTEED"
    #[serde(rename="RESERVATION_TYPE_TAG_GUARANTEED")]
    RESERVATIONTYPETAGGUARANTEED,
}

impl AsRef<str> for InsertionOrderReservationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InsertionOrderReservationTypeEnum::RESERVATIONTYPEUNSPECIFIED => "RESERVATION_TYPE_UNSPECIFIED",
            InsertionOrderReservationTypeEnum::RESERVATIONTYPENOTGUARANTEED => "RESERVATION_TYPE_NOT_GUARANTEED",
            InsertionOrderReservationTypeEnum::RESERVATIONTYPEPROGRAMMATICGUARANTEED => "RESERVATION_TYPE_PROGRAMMATIC_GUARANTEED",
            InsertionOrderReservationTypeEnum::RESERVATIONTYPETAGGUARANTEED => "RESERVATION_TYPE_TAG_GUARANTEED",
        }
    }
}

impl std::convert::TryFrom< &str> for InsertionOrderReservationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESERVATION_TYPE_UNSPECIFIED" => Ok(InsertionOrderReservationTypeEnum::RESERVATIONTYPEUNSPECIFIED),
           "RESERVATION_TYPE_NOT_GUARANTEED" => Ok(InsertionOrderReservationTypeEnum::RESERVATIONTYPENOTGUARANTEED),
           "RESERVATION_TYPE_PROGRAMMATIC_GUARANTEED" => Ok(InsertionOrderReservationTypeEnum::RESERVATIONTYPEPROGRAMMATICGUARANTEED),
           "RESERVATION_TYPE_TAG_GUARANTEED" => Ok(InsertionOrderReservationTypeEnum::RESERVATIONTYPETAGGUARANTEED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InsertionOrderReservationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InsertionOrderBudgetAutomationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of automation used to manage bid and budget for the insertion order. If this field is unspecified in creation, the value defaults to `INSERTION_ORDER_AUTOMATION_TYPE_NONE`.
pub enum InsertionOrderBudgetAutomationTypeEnum {
    

    /// Insertion order automation option is not specified or is unknown in this version.
    ///
    /// "INSERTION_ORDER_AUTOMATION_TYPE_UNSPECIFIED"
    #[serde(rename="INSERTION_ORDER_AUTOMATION_TYPE_UNSPECIFIED")]
    INSERTIONORDERAUTOMATIONTYPEUNSPECIFIED,
    

    /// Automatic budget allocation. Allow the system to automatically shift budget to owning line items to optimize performance defined by kpi. No automation on bid settings.
    ///
    /// "INSERTION_ORDER_AUTOMATION_TYPE_BUDGET"
    #[serde(rename="INSERTION_ORDER_AUTOMATION_TYPE_BUDGET")]
    INSERTIONORDERAUTOMATIONTYPEBUDGET,
    

    /// No automation of bid or budget on insertion order level. Bid and budget must be manually configured at the line item level.
    ///
    /// "INSERTION_ORDER_AUTOMATION_TYPE_NONE"
    #[serde(rename="INSERTION_ORDER_AUTOMATION_TYPE_NONE")]
    INSERTIONORDERAUTOMATIONTYPENONE,
    

    /// Allow the system to automatically adjust bids and shift budget to owning line items to optimize performance defined by kpi.
    ///
    /// "INSERTION_ORDER_AUTOMATION_TYPE_BID_BUDGET"
    #[serde(rename="INSERTION_ORDER_AUTOMATION_TYPE_BID_BUDGET")]
    INSERTIONORDERAUTOMATIONTYPEBIDBUDGET,
}

impl AsRef<str> for InsertionOrderBudgetAutomationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InsertionOrderBudgetAutomationTypeEnum::INSERTIONORDERAUTOMATIONTYPEUNSPECIFIED => "INSERTION_ORDER_AUTOMATION_TYPE_UNSPECIFIED",
            InsertionOrderBudgetAutomationTypeEnum::INSERTIONORDERAUTOMATIONTYPEBUDGET => "INSERTION_ORDER_AUTOMATION_TYPE_BUDGET",
            InsertionOrderBudgetAutomationTypeEnum::INSERTIONORDERAUTOMATIONTYPENONE => "INSERTION_ORDER_AUTOMATION_TYPE_NONE",
            InsertionOrderBudgetAutomationTypeEnum::INSERTIONORDERAUTOMATIONTYPEBIDBUDGET => "INSERTION_ORDER_AUTOMATION_TYPE_BID_BUDGET",
        }
    }
}

impl std::convert::TryFrom< &str> for InsertionOrderBudgetAutomationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSERTION_ORDER_AUTOMATION_TYPE_UNSPECIFIED" => Ok(InsertionOrderBudgetAutomationTypeEnum::INSERTIONORDERAUTOMATIONTYPEUNSPECIFIED),
           "INSERTION_ORDER_AUTOMATION_TYPE_BUDGET" => Ok(InsertionOrderBudgetAutomationTypeEnum::INSERTIONORDERAUTOMATIONTYPEBUDGET),
           "INSERTION_ORDER_AUTOMATION_TYPE_NONE" => Ok(InsertionOrderBudgetAutomationTypeEnum::INSERTIONORDERAUTOMATIONTYPENONE),
           "INSERTION_ORDER_AUTOMATION_TYPE_BID_BUDGET" => Ok(InsertionOrderBudgetAutomationTypeEnum::INSERTIONORDERAUTOMATIONTYPEBIDBUDGET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InsertionOrderBudgetAutomationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InsertionOrderBudgetBudgetUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The budget unit specifies whether the budget is currency based or impression based.
pub enum InsertionOrderBudgetBudgetUnitEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "BUDGET_UNIT_UNSPECIFIED"
    #[serde(rename="BUDGET_UNIT_UNSPECIFIED")]
    BUDGETUNITUNSPECIFIED,
    

    /// Budgeting in currency amounts.
    ///
    /// "BUDGET_UNIT_CURRENCY"
    #[serde(rename="BUDGET_UNIT_CURRENCY")]
    BUDGETUNITCURRENCY,
    

    /// Budgeting in impression amounts.
    ///
    /// "BUDGET_UNIT_IMPRESSIONS"
    #[serde(rename="BUDGET_UNIT_IMPRESSIONS")]
    BUDGETUNITIMPRESSIONS,
}

impl AsRef<str> for InsertionOrderBudgetBudgetUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InsertionOrderBudgetBudgetUnitEnum::BUDGETUNITUNSPECIFIED => "BUDGET_UNIT_UNSPECIFIED",
            InsertionOrderBudgetBudgetUnitEnum::BUDGETUNITCURRENCY => "BUDGET_UNIT_CURRENCY",
            InsertionOrderBudgetBudgetUnitEnum::BUDGETUNITIMPRESSIONS => "BUDGET_UNIT_IMPRESSIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for InsertionOrderBudgetBudgetUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUDGET_UNIT_UNSPECIFIED" => Ok(InsertionOrderBudgetBudgetUnitEnum::BUDGETUNITUNSPECIFIED),
           "BUDGET_UNIT_CURRENCY" => Ok(InsertionOrderBudgetBudgetUnitEnum::BUDGETUNITCURRENCY),
           "BUDGET_UNIT_IMPRESSIONS" => Ok(InsertionOrderBudgetBudgetUnitEnum::BUDGETUNITIMPRESSIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InsertionOrderBudgetBudgetUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceDisplayViewabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Display Viewability section (applicable to display line items only).
pub enum IntegralAdScienceDisplayViewabilityEnum {
    

    /// This enum is only a placeholder and it doesn't specify any display viewability options.
    ///
    /// "PERFORMANCE_VIEWABILITY_UNSPECIFIED"
    #[serde(rename="PERFORMANCE_VIEWABILITY_UNSPECIFIED")]
    PERFORMANCEVIEWABILITYUNSPECIFIED,
    

    /// Target 40% Viewability or Higher.
    ///
    /// "PERFORMANCE_VIEWABILITY_40"
    #[serde(rename="PERFORMANCE_VIEWABILITY_40")]
    PERFORMANCEVIEWABILITY40,
    

    /// Target 50% Viewability or Higher.
    ///
    /// "PERFORMANCE_VIEWABILITY_50"
    #[serde(rename="PERFORMANCE_VIEWABILITY_50")]
    PERFORMANCEVIEWABILITY50,
    

    /// Target 60% Viewability or Higher.
    ///
    /// "PERFORMANCE_VIEWABILITY_60"
    #[serde(rename="PERFORMANCE_VIEWABILITY_60")]
    PERFORMANCEVIEWABILITY60,
    

    /// Target 70% Viewability or Higher.
    ///
    /// "PERFORMANCE_VIEWABILITY_70"
    #[serde(rename="PERFORMANCE_VIEWABILITY_70")]
    PERFORMANCEVIEWABILITY70,
}

impl AsRef<str> for IntegralAdScienceDisplayViewabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceDisplayViewabilityEnum::PERFORMANCEVIEWABILITYUNSPECIFIED => "PERFORMANCE_VIEWABILITY_UNSPECIFIED",
            IntegralAdScienceDisplayViewabilityEnum::PERFORMANCEVIEWABILITY40 => "PERFORMANCE_VIEWABILITY_40",
            IntegralAdScienceDisplayViewabilityEnum::PERFORMANCEVIEWABILITY50 => "PERFORMANCE_VIEWABILITY_50",
            IntegralAdScienceDisplayViewabilityEnum::PERFORMANCEVIEWABILITY60 => "PERFORMANCE_VIEWABILITY_60",
            IntegralAdScienceDisplayViewabilityEnum::PERFORMANCEVIEWABILITY70 => "PERFORMANCE_VIEWABILITY_70",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceDisplayViewabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERFORMANCE_VIEWABILITY_UNSPECIFIED" => Ok(IntegralAdScienceDisplayViewabilityEnum::PERFORMANCEVIEWABILITYUNSPECIFIED),
           "PERFORMANCE_VIEWABILITY_40" => Ok(IntegralAdScienceDisplayViewabilityEnum::PERFORMANCEVIEWABILITY40),
           "PERFORMANCE_VIEWABILITY_50" => Ok(IntegralAdScienceDisplayViewabilityEnum::PERFORMANCEVIEWABILITY50),
           "PERFORMANCE_VIEWABILITY_60" => Ok(IntegralAdScienceDisplayViewabilityEnum::PERFORMANCEVIEWABILITY60),
           "PERFORMANCE_VIEWABILITY_70" => Ok(IntegralAdScienceDisplayViewabilityEnum::PERFORMANCEVIEWABILITY70),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceDisplayViewabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceExcludedAdFraudRiskEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Ad Fraud settings.
pub enum IntegralAdScienceExcludedAdFraudRiskEnum {
    

    /// This enum is only a placeholder and it doesn't specify any ad fraud prevention options.
    ///
    /// "SUSPICIOUS_ACTIVITY_UNSPECIFIED"
    #[serde(rename="SUSPICIOUS_ACTIVITY_UNSPECIFIED")]
    SUSPICIOUSACTIVITYUNSPECIFIED,
    

    /// Ad Fraud - Exclude High Risk.
    ///
    /// "SUSPICIOUS_ACTIVITY_HR"
    #[serde(rename="SUSPICIOUS_ACTIVITY_HR")]
    SUSPICIOUSACTIVITYHR,
    

    /// Ad Fraud - Exclude High and Moderate Risk.
    ///
    /// "SUSPICIOUS_ACTIVITY_HMR"
    #[serde(rename="SUSPICIOUS_ACTIVITY_HMR")]
    SUSPICIOUSACTIVITYHMR,
}

impl AsRef<str> for IntegralAdScienceExcludedAdFraudRiskEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceExcludedAdFraudRiskEnum::SUSPICIOUSACTIVITYUNSPECIFIED => "SUSPICIOUS_ACTIVITY_UNSPECIFIED",
            IntegralAdScienceExcludedAdFraudRiskEnum::SUSPICIOUSACTIVITYHR => "SUSPICIOUS_ACTIVITY_HR",
            IntegralAdScienceExcludedAdFraudRiskEnum::SUSPICIOUSACTIVITYHMR => "SUSPICIOUS_ACTIVITY_HMR",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceExcludedAdFraudRiskEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUSPICIOUS_ACTIVITY_UNSPECIFIED" => Ok(IntegralAdScienceExcludedAdFraudRiskEnum::SUSPICIOUSACTIVITYUNSPECIFIED),
           "SUSPICIOUS_ACTIVITY_HR" => Ok(IntegralAdScienceExcludedAdFraudRiskEnum::SUSPICIOUSACTIVITYHR),
           "SUSPICIOUS_ACTIVITY_HMR" => Ok(IntegralAdScienceExcludedAdFraudRiskEnum::SUSPICIOUSACTIVITYHMR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceExcludedAdFraudRiskEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceExcludedAdultRiskEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Brand Safety - **Adult content**.
pub enum IntegralAdScienceExcludedAdultRiskEnum {
    

    /// This enum is only a placeholder and it doesn't specify any adult options.
    ///
    /// "ADULT_UNSPECIFIED"
    #[serde(rename="ADULT_UNSPECIFIED")]
    ADULTUNSPECIFIED,
    

    /// Adult - Exclude High Risk.
    ///
    /// "ADULT_HR"
    #[serde(rename="ADULT_HR")]
    ADULTHR,
    

    /// Adult - Exclude High and Moderate Risk.
    ///
    /// "ADULT_HMR"
    #[serde(rename="ADULT_HMR")]
    ADULTHMR,
}

impl AsRef<str> for IntegralAdScienceExcludedAdultRiskEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceExcludedAdultRiskEnum::ADULTUNSPECIFIED => "ADULT_UNSPECIFIED",
            IntegralAdScienceExcludedAdultRiskEnum::ADULTHR => "ADULT_HR",
            IntegralAdScienceExcludedAdultRiskEnum::ADULTHMR => "ADULT_HMR",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceExcludedAdultRiskEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADULT_UNSPECIFIED" => Ok(IntegralAdScienceExcludedAdultRiskEnum::ADULTUNSPECIFIED),
           "ADULT_HR" => Ok(IntegralAdScienceExcludedAdultRiskEnum::ADULTHR),
           "ADULT_HMR" => Ok(IntegralAdScienceExcludedAdultRiskEnum::ADULTHMR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceExcludedAdultRiskEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceExcludedAlcoholRiskEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Brand Safety - **Alcohol**.
pub enum IntegralAdScienceExcludedAlcoholRiskEnum {
    

    /// This enum is only a placeholder and it doesn't specify any alcohol options.
    ///
    /// "ALCOHOL_UNSPECIFIED"
    #[serde(rename="ALCOHOL_UNSPECIFIED")]
    ALCOHOLUNSPECIFIED,
    

    /// Alcohol - Exclude High Risk.
    ///
    /// "ALCOHOL_HR"
    #[serde(rename="ALCOHOL_HR")]
    ALCOHOLHR,
    

    /// Alcohol - Exclude High and Moderate Risk.
    ///
    /// "ALCOHOL_HMR"
    #[serde(rename="ALCOHOL_HMR")]
    ALCOHOLHMR,
}

impl AsRef<str> for IntegralAdScienceExcludedAlcoholRiskEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceExcludedAlcoholRiskEnum::ALCOHOLUNSPECIFIED => "ALCOHOL_UNSPECIFIED",
            IntegralAdScienceExcludedAlcoholRiskEnum::ALCOHOLHR => "ALCOHOL_HR",
            IntegralAdScienceExcludedAlcoholRiskEnum::ALCOHOLHMR => "ALCOHOL_HMR",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceExcludedAlcoholRiskEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALCOHOL_UNSPECIFIED" => Ok(IntegralAdScienceExcludedAlcoholRiskEnum::ALCOHOLUNSPECIFIED),
           "ALCOHOL_HR" => Ok(IntegralAdScienceExcludedAlcoholRiskEnum::ALCOHOLHR),
           "ALCOHOL_HMR" => Ok(IntegralAdScienceExcludedAlcoholRiskEnum::ALCOHOLHMR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceExcludedAlcoholRiskEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceExcludedDrugsRiskEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Brand Safety - **Drugs**.
pub enum IntegralAdScienceExcludedDrugsRiskEnum {
    

    /// This enum is only a placeholder and it doesn't specify any drugs options.
    ///
    /// "DRUGS_UNSPECIFIED"
    #[serde(rename="DRUGS_UNSPECIFIED")]
    DRUGSUNSPECIFIED,
    

    /// Drugs - Exclude High Risk.
    ///
    /// "DRUGS_HR"
    #[serde(rename="DRUGS_HR")]
    DRUGSHR,
    

    /// Drugs - Exclude High and Moderate Risk.
    ///
    /// "DRUGS_HMR"
    #[serde(rename="DRUGS_HMR")]
    DRUGSHMR,
}

impl AsRef<str> for IntegralAdScienceExcludedDrugsRiskEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceExcludedDrugsRiskEnum::DRUGSUNSPECIFIED => "DRUGS_UNSPECIFIED",
            IntegralAdScienceExcludedDrugsRiskEnum::DRUGSHR => "DRUGS_HR",
            IntegralAdScienceExcludedDrugsRiskEnum::DRUGSHMR => "DRUGS_HMR",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceExcludedDrugsRiskEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DRUGS_UNSPECIFIED" => Ok(IntegralAdScienceExcludedDrugsRiskEnum::DRUGSUNSPECIFIED),
           "DRUGS_HR" => Ok(IntegralAdScienceExcludedDrugsRiskEnum::DRUGSHR),
           "DRUGS_HMR" => Ok(IntegralAdScienceExcludedDrugsRiskEnum::DRUGSHMR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceExcludedDrugsRiskEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceExcludedGamblingRiskEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Brand Safety - **Gambling**.
pub enum IntegralAdScienceExcludedGamblingRiskEnum {
    

    /// This enum is only a placeholder and it doesn't specify any gambling options.
    ///
    /// "GAMBLING_UNSPECIFIED"
    #[serde(rename="GAMBLING_UNSPECIFIED")]
    GAMBLINGUNSPECIFIED,
    

    /// Gambling - Exclude High Risk.
    ///
    /// "GAMBLING_HR"
    #[serde(rename="GAMBLING_HR")]
    GAMBLINGHR,
    

    /// Gambling - Exclude High and Moderate Risk.
    ///
    /// "GAMBLING_HMR"
    #[serde(rename="GAMBLING_HMR")]
    GAMBLINGHMR,
}

impl AsRef<str> for IntegralAdScienceExcludedGamblingRiskEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceExcludedGamblingRiskEnum::GAMBLINGUNSPECIFIED => "GAMBLING_UNSPECIFIED",
            IntegralAdScienceExcludedGamblingRiskEnum::GAMBLINGHR => "GAMBLING_HR",
            IntegralAdScienceExcludedGamblingRiskEnum::GAMBLINGHMR => "GAMBLING_HMR",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceExcludedGamblingRiskEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GAMBLING_UNSPECIFIED" => Ok(IntegralAdScienceExcludedGamblingRiskEnum::GAMBLINGUNSPECIFIED),
           "GAMBLING_HR" => Ok(IntegralAdScienceExcludedGamblingRiskEnum::GAMBLINGHR),
           "GAMBLING_HMR" => Ok(IntegralAdScienceExcludedGamblingRiskEnum::GAMBLINGHMR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceExcludedGamblingRiskEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceExcludedHateSpeechRiskEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Brand Safety - **Hate speech**.
pub enum IntegralAdScienceExcludedHateSpeechRiskEnum {
    

    /// This enum is only a placeholder and it doesn't specify any hate speech options.
    ///
    /// "HATE_SPEECH_UNSPECIFIED"
    #[serde(rename="HATE_SPEECH_UNSPECIFIED")]
    HATESPEECHUNSPECIFIED,
    

    /// Hate Speech - Exclude High Risk.
    ///
    /// "HATE_SPEECH_HR"
    #[serde(rename="HATE_SPEECH_HR")]
    HATESPEECHHR,
    

    /// Hate Speech - Exclude High and Moderate Risk.
    ///
    /// "HATE_SPEECH_HMR"
    #[serde(rename="HATE_SPEECH_HMR")]
    HATESPEECHHMR,
}

impl AsRef<str> for IntegralAdScienceExcludedHateSpeechRiskEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceExcludedHateSpeechRiskEnum::HATESPEECHUNSPECIFIED => "HATE_SPEECH_UNSPECIFIED",
            IntegralAdScienceExcludedHateSpeechRiskEnum::HATESPEECHHR => "HATE_SPEECH_HR",
            IntegralAdScienceExcludedHateSpeechRiskEnum::HATESPEECHHMR => "HATE_SPEECH_HMR",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceExcludedHateSpeechRiskEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HATE_SPEECH_UNSPECIFIED" => Ok(IntegralAdScienceExcludedHateSpeechRiskEnum::HATESPEECHUNSPECIFIED),
           "HATE_SPEECH_HR" => Ok(IntegralAdScienceExcludedHateSpeechRiskEnum::HATESPEECHHR),
           "HATE_SPEECH_HMR" => Ok(IntegralAdScienceExcludedHateSpeechRiskEnum::HATESPEECHHMR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceExcludedHateSpeechRiskEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceExcludedIllegalDownloadsRiskEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Brand Safety - **Illegal downloads**.
pub enum IntegralAdScienceExcludedIllegalDownloadsRiskEnum {
    

    /// This enum is only a placeholder and it doesn't specify any illegal downloads options.
    ///
    /// "ILLEGAL_DOWNLOADS_UNSPECIFIED"
    #[serde(rename="ILLEGAL_DOWNLOADS_UNSPECIFIED")]
    ILLEGALDOWNLOADSUNSPECIFIED,
    

    /// Illegal Downloads - Exclude High Risk.
    ///
    /// "ILLEGAL_DOWNLOADS_HR"
    #[serde(rename="ILLEGAL_DOWNLOADS_HR")]
    ILLEGALDOWNLOADSHR,
    

    /// Illegal Downloads - Exclude High and Moderate Risk.
    ///
    /// "ILLEGAL_DOWNLOADS_HMR"
    #[serde(rename="ILLEGAL_DOWNLOADS_HMR")]
    ILLEGALDOWNLOADSHMR,
}

impl AsRef<str> for IntegralAdScienceExcludedIllegalDownloadsRiskEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceExcludedIllegalDownloadsRiskEnum::ILLEGALDOWNLOADSUNSPECIFIED => "ILLEGAL_DOWNLOADS_UNSPECIFIED",
            IntegralAdScienceExcludedIllegalDownloadsRiskEnum::ILLEGALDOWNLOADSHR => "ILLEGAL_DOWNLOADS_HR",
            IntegralAdScienceExcludedIllegalDownloadsRiskEnum::ILLEGALDOWNLOADSHMR => "ILLEGAL_DOWNLOADS_HMR",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceExcludedIllegalDownloadsRiskEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ILLEGAL_DOWNLOADS_UNSPECIFIED" => Ok(IntegralAdScienceExcludedIllegalDownloadsRiskEnum::ILLEGALDOWNLOADSUNSPECIFIED),
           "ILLEGAL_DOWNLOADS_HR" => Ok(IntegralAdScienceExcludedIllegalDownloadsRiskEnum::ILLEGALDOWNLOADSHR),
           "ILLEGAL_DOWNLOADS_HMR" => Ok(IntegralAdScienceExcludedIllegalDownloadsRiskEnum::ILLEGALDOWNLOADSHMR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceExcludedIllegalDownloadsRiskEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceExcludedOffensiveLanguageRiskEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Brand Safety - **Offensive language**.
pub enum IntegralAdScienceExcludedOffensiveLanguageRiskEnum {
    

    /// This enum is only a placeholder and it doesn't specify any language options.
    ///
    /// "OFFENSIVE_LANGUAGE_UNSPECIFIED"
    #[serde(rename="OFFENSIVE_LANGUAGE_UNSPECIFIED")]
    OFFENSIVELANGUAGEUNSPECIFIED,
    

    /// Offensive Language - Exclude High Risk.
    ///
    /// "OFFENSIVE_LANGUAGE_HR"
    #[serde(rename="OFFENSIVE_LANGUAGE_HR")]
    OFFENSIVELANGUAGEHR,
    

    /// Offensive Language - Exclude High and Moderate Risk.
    ///
    /// "OFFENSIVE_LANGUAGE_HMR"
    #[serde(rename="OFFENSIVE_LANGUAGE_HMR")]
    OFFENSIVELANGUAGEHMR,
}

impl AsRef<str> for IntegralAdScienceExcludedOffensiveLanguageRiskEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceExcludedOffensiveLanguageRiskEnum::OFFENSIVELANGUAGEUNSPECIFIED => "OFFENSIVE_LANGUAGE_UNSPECIFIED",
            IntegralAdScienceExcludedOffensiveLanguageRiskEnum::OFFENSIVELANGUAGEHR => "OFFENSIVE_LANGUAGE_HR",
            IntegralAdScienceExcludedOffensiveLanguageRiskEnum::OFFENSIVELANGUAGEHMR => "OFFENSIVE_LANGUAGE_HMR",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceExcludedOffensiveLanguageRiskEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFFENSIVE_LANGUAGE_UNSPECIFIED" => Ok(IntegralAdScienceExcludedOffensiveLanguageRiskEnum::OFFENSIVELANGUAGEUNSPECIFIED),
           "OFFENSIVE_LANGUAGE_HR" => Ok(IntegralAdScienceExcludedOffensiveLanguageRiskEnum::OFFENSIVELANGUAGEHR),
           "OFFENSIVE_LANGUAGE_HMR" => Ok(IntegralAdScienceExcludedOffensiveLanguageRiskEnum::OFFENSIVELANGUAGEHMR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceExcludedOffensiveLanguageRiskEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceExcludedViolenceRiskEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Brand Safety - **Violence**.
pub enum IntegralAdScienceExcludedViolenceRiskEnum {
    

    /// This enum is only a placeholder and it doesn't specify any violence options.
    ///
    /// "VIOLENCE_UNSPECIFIED"
    #[serde(rename="VIOLENCE_UNSPECIFIED")]
    VIOLENCEUNSPECIFIED,
    

    /// Violence - Exclude High Risk.
    ///
    /// "VIOLENCE_HR"
    #[serde(rename="VIOLENCE_HR")]
    VIOLENCEHR,
    

    /// Violence - Exclude High and Moderate Risk.
    ///
    /// "VIOLENCE_HMR"
    #[serde(rename="VIOLENCE_HMR")]
    VIOLENCEHMR,
}

impl AsRef<str> for IntegralAdScienceExcludedViolenceRiskEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceExcludedViolenceRiskEnum::VIOLENCEUNSPECIFIED => "VIOLENCE_UNSPECIFIED",
            IntegralAdScienceExcludedViolenceRiskEnum::VIOLENCEHR => "VIOLENCE_HR",
            IntegralAdScienceExcludedViolenceRiskEnum::VIOLENCEHMR => "VIOLENCE_HMR",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceExcludedViolenceRiskEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIOLENCE_UNSPECIFIED" => Ok(IntegralAdScienceExcludedViolenceRiskEnum::VIOLENCEUNSPECIFIED),
           "VIOLENCE_HR" => Ok(IntegralAdScienceExcludedViolenceRiskEnum::VIOLENCEHR),
           "VIOLENCE_HMR" => Ok(IntegralAdScienceExcludedViolenceRiskEnum::VIOLENCEHMR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceExcludedViolenceRiskEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceTraqScoreOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// True advertising quality (applicable to Display line items only).
pub enum IntegralAdScienceTraqScoreOptionEnum {
    

    /// This enum is only a placeholder and it doesn't specify any true advertising quality scores.
    ///
    /// "TRAQ_UNSPECIFIED"
    #[serde(rename="TRAQ_UNSPECIFIED")]
    TRAQUNSPECIFIED,
    

    /// TRAQ score 250-1000.
    ///
    /// "TRAQ_250"
    #[serde(rename="TRAQ_250")]
    TRAQ250,
    

    /// TRAQ score 500-1000.
    ///
    /// "TRAQ_500"
    #[serde(rename="TRAQ_500")]
    TRAQ500,
    

    /// TRAQ score 600-1000.
    ///
    /// "TRAQ_600"
    #[serde(rename="TRAQ_600")]
    TRAQ600,
    

    /// TRAQ score 700-1000.
    ///
    /// "TRAQ_700"
    #[serde(rename="TRAQ_700")]
    TRAQ700,
    

    /// TRAQ score 750-1000.
    ///
    /// "TRAQ_750"
    #[serde(rename="TRAQ_750")]
    TRAQ750,
    

    /// TRAQ score 875-1000.
    ///
    /// "TRAQ_875"
    #[serde(rename="TRAQ_875")]
    TRAQ875,
    

    /// TRAQ score 1000.
    ///
    /// "TRAQ_1000"
    #[serde(rename="TRAQ_1000")]
    TRAQ1000,
}

impl AsRef<str> for IntegralAdScienceTraqScoreOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceTraqScoreOptionEnum::TRAQUNSPECIFIED => "TRAQ_UNSPECIFIED",
            IntegralAdScienceTraqScoreOptionEnum::TRAQ250 => "TRAQ_250",
            IntegralAdScienceTraqScoreOptionEnum::TRAQ500 => "TRAQ_500",
            IntegralAdScienceTraqScoreOptionEnum::TRAQ600 => "TRAQ_600",
            IntegralAdScienceTraqScoreOptionEnum::TRAQ700 => "TRAQ_700",
            IntegralAdScienceTraqScoreOptionEnum::TRAQ750 => "TRAQ_750",
            IntegralAdScienceTraqScoreOptionEnum::TRAQ875 => "TRAQ_875",
            IntegralAdScienceTraqScoreOptionEnum::TRAQ1000 => "TRAQ_1000",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceTraqScoreOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRAQ_UNSPECIFIED" => Ok(IntegralAdScienceTraqScoreOptionEnum::TRAQUNSPECIFIED),
           "TRAQ_250" => Ok(IntegralAdScienceTraqScoreOptionEnum::TRAQ250),
           "TRAQ_500" => Ok(IntegralAdScienceTraqScoreOptionEnum::TRAQ500),
           "TRAQ_600" => Ok(IntegralAdScienceTraqScoreOptionEnum::TRAQ600),
           "TRAQ_700" => Ok(IntegralAdScienceTraqScoreOptionEnum::TRAQ700),
           "TRAQ_750" => Ok(IntegralAdScienceTraqScoreOptionEnum::TRAQ750),
           "TRAQ_875" => Ok(IntegralAdScienceTraqScoreOptionEnum::TRAQ875),
           "TRAQ_1000" => Ok(IntegralAdScienceTraqScoreOptionEnum::TRAQ1000),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceTraqScoreOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntegralAdScienceVideoViewabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Video Viewability Section (applicable to video line items only).
pub enum IntegralAdScienceVideoViewabilityEnum {
    

    /// This enum is only a placeholder and it doesn't specify any video viewability options.
    ///
    /// "VIDEO_VIEWABILITY_UNSPECIFIED"
    #[serde(rename="VIDEO_VIEWABILITY_UNSPECIFIED")]
    VIDEOVIEWABILITYUNSPECIFIED,
    

    /// 40%+ in view (IAB video viewability standard).
    ///
    /// "VIDEO_VIEWABILITY_40"
    #[serde(rename="VIDEO_VIEWABILITY_40")]
    VIDEOVIEWABILITY40,
    

    /// 50%+ in view (IAB video viewability standard).
    ///
    /// "VIDEO_VIEWABILITY_50"
    #[serde(rename="VIDEO_VIEWABILITY_50")]
    VIDEOVIEWABILITY50,
    

    /// 60%+ in view (IAB video viewability standard).
    ///
    /// "VIDEO_VIEWABILITY_60"
    #[serde(rename="VIDEO_VIEWABILITY_60")]
    VIDEOVIEWABILITY60,
    

    /// 70%+ in view (IAB video viewability standard).
    ///
    /// "VIDEO_VIEWABILITY_70"
    #[serde(rename="VIDEO_VIEWABILITY_70")]
    VIDEOVIEWABILITY70,
}

impl AsRef<str> for IntegralAdScienceVideoViewabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntegralAdScienceVideoViewabilityEnum::VIDEOVIEWABILITYUNSPECIFIED => "VIDEO_VIEWABILITY_UNSPECIFIED",
            IntegralAdScienceVideoViewabilityEnum::VIDEOVIEWABILITY40 => "VIDEO_VIEWABILITY_40",
            IntegralAdScienceVideoViewabilityEnum::VIDEOVIEWABILITY50 => "VIDEO_VIEWABILITY_50",
            IntegralAdScienceVideoViewabilityEnum::VIDEOVIEWABILITY60 => "VIDEO_VIEWABILITY_60",
            IntegralAdScienceVideoViewabilityEnum::VIDEOVIEWABILITY70 => "VIDEO_VIEWABILITY_70",
        }
    }
}

impl std::convert::TryFrom< &str> for IntegralAdScienceVideoViewabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIDEO_VIEWABILITY_UNSPECIFIED" => Ok(IntegralAdScienceVideoViewabilityEnum::VIDEOVIEWABILITYUNSPECIFIED),
           "VIDEO_VIEWABILITY_40" => Ok(IntegralAdScienceVideoViewabilityEnum::VIDEOVIEWABILITY40),
           "VIDEO_VIEWABILITY_50" => Ok(IntegralAdScienceVideoViewabilityEnum::VIDEOVIEWABILITY50),
           "VIDEO_VIEWABILITY_60" => Ok(IntegralAdScienceVideoViewabilityEnum::VIDEOVIEWABILITY60),
           "VIDEO_VIEWABILITY_70" => Ok(IntegralAdScienceVideoViewabilityEnum::VIDEOVIEWABILITY70),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntegralAdScienceVideoViewabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventorySourceCommitmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the inventory source has a guaranteed or non-guaranteed delivery.
pub enum InventorySourceCommitmentEnum {
    

    /// The commitment is not specified or is unknown in this version.
    ///
    /// "INVENTORY_SOURCE_COMMITMENT_UNSPECIFIED"
    #[serde(rename="INVENTORY_SOURCE_COMMITMENT_UNSPECIFIED")]
    INVENTORYSOURCECOMMITMENTUNSPECIFIED,
    

    /// The commitment is guaranteed delivery.
    ///
    /// "INVENTORY_SOURCE_COMMITMENT_GUARANTEED"
    #[serde(rename="INVENTORY_SOURCE_COMMITMENT_GUARANTEED")]
    INVENTORYSOURCECOMMITMENTGUARANTEED,
    

    /// The commitment is non-guaranteed delivery.
    ///
    /// "INVENTORY_SOURCE_COMMITMENT_NON_GUARANTEED"
    #[serde(rename="INVENTORY_SOURCE_COMMITMENT_NON_GUARANTEED")]
    INVENTORYSOURCECOMMITMENTNONGUARANTEED,
}

impl AsRef<str> for InventorySourceCommitmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventorySourceCommitmentEnum::INVENTORYSOURCECOMMITMENTUNSPECIFIED => "INVENTORY_SOURCE_COMMITMENT_UNSPECIFIED",
            InventorySourceCommitmentEnum::INVENTORYSOURCECOMMITMENTGUARANTEED => "INVENTORY_SOURCE_COMMITMENT_GUARANTEED",
            InventorySourceCommitmentEnum::INVENTORYSOURCECOMMITMENTNONGUARANTEED => "INVENTORY_SOURCE_COMMITMENT_NON_GUARANTEED",
        }
    }
}

impl std::convert::TryFrom< &str> for InventorySourceCommitmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVENTORY_SOURCE_COMMITMENT_UNSPECIFIED" => Ok(InventorySourceCommitmentEnum::INVENTORYSOURCECOMMITMENTUNSPECIFIED),
           "INVENTORY_SOURCE_COMMITMENT_GUARANTEED" => Ok(InventorySourceCommitmentEnum::INVENTORYSOURCECOMMITMENTGUARANTEED),
           "INVENTORY_SOURCE_COMMITMENT_NON_GUARANTEED" => Ok(InventorySourceCommitmentEnum::INVENTORYSOURCECOMMITMENTNONGUARANTEED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventorySourceCommitmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventorySourceDeliveryMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The delivery method of the inventory source. * For non-guaranteed inventory sources, the only acceptable value is `INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC`. * For guaranteed inventory sources, acceptable values are `INVENTORY_SOURCE_DELIVERY_METHOD_TAG` and `INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC`.
pub enum InventorySourceDeliveryMethodEnum {
    

    /// The delivery method is not specified or is unknown in this version.
    ///
    /// "INVENTORY_SOURCE_DELIVERY_METHOD_UNSPECIFIED"
    #[serde(rename="INVENTORY_SOURCE_DELIVERY_METHOD_UNSPECIFIED")]
    INVENTORYSOURCEDELIVERYMETHODUNSPECIFIED,
    

    /// The delivery method is programmatic.
    ///
    /// "INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC"
    #[serde(rename="INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC")]
    INVENTORYSOURCEDELIVERYMETHODPROGRAMMATIC,
    

    /// The delivery method is tag.
    ///
    /// "INVENTORY_SOURCE_DELIVERY_METHOD_TAG"
    #[serde(rename="INVENTORY_SOURCE_DELIVERY_METHOD_TAG")]
    INVENTORYSOURCEDELIVERYMETHODTAG,
}

impl AsRef<str> for InventorySourceDeliveryMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventorySourceDeliveryMethodEnum::INVENTORYSOURCEDELIVERYMETHODUNSPECIFIED => "INVENTORY_SOURCE_DELIVERY_METHOD_UNSPECIFIED",
            InventorySourceDeliveryMethodEnum::INVENTORYSOURCEDELIVERYMETHODPROGRAMMATIC => "INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC",
            InventorySourceDeliveryMethodEnum::INVENTORYSOURCEDELIVERYMETHODTAG => "INVENTORY_SOURCE_DELIVERY_METHOD_TAG",
        }
    }
}

impl std::convert::TryFrom< &str> for InventorySourceDeliveryMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVENTORY_SOURCE_DELIVERY_METHOD_UNSPECIFIED" => Ok(InventorySourceDeliveryMethodEnum::INVENTORYSOURCEDELIVERYMETHODUNSPECIFIED),
           "INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC" => Ok(InventorySourceDeliveryMethodEnum::INVENTORYSOURCEDELIVERYMETHODPROGRAMMATIC),
           "INVENTORY_SOURCE_DELIVERY_METHOD_TAG" => Ok(InventorySourceDeliveryMethodEnum::INVENTORYSOURCEDELIVERYMETHODTAG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventorySourceDeliveryMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventorySourceExchangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The exchange to which the inventory source belongs.
pub enum InventorySourceExchangeEnum {
    

    /// Exchange is not specified or is unknown in this version.
    ///
    /// "EXCHANGE_UNSPECIFIED"
    #[serde(rename="EXCHANGE_UNSPECIFIED")]
    EXCHANGEUNSPECIFIED,
    

    /// Google Ad Manager.
    ///
    /// "EXCHANGE_GOOGLE_AD_MANAGER"
    #[serde(rename="EXCHANGE_GOOGLE_AD_MANAGER")]
    EXCHANGEGOOGLEADMANAGER,
    

    /// AppNexus.
    ///
    /// "EXCHANGE_APPNEXUS"
    #[serde(rename="EXCHANGE_APPNEXUS")]
    EXCHANGEAPPNEXUS,
    

    /// BrightRoll Exchange for Video from Yahoo!.
    ///
    /// "EXCHANGE_BRIGHTROLL"
    #[serde(rename="EXCHANGE_BRIGHTROLL")]
    EXCHANGEBRIGHTROLL,
    

    /// Adform.
    ///
    /// "EXCHANGE_ADFORM"
    #[serde(rename="EXCHANGE_ADFORM")]
    EXCHANGEADFORM,
    

    /// Admeta.
    ///
    /// "EXCHANGE_ADMETA"
    #[serde(rename="EXCHANGE_ADMETA")]
    EXCHANGEADMETA,
    

    /// Admixer.
    ///
    /// "EXCHANGE_ADMIXER"
    #[serde(rename="EXCHANGE_ADMIXER")]
    EXCHANGEADMIXER,
    

    /// AdsMogo.
    ///
    /// "EXCHANGE_ADSMOGO"
    #[serde(rename="EXCHANGE_ADSMOGO")]
    EXCHANGEADSMOGO,
    

    /// AdsWizz.
    ///
    /// "EXCHANGE_ADSWIZZ"
    #[serde(rename="EXCHANGE_ADSWIZZ")]
    EXCHANGEADSWIZZ,
    

    /// BidSwitch.
    ///
    /// "EXCHANGE_BIDSWITCH"
    #[serde(rename="EXCHANGE_BIDSWITCH")]
    EXCHANGEBIDSWITCH,
    

    /// BrightRoll Exchange for Display from Yahoo!.
    ///
    /// "EXCHANGE_BRIGHTROLL_DISPLAY"
    #[serde(rename="EXCHANGE_BRIGHTROLL_DISPLAY")]
    EXCHANGEBRIGHTROLLDISPLAY,
    

    /// Cadreon.
    ///
    /// "EXCHANGE_CADREON"
    #[serde(rename="EXCHANGE_CADREON")]
    EXCHANGECADREON,
    

    /// Dailymotion.
    ///
    /// "EXCHANGE_DAILYMOTION"
    #[serde(rename="EXCHANGE_DAILYMOTION")]
    EXCHANGEDAILYMOTION,
    

    /// Five.
    ///
    /// "EXCHANGE_FIVE"
    #[serde(rename="EXCHANGE_FIVE")]
    EXCHANGEFIVE,
    

    /// Fluct.
    ///
    /// "EXCHANGE_FLUCT"
    #[serde(rename="EXCHANGE_FLUCT")]
    EXCHANGEFLUCT,
    

    /// FreeWheel SSP.
    ///
    /// "EXCHANGE_FREEWHEEL"
    #[serde(rename="EXCHANGE_FREEWHEEL")]
    EXCHANGEFREEWHEEL,
    

    /// Geniee.
    ///
    /// "EXCHANGE_GENIEE"
    #[serde(rename="EXCHANGE_GENIEE")]
    EXCHANGEGENIEE,
    

    /// GumGum.
    ///
    /// "EXCHANGE_GUMGUM"
    #[serde(rename="EXCHANGE_GUMGUM")]
    EXCHANGEGUMGUM,
    

    /// i-mobile.
    ///
    /// "EXCHANGE_IMOBILE"
    #[serde(rename="EXCHANGE_IMOBILE")]
    EXCHANGEIMOBILE,
    

    /// iBILLBOARD.
    ///
    /// "EXCHANGE_IBILLBOARD"
    #[serde(rename="EXCHANGE_IBILLBOARD")]
    EXCHANGEIBILLBOARD,
    

    /// Improve Digital.
    ///
    /// "EXCHANGE_IMPROVE_DIGITAL"
    #[serde(rename="EXCHANGE_IMPROVE_DIGITAL")]
    EXCHANGEIMPROVEDIGITAL,
    

    /// Index Exchange.
    ///
    /// "EXCHANGE_INDEX"
    #[serde(rename="EXCHANGE_INDEX")]
    EXCHANGEINDEX,
    

    /// Kargo.
    ///
    /// "EXCHANGE_KARGO"
    #[serde(rename="EXCHANGE_KARGO")]
    EXCHANGEKARGO,
    

    /// MicroAd.
    ///
    /// "EXCHANGE_MICROAD"
    #[serde(rename="EXCHANGE_MICROAD")]
    EXCHANGEMICROAD,
    

    /// MoPub.
    ///
    /// "EXCHANGE_MOPUB"
    #[serde(rename="EXCHANGE_MOPUB")]
    EXCHANGEMOPUB,
    

    /// Nend.
    ///
    /// "EXCHANGE_NEND"
    #[serde(rename="EXCHANGE_NEND")]
    EXCHANGENEND,
    

    /// ONE by AOL: Display Market Place.
    ///
    /// "EXCHANGE_ONE_BY_AOL_DISPLAY"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_DISPLAY")]
    EXCHANGEONEBYAOLDISPLAY,
    

    /// ONE by AOL: Mobile.
    ///
    /// "EXCHANGE_ONE_BY_AOL_MOBILE"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_MOBILE")]
    EXCHANGEONEBYAOLMOBILE,
    

    /// ONE by AOL: Video.
    ///
    /// "EXCHANGE_ONE_BY_AOL_VIDEO"
    #[serde(rename="EXCHANGE_ONE_BY_AOL_VIDEO")]
    EXCHANGEONEBYAOLVIDEO,
    

    /// Ooyala.
    ///
    /// "EXCHANGE_OOYALA"
    #[serde(rename="EXCHANGE_OOYALA")]
    EXCHANGEOOYALA,
    

    /// OpenX.
    ///
    /// "EXCHANGE_OPENX"
    #[serde(rename="EXCHANGE_OPENX")]
    EXCHANGEOPENX,
    

    /// Permodo.
    ///
    /// "EXCHANGE_PERMODO"
    #[serde(rename="EXCHANGE_PERMODO")]
    EXCHANGEPERMODO,
    

    /// Platform One.
    ///
    /// "EXCHANGE_PLATFORMONE"
    #[serde(rename="EXCHANGE_PLATFORMONE")]
    EXCHANGEPLATFORMONE,
    

    /// PlatformId.
    ///
    /// "EXCHANGE_PLATFORMID"
    #[serde(rename="EXCHANGE_PLATFORMID")]
    EXCHANGEPLATFORMID,
    

    /// PubMatic.
    ///
    /// "EXCHANGE_PUBMATIC"
    #[serde(rename="EXCHANGE_PUBMATIC")]
    EXCHANGEPUBMATIC,
    

    /// PulsePoint.
    ///
    /// "EXCHANGE_PULSEPOINT"
    #[serde(rename="EXCHANGE_PULSEPOINT")]
    EXCHANGEPULSEPOINT,
    

    /// RevenueMax.
    ///
    /// "EXCHANGE_REVENUEMAX"
    #[serde(rename="EXCHANGE_REVENUEMAX")]
    EXCHANGEREVENUEMAX,
    

    /// Rubicon.
    ///
    /// "EXCHANGE_RUBICON"
    #[serde(rename="EXCHANGE_RUBICON")]
    EXCHANGERUBICON,
    

    /// SmartClip.
    ///
    /// "EXCHANGE_SMARTCLIP"
    #[serde(rename="EXCHANGE_SMARTCLIP")]
    EXCHANGESMARTCLIP,
    

    /// SmartRTB+.
    ///
    /// "EXCHANGE_SMARTRTB"
    #[serde(rename="EXCHANGE_SMARTRTB")]
    EXCHANGESMARTRTB,
    

    /// SmartstreamTv.
    ///
    /// "EXCHANGE_SMARTSTREAMTV"
    #[serde(rename="EXCHANGE_SMARTSTREAMTV")]
    EXCHANGESMARTSTREAMTV,
    

    /// Sovrn.
    ///
    /// "EXCHANGE_SOVRN"
    #[serde(rename="EXCHANGE_SOVRN")]
    EXCHANGESOVRN,
    

    /// SpotXchange.
    ///
    /// "EXCHANGE_SPOTXCHANGE"
    #[serde(rename="EXCHANGE_SPOTXCHANGE")]
    EXCHANGESPOTXCHANGE,
    

    /// Ströer SSP.
    ///
    /// "EXCHANGE_STROER"
    #[serde(rename="EXCHANGE_STROER")]
    EXCHANGESTROER,
    

    /// TeadsTv.
    ///
    /// "EXCHANGE_TEADSTV"
    #[serde(rename="EXCHANGE_TEADSTV")]
    EXCHANGETEADSTV,
    

    /// Telaria.
    ///
    /// "EXCHANGE_TELARIA"
    #[serde(rename="EXCHANGE_TELARIA")]
    EXCHANGETELARIA,
    

    /// TVN.
    ///
    /// "EXCHANGE_TVN"
    #[serde(rename="EXCHANGE_TVN")]
    EXCHANGETVN,
    

    /// United.
    ///
    /// "EXCHANGE_UNITED"
    #[serde(rename="EXCHANGE_UNITED")]
    EXCHANGEUNITED,
    

    /// Yieldlab.
    ///
    /// "EXCHANGE_YIELDLAB"
    #[serde(rename="EXCHANGE_YIELDLAB")]
    EXCHANGEYIELDLAB,
    

    /// Yieldmo.
    ///
    /// "EXCHANGE_YIELDMO"
    #[serde(rename="EXCHANGE_YIELDMO")]
    EXCHANGEYIELDMO,
    

    /// UnrulyX.
    ///
    /// "EXCHANGE_UNRULYX"
    #[serde(rename="EXCHANGE_UNRULYX")]
    EXCHANGEUNRULYX,
    

    /// Open8.
    ///
    /// "EXCHANGE_OPEN8"
    #[serde(rename="EXCHANGE_OPEN8")]
    EXCHANGEOPEN8,
    

    /// Triton.
    ///
    /// "EXCHANGE_TRITON"
    #[serde(rename="EXCHANGE_TRITON")]
    EXCHANGETRITON,
    

    /// TripleLift.
    ///
    /// "EXCHANGE_TRIPLELIFT"
    #[serde(rename="EXCHANGE_TRIPLELIFT")]
    EXCHANGETRIPLELIFT,
    

    /// Taboola.
    ///
    /// "EXCHANGE_TABOOLA"
    #[serde(rename="EXCHANGE_TABOOLA")]
    EXCHANGETABOOLA,
    

    /// InMobi.
    ///
    /// "EXCHANGE_INMOBI"
    #[serde(rename="EXCHANGE_INMOBI")]
    EXCHANGEINMOBI,
    

    /// Smaato.
    ///
    /// "EXCHANGE_SMAATO"
    #[serde(rename="EXCHANGE_SMAATO")]
    EXCHANGESMAATO,
    

    /// Aja.
    ///
    /// "EXCHANGE_AJA"
    #[serde(rename="EXCHANGE_AJA")]
    EXCHANGEAJA,
    

    /// Supership.
    ///
    /// "EXCHANGE_SUPERSHIP"
    #[serde(rename="EXCHANGE_SUPERSHIP")]
    EXCHANGESUPERSHIP,
    

    /// Nexstar Digital.
    ///
    /// "EXCHANGE_NEXSTAR_DIGITAL"
    #[serde(rename="EXCHANGE_NEXSTAR_DIGITAL")]
    EXCHANGENEXSTARDIGITAL,
    

    /// Waze.
    ///
    /// "EXCHANGE_WAZE"
    #[serde(rename="EXCHANGE_WAZE")]
    EXCHANGEWAZE,
    

    /// SoundCast.
    ///
    /// "EXCHANGE_SOUNDCAST"
    #[serde(rename="EXCHANGE_SOUNDCAST")]
    EXCHANGESOUNDCAST,
    

    /// Sharethrough.
    ///
    /// "EXCHANGE_SHARETHROUGH"
    #[serde(rename="EXCHANGE_SHARETHROUGH")]
    EXCHANGESHARETHROUGH,
    

    /// Fyber.
    ///
    /// "EXCHANGE_FYBER"
    #[serde(rename="EXCHANGE_FYBER")]
    EXCHANGEFYBER,
    

    /// Red For Publishers.
    ///
    /// "EXCHANGE_RED_FOR_PUBLISHERS"
    #[serde(rename="EXCHANGE_RED_FOR_PUBLISHERS")]
    EXCHANGEREDFORPUBLISHERS,
    

    /// Media.net.
    ///
    /// "EXCHANGE_MEDIANET"
    #[serde(rename="EXCHANGE_MEDIANET")]
    EXCHANGEMEDIANET,
    

    /// Tapjoy.
    ///
    /// "EXCHANGE_TAPJOY"
    #[serde(rename="EXCHANGE_TAPJOY")]
    EXCHANGETAPJOY,
    

    /// Vistar.
    ///
    /// "EXCHANGE_VISTAR"
    #[serde(rename="EXCHANGE_VISTAR")]
    EXCHANGEVISTAR,
    

    /// DAX.
    ///
    /// "EXCHANGE_DAX"
    #[serde(rename="EXCHANGE_DAX")]
    EXCHANGEDAX,
    

    /// JCD.
    ///
    /// "EXCHANGE_JCD"
    #[serde(rename="EXCHANGE_JCD")]
    EXCHANGEJCD,
    

    /// Place Exchange.
    ///
    /// "EXCHANGE_PLACE_EXCHANGE"
    #[serde(rename="EXCHANGE_PLACE_EXCHANGE")]
    EXCHANGEPLACEEXCHANGE,
    

    /// AppLovin.
    ///
    /// "EXCHANGE_APPLOVIN"
    #[serde(rename="EXCHANGE_APPLOVIN")]
    EXCHANGEAPPLOVIN,
    

    /// Connatix.
    ///
    /// "EXCHANGE_CONNATIX"
    #[serde(rename="EXCHANGE_CONNATIX")]
    EXCHANGECONNATIX,
    

    /// Reset Digital.
    ///
    /// "EXCHANGE_RESET_DIGITAL"
    #[serde(rename="EXCHANGE_RESET_DIGITAL")]
    EXCHANGERESETDIGITAL,
    

    /// Hivestack.
    ///
    /// "EXCHANGE_HIVESTACK"
    #[serde(rename="EXCHANGE_HIVESTACK")]
    EXCHANGEHIVESTACK,
    

    /// Drax.
    ///
    /// "EXCHANGE_DRAX"
    #[serde(rename="EXCHANGE_DRAX")]
    EXCHANGEDRAX,
    

    /// AppLovin MAX.
    ///
    /// "EXCHANGE_APPLOVIN_GBID"
    #[serde(rename="EXCHANGE_APPLOVIN_GBID")]
    EXCHANGEAPPLOVINGBID,
    

    /// DT Fairbid.
    ///
    /// "EXCHANGE_FYBER_GBID"
    #[serde(rename="EXCHANGE_FYBER_GBID")]
    EXCHANGEFYBERGBID,
    

    /// Unity LevelPlay.
    ///
    /// "EXCHANGE_UNITY_GBID"
    #[serde(rename="EXCHANGE_UNITY_GBID")]
    EXCHANGEUNITYGBID,
    

    /// Chartboost Mediation.
    ///
    /// "EXCHANGE_CHARTBOOST_GBID"
    #[serde(rename="EXCHANGE_CHARTBOOST_GBID")]
    EXCHANGECHARTBOOSTGBID,
    

    /// AdMost.
    ///
    /// "EXCHANGE_ADMOST_GBID"
    #[serde(rename="EXCHANGE_ADMOST_GBID")]
    EXCHANGEADMOSTGBID,
    

    /// TopOn.
    ///
    /// "EXCHANGE_TOPON_GBID"
    #[serde(rename="EXCHANGE_TOPON_GBID")]
    EXCHANGETOPONGBID,
}

impl AsRef<str> for InventorySourceExchangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventorySourceExchangeEnum::EXCHANGEUNSPECIFIED => "EXCHANGE_UNSPECIFIED",
            InventorySourceExchangeEnum::EXCHANGEGOOGLEADMANAGER => "EXCHANGE_GOOGLE_AD_MANAGER",
            InventorySourceExchangeEnum::EXCHANGEAPPNEXUS => "EXCHANGE_APPNEXUS",
            InventorySourceExchangeEnum::EXCHANGEBRIGHTROLL => "EXCHANGE_BRIGHTROLL",
            InventorySourceExchangeEnum::EXCHANGEADFORM => "EXCHANGE_ADFORM",
            InventorySourceExchangeEnum::EXCHANGEADMETA => "EXCHANGE_ADMETA",
            InventorySourceExchangeEnum::EXCHANGEADMIXER => "EXCHANGE_ADMIXER",
            InventorySourceExchangeEnum::EXCHANGEADSMOGO => "EXCHANGE_ADSMOGO",
            InventorySourceExchangeEnum::EXCHANGEADSWIZZ => "EXCHANGE_ADSWIZZ",
            InventorySourceExchangeEnum::EXCHANGEBIDSWITCH => "EXCHANGE_BIDSWITCH",
            InventorySourceExchangeEnum::EXCHANGEBRIGHTROLLDISPLAY => "EXCHANGE_BRIGHTROLL_DISPLAY",
            InventorySourceExchangeEnum::EXCHANGECADREON => "EXCHANGE_CADREON",
            InventorySourceExchangeEnum::EXCHANGEDAILYMOTION => "EXCHANGE_DAILYMOTION",
            InventorySourceExchangeEnum::EXCHANGEFIVE => "EXCHANGE_FIVE",
            InventorySourceExchangeEnum::EXCHANGEFLUCT => "EXCHANGE_FLUCT",
            InventorySourceExchangeEnum::EXCHANGEFREEWHEEL => "EXCHANGE_FREEWHEEL",
            InventorySourceExchangeEnum::EXCHANGEGENIEE => "EXCHANGE_GENIEE",
            InventorySourceExchangeEnum::EXCHANGEGUMGUM => "EXCHANGE_GUMGUM",
            InventorySourceExchangeEnum::EXCHANGEIMOBILE => "EXCHANGE_IMOBILE",
            InventorySourceExchangeEnum::EXCHANGEIBILLBOARD => "EXCHANGE_IBILLBOARD",
            InventorySourceExchangeEnum::EXCHANGEIMPROVEDIGITAL => "EXCHANGE_IMPROVE_DIGITAL",
            InventorySourceExchangeEnum::EXCHANGEINDEX => "EXCHANGE_INDEX",
            InventorySourceExchangeEnum::EXCHANGEKARGO => "EXCHANGE_KARGO",
            InventorySourceExchangeEnum::EXCHANGEMICROAD => "EXCHANGE_MICROAD",
            InventorySourceExchangeEnum::EXCHANGEMOPUB => "EXCHANGE_MOPUB",
            InventorySourceExchangeEnum::EXCHANGENEND => "EXCHANGE_NEND",
            InventorySourceExchangeEnum::EXCHANGEONEBYAOLDISPLAY => "EXCHANGE_ONE_BY_AOL_DISPLAY",
            InventorySourceExchangeEnum::EXCHANGEONEBYAOLMOBILE => "EXCHANGE_ONE_BY_AOL_MOBILE",
            InventorySourceExchangeEnum::EXCHANGEONEBYAOLVIDEO => "EXCHANGE_ONE_BY_AOL_VIDEO",
            InventorySourceExchangeEnum::EXCHANGEOOYALA => "EXCHANGE_OOYALA",
            InventorySourceExchangeEnum::EXCHANGEOPENX => "EXCHANGE_OPENX",
            InventorySourceExchangeEnum::EXCHANGEPERMODO => "EXCHANGE_PERMODO",
            InventorySourceExchangeEnum::EXCHANGEPLATFORMONE => "EXCHANGE_PLATFORMONE",
            InventorySourceExchangeEnum::EXCHANGEPLATFORMID => "EXCHANGE_PLATFORMID",
            InventorySourceExchangeEnum::EXCHANGEPUBMATIC => "EXCHANGE_PUBMATIC",
            InventorySourceExchangeEnum::EXCHANGEPULSEPOINT => "EXCHANGE_PULSEPOINT",
            InventorySourceExchangeEnum::EXCHANGEREVENUEMAX => "EXCHANGE_REVENUEMAX",
            InventorySourceExchangeEnum::EXCHANGERUBICON => "EXCHANGE_RUBICON",
            InventorySourceExchangeEnum::EXCHANGESMARTCLIP => "EXCHANGE_SMARTCLIP",
            InventorySourceExchangeEnum::EXCHANGESMARTRTB => "EXCHANGE_SMARTRTB",
            InventorySourceExchangeEnum::EXCHANGESMARTSTREAMTV => "EXCHANGE_SMARTSTREAMTV",
            InventorySourceExchangeEnum::EXCHANGESOVRN => "EXCHANGE_SOVRN",
            InventorySourceExchangeEnum::EXCHANGESPOTXCHANGE => "EXCHANGE_SPOTXCHANGE",
            InventorySourceExchangeEnum::EXCHANGESTROER => "EXCHANGE_STROER",
            InventorySourceExchangeEnum::EXCHANGETEADSTV => "EXCHANGE_TEADSTV",
            InventorySourceExchangeEnum::EXCHANGETELARIA => "EXCHANGE_TELARIA",
            InventorySourceExchangeEnum::EXCHANGETVN => "EXCHANGE_TVN",
            InventorySourceExchangeEnum::EXCHANGEUNITED => "EXCHANGE_UNITED",
            InventorySourceExchangeEnum::EXCHANGEYIELDLAB => "EXCHANGE_YIELDLAB",
            InventorySourceExchangeEnum::EXCHANGEYIELDMO => "EXCHANGE_YIELDMO",
            InventorySourceExchangeEnum::EXCHANGEUNRULYX => "EXCHANGE_UNRULYX",
            InventorySourceExchangeEnum::EXCHANGEOPEN8 => "EXCHANGE_OPEN8",
            InventorySourceExchangeEnum::EXCHANGETRITON => "EXCHANGE_TRITON",
            InventorySourceExchangeEnum::EXCHANGETRIPLELIFT => "EXCHANGE_TRIPLELIFT",
            InventorySourceExchangeEnum::EXCHANGETABOOLA => "EXCHANGE_TABOOLA",
            InventorySourceExchangeEnum::EXCHANGEINMOBI => "EXCHANGE_INMOBI",
            InventorySourceExchangeEnum::EXCHANGESMAATO => "EXCHANGE_SMAATO",
            InventorySourceExchangeEnum::EXCHANGEAJA => "EXCHANGE_AJA",
            InventorySourceExchangeEnum::EXCHANGESUPERSHIP => "EXCHANGE_SUPERSHIP",
            InventorySourceExchangeEnum::EXCHANGENEXSTARDIGITAL => "EXCHANGE_NEXSTAR_DIGITAL",
            InventorySourceExchangeEnum::EXCHANGEWAZE => "EXCHANGE_WAZE",
            InventorySourceExchangeEnum::EXCHANGESOUNDCAST => "EXCHANGE_SOUNDCAST",
            InventorySourceExchangeEnum::EXCHANGESHARETHROUGH => "EXCHANGE_SHARETHROUGH",
            InventorySourceExchangeEnum::EXCHANGEFYBER => "EXCHANGE_FYBER",
            InventorySourceExchangeEnum::EXCHANGEREDFORPUBLISHERS => "EXCHANGE_RED_FOR_PUBLISHERS",
            InventorySourceExchangeEnum::EXCHANGEMEDIANET => "EXCHANGE_MEDIANET",
            InventorySourceExchangeEnum::EXCHANGETAPJOY => "EXCHANGE_TAPJOY",
            InventorySourceExchangeEnum::EXCHANGEVISTAR => "EXCHANGE_VISTAR",
            InventorySourceExchangeEnum::EXCHANGEDAX => "EXCHANGE_DAX",
            InventorySourceExchangeEnum::EXCHANGEJCD => "EXCHANGE_JCD",
            InventorySourceExchangeEnum::EXCHANGEPLACEEXCHANGE => "EXCHANGE_PLACE_EXCHANGE",
            InventorySourceExchangeEnum::EXCHANGEAPPLOVIN => "EXCHANGE_APPLOVIN",
            InventorySourceExchangeEnum::EXCHANGECONNATIX => "EXCHANGE_CONNATIX",
            InventorySourceExchangeEnum::EXCHANGERESETDIGITAL => "EXCHANGE_RESET_DIGITAL",
            InventorySourceExchangeEnum::EXCHANGEHIVESTACK => "EXCHANGE_HIVESTACK",
            InventorySourceExchangeEnum::EXCHANGEDRAX => "EXCHANGE_DRAX",
            InventorySourceExchangeEnum::EXCHANGEAPPLOVINGBID => "EXCHANGE_APPLOVIN_GBID",
            InventorySourceExchangeEnum::EXCHANGEFYBERGBID => "EXCHANGE_FYBER_GBID",
            InventorySourceExchangeEnum::EXCHANGEUNITYGBID => "EXCHANGE_UNITY_GBID",
            InventorySourceExchangeEnum::EXCHANGECHARTBOOSTGBID => "EXCHANGE_CHARTBOOST_GBID",
            InventorySourceExchangeEnum::EXCHANGEADMOSTGBID => "EXCHANGE_ADMOST_GBID",
            InventorySourceExchangeEnum::EXCHANGETOPONGBID => "EXCHANGE_TOPON_GBID",
        }
    }
}

impl std::convert::TryFrom< &str> for InventorySourceExchangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCHANGE_UNSPECIFIED" => Ok(InventorySourceExchangeEnum::EXCHANGEUNSPECIFIED),
           "EXCHANGE_GOOGLE_AD_MANAGER" => Ok(InventorySourceExchangeEnum::EXCHANGEGOOGLEADMANAGER),
           "EXCHANGE_APPNEXUS" => Ok(InventorySourceExchangeEnum::EXCHANGEAPPNEXUS),
           "EXCHANGE_BRIGHTROLL" => Ok(InventorySourceExchangeEnum::EXCHANGEBRIGHTROLL),
           "EXCHANGE_ADFORM" => Ok(InventorySourceExchangeEnum::EXCHANGEADFORM),
           "EXCHANGE_ADMETA" => Ok(InventorySourceExchangeEnum::EXCHANGEADMETA),
           "EXCHANGE_ADMIXER" => Ok(InventorySourceExchangeEnum::EXCHANGEADMIXER),
           "EXCHANGE_ADSMOGO" => Ok(InventorySourceExchangeEnum::EXCHANGEADSMOGO),
           "EXCHANGE_ADSWIZZ" => Ok(InventorySourceExchangeEnum::EXCHANGEADSWIZZ),
           "EXCHANGE_BIDSWITCH" => Ok(InventorySourceExchangeEnum::EXCHANGEBIDSWITCH),
           "EXCHANGE_BRIGHTROLL_DISPLAY" => Ok(InventorySourceExchangeEnum::EXCHANGEBRIGHTROLLDISPLAY),
           "EXCHANGE_CADREON" => Ok(InventorySourceExchangeEnum::EXCHANGECADREON),
           "EXCHANGE_DAILYMOTION" => Ok(InventorySourceExchangeEnum::EXCHANGEDAILYMOTION),
           "EXCHANGE_FIVE" => Ok(InventorySourceExchangeEnum::EXCHANGEFIVE),
           "EXCHANGE_FLUCT" => Ok(InventorySourceExchangeEnum::EXCHANGEFLUCT),
           "EXCHANGE_FREEWHEEL" => Ok(InventorySourceExchangeEnum::EXCHANGEFREEWHEEL),
           "EXCHANGE_GENIEE" => Ok(InventorySourceExchangeEnum::EXCHANGEGENIEE),
           "EXCHANGE_GUMGUM" => Ok(InventorySourceExchangeEnum::EXCHANGEGUMGUM),
           "EXCHANGE_IMOBILE" => Ok(InventorySourceExchangeEnum::EXCHANGEIMOBILE),
           "EXCHANGE_IBILLBOARD" => Ok(InventorySourceExchangeEnum::EXCHANGEIBILLBOARD),
           "EXCHANGE_IMPROVE_DIGITAL" => Ok(InventorySourceExchangeEnum::EXCHANGEIMPROVEDIGITAL),
           "EXCHANGE_INDEX" => Ok(InventorySourceExchangeEnum::EXCHANGEINDEX),
           "EXCHANGE_KARGO" => Ok(InventorySourceExchangeEnum::EXCHANGEKARGO),
           "EXCHANGE_MICROAD" => Ok(InventorySourceExchangeEnum::EXCHANGEMICROAD),
           "EXCHANGE_MOPUB" => Ok(InventorySourceExchangeEnum::EXCHANGEMOPUB),
           "EXCHANGE_NEND" => Ok(InventorySourceExchangeEnum::EXCHANGENEND),
           "EXCHANGE_ONE_BY_AOL_DISPLAY" => Ok(InventorySourceExchangeEnum::EXCHANGEONEBYAOLDISPLAY),
           "EXCHANGE_ONE_BY_AOL_MOBILE" => Ok(InventorySourceExchangeEnum::EXCHANGEONEBYAOLMOBILE),
           "EXCHANGE_ONE_BY_AOL_VIDEO" => Ok(InventorySourceExchangeEnum::EXCHANGEONEBYAOLVIDEO),
           "EXCHANGE_OOYALA" => Ok(InventorySourceExchangeEnum::EXCHANGEOOYALA),
           "EXCHANGE_OPENX" => Ok(InventorySourceExchangeEnum::EXCHANGEOPENX),
           "EXCHANGE_PERMODO" => Ok(InventorySourceExchangeEnum::EXCHANGEPERMODO),
           "EXCHANGE_PLATFORMONE" => Ok(InventorySourceExchangeEnum::EXCHANGEPLATFORMONE),
           "EXCHANGE_PLATFORMID" => Ok(InventorySourceExchangeEnum::EXCHANGEPLATFORMID),
           "EXCHANGE_PUBMATIC" => Ok(InventorySourceExchangeEnum::EXCHANGEPUBMATIC),
           "EXCHANGE_PULSEPOINT" => Ok(InventorySourceExchangeEnum::EXCHANGEPULSEPOINT),
           "EXCHANGE_REVENUEMAX" => Ok(InventorySourceExchangeEnum::EXCHANGEREVENUEMAX),
           "EXCHANGE_RUBICON" => Ok(InventorySourceExchangeEnum::EXCHANGERUBICON),
           "EXCHANGE_SMARTCLIP" => Ok(InventorySourceExchangeEnum::EXCHANGESMARTCLIP),
           "EXCHANGE_SMARTRTB" => Ok(InventorySourceExchangeEnum::EXCHANGESMARTRTB),
           "EXCHANGE_SMARTSTREAMTV" => Ok(InventorySourceExchangeEnum::EXCHANGESMARTSTREAMTV),
           "EXCHANGE_SOVRN" => Ok(InventorySourceExchangeEnum::EXCHANGESOVRN),
           "EXCHANGE_SPOTXCHANGE" => Ok(InventorySourceExchangeEnum::EXCHANGESPOTXCHANGE),
           "EXCHANGE_STROER" => Ok(InventorySourceExchangeEnum::EXCHANGESTROER),
           "EXCHANGE_TEADSTV" => Ok(InventorySourceExchangeEnum::EXCHANGETEADSTV),
           "EXCHANGE_TELARIA" => Ok(InventorySourceExchangeEnum::EXCHANGETELARIA),
           "EXCHANGE_TVN" => Ok(InventorySourceExchangeEnum::EXCHANGETVN),
           "EXCHANGE_UNITED" => Ok(InventorySourceExchangeEnum::EXCHANGEUNITED),
           "EXCHANGE_YIELDLAB" => Ok(InventorySourceExchangeEnum::EXCHANGEYIELDLAB),
           "EXCHANGE_YIELDMO" => Ok(InventorySourceExchangeEnum::EXCHANGEYIELDMO),
           "EXCHANGE_UNRULYX" => Ok(InventorySourceExchangeEnum::EXCHANGEUNRULYX),
           "EXCHANGE_OPEN8" => Ok(InventorySourceExchangeEnum::EXCHANGEOPEN8),
           "EXCHANGE_TRITON" => Ok(InventorySourceExchangeEnum::EXCHANGETRITON),
           "EXCHANGE_TRIPLELIFT" => Ok(InventorySourceExchangeEnum::EXCHANGETRIPLELIFT),
           "EXCHANGE_TABOOLA" => Ok(InventorySourceExchangeEnum::EXCHANGETABOOLA),
           "EXCHANGE_INMOBI" => Ok(InventorySourceExchangeEnum::EXCHANGEINMOBI),
           "EXCHANGE_SMAATO" => Ok(InventorySourceExchangeEnum::EXCHANGESMAATO),
           "EXCHANGE_AJA" => Ok(InventorySourceExchangeEnum::EXCHANGEAJA),
           "EXCHANGE_SUPERSHIP" => Ok(InventorySourceExchangeEnum::EXCHANGESUPERSHIP),
           "EXCHANGE_NEXSTAR_DIGITAL" => Ok(InventorySourceExchangeEnum::EXCHANGENEXSTARDIGITAL),
           "EXCHANGE_WAZE" => Ok(InventorySourceExchangeEnum::EXCHANGEWAZE),
           "EXCHANGE_SOUNDCAST" => Ok(InventorySourceExchangeEnum::EXCHANGESOUNDCAST),
           "EXCHANGE_SHARETHROUGH" => Ok(InventorySourceExchangeEnum::EXCHANGESHARETHROUGH),
           "EXCHANGE_FYBER" => Ok(InventorySourceExchangeEnum::EXCHANGEFYBER),
           "EXCHANGE_RED_FOR_PUBLISHERS" => Ok(InventorySourceExchangeEnum::EXCHANGEREDFORPUBLISHERS),
           "EXCHANGE_MEDIANET" => Ok(InventorySourceExchangeEnum::EXCHANGEMEDIANET),
           "EXCHANGE_TAPJOY" => Ok(InventorySourceExchangeEnum::EXCHANGETAPJOY),
           "EXCHANGE_VISTAR" => Ok(InventorySourceExchangeEnum::EXCHANGEVISTAR),
           "EXCHANGE_DAX" => Ok(InventorySourceExchangeEnum::EXCHANGEDAX),
           "EXCHANGE_JCD" => Ok(InventorySourceExchangeEnum::EXCHANGEJCD),
           "EXCHANGE_PLACE_EXCHANGE" => Ok(InventorySourceExchangeEnum::EXCHANGEPLACEEXCHANGE),
           "EXCHANGE_APPLOVIN" => Ok(InventorySourceExchangeEnum::EXCHANGEAPPLOVIN),
           "EXCHANGE_CONNATIX" => Ok(InventorySourceExchangeEnum::EXCHANGECONNATIX),
           "EXCHANGE_RESET_DIGITAL" => Ok(InventorySourceExchangeEnum::EXCHANGERESETDIGITAL),
           "EXCHANGE_HIVESTACK" => Ok(InventorySourceExchangeEnum::EXCHANGEHIVESTACK),
           "EXCHANGE_DRAX" => Ok(InventorySourceExchangeEnum::EXCHANGEDRAX),
           "EXCHANGE_APPLOVIN_GBID" => Ok(InventorySourceExchangeEnum::EXCHANGEAPPLOVINGBID),
           "EXCHANGE_FYBER_GBID" => Ok(InventorySourceExchangeEnum::EXCHANGEFYBERGBID),
           "EXCHANGE_UNITY_GBID" => Ok(InventorySourceExchangeEnum::EXCHANGEUNITYGBID),
           "EXCHANGE_CHARTBOOST_GBID" => Ok(InventorySourceExchangeEnum::EXCHANGECHARTBOOSTGBID),
           "EXCHANGE_ADMOST_GBID" => Ok(InventorySourceExchangeEnum::EXCHANGEADMOSTGBID),
           "EXCHANGE_TOPON_GBID" => Ok(InventorySourceExchangeEnum::EXCHANGETOPONGBID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventorySourceExchangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventorySourceInventorySourceProductTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The product type of the inventory source, denoting the way through which it sells inventory.
pub enum InventorySourceInventorySourceProductTypeEnum {
    

    /// The product type is not specified or is unknown in this version. Modifying inventory sources of this product type are not supported via API.
    ///
    /// "INVENTORY_SOURCE_PRODUCT_TYPE_UNSPECIFIED"
    #[serde(rename="INVENTORY_SOURCE_PRODUCT_TYPE_UNSPECIFIED")]
    INVENTORYSOURCEPRODUCTTYPEUNSPECIFIED,
    

    /// The inventory source sells inventory through Preferred Deal.
    ///
    /// "PREFERRED_DEAL"
    #[serde(rename="PREFERRED_DEAL")]
    PREFERREDDEAL,
    

    /// The inventory source sells inventory through Private Auction.
    ///
    /// "PRIVATE_AUCTION"
    #[serde(rename="PRIVATE_AUCTION")]
    PRIVATEAUCTION,
    

    /// The inventory source sells inventory through Programmatic Guaranteed.
    ///
    /// "PROGRAMMATIC_GUARANTEED"
    #[serde(rename="PROGRAMMATIC_GUARANTEED")]
    PROGRAMMATICGUARANTEED,
    

    /// The inventory source sells inventory through Tag Guaranteed.
    ///
    /// "TAG_GUARANTEED"
    #[serde(rename="TAG_GUARANTEED")]
    TAGGUARANTEED,
    

    /// The inventory source sells inventory through YouTube Reserve.
    ///
    /// "YOUTUBE_RESERVE"
    #[serde(rename="YOUTUBE_RESERVE")]
    YOUTUBERESERVE,
    

    /// The inventory source sells inventory through Instant Reserve. Modifying inventory sources of this product type are not supported via API.
    ///
    /// "INSTANT_RESERVE"
    #[serde(rename="INSTANT_RESERVE")]
    INSTANTRESERVE,
    

    /// The inventory source sells inventory through Guaranteed Package. Modifying inventory sources of this product type are not supported via API.
    ///
    /// "GUARANTEED_PACKAGE"
    #[serde(rename="GUARANTEED_PACKAGE")]
    GUARANTEEDPACKAGE,
    

    /// The inventory source sells inventory through Programmtic TV. Modifying inventory sources of this product type are not supported via API.
    ///
    /// "PROGRAMMATIC_TV"
    #[serde(rename="PROGRAMMATIC_TV")]
    PROGRAMMATICTV,
    

    /// The inventory source sells inventory through Auction Package. Modifying inventory sources of this product type are not supported via API.
    ///
    /// "AUCTION_PACKAGE"
    #[serde(rename="AUCTION_PACKAGE")]
    AUCTIONPACKAGE,
}

impl AsRef<str> for InventorySourceInventorySourceProductTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventorySourceInventorySourceProductTypeEnum::INVENTORYSOURCEPRODUCTTYPEUNSPECIFIED => "INVENTORY_SOURCE_PRODUCT_TYPE_UNSPECIFIED",
            InventorySourceInventorySourceProductTypeEnum::PREFERREDDEAL => "PREFERRED_DEAL",
            InventorySourceInventorySourceProductTypeEnum::PRIVATEAUCTION => "PRIVATE_AUCTION",
            InventorySourceInventorySourceProductTypeEnum::PROGRAMMATICGUARANTEED => "PROGRAMMATIC_GUARANTEED",
            InventorySourceInventorySourceProductTypeEnum::TAGGUARANTEED => "TAG_GUARANTEED",
            InventorySourceInventorySourceProductTypeEnum::YOUTUBERESERVE => "YOUTUBE_RESERVE",
            InventorySourceInventorySourceProductTypeEnum::INSTANTRESERVE => "INSTANT_RESERVE",
            InventorySourceInventorySourceProductTypeEnum::GUARANTEEDPACKAGE => "GUARANTEED_PACKAGE",
            InventorySourceInventorySourceProductTypeEnum::PROGRAMMATICTV => "PROGRAMMATIC_TV",
            InventorySourceInventorySourceProductTypeEnum::AUCTIONPACKAGE => "AUCTION_PACKAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for InventorySourceInventorySourceProductTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVENTORY_SOURCE_PRODUCT_TYPE_UNSPECIFIED" => Ok(InventorySourceInventorySourceProductTypeEnum::INVENTORYSOURCEPRODUCTTYPEUNSPECIFIED),
           "PREFERRED_DEAL" => Ok(InventorySourceInventorySourceProductTypeEnum::PREFERREDDEAL),
           "PRIVATE_AUCTION" => Ok(InventorySourceInventorySourceProductTypeEnum::PRIVATEAUCTION),
           "PROGRAMMATIC_GUARANTEED" => Ok(InventorySourceInventorySourceProductTypeEnum::PROGRAMMATICGUARANTEED),
           "TAG_GUARANTEED" => Ok(InventorySourceInventorySourceProductTypeEnum::TAGGUARANTEED),
           "YOUTUBE_RESERVE" => Ok(InventorySourceInventorySourceProductTypeEnum::YOUTUBERESERVE),
           "INSTANT_RESERVE" => Ok(InventorySourceInventorySourceProductTypeEnum::INSTANTRESERVE),
           "GUARANTEED_PACKAGE" => Ok(InventorySourceInventorySourceProductTypeEnum::GUARANTEEDPACKAGE),
           "PROGRAMMATIC_TV" => Ok(InventorySourceInventorySourceProductTypeEnum::PROGRAMMATICTV),
           "AUCTION_PACKAGE" => Ok(InventorySourceInventorySourceProductTypeEnum::AUCTIONPACKAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventorySourceInventorySourceProductTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventorySourceInventorySourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Denotes the type of the inventory source.
pub enum InventorySourceInventorySourceTypeEnum {
    

    /// The inventory source type is not specified or is unknown in this version.
    ///
    /// "INVENTORY_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="INVENTORY_SOURCE_TYPE_UNSPECIFIED")]
    INVENTORYSOURCETYPEUNSPECIFIED,
    

    /// Private inventory source.
    ///
    /// "INVENTORY_SOURCE_TYPE_PRIVATE"
    #[serde(rename="INVENTORY_SOURCE_TYPE_PRIVATE")]
    INVENTORYSOURCETYPEPRIVATE,
    

    /// Auction package.
    ///
    /// "INVENTORY_SOURCE_TYPE_AUCTION_PACKAGE"
    #[serde(rename="INVENTORY_SOURCE_TYPE_AUCTION_PACKAGE")]
    INVENTORYSOURCETYPEAUCTIONPACKAGE,
}

impl AsRef<str> for InventorySourceInventorySourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventorySourceInventorySourceTypeEnum::INVENTORYSOURCETYPEUNSPECIFIED => "INVENTORY_SOURCE_TYPE_UNSPECIFIED",
            InventorySourceInventorySourceTypeEnum::INVENTORYSOURCETYPEPRIVATE => "INVENTORY_SOURCE_TYPE_PRIVATE",
            InventorySourceInventorySourceTypeEnum::INVENTORYSOURCETYPEAUCTIONPACKAGE => "INVENTORY_SOURCE_TYPE_AUCTION_PACKAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for InventorySourceInventorySourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVENTORY_SOURCE_TYPE_UNSPECIFIED" => Ok(InventorySourceInventorySourceTypeEnum::INVENTORYSOURCETYPEUNSPECIFIED),
           "INVENTORY_SOURCE_TYPE_PRIVATE" => Ok(InventorySourceInventorySourceTypeEnum::INVENTORYSOURCETYPEPRIVATE),
           "INVENTORY_SOURCE_TYPE_AUCTION_PACKAGE" => Ok(InventorySourceInventorySourceTypeEnum::INVENTORYSOURCETYPEAUCTIONPACKAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventorySourceInventorySourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventorySourceStatusConfigStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The configuration status of the inventory source. Only applicable for guaranteed inventory sources. Acceptable values are `INVENTORY_SOURCE_CONFIG_STATUS_PENDING` and `INVENTORY_SOURCE_CONFIG_STATUS_COMPLETED`. An inventory source must be configured (fill in the required fields, choose creatives, and select a default campaign) before it can serve.
pub enum InventorySourceStatusConfigStatusEnum {
    

    /// The approval status is not specified or is unknown in this version.
    ///
    /// "INVENTORY_SOURCE_CONFIG_STATUS_UNSPECIFIED"
    #[serde(rename="INVENTORY_SOURCE_CONFIG_STATUS_UNSPECIFIED")]
    INVENTORYSOURCECONFIGSTATUSUNSPECIFIED,
    

    /// The beginning state of a guaranteed inventory source. The inventory source in this state needs to be configured.
    ///
    /// "INVENTORY_SOURCE_CONFIG_STATUS_PENDING"
    #[serde(rename="INVENTORY_SOURCE_CONFIG_STATUS_PENDING")]
    INVENTORYSOURCECONFIGSTATUSPENDING,
    

    /// The state after the buyer configures a guaranteed inventory source.
    ///
    /// "INVENTORY_SOURCE_CONFIG_STATUS_COMPLETED"
    #[serde(rename="INVENTORY_SOURCE_CONFIG_STATUS_COMPLETED")]
    INVENTORYSOURCECONFIGSTATUSCOMPLETED,
}

impl AsRef<str> for InventorySourceStatusConfigStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventorySourceStatusConfigStatusEnum::INVENTORYSOURCECONFIGSTATUSUNSPECIFIED => "INVENTORY_SOURCE_CONFIG_STATUS_UNSPECIFIED",
            InventorySourceStatusConfigStatusEnum::INVENTORYSOURCECONFIGSTATUSPENDING => "INVENTORY_SOURCE_CONFIG_STATUS_PENDING",
            InventorySourceStatusConfigStatusEnum::INVENTORYSOURCECONFIGSTATUSCOMPLETED => "INVENTORY_SOURCE_CONFIG_STATUS_COMPLETED",
        }
    }
}

impl std::convert::TryFrom< &str> for InventorySourceStatusConfigStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVENTORY_SOURCE_CONFIG_STATUS_UNSPECIFIED" => Ok(InventorySourceStatusConfigStatusEnum::INVENTORYSOURCECONFIGSTATUSUNSPECIFIED),
           "INVENTORY_SOURCE_CONFIG_STATUS_PENDING" => Ok(InventorySourceStatusConfigStatusEnum::INVENTORYSOURCECONFIGSTATUSPENDING),
           "INVENTORY_SOURCE_CONFIG_STATUS_COMPLETED" => Ok(InventorySourceStatusConfigStatusEnum::INVENTORYSOURCECONFIGSTATUSCOMPLETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventorySourceStatusConfigStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventorySourceStatusEntityStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether or not the inventory source is servable. Acceptable values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_ARCHIVED`, and `ENTITY_STATUS_PAUSED`. Default value is `ENTITY_STATUS_ACTIVE`.
pub enum InventorySourceStatusEntityStatusEnum {
    

    /// Default value when status is not specified or is unknown in this version.
    ///
    /// "ENTITY_STATUS_UNSPECIFIED"
    #[serde(rename="ENTITY_STATUS_UNSPECIFIED")]
    ENTITYSTATUSUNSPECIFIED,
    

    /// The entity is enabled to bid and spend budget.
    ///
    /// "ENTITY_STATUS_ACTIVE"
    #[serde(rename="ENTITY_STATUS_ACTIVE")]
    ENTITYSTATUSACTIVE,
    

    /// The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved.
    ///
    /// "ENTITY_STATUS_ARCHIVED"
    #[serde(rename="ENTITY_STATUS_ARCHIVED")]
    ENTITYSTATUSARCHIVED,
    

    /// The entity is under draft. Bidding and budget spending are disabled.
    ///
    /// "ENTITY_STATUS_DRAFT"
    #[serde(rename="ENTITY_STATUS_DRAFT")]
    ENTITYSTATUSDRAFT,
    

    /// Bidding and budget spending are paused for the entity.
    ///
    /// "ENTITY_STATUS_PAUSED"
    #[serde(rename="ENTITY_STATUS_PAUSED")]
    ENTITYSTATUSPAUSED,
    

    /// The entity is scheduled for deletion.
    ///
    /// "ENTITY_STATUS_SCHEDULED_FOR_DELETION"
    #[serde(rename="ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    ENTITYSTATUSSCHEDULEDFORDELETION,
}

impl AsRef<str> for InventorySourceStatusEntityStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventorySourceStatusEntityStatusEnum::ENTITYSTATUSUNSPECIFIED => "ENTITY_STATUS_UNSPECIFIED",
            InventorySourceStatusEntityStatusEnum::ENTITYSTATUSACTIVE => "ENTITY_STATUS_ACTIVE",
            InventorySourceStatusEntityStatusEnum::ENTITYSTATUSARCHIVED => "ENTITY_STATUS_ARCHIVED",
            InventorySourceStatusEntityStatusEnum::ENTITYSTATUSDRAFT => "ENTITY_STATUS_DRAFT",
            InventorySourceStatusEntityStatusEnum::ENTITYSTATUSPAUSED => "ENTITY_STATUS_PAUSED",
            InventorySourceStatusEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION => "ENTITY_STATUS_SCHEDULED_FOR_DELETION",
        }
    }
}

impl std::convert::TryFrom< &str> for InventorySourceStatusEntityStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_STATUS_UNSPECIFIED" => Ok(InventorySourceStatusEntityStatusEnum::ENTITYSTATUSUNSPECIFIED),
           "ENTITY_STATUS_ACTIVE" => Ok(InventorySourceStatusEntityStatusEnum::ENTITYSTATUSACTIVE),
           "ENTITY_STATUS_ARCHIVED" => Ok(InventorySourceStatusEntityStatusEnum::ENTITYSTATUSARCHIVED),
           "ENTITY_STATUS_DRAFT" => Ok(InventorySourceStatusEntityStatusEnum::ENTITYSTATUSDRAFT),
           "ENTITY_STATUS_PAUSED" => Ok(InventorySourceStatusEntityStatusEnum::ENTITYSTATUSPAUSED),
           "ENTITY_STATUS_SCHEDULED_FOR_DELETION" => Ok(InventorySourceStatusEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventorySourceStatusEntityStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventorySourceStatusSellerStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The status set by the seller for the inventory source. Only applicable for inventory sources synced directly from the publishers. Acceptable values are `ENTITY_STATUS_ACTIVE` and `ENTITY_STATUS_PAUSED`.
pub enum InventorySourceStatusSellerStatusEnum {
    

    /// Default value when status is not specified or is unknown in this version.
    ///
    /// "ENTITY_STATUS_UNSPECIFIED"
    #[serde(rename="ENTITY_STATUS_UNSPECIFIED")]
    ENTITYSTATUSUNSPECIFIED,
    

    /// The entity is enabled to bid and spend budget.
    ///
    /// "ENTITY_STATUS_ACTIVE"
    #[serde(rename="ENTITY_STATUS_ACTIVE")]
    ENTITYSTATUSACTIVE,
    

    /// The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved.
    ///
    /// "ENTITY_STATUS_ARCHIVED"
    #[serde(rename="ENTITY_STATUS_ARCHIVED")]
    ENTITYSTATUSARCHIVED,
    

    /// The entity is under draft. Bidding and budget spending are disabled.
    ///
    /// "ENTITY_STATUS_DRAFT"
    #[serde(rename="ENTITY_STATUS_DRAFT")]
    ENTITYSTATUSDRAFT,
    

    /// Bidding and budget spending are paused for the entity.
    ///
    /// "ENTITY_STATUS_PAUSED"
    #[serde(rename="ENTITY_STATUS_PAUSED")]
    ENTITYSTATUSPAUSED,
    

    /// The entity is scheduled for deletion.
    ///
    /// "ENTITY_STATUS_SCHEDULED_FOR_DELETION"
    #[serde(rename="ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    ENTITYSTATUSSCHEDULEDFORDELETION,
}

impl AsRef<str> for InventorySourceStatusSellerStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventorySourceStatusSellerStatusEnum::ENTITYSTATUSUNSPECIFIED => "ENTITY_STATUS_UNSPECIFIED",
            InventorySourceStatusSellerStatusEnum::ENTITYSTATUSACTIVE => "ENTITY_STATUS_ACTIVE",
            InventorySourceStatusSellerStatusEnum::ENTITYSTATUSARCHIVED => "ENTITY_STATUS_ARCHIVED",
            InventorySourceStatusSellerStatusEnum::ENTITYSTATUSDRAFT => "ENTITY_STATUS_DRAFT",
            InventorySourceStatusSellerStatusEnum::ENTITYSTATUSPAUSED => "ENTITY_STATUS_PAUSED",
            InventorySourceStatusSellerStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION => "ENTITY_STATUS_SCHEDULED_FOR_DELETION",
        }
    }
}

impl std::convert::TryFrom< &str> for InventorySourceStatusSellerStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_STATUS_UNSPECIFIED" => Ok(InventorySourceStatusSellerStatusEnum::ENTITYSTATUSUNSPECIFIED),
           "ENTITY_STATUS_ACTIVE" => Ok(InventorySourceStatusSellerStatusEnum::ENTITYSTATUSACTIVE),
           "ENTITY_STATUS_ARCHIVED" => Ok(InventorySourceStatusSellerStatusEnum::ENTITYSTATUSARCHIVED),
           "ENTITY_STATUS_DRAFT" => Ok(InventorySourceStatusSellerStatusEnum::ENTITYSTATUSDRAFT),
           "ENTITY_STATUS_PAUSED" => Ok(InventorySourceStatusSellerStatusEnum::ENTITYSTATUSPAUSED),
           "ENTITY_STATUS_SCHEDULED_FOR_DELETION" => Ok(InventorySourceStatusSellerStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventorySourceStatusSellerStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InvoiceInvoiceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of invoice document.
pub enum InvoiceInvoiceTypeEnum {
    

    /// Not specified or is unknown in this version.
    ///
    /// "INVOICE_TYPE_UNSPECIFIED"
    #[serde(rename="INVOICE_TYPE_UNSPECIFIED")]
    INVOICETYPEUNSPECIFIED,
    

    /// The invoice has a negative amount.
    ///
    /// "INVOICE_TYPE_CREDIT"
    #[serde(rename="INVOICE_TYPE_CREDIT")]
    INVOICETYPECREDIT,
    

    /// The invoice has a positive amount.
    ///
    /// "INVOICE_TYPE_INVOICE"
    #[serde(rename="INVOICE_TYPE_INVOICE")]
    INVOICETYPEINVOICE,
}

impl AsRef<str> for InvoiceInvoiceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvoiceInvoiceTypeEnum::INVOICETYPEUNSPECIFIED => "INVOICE_TYPE_UNSPECIFIED",
            InvoiceInvoiceTypeEnum::INVOICETYPECREDIT => "INVOICE_TYPE_CREDIT",
            InvoiceInvoiceTypeEnum::INVOICETYPEINVOICE => "INVOICE_TYPE_INVOICE",
        }
    }
}

impl std::convert::TryFrom< &str> for InvoiceInvoiceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVOICE_TYPE_UNSPECIFIED" => Ok(InvoiceInvoiceTypeEnum::INVOICETYPEUNSPECIFIED),
           "INVOICE_TYPE_CREDIT" => Ok(InvoiceInvoiceTypeEnum::INVOICETYPECREDIT),
           "INVOICE_TYPE_INVOICE" => Ok(InvoiceInvoiceTypeEnum::INVOICETYPEINVOICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InvoiceInvoiceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LineItemEntityStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Controls whether or not the line item can spend its budget and bid on inventory. * For CreateLineItem method, only `ENTITY_STATUS_DRAFT` is allowed. To activate a line item, use UpdateLineItem method and update the status to `ENTITY_STATUS_ACTIVE` after creation. * A line item cannot be changed back to `ENTITY_STATUS_DRAFT` status from any other status. * If the line item's parent insertion order is not active, the line item can't spend its budget even if its own status is `ENTITY_STATUS_ACTIVE`.
pub enum LineItemEntityStatusEnum {
    

    /// Default value when status is not specified or is unknown in this version.
    ///
    /// "ENTITY_STATUS_UNSPECIFIED"
    #[serde(rename="ENTITY_STATUS_UNSPECIFIED")]
    ENTITYSTATUSUNSPECIFIED,
    

    /// The entity is enabled to bid and spend budget.
    ///
    /// "ENTITY_STATUS_ACTIVE"
    #[serde(rename="ENTITY_STATUS_ACTIVE")]
    ENTITYSTATUSACTIVE,
    

    /// The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved.
    ///
    /// "ENTITY_STATUS_ARCHIVED"
    #[serde(rename="ENTITY_STATUS_ARCHIVED")]
    ENTITYSTATUSARCHIVED,
    

    /// The entity is under draft. Bidding and budget spending are disabled.
    ///
    /// "ENTITY_STATUS_DRAFT"
    #[serde(rename="ENTITY_STATUS_DRAFT")]
    ENTITYSTATUSDRAFT,
    

    /// Bidding and budget spending are paused for the entity.
    ///
    /// "ENTITY_STATUS_PAUSED"
    #[serde(rename="ENTITY_STATUS_PAUSED")]
    ENTITYSTATUSPAUSED,
    

    /// The entity is scheduled for deletion.
    ///
    /// "ENTITY_STATUS_SCHEDULED_FOR_DELETION"
    #[serde(rename="ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    ENTITYSTATUSSCHEDULEDFORDELETION,
}

impl AsRef<str> for LineItemEntityStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LineItemEntityStatusEnum::ENTITYSTATUSUNSPECIFIED => "ENTITY_STATUS_UNSPECIFIED",
            LineItemEntityStatusEnum::ENTITYSTATUSACTIVE => "ENTITY_STATUS_ACTIVE",
            LineItemEntityStatusEnum::ENTITYSTATUSARCHIVED => "ENTITY_STATUS_ARCHIVED",
            LineItemEntityStatusEnum::ENTITYSTATUSDRAFT => "ENTITY_STATUS_DRAFT",
            LineItemEntityStatusEnum::ENTITYSTATUSPAUSED => "ENTITY_STATUS_PAUSED",
            LineItemEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION => "ENTITY_STATUS_SCHEDULED_FOR_DELETION",
        }
    }
}

impl std::convert::TryFrom< &str> for LineItemEntityStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_STATUS_UNSPECIFIED" => Ok(LineItemEntityStatusEnum::ENTITYSTATUSUNSPECIFIED),
           "ENTITY_STATUS_ACTIVE" => Ok(LineItemEntityStatusEnum::ENTITYSTATUSACTIVE),
           "ENTITY_STATUS_ARCHIVED" => Ok(LineItemEntityStatusEnum::ENTITYSTATUSARCHIVED),
           "ENTITY_STATUS_DRAFT" => Ok(LineItemEntityStatusEnum::ENTITYSTATUSDRAFT),
           "ENTITY_STATUS_PAUSED" => Ok(LineItemEntityStatusEnum::ENTITYSTATUSPAUSED),
           "ENTITY_STATUS_SCHEDULED_FOR_DELETION" => Ok(LineItemEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LineItemEntityStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LineItemLineItemTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The type of the line item.
pub enum LineItemLineItemTypeEnum {
    

    /// Type value is not specified or is unknown in this version. Line items of this type and their targeting cannot be created or updated using the API.
    ///
    /// "LINE_ITEM_TYPE_UNSPECIFIED"
    #[serde(rename="LINE_ITEM_TYPE_UNSPECIFIED")]
    LINEITEMTYPEUNSPECIFIED,
    

    /// Image, HTML5, native, or rich media ads.
    ///
    /// "LINE_ITEM_TYPE_DISPLAY_DEFAULT"
    #[serde(rename="LINE_ITEM_TYPE_DISPLAY_DEFAULT")]
    LINEITEMTYPEDISPLAYDEFAULT,
    

    /// Display ads that drive installs of an app.
    ///
    /// "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL"
    #[serde(rename="LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL")]
    LINEITEMTYPEDISPLAYMOBILEAPPINSTALL,
    

    /// Video ads sold on a CPM basis for a variety of environments.
    ///
    /// "LINE_ITEM_TYPE_VIDEO_DEFAULT"
    #[serde(rename="LINE_ITEM_TYPE_VIDEO_DEFAULT")]
    LINEITEMTYPEVIDEODEFAULT,
    

    /// Video ads that drive installs of an app.
    ///
    /// "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL"
    #[serde(rename="LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL")]
    LINEITEMTYPEVIDEOMOBILEAPPINSTALL,
    

    /// Display ads served on mobile app inventory. Line items of this type and their targeting cannot be created or updated using the API.
    ///
    /// "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INVENTORY"
    #[serde(rename="LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INVENTORY")]
    LINEITEMTYPEDISPLAYMOBILEAPPINVENTORY,
    

    /// Video ads served on mobile app inventory. Line items of this type and their targeting cannot be created or updated using the API.
    ///
    /// "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INVENTORY"
    #[serde(rename="LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INVENTORY")]
    LINEITEMTYPEVIDEOMOBILEAPPINVENTORY,
    

    /// RTB Audio ads sold for a variety of environments.
    ///
    /// "LINE_ITEM_TYPE_AUDIO_DEFAULT"
    #[serde(rename="LINE_ITEM_TYPE_AUDIO_DEFAULT")]
    LINEITEMTYPEAUDIODEFAULT,
    

    /// Over-the-top ads present in OTT insertion orders. This type is only applicable to line items with an insertion order of insertion_order_type `OVER_THE_TOP`.
    ///
    /// "LINE_ITEM_TYPE_VIDEO_OVER_THE_TOP"
    #[serde(rename="LINE_ITEM_TYPE_VIDEO_OVER_THE_TOP")]
    LINEITEMTYPEVIDEOOVERTHETOP,
    

    /// Display ads served on digital-out-of-home inventory. Line items of this type and their targeting cannot be created or updated using the API.
    ///
    /// "LINE_ITEM_TYPE_DISPLAY_OUT_OF_HOME"
    #[serde(rename="LINE_ITEM_TYPE_DISPLAY_OUT_OF_HOME")]
    LINEITEMTYPEDISPLAYOUTOFHOME,
    

    /// Video ads served on digital-out-of-home inventory. Line items of this type and their targeting cannot be created or updated using the API.
    ///
    /// "LINE_ITEM_TYPE_VIDEO_OUT_OF_HOME"
    #[serde(rename="LINE_ITEM_TYPE_VIDEO_OUT_OF_HOME")]
    LINEITEMTYPEVIDEOOUTOFHOME,
}

impl AsRef<str> for LineItemLineItemTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LineItemLineItemTypeEnum::LINEITEMTYPEUNSPECIFIED => "LINE_ITEM_TYPE_UNSPECIFIED",
            LineItemLineItemTypeEnum::LINEITEMTYPEDISPLAYDEFAULT => "LINE_ITEM_TYPE_DISPLAY_DEFAULT",
            LineItemLineItemTypeEnum::LINEITEMTYPEDISPLAYMOBILEAPPINSTALL => "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL",
            LineItemLineItemTypeEnum::LINEITEMTYPEVIDEODEFAULT => "LINE_ITEM_TYPE_VIDEO_DEFAULT",
            LineItemLineItemTypeEnum::LINEITEMTYPEVIDEOMOBILEAPPINSTALL => "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL",
            LineItemLineItemTypeEnum::LINEITEMTYPEDISPLAYMOBILEAPPINVENTORY => "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INVENTORY",
            LineItemLineItemTypeEnum::LINEITEMTYPEVIDEOMOBILEAPPINVENTORY => "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INVENTORY",
            LineItemLineItemTypeEnum::LINEITEMTYPEAUDIODEFAULT => "LINE_ITEM_TYPE_AUDIO_DEFAULT",
            LineItemLineItemTypeEnum::LINEITEMTYPEVIDEOOVERTHETOP => "LINE_ITEM_TYPE_VIDEO_OVER_THE_TOP",
            LineItemLineItemTypeEnum::LINEITEMTYPEDISPLAYOUTOFHOME => "LINE_ITEM_TYPE_DISPLAY_OUT_OF_HOME",
            LineItemLineItemTypeEnum::LINEITEMTYPEVIDEOOUTOFHOME => "LINE_ITEM_TYPE_VIDEO_OUT_OF_HOME",
        }
    }
}

impl std::convert::TryFrom< &str> for LineItemLineItemTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINE_ITEM_TYPE_UNSPECIFIED" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEUNSPECIFIED),
           "LINE_ITEM_TYPE_DISPLAY_DEFAULT" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEDISPLAYDEFAULT),
           "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEDISPLAYMOBILEAPPINSTALL),
           "LINE_ITEM_TYPE_VIDEO_DEFAULT" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEVIDEODEFAULT),
           "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEVIDEOMOBILEAPPINSTALL),
           "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INVENTORY" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEDISPLAYMOBILEAPPINVENTORY),
           "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INVENTORY" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEVIDEOMOBILEAPPINVENTORY),
           "LINE_ITEM_TYPE_AUDIO_DEFAULT" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEAUDIODEFAULT),
           "LINE_ITEM_TYPE_VIDEO_OVER_THE_TOP" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEVIDEOOVERTHETOP),
           "LINE_ITEM_TYPE_DISPLAY_OUT_OF_HOME" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEDISPLAYOUTOFHOME),
           "LINE_ITEM_TYPE_VIDEO_OUT_OF_HOME" => Ok(LineItemLineItemTypeEnum::LINEITEMTYPEVIDEOOUTOFHOME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LineItemLineItemTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LineItemReservationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The reservation type of the line item.
pub enum LineItemReservationTypeEnum {
    

    /// Reservation type value is not specified or is unknown in this version.
    ///
    /// "RESERVATION_TYPE_UNSPECIFIED"
    #[serde(rename="RESERVATION_TYPE_UNSPECIFIED")]
    RESERVATIONTYPEUNSPECIFIED,
    

    /// Not created through a guaranteed inventory source.
    ///
    /// "RESERVATION_TYPE_NOT_GUARANTEED"
    #[serde(rename="RESERVATION_TYPE_NOT_GUARANTEED")]
    RESERVATIONTYPENOTGUARANTEED,
    

    /// Created through a programmatic guaranteed inventory source.
    ///
    /// "RESERVATION_TYPE_PROGRAMMATIC_GUARANTEED"
    #[serde(rename="RESERVATION_TYPE_PROGRAMMATIC_GUARANTEED")]
    RESERVATIONTYPEPROGRAMMATICGUARANTEED,
    

    /// Created through a tag guaranteed inventory source.
    ///
    /// "RESERVATION_TYPE_TAG_GUARANTEED"
    #[serde(rename="RESERVATION_TYPE_TAG_GUARANTEED")]
    RESERVATIONTYPETAGGUARANTEED,
}

impl AsRef<str> for LineItemReservationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LineItemReservationTypeEnum::RESERVATIONTYPEUNSPECIFIED => "RESERVATION_TYPE_UNSPECIFIED",
            LineItemReservationTypeEnum::RESERVATIONTYPENOTGUARANTEED => "RESERVATION_TYPE_NOT_GUARANTEED",
            LineItemReservationTypeEnum::RESERVATIONTYPEPROGRAMMATICGUARANTEED => "RESERVATION_TYPE_PROGRAMMATIC_GUARANTEED",
            LineItemReservationTypeEnum::RESERVATIONTYPETAGGUARANTEED => "RESERVATION_TYPE_TAG_GUARANTEED",
        }
    }
}

impl std::convert::TryFrom< &str> for LineItemReservationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESERVATION_TYPE_UNSPECIFIED" => Ok(LineItemReservationTypeEnum::RESERVATIONTYPEUNSPECIFIED),
           "RESERVATION_TYPE_NOT_GUARANTEED" => Ok(LineItemReservationTypeEnum::RESERVATIONTYPENOTGUARANTEED),
           "RESERVATION_TYPE_PROGRAMMATIC_GUARANTEED" => Ok(LineItemReservationTypeEnum::RESERVATIONTYPEPROGRAMMATICGUARANTEED),
           "RESERVATION_TYPE_TAG_GUARANTEED" => Ok(LineItemReservationTypeEnum::RESERVATIONTYPETAGGUARANTEED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LineItemReservationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LineItemWarningMessagesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The warning messages generated by the line item. These warnings do not block saving the line item, but some may block the line item from running.
pub enum LineItemWarningMessagesEnum {
    

    /// Not specified or is unknown.
    ///
    /// "LINE_ITEM_WARNING_MESSAGE_UNSPECIFIED"
    #[serde(rename="LINE_ITEM_WARNING_MESSAGE_UNSPECIFIED")]
    LINEITEMWARNINGMESSAGEUNSPECIFIED,
    

    /// This line item has invalid flight dates. The line item will not run.
    ///
    /// "INVALID_FLIGHT_DATES"
    #[serde(rename="INVALID_FLIGHT_DATES")]
    INVALIDFLIGHTDATES,
    

    /// This line item's end date is in the past.
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// This line item will begin running in the future.
    ///
    /// "PENDING_FLIGHT"
    #[serde(rename="PENDING_FLIGHT")]
    PENDINGFLIGHT,
    

    /// All partner enabled exchanges are negatively targeted. The line item will not run.
    ///
    /// "ALL_PARTNER_ENABLED_EXCHANGES_NEGATIVELY_TARGETED"
    #[serde(rename="ALL_PARTNER_ENABLED_EXCHANGES_NEGATIVELY_TARGETED")]
    ALLPARTNERENABLEDEXCHANGESNEGATIVELYTARGETED,
    

    /// No active inventory sources are being targeted. The line item will not run.
    ///
    /// "INVALID_INVENTORY_SOURCE"
    #[serde(rename="INVALID_INVENTORY_SOURCE")]
    INVALIDINVENTORYSOURCE,
    

    /// This line item's Apps & URLs targeting doesn't include any mobile apps. This line item's type requires you to include mobile apps in your channel, sitelist, or apps targeting. The line item will not run.
    ///
    /// "APP_INVENTORY_INVALID_SITE_TARGETING"
    #[serde(rename="APP_INVENTORY_INVALID_SITE_TARGETING")]
    APPINVENTORYINVALIDSITETARGETING,
    

    /// This line item isn't targeting any mobile users. This line item's type requires you to target a user list with mobile users. The line item will not run.
    ///
    /// "APP_INVENTORY_INVALID_AUDIENCE_LISTS"
    #[serde(rename="APP_INVENTORY_INVALID_AUDIENCE_LISTS")]
    APPINVENTORYINVALIDAUDIENCELISTS,
    

    /// This line item does not contain any valid creative. The line item will not run.
    ///
    /// "NO_VALID_CREATIVE"
    #[serde(rename="NO_VALID_CREATIVE")]
    NOVALIDCREATIVE,
    

    /// The insertion order of this line item is paused. The line item will not run.
    ///
    /// "PARENT_INSERTION_ORDER_PAUSED"
    #[serde(rename="PARENT_INSERTION_ORDER_PAUSED")]
    PARENTINSERTIONORDERPAUSED,
    

    /// The insertion order of this line item has its end date set in the past. The line item will not run.
    ///
    /// "PARENT_INSERTION_ORDER_EXPIRED"
    #[serde(rename="PARENT_INSERTION_ORDER_EXPIRED")]
    PARENTINSERTIONORDEREXPIRED,
    

    /// This line item does not target any audience lists, which may result in spending your budget too quickly.
    ///
    /// "NO_POSITIVE_AUDIENCE_LIST_TARGETED"
    #[serde(rename="NO_POSITIVE_AUDIENCE_LIST_TARGETED")]
    NOPOSITIVEAUDIENCELISTTARGETED,
    

    /// This app install line item does not have any conversion pixel set up.
    ///
    /// "APP_INSTALL_NO_CONVERSION_PIXEL"
    #[serde(rename="APP_INSTALL_NO_CONVERSION_PIXEL")]
    APPINSTALLNOCONVERSIONPIXEL,
    

    /// This line item targets one or more user lists that are no longer available. In the future, this will prevent the line item from serving, so consider removing these lists from your targeting.
    ///
    /// "TARGETING_REVOKED_OR_CLOSED_USER_LIST"
    #[serde(rename="TARGETING_REVOKED_OR_CLOSED_USER_LIST")]
    TARGETINGREVOKEDORCLOSEDUSERLIST,
    

    /// This app install line item does not have an optimal bidding strategy.
    ///
    /// "APP_INSTALL_NO_OPTIMAL_BIDDING_STRATEGY"
    #[serde(rename="APP_INSTALL_NO_OPTIMAL_BIDDING_STRATEGY")]
    APPINSTALLNOOPTIMALBIDDINGSTRATEGY,
    

    /// Deals targeted by this line item accept creative sizes which are not in use. This may limit the line item's delivery or performance.
    ///
    /// "CREATIVE_SIZE_NOT_IN_USE_FOR_TARGETED_DEALS"
    #[serde(rename="CREATIVE_SIZE_NOT_IN_USE_FOR_TARGETED_DEALS")]
    CREATIVESIZENOTINUSEFORTARGETEDDEALS,
    

    /// This line item does not contain any creative for the targeted deals.
    ///
    /// "NO_CREATIVE_FOR_TARGETED_DEALS"
    #[serde(rename="NO_CREATIVE_FOR_TARGETED_DEALS")]
    NOCREATIVEFORTARGETEDDEALS,
    

    /// This line item targets a geo target that is deprecated.
    ///
    /// "TARGETING_DEPRECATED_GEO_TARGET"
    #[serde(rename="TARGETING_DEPRECATED_GEO_TARGET")]
    TARGETINGDEPRECATEDGEOTARGET,
    

    /// This line item uses the exclude_first_party_audience setting, which is deprecated and scheduled to sunset after **March 25, 2023**. Update your API integration to directly exclude any first-party audiences using audience targeting before **March 25, 2023** to account for the sunset of the exclude_first_party_audience field.
    ///
    /// "DEPRECATED_FIRST_PARTY_AUDIENCE_EXCLUSION"
    #[serde(rename="DEPRECATED_FIRST_PARTY_AUDIENCE_EXCLUSION")]
    DEPRECATEDFIRSTPARTYAUDIENCEEXCLUSION,
}

impl AsRef<str> for LineItemWarningMessagesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LineItemWarningMessagesEnum::LINEITEMWARNINGMESSAGEUNSPECIFIED => "LINE_ITEM_WARNING_MESSAGE_UNSPECIFIED",
            LineItemWarningMessagesEnum::INVALIDFLIGHTDATES => "INVALID_FLIGHT_DATES",
            LineItemWarningMessagesEnum::EXPIRED => "EXPIRED",
            LineItemWarningMessagesEnum::PENDINGFLIGHT => "PENDING_FLIGHT",
            LineItemWarningMessagesEnum::ALLPARTNERENABLEDEXCHANGESNEGATIVELYTARGETED => "ALL_PARTNER_ENABLED_EXCHANGES_NEGATIVELY_TARGETED",
            LineItemWarningMessagesEnum::INVALIDINVENTORYSOURCE => "INVALID_INVENTORY_SOURCE",
            LineItemWarningMessagesEnum::APPINVENTORYINVALIDSITETARGETING => "APP_INVENTORY_INVALID_SITE_TARGETING",
            LineItemWarningMessagesEnum::APPINVENTORYINVALIDAUDIENCELISTS => "APP_INVENTORY_INVALID_AUDIENCE_LISTS",
            LineItemWarningMessagesEnum::NOVALIDCREATIVE => "NO_VALID_CREATIVE",
            LineItemWarningMessagesEnum::PARENTINSERTIONORDERPAUSED => "PARENT_INSERTION_ORDER_PAUSED",
            LineItemWarningMessagesEnum::PARENTINSERTIONORDEREXPIRED => "PARENT_INSERTION_ORDER_EXPIRED",
            LineItemWarningMessagesEnum::NOPOSITIVEAUDIENCELISTTARGETED => "NO_POSITIVE_AUDIENCE_LIST_TARGETED",
            LineItemWarningMessagesEnum::APPINSTALLNOCONVERSIONPIXEL => "APP_INSTALL_NO_CONVERSION_PIXEL",
            LineItemWarningMessagesEnum::TARGETINGREVOKEDORCLOSEDUSERLIST => "TARGETING_REVOKED_OR_CLOSED_USER_LIST",
            LineItemWarningMessagesEnum::APPINSTALLNOOPTIMALBIDDINGSTRATEGY => "APP_INSTALL_NO_OPTIMAL_BIDDING_STRATEGY",
            LineItemWarningMessagesEnum::CREATIVESIZENOTINUSEFORTARGETEDDEALS => "CREATIVE_SIZE_NOT_IN_USE_FOR_TARGETED_DEALS",
            LineItemWarningMessagesEnum::NOCREATIVEFORTARGETEDDEALS => "NO_CREATIVE_FOR_TARGETED_DEALS",
            LineItemWarningMessagesEnum::TARGETINGDEPRECATEDGEOTARGET => "TARGETING_DEPRECATED_GEO_TARGET",
            LineItemWarningMessagesEnum::DEPRECATEDFIRSTPARTYAUDIENCEEXCLUSION => "DEPRECATED_FIRST_PARTY_AUDIENCE_EXCLUSION",
        }
    }
}

impl std::convert::TryFrom< &str> for LineItemWarningMessagesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINE_ITEM_WARNING_MESSAGE_UNSPECIFIED" => Ok(LineItemWarningMessagesEnum::LINEITEMWARNINGMESSAGEUNSPECIFIED),
           "INVALID_FLIGHT_DATES" => Ok(LineItemWarningMessagesEnum::INVALIDFLIGHTDATES),
           "EXPIRED" => Ok(LineItemWarningMessagesEnum::EXPIRED),
           "PENDING_FLIGHT" => Ok(LineItemWarningMessagesEnum::PENDINGFLIGHT),
           "ALL_PARTNER_ENABLED_EXCHANGES_NEGATIVELY_TARGETED" => Ok(LineItemWarningMessagesEnum::ALLPARTNERENABLEDEXCHANGESNEGATIVELYTARGETED),
           "INVALID_INVENTORY_SOURCE" => Ok(LineItemWarningMessagesEnum::INVALIDINVENTORYSOURCE),
           "APP_INVENTORY_INVALID_SITE_TARGETING" => Ok(LineItemWarningMessagesEnum::APPINVENTORYINVALIDSITETARGETING),
           "APP_INVENTORY_INVALID_AUDIENCE_LISTS" => Ok(LineItemWarningMessagesEnum::APPINVENTORYINVALIDAUDIENCELISTS),
           "NO_VALID_CREATIVE" => Ok(LineItemWarningMessagesEnum::NOVALIDCREATIVE),
           "PARENT_INSERTION_ORDER_PAUSED" => Ok(LineItemWarningMessagesEnum::PARENTINSERTIONORDERPAUSED),
           "PARENT_INSERTION_ORDER_EXPIRED" => Ok(LineItemWarningMessagesEnum::PARENTINSERTIONORDEREXPIRED),
           "NO_POSITIVE_AUDIENCE_LIST_TARGETED" => Ok(LineItemWarningMessagesEnum::NOPOSITIVEAUDIENCELISTTARGETED),
           "APP_INSTALL_NO_CONVERSION_PIXEL" => Ok(LineItemWarningMessagesEnum::APPINSTALLNOCONVERSIONPIXEL),
           "TARGETING_REVOKED_OR_CLOSED_USER_LIST" => Ok(LineItemWarningMessagesEnum::TARGETINGREVOKEDORCLOSEDUSERLIST),
           "APP_INSTALL_NO_OPTIMAL_BIDDING_STRATEGY" => Ok(LineItemWarningMessagesEnum::APPINSTALLNOOPTIMALBIDDINGSTRATEGY),
           "CREATIVE_SIZE_NOT_IN_USE_FOR_TARGETED_DEALS" => Ok(LineItemWarningMessagesEnum::CREATIVESIZENOTINUSEFORTARGETEDDEALS),
           "NO_CREATIVE_FOR_TARGETED_DEALS" => Ok(LineItemWarningMessagesEnum::NOCREATIVEFORTARGETEDDEALS),
           "TARGETING_DEPRECATED_GEO_TARGET" => Ok(LineItemWarningMessagesEnum::TARGETINGDEPRECATEDGEOTARGET),
           "DEPRECATED_FIRST_PARTY_AUDIENCE_EXCLUSION" => Ok(LineItemWarningMessagesEnum::DEPRECATEDFIRSTPARTYAUDIENCEEXCLUSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LineItemWarningMessagesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LineItemBudgetBudgetAllocationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the budget allocation. `LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC` is only applicable when automatic budget allocation is enabled for the parent insertion order.
pub enum LineItemBudgetBudgetAllocationTypeEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNSPECIFIED"
    #[serde(rename="LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNSPECIFIED")]
    LINEITEMBUDGETALLOCATIONTYPEUNSPECIFIED,
    

    /// Automatic budget allocation is enabled for the line item.
    ///
    /// "LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC"
    #[serde(rename="LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC")]
    LINEITEMBUDGETALLOCATIONTYPEAUTOMATIC,
    

    /// A fixed max budget amount is allocated for the line item.
    ///
    /// "LINE_ITEM_BUDGET_ALLOCATION_TYPE_FIXED"
    #[serde(rename="LINE_ITEM_BUDGET_ALLOCATION_TYPE_FIXED")]
    LINEITEMBUDGETALLOCATIONTYPEFIXED,
    

    /// No budget limit is applied to the line item.
    ///
    /// "LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNLIMITED"
    #[serde(rename="LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNLIMITED")]
    LINEITEMBUDGETALLOCATIONTYPEUNLIMITED,
}

impl AsRef<str> for LineItemBudgetBudgetAllocationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LineItemBudgetBudgetAllocationTypeEnum::LINEITEMBUDGETALLOCATIONTYPEUNSPECIFIED => "LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNSPECIFIED",
            LineItemBudgetBudgetAllocationTypeEnum::LINEITEMBUDGETALLOCATIONTYPEAUTOMATIC => "LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC",
            LineItemBudgetBudgetAllocationTypeEnum::LINEITEMBUDGETALLOCATIONTYPEFIXED => "LINE_ITEM_BUDGET_ALLOCATION_TYPE_FIXED",
            LineItemBudgetBudgetAllocationTypeEnum::LINEITEMBUDGETALLOCATIONTYPEUNLIMITED => "LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNLIMITED",
        }
    }
}

impl std::convert::TryFrom< &str> for LineItemBudgetBudgetAllocationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNSPECIFIED" => Ok(LineItemBudgetBudgetAllocationTypeEnum::LINEITEMBUDGETALLOCATIONTYPEUNSPECIFIED),
           "LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC" => Ok(LineItemBudgetBudgetAllocationTypeEnum::LINEITEMBUDGETALLOCATIONTYPEAUTOMATIC),
           "LINE_ITEM_BUDGET_ALLOCATION_TYPE_FIXED" => Ok(LineItemBudgetBudgetAllocationTypeEnum::LINEITEMBUDGETALLOCATIONTYPEFIXED),
           "LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNLIMITED" => Ok(LineItemBudgetBudgetAllocationTypeEnum::LINEITEMBUDGETALLOCATIONTYPEUNLIMITED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LineItemBudgetBudgetAllocationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LineItemBudgetBudgetUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The budget unit specifies whether the budget is currency based or impression based. This value is inherited from the parent insertion order.
pub enum LineItemBudgetBudgetUnitEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "BUDGET_UNIT_UNSPECIFIED"
    #[serde(rename="BUDGET_UNIT_UNSPECIFIED")]
    BUDGETUNITUNSPECIFIED,
    

    /// Budgeting in currency amounts.
    ///
    /// "BUDGET_UNIT_CURRENCY"
    #[serde(rename="BUDGET_UNIT_CURRENCY")]
    BUDGETUNITCURRENCY,
    

    /// Budgeting in impression amounts.
    ///
    /// "BUDGET_UNIT_IMPRESSIONS"
    #[serde(rename="BUDGET_UNIT_IMPRESSIONS")]
    BUDGETUNITIMPRESSIONS,
}

impl AsRef<str> for LineItemBudgetBudgetUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LineItemBudgetBudgetUnitEnum::BUDGETUNITUNSPECIFIED => "BUDGET_UNIT_UNSPECIFIED",
            LineItemBudgetBudgetUnitEnum::BUDGETUNITCURRENCY => "BUDGET_UNIT_CURRENCY",
            LineItemBudgetBudgetUnitEnum::BUDGETUNITIMPRESSIONS => "BUDGET_UNIT_IMPRESSIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for LineItemBudgetBudgetUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUDGET_UNIT_UNSPECIFIED" => Ok(LineItemBudgetBudgetUnitEnum::BUDGETUNITUNSPECIFIED),
           "BUDGET_UNIT_CURRENCY" => Ok(LineItemBudgetBudgetUnitEnum::BUDGETUNITCURRENCY),
           "BUDGET_UNIT_IMPRESSIONS" => Ok(LineItemBudgetBudgetUnitEnum::BUDGETUNITIMPRESSIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LineItemBudgetBudgetUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LineItemFlightFlightDateTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the line item's flight dates.
pub enum LineItemFlightFlightDateTypeEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "LINE_ITEM_FLIGHT_DATE_TYPE_UNSPECIFIED"
    #[serde(rename="LINE_ITEM_FLIGHT_DATE_TYPE_UNSPECIFIED")]
    LINEITEMFLIGHTDATETYPEUNSPECIFIED,
    

    /// The line item's flight dates are inherited from its parent insertion order.
    ///
    /// "LINE_ITEM_FLIGHT_DATE_TYPE_INHERITED"
    #[serde(rename="LINE_ITEM_FLIGHT_DATE_TYPE_INHERITED")]
    LINEITEMFLIGHTDATETYPEINHERITED,
    

    /// The line item uses its own custom flight dates.
    ///
    /// "LINE_ITEM_FLIGHT_DATE_TYPE_CUSTOM"
    #[serde(rename="LINE_ITEM_FLIGHT_DATE_TYPE_CUSTOM")]
    LINEITEMFLIGHTDATETYPECUSTOM,
    

    /// The line item uses a trigger. **Warning:** Line Items using manual triggers no longer serve in Display & Video 360. This value will sunset on August 1, 2023. Read our [feature deprecation announcement](/display-video/api/deprecations#features.manual_triggers) for more information.
    ///
    /// "LINE_ITEM_FLIGHT_DATE_TYPE_TRIGGER"
    #[serde(rename="LINE_ITEM_FLIGHT_DATE_TYPE_TRIGGER")]
    LINEITEMFLIGHTDATETYPETRIGGER,
}

impl AsRef<str> for LineItemFlightFlightDateTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LineItemFlightFlightDateTypeEnum::LINEITEMFLIGHTDATETYPEUNSPECIFIED => "LINE_ITEM_FLIGHT_DATE_TYPE_UNSPECIFIED",
            LineItemFlightFlightDateTypeEnum::LINEITEMFLIGHTDATETYPEINHERITED => "LINE_ITEM_FLIGHT_DATE_TYPE_INHERITED",
            LineItemFlightFlightDateTypeEnum::LINEITEMFLIGHTDATETYPECUSTOM => "LINE_ITEM_FLIGHT_DATE_TYPE_CUSTOM",
            LineItemFlightFlightDateTypeEnum::LINEITEMFLIGHTDATETYPETRIGGER => "LINE_ITEM_FLIGHT_DATE_TYPE_TRIGGER",
        }
    }
}

impl std::convert::TryFrom< &str> for LineItemFlightFlightDateTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINE_ITEM_FLIGHT_DATE_TYPE_UNSPECIFIED" => Ok(LineItemFlightFlightDateTypeEnum::LINEITEMFLIGHTDATETYPEUNSPECIFIED),
           "LINE_ITEM_FLIGHT_DATE_TYPE_INHERITED" => Ok(LineItemFlightFlightDateTypeEnum::LINEITEMFLIGHTDATETYPEINHERITED),
           "LINE_ITEM_FLIGHT_DATE_TYPE_CUSTOM" => Ok(LineItemFlightFlightDateTypeEnum::LINEITEMFLIGHTDATETYPECUSTOM),
           "LINE_ITEM_FLIGHT_DATE_TYPE_TRIGGER" => Ok(LineItemFlightFlightDateTypeEnum::LINEITEMFLIGHTDATETYPETRIGGER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LineItemFlightFlightDateTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LocationListLocationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The type of location. All locations in the list will share this type.
pub enum LocationListLocationTypeEnum {
    

    /// Default value when type is not specified or is unknown.
    ///
    /// "TARGETING_LOCATION_TYPE_UNSPECIFIED"
    #[serde(rename="TARGETING_LOCATION_TYPE_UNSPECIFIED")]
    TARGETINGLOCATIONTYPEUNSPECIFIED,
    

    /// The type for proximity geo location.
    ///
    /// "TARGETING_LOCATION_TYPE_PROXIMITY"
    #[serde(rename="TARGETING_LOCATION_TYPE_PROXIMITY")]
    TARGETINGLOCATIONTYPEPROXIMITY,
    

    /// The type for regional geo location.
    ///
    /// "TARGETING_LOCATION_TYPE_REGIONAL"
    #[serde(rename="TARGETING_LOCATION_TYPE_REGIONAL")]
    TARGETINGLOCATIONTYPEREGIONAL,
}

impl AsRef<str> for LocationListLocationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LocationListLocationTypeEnum::TARGETINGLOCATIONTYPEUNSPECIFIED => "TARGETING_LOCATION_TYPE_UNSPECIFIED",
            LocationListLocationTypeEnum::TARGETINGLOCATIONTYPEPROXIMITY => "TARGETING_LOCATION_TYPE_PROXIMITY",
            LocationListLocationTypeEnum::TARGETINGLOCATIONTYPEREGIONAL => "TARGETING_LOCATION_TYPE_REGIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for LocationListLocationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_LOCATION_TYPE_UNSPECIFIED" => Ok(LocationListLocationTypeEnum::TARGETINGLOCATIONTYPEUNSPECIFIED),
           "TARGETING_LOCATION_TYPE_PROXIMITY" => Ok(LocationListLocationTypeEnum::TARGETINGLOCATIONTYPEPROXIMITY),
           "TARGETING_LOCATION_TYPE_REGIONAL" => Ok(LocationListLocationTypeEnum::TARGETINGLOCATIONTYPEREGIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LocationListLocationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManualTriggerStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the manual trigger. Will be set to the `INACTIVE` state upon creation.
pub enum ManualTriggerStateEnum {
    

    /// Default value when state is not specified or is unknown in this version.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The trigger is currently inactive and ready to be activated.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// The trigger is currently active (activated).
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for ManualTriggerStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManualTriggerStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ManualTriggerStateEnum::INACTIVE => "INACTIVE",
            ManualTriggerStateEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ManualTriggerStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ManualTriggerStateEnum::STATEUNSPECIFIED),
           "INACTIVE" => Ok(ManualTriggerStateEnum::INACTIVE),
           "ACTIVE" => Ok(ManualTriggerStateEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManualTriggerStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MaximizeSpendBidStrategyPerformanceGoalTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the performance goal that the bidding strategy tries to minimize while spending the full budget. `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM` is not supported for this strategy.
pub enum MaximizeSpendBidStrategyPerformanceGoalTypeEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_UNSPECIFIED"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_UNSPECIFIED")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPEUNSPECIFIED,
    

    /// Cost per action.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPECPA,
    

    /// Cost per click.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPECPC,
    

    /// Viewable CPM.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPEVIEWABLECPM,
    

    /// Custom bidding algorithm.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPECUSTOMALGO,
    

    /// Completed inview and audible views.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPECIVA,
    

    /// Inview time over 10 secs views.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPEIVOTEN,
    

    /// Viewable impressions.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPEAVVIEWED,
}

impl AsRef<str> for MaximizeSpendBidStrategyPerformanceGoalTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEUNSPECIFIED => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_UNSPECIFIED",
            MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECPA => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA",
            MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECPC => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC",
            MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEVIEWABLECPM => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM",
            MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECUSTOMALGO => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO",
            MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECIVA => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA",
            MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEIVOTEN => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN",
            MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEAVVIEWED => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED",
        }
    }
}

impl std::convert::TryFrom< &str> for MaximizeSpendBidStrategyPerformanceGoalTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_UNSPECIFIED" => Ok(MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEUNSPECIFIED),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA" => Ok(MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECPA),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC" => Ok(MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECPC),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM" => Ok(MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEVIEWABLECPM),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO" => Ok(MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECUSTOMALGO),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA" => Ok(MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECIVA),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN" => Ok(MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEIVOTEN),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED" => Ok(MaximizeSpendBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEAVVIEWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MaximizeSpendBidStrategyPerformanceGoalTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MobileAppPlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The app platform.
pub enum MobileAppPlatformEnum {
    

    /// Platform is not specified.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// iOS platform.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Android platform.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
}

impl AsRef<str> for MobileAppPlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MobileAppPlatformEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            MobileAppPlatformEnum::IOS => "IOS",
            MobileAppPlatformEnum::ANDROID => "ANDROID",
        }
    }
}

impl std::convert::TryFrom< &str> for MobileAppPlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(MobileAppPlatformEnum::PLATFORMUNSPECIFIED),
           "IOS" => Ok(MobileAppPlatformEnum::IOS),
           "ANDROID" => Ok(MobileAppPlatformEnum::ANDROID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MobileAppPlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The content position.
pub enum NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum {
    

    /// Native content position is not specified in this version. This enum is a place holder for a default value and does not represent a real native content position.
    ///
    /// "NATIVE_CONTENT_POSITION_UNSPECIFIED"
    #[serde(rename="NATIVE_CONTENT_POSITION_UNSPECIFIED")]
    NATIVECONTENTPOSITIONUNSPECIFIED,
    

    /// The native content position is unknown.
    ///
    /// "NATIVE_CONTENT_POSITION_UNKNOWN"
    #[serde(rename="NATIVE_CONTENT_POSITION_UNKNOWN")]
    NATIVECONTENTPOSITIONUNKNOWN,
    

    /// Native content position is in-article, i.e., ads appear between the paragraphs of pages.
    ///
    /// "NATIVE_CONTENT_POSITION_IN_ARTICLE"
    #[serde(rename="NATIVE_CONTENT_POSITION_IN_ARTICLE")]
    NATIVECONTENTPOSITIONINARTICLE,
    

    /// Native content position is in-feed, i.e., ads appear in a scrollable stream of content. A feed is typically editorial (e.g. a list of articles or news) or listings (e.g. a list of products or services).
    ///
    /// "NATIVE_CONTENT_POSITION_IN_FEED"
    #[serde(rename="NATIVE_CONTENT_POSITION_IN_FEED")]
    NATIVECONTENTPOSITIONINFEED,
    

    /// Native content position is peripheral, i.e., ads appear outside of core content on pages, such as the right- or left-hand side of the page.
    ///
    /// "NATIVE_CONTENT_POSITION_PERIPHERAL"
    #[serde(rename="NATIVE_CONTENT_POSITION_PERIPHERAL")]
    NATIVECONTENTPOSITIONPERIPHERAL,
    

    /// Native content position is recommendation, i.e., ads appear in sections for recommended content.
    ///
    /// "NATIVE_CONTENT_POSITION_RECOMMENDATION"
    #[serde(rename="NATIVE_CONTENT_POSITION_RECOMMENDATION")]
    NATIVECONTENTPOSITIONRECOMMENDATION,
}

impl AsRef<str> for NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONUNSPECIFIED => "NATIVE_CONTENT_POSITION_UNSPECIFIED",
            NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONUNKNOWN => "NATIVE_CONTENT_POSITION_UNKNOWN",
            NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONINARTICLE => "NATIVE_CONTENT_POSITION_IN_ARTICLE",
            NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONINFEED => "NATIVE_CONTENT_POSITION_IN_FEED",
            NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONPERIPHERAL => "NATIVE_CONTENT_POSITION_PERIPHERAL",
            NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONRECOMMENDATION => "NATIVE_CONTENT_POSITION_RECOMMENDATION",
        }
    }
}

impl std::convert::TryFrom< &str> for NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NATIVE_CONTENT_POSITION_UNSPECIFIED" => Ok(NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONUNSPECIFIED),
           "NATIVE_CONTENT_POSITION_UNKNOWN" => Ok(NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONUNKNOWN),
           "NATIVE_CONTENT_POSITION_IN_ARTICLE" => Ok(NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONINARTICLE),
           "NATIVE_CONTENT_POSITION_IN_FEED" => Ok(NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONINFEED),
           "NATIVE_CONTENT_POSITION_PERIPHERAL" => Ok(NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONPERIPHERAL),
           "NATIVE_CONTENT_POSITION_RECOMMENDATION" => Ok(NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONRECOMMENDATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NativeContentPositionAssignedTargetingOptionDetailContentPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NativeContentPositionTargetingOptionDetailContentPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The content position.
pub enum NativeContentPositionTargetingOptionDetailContentPositionEnum {
    

    /// Native content position is not specified in this version. This enum is a place holder for a default value and does not represent a real native content position.
    ///
    /// "NATIVE_CONTENT_POSITION_UNSPECIFIED"
    #[serde(rename="NATIVE_CONTENT_POSITION_UNSPECIFIED")]
    NATIVECONTENTPOSITIONUNSPECIFIED,
    

    /// The native content position is unknown.
    ///
    /// "NATIVE_CONTENT_POSITION_UNKNOWN"
    #[serde(rename="NATIVE_CONTENT_POSITION_UNKNOWN")]
    NATIVECONTENTPOSITIONUNKNOWN,
    

    /// Native content position is in-article, i.e., ads appear between the paragraphs of pages.
    ///
    /// "NATIVE_CONTENT_POSITION_IN_ARTICLE"
    #[serde(rename="NATIVE_CONTENT_POSITION_IN_ARTICLE")]
    NATIVECONTENTPOSITIONINARTICLE,
    

    /// Native content position is in-feed, i.e., ads appear in a scrollable stream of content. A feed is typically editorial (e.g. a list of articles or news) or listings (e.g. a list of products or services).
    ///
    /// "NATIVE_CONTENT_POSITION_IN_FEED"
    #[serde(rename="NATIVE_CONTENT_POSITION_IN_FEED")]
    NATIVECONTENTPOSITIONINFEED,
    

    /// Native content position is peripheral, i.e., ads appear outside of core content on pages, such as the right- or left-hand side of the page.
    ///
    /// "NATIVE_CONTENT_POSITION_PERIPHERAL"
    #[serde(rename="NATIVE_CONTENT_POSITION_PERIPHERAL")]
    NATIVECONTENTPOSITIONPERIPHERAL,
    

    /// Native content position is recommendation, i.e., ads appear in sections for recommended content.
    ///
    /// "NATIVE_CONTENT_POSITION_RECOMMENDATION"
    #[serde(rename="NATIVE_CONTENT_POSITION_RECOMMENDATION")]
    NATIVECONTENTPOSITIONRECOMMENDATION,
}

impl AsRef<str> for NativeContentPositionTargetingOptionDetailContentPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONUNSPECIFIED => "NATIVE_CONTENT_POSITION_UNSPECIFIED",
            NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONUNKNOWN => "NATIVE_CONTENT_POSITION_UNKNOWN",
            NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONINARTICLE => "NATIVE_CONTENT_POSITION_IN_ARTICLE",
            NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONINFEED => "NATIVE_CONTENT_POSITION_IN_FEED",
            NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONPERIPHERAL => "NATIVE_CONTENT_POSITION_PERIPHERAL",
            NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONRECOMMENDATION => "NATIVE_CONTENT_POSITION_RECOMMENDATION",
        }
    }
}

impl std::convert::TryFrom< &str> for NativeContentPositionTargetingOptionDetailContentPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NATIVE_CONTENT_POSITION_UNSPECIFIED" => Ok(NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONUNSPECIFIED),
           "NATIVE_CONTENT_POSITION_UNKNOWN" => Ok(NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONUNKNOWN),
           "NATIVE_CONTENT_POSITION_IN_ARTICLE" => Ok(NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONINARTICLE),
           "NATIVE_CONTENT_POSITION_IN_FEED" => Ok(NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONINFEED),
           "NATIVE_CONTENT_POSITION_PERIPHERAL" => Ok(NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONPERIPHERAL),
           "NATIVE_CONTENT_POSITION_RECOMMENDATION" => Ok(NativeContentPositionTargetingOptionDetailContentPositionEnum::NATIVECONTENTPOSITIONRECOMMENDATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NativeContentPositionTargetingOptionDetailContentPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ObaIconPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The position of the OBA icon on the creative.
pub enum ObaIconPositionEnum {
    

    /// The OBA icon position is not specified.
    ///
    /// "OBA_ICON_POSITION_UNSPECIFIED"
    #[serde(rename="OBA_ICON_POSITION_UNSPECIFIED")]
    OBAICONPOSITIONUNSPECIFIED,
    

    /// At the upper right side of the creative.
    ///
    /// "OBA_ICON_POSITION_UPPER_RIGHT"
    #[serde(rename="OBA_ICON_POSITION_UPPER_RIGHT")]
    OBAICONPOSITIONUPPERRIGHT,
    

    /// At the upper left side of the creative.
    ///
    /// "OBA_ICON_POSITION_UPPER_LEFT"
    #[serde(rename="OBA_ICON_POSITION_UPPER_LEFT")]
    OBAICONPOSITIONUPPERLEFT,
    

    /// At the lower right side of the creative.
    ///
    /// "OBA_ICON_POSITION_LOWER_RIGHT"
    #[serde(rename="OBA_ICON_POSITION_LOWER_RIGHT")]
    OBAICONPOSITIONLOWERRIGHT,
    

    /// At the lower left side of the creative.
    ///
    /// "OBA_ICON_POSITION_LOWER_LEFT"
    #[serde(rename="OBA_ICON_POSITION_LOWER_LEFT")]
    OBAICONPOSITIONLOWERLEFT,
}

impl AsRef<str> for ObaIconPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ObaIconPositionEnum::OBAICONPOSITIONUNSPECIFIED => "OBA_ICON_POSITION_UNSPECIFIED",
            ObaIconPositionEnum::OBAICONPOSITIONUPPERRIGHT => "OBA_ICON_POSITION_UPPER_RIGHT",
            ObaIconPositionEnum::OBAICONPOSITIONUPPERLEFT => "OBA_ICON_POSITION_UPPER_LEFT",
            ObaIconPositionEnum::OBAICONPOSITIONLOWERRIGHT => "OBA_ICON_POSITION_LOWER_RIGHT",
            ObaIconPositionEnum::OBAICONPOSITIONLOWERLEFT => "OBA_ICON_POSITION_LOWER_LEFT",
        }
    }
}

impl std::convert::TryFrom< &str> for ObaIconPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OBA_ICON_POSITION_UNSPECIFIED" => Ok(ObaIconPositionEnum::OBAICONPOSITIONUNSPECIFIED),
           "OBA_ICON_POSITION_UPPER_RIGHT" => Ok(ObaIconPositionEnum::OBAICONPOSITIONUPPERRIGHT),
           "OBA_ICON_POSITION_UPPER_LEFT" => Ok(ObaIconPositionEnum::OBAICONPOSITIONUPPERLEFT),
           "OBA_ICON_POSITION_LOWER_RIGHT" => Ok(ObaIconPositionEnum::OBAICONPOSITIONLOWERRIGHT),
           "OBA_ICON_POSITION_LOWER_LEFT" => Ok(ObaIconPositionEnum::OBAICONPOSITIONLOWERLEFT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ObaIconPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OmidAssignedTargetingOptionDetailOmidEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of Open Measurement enabled inventory.
pub enum OmidAssignedTargetingOptionDetailOmidEnum {
    

    /// Default value when omid targeting is not specified in this version.
    ///
    /// "OMID_UNSPECIFIED"
    #[serde(rename="OMID_UNSPECIFIED")]
    OMIDUNSPECIFIED,
    

    /// Open Measurement enabled mobile display inventory.
    ///
    /// "OMID_FOR_MOBILE_DISPLAY_ADS"
    #[serde(rename="OMID_FOR_MOBILE_DISPLAY_ADS")]
    OMIDFORMOBILEDISPLAYADS,
}

impl AsRef<str> for OmidAssignedTargetingOptionDetailOmidEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OmidAssignedTargetingOptionDetailOmidEnum::OMIDUNSPECIFIED => "OMID_UNSPECIFIED",
            OmidAssignedTargetingOptionDetailOmidEnum::OMIDFORMOBILEDISPLAYADS => "OMID_FOR_MOBILE_DISPLAY_ADS",
        }
    }
}

impl std::convert::TryFrom< &str> for OmidAssignedTargetingOptionDetailOmidEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OMID_UNSPECIFIED" => Ok(OmidAssignedTargetingOptionDetailOmidEnum::OMIDUNSPECIFIED),
           "OMID_FOR_MOBILE_DISPLAY_ADS" => Ok(OmidAssignedTargetingOptionDetailOmidEnum::OMIDFORMOBILEDISPLAYADS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OmidAssignedTargetingOptionDetailOmidEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OmidTargetingOptionDetailOmidEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of Open Measurement enabled inventory.
pub enum OmidTargetingOptionDetailOmidEnum {
    

    /// Default value when omid targeting is not specified in this version.
    ///
    /// "OMID_UNSPECIFIED"
    #[serde(rename="OMID_UNSPECIFIED")]
    OMIDUNSPECIFIED,
    

    /// Open Measurement enabled mobile display inventory.
    ///
    /// "OMID_FOR_MOBILE_DISPLAY_ADS"
    #[serde(rename="OMID_FOR_MOBILE_DISPLAY_ADS")]
    OMIDFORMOBILEDISPLAYADS,
}

impl AsRef<str> for OmidTargetingOptionDetailOmidEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OmidTargetingOptionDetailOmidEnum::OMIDUNSPECIFIED => "OMID_UNSPECIFIED",
            OmidTargetingOptionDetailOmidEnum::OMIDFORMOBILEDISPLAYADS => "OMID_FOR_MOBILE_DISPLAY_ADS",
        }
    }
}

impl std::convert::TryFrom< &str> for OmidTargetingOptionDetailOmidEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OMID_UNSPECIFIED" => Ok(OmidTargetingOptionDetailOmidEnum::OMIDUNSPECIFIED),
           "OMID_FOR_MOBILE_DISPLAY_ADS" => Ok(OmidTargetingOptionDetailOmidEnum::OMIDFORMOBILEDISPLAYADS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OmidTargetingOptionDetailOmidEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_DISPLAY`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_DISPLAY_DEFAULT`. * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`.
pub enum OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum {
    

    /// Ad type is not specified or is unknown in this version.
    ///
    /// "AD_TYPE_UNSPECIFIED"
    #[serde(rename="AD_TYPE_UNSPECIFIED")]
    ADTYPEUNSPECIFIED,
    

    /// Display creatives, e.g. image and HTML5.
    ///
    /// "AD_TYPE_DISPLAY"
    #[serde(rename="AD_TYPE_DISPLAY")]
    ADTYPEDISPLAY,
    

    /// Video creatives, e.g. video ads that play during streaming content in video players.
    ///
    /// "AD_TYPE_VIDEO"
    #[serde(rename="AD_TYPE_VIDEO")]
    ADTYPEVIDEO,
    

    /// Audio creatives, e.g. audio ads that play during audio content.
    ///
    /// "AD_TYPE_AUDIO"
    #[serde(rename="AD_TYPE_AUDIO")]
    ADTYPEAUDIO,
}

impl AsRef<str> for OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEUNSPECIFIED => "AD_TYPE_UNSPECIFIED",
            OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEDISPLAY => "AD_TYPE_DISPLAY",
            OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEVIDEO => "AD_TYPE_VIDEO",
            OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEAUDIO => "AD_TYPE_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AD_TYPE_UNSPECIFIED" => Ok(OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEUNSPECIFIED),
           "AD_TYPE_DISPLAY" => Ok(OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEDISPLAY),
           "AD_TYPE_VIDEO" => Ok(OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEVIDEO),
           "AD_TYPE_AUDIO" => Ok(OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum::ADTYPEAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OnScreenPositionAssignedTargetingOptionDetailAdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The on screen position.
pub enum OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum {
    

    /// On screen position is not specified in this version. This enum is a place holder for a default value and does not represent a real on screen position.
    ///
    /// "ON_SCREEN_POSITION_UNSPECIFIED"
    #[serde(rename="ON_SCREEN_POSITION_UNSPECIFIED")]
    ONSCREENPOSITIONUNSPECIFIED,
    

    /// The ad position is unknown on the screen.
    ///
    /// "ON_SCREEN_POSITION_UNKNOWN"
    #[serde(rename="ON_SCREEN_POSITION_UNKNOWN")]
    ONSCREENPOSITIONUNKNOWN,
    

    /// The ad is located above the fold.
    ///
    /// "ON_SCREEN_POSITION_ABOVE_THE_FOLD"
    #[serde(rename="ON_SCREEN_POSITION_ABOVE_THE_FOLD")]
    ONSCREENPOSITIONABOVETHEFOLD,
    

    /// The ad is located below the fold.
    ///
    /// "ON_SCREEN_POSITION_BELOW_THE_FOLD"
    #[serde(rename="ON_SCREEN_POSITION_BELOW_THE_FOLD")]
    ONSCREENPOSITIONBELOWTHEFOLD,
}

impl AsRef<str> for OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONUNSPECIFIED => "ON_SCREEN_POSITION_UNSPECIFIED",
            OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONUNKNOWN => "ON_SCREEN_POSITION_UNKNOWN",
            OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONABOVETHEFOLD => "ON_SCREEN_POSITION_ABOVE_THE_FOLD",
            OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONBELOWTHEFOLD => "ON_SCREEN_POSITION_BELOW_THE_FOLD",
        }
    }
}

impl std::convert::TryFrom< &str> for OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ON_SCREEN_POSITION_UNSPECIFIED" => Ok(OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONUNSPECIFIED),
           "ON_SCREEN_POSITION_UNKNOWN" => Ok(OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONUNKNOWN),
           "ON_SCREEN_POSITION_ABOVE_THE_FOLD" => Ok(OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONABOVETHEFOLD),
           "ON_SCREEN_POSITION_BELOW_THE_FOLD" => Ok(OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONBELOWTHEFOLD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OnScreenPositionAssignedTargetingOptionDetailOnScreenPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OnScreenPositionTargetingOptionDetailOnScreenPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The on screen position.
pub enum OnScreenPositionTargetingOptionDetailOnScreenPositionEnum {
    

    /// On screen position is not specified in this version. This enum is a place holder for a default value and does not represent a real on screen position.
    ///
    /// "ON_SCREEN_POSITION_UNSPECIFIED"
    #[serde(rename="ON_SCREEN_POSITION_UNSPECIFIED")]
    ONSCREENPOSITIONUNSPECIFIED,
    

    /// The ad position is unknown on the screen.
    ///
    /// "ON_SCREEN_POSITION_UNKNOWN"
    #[serde(rename="ON_SCREEN_POSITION_UNKNOWN")]
    ONSCREENPOSITIONUNKNOWN,
    

    /// The ad is located above the fold.
    ///
    /// "ON_SCREEN_POSITION_ABOVE_THE_FOLD"
    #[serde(rename="ON_SCREEN_POSITION_ABOVE_THE_FOLD")]
    ONSCREENPOSITIONABOVETHEFOLD,
    

    /// The ad is located below the fold.
    ///
    /// "ON_SCREEN_POSITION_BELOW_THE_FOLD"
    #[serde(rename="ON_SCREEN_POSITION_BELOW_THE_FOLD")]
    ONSCREENPOSITIONBELOWTHEFOLD,
}

impl AsRef<str> for OnScreenPositionTargetingOptionDetailOnScreenPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OnScreenPositionTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONUNSPECIFIED => "ON_SCREEN_POSITION_UNSPECIFIED",
            OnScreenPositionTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONUNKNOWN => "ON_SCREEN_POSITION_UNKNOWN",
            OnScreenPositionTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONABOVETHEFOLD => "ON_SCREEN_POSITION_ABOVE_THE_FOLD",
            OnScreenPositionTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONBELOWTHEFOLD => "ON_SCREEN_POSITION_BELOW_THE_FOLD",
        }
    }
}

impl std::convert::TryFrom< &str> for OnScreenPositionTargetingOptionDetailOnScreenPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ON_SCREEN_POSITION_UNSPECIFIED" => Ok(OnScreenPositionTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONUNSPECIFIED),
           "ON_SCREEN_POSITION_UNKNOWN" => Ok(OnScreenPositionTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONUNKNOWN),
           "ON_SCREEN_POSITION_ABOVE_THE_FOLD" => Ok(OnScreenPositionTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONABOVETHEFOLD),
           "ON_SCREEN_POSITION_BELOW_THE_FOLD" => Ok(OnScreenPositionTargetingOptionDetailOnScreenPositionEnum::ONSCREENPOSITIONBELOWTHEFOLD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OnScreenPositionTargetingOptionDetailOnScreenPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PacingPacingPeriodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The time period in which the pacing budget will be spent. When automatic budget allocation is enabled at the insertion order via automationType, this field is output only and defaults to `PACING_PERIOD_FLIGHT`.
pub enum PacingPacingPeriodEnum {
    

    /// Period value is not specified or is unknown in this version.
    ///
    /// "PACING_PERIOD_UNSPECIFIED"
    #[serde(rename="PACING_PERIOD_UNSPECIFIED")]
    PACINGPERIODUNSPECIFIED,
    

    /// The pacing setting will be applied on daily basis.
    ///
    /// "PACING_PERIOD_DAILY"
    #[serde(rename="PACING_PERIOD_DAILY")]
    PACINGPERIODDAILY,
    

    /// The pacing setting will be applied to the whole flight duration.
    ///
    /// "PACING_PERIOD_FLIGHT"
    #[serde(rename="PACING_PERIOD_FLIGHT")]
    PACINGPERIODFLIGHT,
}

impl AsRef<str> for PacingPacingPeriodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PacingPacingPeriodEnum::PACINGPERIODUNSPECIFIED => "PACING_PERIOD_UNSPECIFIED",
            PacingPacingPeriodEnum::PACINGPERIODDAILY => "PACING_PERIOD_DAILY",
            PacingPacingPeriodEnum::PACINGPERIODFLIGHT => "PACING_PERIOD_FLIGHT",
        }
    }
}

impl std::convert::TryFrom< &str> for PacingPacingPeriodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PACING_PERIOD_UNSPECIFIED" => Ok(PacingPacingPeriodEnum::PACINGPERIODUNSPECIFIED),
           "PACING_PERIOD_DAILY" => Ok(PacingPacingPeriodEnum::PACINGPERIODDAILY),
           "PACING_PERIOD_FLIGHT" => Ok(PacingPacingPeriodEnum::PACINGPERIODFLIGHT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PacingPacingPeriodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PacingPacingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of pacing that defines how the budget amount will be spent across the pacing_period.
pub enum PacingPacingTypeEnum {
    

    /// Pacing mode value is not specified or is unknown in this version.
    ///
    /// "PACING_TYPE_UNSPECIFIED"
    #[serde(rename="PACING_TYPE_UNSPECIFIED")]
    PACINGTYPEUNSPECIFIED,
    

    /// Only applicable to `PACING_PERIOD_FLIGHT` pacing period. Ahead pacing attempts to spend faster than evenly, to make sure the entire budget is spent by the end of the flight.
    ///
    /// "PACING_TYPE_AHEAD"
    #[serde(rename="PACING_TYPE_AHEAD")]
    PACINGTYPEAHEAD,
    

    /// Spend all of pacing budget amount as quick as possible.
    ///
    /// "PACING_TYPE_ASAP"
    #[serde(rename="PACING_TYPE_ASAP")]
    PACINGTYPEASAP,
    

    /// Spend a consistent budget amount every period of time.
    ///
    /// "PACING_TYPE_EVEN"
    #[serde(rename="PACING_TYPE_EVEN")]
    PACINGTYPEEVEN,
}

impl AsRef<str> for PacingPacingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PacingPacingTypeEnum::PACINGTYPEUNSPECIFIED => "PACING_TYPE_UNSPECIFIED",
            PacingPacingTypeEnum::PACINGTYPEAHEAD => "PACING_TYPE_AHEAD",
            PacingPacingTypeEnum::PACINGTYPEASAP => "PACING_TYPE_ASAP",
            PacingPacingTypeEnum::PACINGTYPEEVEN => "PACING_TYPE_EVEN",
        }
    }
}

impl std::convert::TryFrom< &str> for PacingPacingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PACING_TYPE_UNSPECIFIED" => Ok(PacingPacingTypeEnum::PACINGTYPEUNSPECIFIED),
           "PACING_TYPE_AHEAD" => Ok(PacingPacingTypeEnum::PACINGTYPEAHEAD),
           "PACING_TYPE_ASAP" => Ok(PacingPacingTypeEnum::PACINGTYPEASAP),
           "PACING_TYPE_EVEN" => Ok(PacingPacingTypeEnum::PACINGTYPEEVEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PacingPacingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParentEntityFilterFileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. File types that will be returned.
pub enum ParentEntityFilterFileTypeEnum {
    

    /// Default value when type is unspecified or is unknown in this version.
    ///
    /// "FILE_TYPE_UNSPECIFIED"
    #[serde(rename="FILE_TYPE_UNSPECIFIED")]
    FILETYPEUNSPECIFIED,
    

    /// Campaign.
    ///
    /// "FILE_TYPE_CAMPAIGN"
    #[serde(rename="FILE_TYPE_CAMPAIGN")]
    FILETYPECAMPAIGN,
    

    /// Media Product.
    ///
    /// "FILE_TYPE_MEDIA_PRODUCT"
    #[serde(rename="FILE_TYPE_MEDIA_PRODUCT")]
    FILETYPEMEDIAPRODUCT,
    

    /// Insertion Order.
    ///
    /// "FILE_TYPE_INSERTION_ORDER"
    #[serde(rename="FILE_TYPE_INSERTION_ORDER")]
    FILETYPEINSERTIONORDER,
    

    /// Line Item.
    ///
    /// "FILE_TYPE_LINE_ITEM"
    #[serde(rename="FILE_TYPE_LINE_ITEM")]
    FILETYPELINEITEM,
    

    /// YouTube Ad Group.
    ///
    /// "FILE_TYPE_AD_GROUP"
    #[serde(rename="FILE_TYPE_AD_GROUP")]
    FILETYPEADGROUP,
    

    /// YouTube Ad.
    ///
    /// "FILE_TYPE_AD"
    #[serde(rename="FILE_TYPE_AD")]
    FILETYPEAD,
}

impl AsRef<str> for ParentEntityFilterFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParentEntityFilterFileTypeEnum::FILETYPEUNSPECIFIED => "FILE_TYPE_UNSPECIFIED",
            ParentEntityFilterFileTypeEnum::FILETYPECAMPAIGN => "FILE_TYPE_CAMPAIGN",
            ParentEntityFilterFileTypeEnum::FILETYPEMEDIAPRODUCT => "FILE_TYPE_MEDIA_PRODUCT",
            ParentEntityFilterFileTypeEnum::FILETYPEINSERTIONORDER => "FILE_TYPE_INSERTION_ORDER",
            ParentEntityFilterFileTypeEnum::FILETYPELINEITEM => "FILE_TYPE_LINE_ITEM",
            ParentEntityFilterFileTypeEnum::FILETYPEADGROUP => "FILE_TYPE_AD_GROUP",
            ParentEntityFilterFileTypeEnum::FILETYPEAD => "FILE_TYPE_AD",
        }
    }
}

impl std::convert::TryFrom< &str> for ParentEntityFilterFileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILE_TYPE_UNSPECIFIED" => Ok(ParentEntityFilterFileTypeEnum::FILETYPEUNSPECIFIED),
           "FILE_TYPE_CAMPAIGN" => Ok(ParentEntityFilterFileTypeEnum::FILETYPECAMPAIGN),
           "FILE_TYPE_MEDIA_PRODUCT" => Ok(ParentEntityFilterFileTypeEnum::FILETYPEMEDIAPRODUCT),
           "FILE_TYPE_INSERTION_ORDER" => Ok(ParentEntityFilterFileTypeEnum::FILETYPEINSERTIONORDER),
           "FILE_TYPE_LINE_ITEM" => Ok(ParentEntityFilterFileTypeEnum::FILETYPELINEITEM),
           "FILE_TYPE_AD_GROUP" => Ok(ParentEntityFilterFileTypeEnum::FILETYPEADGROUP),
           "FILE_TYPE_AD" => Ok(ParentEntityFilterFileTypeEnum::FILETYPEAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParentEntityFilterFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParentEntityFilterFilterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Filter type used to filter fetched entities.
pub enum ParentEntityFilterFilterTypeEnum {
    

    /// Default value when type is unspecified or is unknown in this version.
    ///
    /// "FILTER_TYPE_UNSPECIFIED"
    #[serde(rename="FILTER_TYPE_UNSPECIFIED")]
    FILTERTYPEUNSPECIFIED,
    

    /// If selected, no filter will be applied to the download. Can only be used if an Advertiser is specified in CreateSdfDownloadTaskRequest.
    ///
    /// "FILTER_TYPE_NONE"
    #[serde(rename="FILTER_TYPE_NONE")]
    FILTERTYPENONE,
    

    /// Advertiser ID. If selected, all filter IDs must be Advertiser IDs that belong to the Partner specified in CreateSdfDownloadTaskRequest.
    ///
    /// "FILTER_TYPE_ADVERTISER_ID"
    #[serde(rename="FILTER_TYPE_ADVERTISER_ID")]
    FILTERTYPEADVERTISERID,
    

    /// Campaign ID. If selected, all filter IDs must be Campaign IDs that belong to the Advertiser or Partner specified in CreateSdfDownloadTaskRequest.
    ///
    /// "FILTER_TYPE_CAMPAIGN_ID"
    #[serde(rename="FILTER_TYPE_CAMPAIGN_ID")]
    FILTERTYPECAMPAIGNID,
    

    /// Media Product ID. If selected, all filter IDs must be Media Product IDs that belong to the Advertiser or Partner specified in CreateSdfDownloadTaskRequest. Can only be used for downloading `FILE_TYPE_MEDIA_PRODUCT`.
    ///
    /// "FILTER_TYPE_MEDIA_PRODUCT_ID"
    #[serde(rename="FILTER_TYPE_MEDIA_PRODUCT_ID")]
    FILTERTYPEMEDIAPRODUCTID,
    

    /// Insertion Order ID. If selected, all filter IDs must be Insertion Order IDs that belong to the Advertiser or Partner specified in CreateSdfDownloadTaskRequest. Can only be used for downloading `FILE_TYPE_INSERTION_ORDER`, `FILE_TYPE_LINE_ITEM`, `FILE_TYPE_AD_GROUP`, and `FILE_TYPE_AD`.
    ///
    /// "FILTER_TYPE_INSERTION_ORDER_ID"
    #[serde(rename="FILTER_TYPE_INSERTION_ORDER_ID")]
    FILTERTYPEINSERTIONORDERID,
    

    /// Line Item ID. If selected, all filter IDs must be Line Item IDs that belong to the Advertiser or Partner specified in CreateSdfDownloadTaskRequest. Can only be used for downloading `FILE_TYPE_LINE_ITEM`, `FILE_TYPE_AD_GROUP`, and `FILE_TYPE_AD`.
    ///
    /// "FILTER_TYPE_LINE_ITEM_ID"
    #[serde(rename="FILTER_TYPE_LINE_ITEM_ID")]
    FILTERTYPELINEITEMID,
}

impl AsRef<str> for ParentEntityFilterFilterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParentEntityFilterFilterTypeEnum::FILTERTYPEUNSPECIFIED => "FILTER_TYPE_UNSPECIFIED",
            ParentEntityFilterFilterTypeEnum::FILTERTYPENONE => "FILTER_TYPE_NONE",
            ParentEntityFilterFilterTypeEnum::FILTERTYPEADVERTISERID => "FILTER_TYPE_ADVERTISER_ID",
            ParentEntityFilterFilterTypeEnum::FILTERTYPECAMPAIGNID => "FILTER_TYPE_CAMPAIGN_ID",
            ParentEntityFilterFilterTypeEnum::FILTERTYPEMEDIAPRODUCTID => "FILTER_TYPE_MEDIA_PRODUCT_ID",
            ParentEntityFilterFilterTypeEnum::FILTERTYPEINSERTIONORDERID => "FILTER_TYPE_INSERTION_ORDER_ID",
            ParentEntityFilterFilterTypeEnum::FILTERTYPELINEITEMID => "FILTER_TYPE_LINE_ITEM_ID",
        }
    }
}

impl std::convert::TryFrom< &str> for ParentEntityFilterFilterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILTER_TYPE_UNSPECIFIED" => Ok(ParentEntityFilterFilterTypeEnum::FILTERTYPEUNSPECIFIED),
           "FILTER_TYPE_NONE" => Ok(ParentEntityFilterFilterTypeEnum::FILTERTYPENONE),
           "FILTER_TYPE_ADVERTISER_ID" => Ok(ParentEntityFilterFilterTypeEnum::FILTERTYPEADVERTISERID),
           "FILTER_TYPE_CAMPAIGN_ID" => Ok(ParentEntityFilterFilterTypeEnum::FILTERTYPECAMPAIGNID),
           "FILTER_TYPE_MEDIA_PRODUCT_ID" => Ok(ParentEntityFilterFilterTypeEnum::FILTERTYPEMEDIAPRODUCTID),
           "FILTER_TYPE_INSERTION_ORDER_ID" => Ok(ParentEntityFilterFilterTypeEnum::FILTERTYPEINSERTIONORDERID),
           "FILTER_TYPE_LINE_ITEM_ID" => Ok(ParentEntityFilterFilterTypeEnum::FILTERTYPELINEITEMID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParentEntityFilterFilterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The parental status of the audience.
pub enum ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum {
    

    /// Default value when parental status is not specified in this version. This enum is a place holder for default value and does not represent a real parental status option.
    ///
    /// "PARENTAL_STATUS_UNSPECIFIED"
    #[serde(rename="PARENTAL_STATUS_UNSPECIFIED")]
    PARENTALSTATUSUNSPECIFIED,
    

    /// The audience is a parent.
    ///
    /// "PARENTAL_STATUS_PARENT"
    #[serde(rename="PARENTAL_STATUS_PARENT")]
    PARENTALSTATUSPARENT,
    

    /// The audience is not a parent.
    ///
    /// "PARENTAL_STATUS_NOT_A_PARENT"
    #[serde(rename="PARENTAL_STATUS_NOT_A_PARENT")]
    PARENTALSTATUSNOTAPARENT,
    

    /// The parental status of the audience is unknown.
    ///
    /// "PARENTAL_STATUS_UNKNOWN"
    #[serde(rename="PARENTAL_STATUS_UNKNOWN")]
    PARENTALSTATUSUNKNOWN,
}

impl AsRef<str> for ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSUNSPECIFIED => "PARENTAL_STATUS_UNSPECIFIED",
            ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSPARENT => "PARENTAL_STATUS_PARENT",
            ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSNOTAPARENT => "PARENTAL_STATUS_NOT_A_PARENT",
            ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSUNKNOWN => "PARENTAL_STATUS_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARENTAL_STATUS_UNSPECIFIED" => Ok(ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSUNSPECIFIED),
           "PARENTAL_STATUS_PARENT" => Ok(ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSPARENT),
           "PARENTAL_STATUS_NOT_A_PARENT" => Ok(ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSNOTAPARENT),
           "PARENTAL_STATUS_UNKNOWN" => Ok(ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParentalStatusAssignedTargetingOptionDetailParentalStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParentalStatusTargetingOptionDetailParentalStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The parental status of an audience.
pub enum ParentalStatusTargetingOptionDetailParentalStatusEnum {
    

    /// Default value when parental status is not specified in this version. This enum is a place holder for default value and does not represent a real parental status option.
    ///
    /// "PARENTAL_STATUS_UNSPECIFIED"
    #[serde(rename="PARENTAL_STATUS_UNSPECIFIED")]
    PARENTALSTATUSUNSPECIFIED,
    

    /// The audience is a parent.
    ///
    /// "PARENTAL_STATUS_PARENT"
    #[serde(rename="PARENTAL_STATUS_PARENT")]
    PARENTALSTATUSPARENT,
    

    /// The audience is not a parent.
    ///
    /// "PARENTAL_STATUS_NOT_A_PARENT"
    #[serde(rename="PARENTAL_STATUS_NOT_A_PARENT")]
    PARENTALSTATUSNOTAPARENT,
    

    /// The parental status of the audience is unknown.
    ///
    /// "PARENTAL_STATUS_UNKNOWN"
    #[serde(rename="PARENTAL_STATUS_UNKNOWN")]
    PARENTALSTATUSUNKNOWN,
}

impl AsRef<str> for ParentalStatusTargetingOptionDetailParentalStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParentalStatusTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSUNSPECIFIED => "PARENTAL_STATUS_UNSPECIFIED",
            ParentalStatusTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSPARENT => "PARENTAL_STATUS_PARENT",
            ParentalStatusTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSNOTAPARENT => "PARENTAL_STATUS_NOT_A_PARENT",
            ParentalStatusTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSUNKNOWN => "PARENTAL_STATUS_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for ParentalStatusTargetingOptionDetailParentalStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARENTAL_STATUS_UNSPECIFIED" => Ok(ParentalStatusTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSUNSPECIFIED),
           "PARENTAL_STATUS_PARENT" => Ok(ParentalStatusTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSPARENT),
           "PARENTAL_STATUS_NOT_A_PARENT" => Ok(ParentalStatusTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSNOTAPARENT),
           "PARENTAL_STATUS_UNKNOWN" => Ok(ParentalStatusTargetingOptionDetailParentalStatusEnum::PARENTALSTATUSUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParentalStatusTargetingOptionDetailParentalStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartnerEntityStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The status of the partner.
pub enum PartnerEntityStatusEnum {
    

    /// Default value when status is not specified or is unknown in this version.
    ///
    /// "ENTITY_STATUS_UNSPECIFIED"
    #[serde(rename="ENTITY_STATUS_UNSPECIFIED")]
    ENTITYSTATUSUNSPECIFIED,
    

    /// The entity is enabled to bid and spend budget.
    ///
    /// "ENTITY_STATUS_ACTIVE"
    #[serde(rename="ENTITY_STATUS_ACTIVE")]
    ENTITYSTATUSACTIVE,
    

    /// The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved.
    ///
    /// "ENTITY_STATUS_ARCHIVED"
    #[serde(rename="ENTITY_STATUS_ARCHIVED")]
    ENTITYSTATUSARCHIVED,
    

    /// The entity is under draft. Bidding and budget spending are disabled.
    ///
    /// "ENTITY_STATUS_DRAFT"
    #[serde(rename="ENTITY_STATUS_DRAFT")]
    ENTITYSTATUSDRAFT,
    

    /// Bidding and budget spending are paused for the entity.
    ///
    /// "ENTITY_STATUS_PAUSED"
    #[serde(rename="ENTITY_STATUS_PAUSED")]
    ENTITYSTATUSPAUSED,
    

    /// The entity is scheduled for deletion.
    ///
    /// "ENTITY_STATUS_SCHEDULED_FOR_DELETION"
    #[serde(rename="ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    ENTITYSTATUSSCHEDULEDFORDELETION,
}

impl AsRef<str> for PartnerEntityStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartnerEntityStatusEnum::ENTITYSTATUSUNSPECIFIED => "ENTITY_STATUS_UNSPECIFIED",
            PartnerEntityStatusEnum::ENTITYSTATUSACTIVE => "ENTITY_STATUS_ACTIVE",
            PartnerEntityStatusEnum::ENTITYSTATUSARCHIVED => "ENTITY_STATUS_ARCHIVED",
            PartnerEntityStatusEnum::ENTITYSTATUSDRAFT => "ENTITY_STATUS_DRAFT",
            PartnerEntityStatusEnum::ENTITYSTATUSPAUSED => "ENTITY_STATUS_PAUSED",
            PartnerEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION => "ENTITY_STATUS_SCHEDULED_FOR_DELETION",
        }
    }
}

impl std::convert::TryFrom< &str> for PartnerEntityStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_STATUS_UNSPECIFIED" => Ok(PartnerEntityStatusEnum::ENTITYSTATUSUNSPECIFIED),
           "ENTITY_STATUS_ACTIVE" => Ok(PartnerEntityStatusEnum::ENTITYSTATUSACTIVE),
           "ENTITY_STATUS_ARCHIVED" => Ok(PartnerEntityStatusEnum::ENTITYSTATUSARCHIVED),
           "ENTITY_STATUS_DRAFT" => Ok(PartnerEntityStatusEnum::ENTITYSTATUSDRAFT),
           "ENTITY_STATUS_PAUSED" => Ok(PartnerEntityStatusEnum::ENTITYSTATUSPAUSED),
           "ENTITY_STATUS_SCHEDULED_FOR_DELETION" => Ok(PartnerEntityStatusEnum::ENTITYSTATUSSCHEDULEDFORDELETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartnerEntityStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartnerCostCostTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the partner cost.
pub enum PartnerCostCostTypeEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "PARTNER_COST_TYPE_UNSPECIFIED"
    #[serde(rename="PARTNER_COST_TYPE_UNSPECIFIED")]
    PARTNERCOSTTYPEUNSPECIFIED,
    

    /// The cost is charged for using Adloox. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_ADLOOX"
    #[serde(rename="PARTNER_COST_TYPE_ADLOOX")]
    PARTNERCOSTTYPEADLOOX,
    

    /// The cost is charged for using Adloox Pre-Bid. Billed through DV360.
    ///
    /// "PARTNER_COST_TYPE_ADLOOX_PREBID"
    #[serde(rename="PARTNER_COST_TYPE_ADLOOX_PREBID")]
    PARTNERCOSTTYPEADLOOXPREBID,
    

    /// The cost is charged for using AdSafe. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_ADSAFE"
    #[serde(rename="PARTNER_COST_TYPE_ADSAFE")]
    PARTNERCOSTTYPEADSAFE,
    

    /// The cost is charged for using AdExpose. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_ADXPOSE"
    #[serde(rename="PARTNER_COST_TYPE_ADXPOSE")]
    PARTNERCOSTTYPEADXPOSE,
    

    /// The cost is charged for using Aggregate Knowledge. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_AGGREGATE_KNOWLEDGE"
    #[serde(rename="PARTNER_COST_TYPE_AGGREGATE_KNOWLEDGE")]
    PARTNERCOSTTYPEAGGREGATEKNOWLEDGE,
    

    /// The cost is charged for using an Agency Trading Desk. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_AGENCY_TRADING_DESK"
    #[serde(rename="PARTNER_COST_TYPE_AGENCY_TRADING_DESK")]
    PARTNERCOSTTYPEAGENCYTRADINGDESK,
    

    /// The cost is charged for using DV360. Billed through DV360.
    ///
    /// "PARTNER_COST_TYPE_DV360_FEE"
    #[serde(rename="PARTNER_COST_TYPE_DV360_FEE")]
    PARTNERCOSTTYPEDV360FEE,
    

    /// The cost is charged for using comScore vCE. Billed through DV360.
    ///
    /// "PARTNER_COST_TYPE_COMSCORE_VCE"
    #[serde(rename="PARTNER_COST_TYPE_COMSCORE_VCE")]
    PARTNERCOSTTYPECOMSCOREVCE,
    

    /// The cost is charged for using a Data Management Platform. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_DATA_MANAGEMENT_PLATFORM"
    #[serde(rename="PARTNER_COST_TYPE_DATA_MANAGEMENT_PLATFORM")]
    PARTNERCOSTTYPEDATAMANAGEMENTPLATFORM,
    

    /// The default cost type. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_DEFAULT"
    #[serde(rename="PARTNER_COST_TYPE_DEFAULT")]
    PARTNERCOSTTYPEDEFAULT,
    

    /// The cost is charged for using DoubleVerify. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_DOUBLE_VERIFY"
    #[serde(rename="PARTNER_COST_TYPE_DOUBLE_VERIFY")]
    PARTNERCOSTTYPEDOUBLEVERIFY,
    

    /// The cost is charged for using DoubleVerify Pre-Bid. Billed through DV360.
    ///
    /// "PARTNER_COST_TYPE_DOUBLE_VERIFY_PREBID"
    #[serde(rename="PARTNER_COST_TYPE_DOUBLE_VERIFY_PREBID")]
    PARTNERCOSTTYPEDOUBLEVERIFYPREBID,
    

    /// The cost is charged for using Evidon. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_EVIDON"
    #[serde(rename="PARTNER_COST_TYPE_EVIDON")]
    PARTNERCOSTTYPEEVIDON,
    

    /// The cost is charged for using Integral Ad Science Video. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_VIDEO"
    #[serde(rename="PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_VIDEO")]
    PARTNERCOSTTYPEINTEGRALADSCIENCEVIDEO,
    

    /// The cost is charged for using Integral Ad Science Pre-Bid. Billed through DV360.
    ///
    /// "PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_PREBID"
    #[serde(rename="PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_PREBID")]
    PARTNERCOSTTYPEINTEGRALADSCIENCEPREBID,
    

    /// The cost is charged for using media cost data. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_MEDIA_COST_DATA"
    #[serde(rename="PARTNER_COST_TYPE_MEDIA_COST_DATA")]
    PARTNERCOSTTYPEMEDIACOSTDATA,
    

    /// The cost is charged for using MOAT Video. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_MOAT_VIDEO"
    #[serde(rename="PARTNER_COST_TYPE_MOAT_VIDEO")]
    PARTNERCOSTTYPEMOATVIDEO,
    

    /// The cost is charged for using Nielsen Digital Ad Ratings. Billed through DV360.
    ///
    /// "PARTNER_COST_TYPE_NIELSEN_DAR"
    #[serde(rename="PARTNER_COST_TYPE_NIELSEN_DAR")]
    PARTNERCOSTTYPENIELSENDAR,
    

    /// The cost is charged for using ShopLocal. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_SHOP_LOCAL"
    #[serde(rename="PARTNER_COST_TYPE_SHOP_LOCAL")]
    PARTNERCOSTTYPESHOPLOCAL,
    

    /// The cost is charged for using Teracent. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_TERACENT"
    #[serde(rename="PARTNER_COST_TYPE_TERACENT")]
    PARTNERCOSTTYPETERACENT,
    

    /// The cost is charged for using a third-party ad server. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_THIRD_PARTY_AD_SERVER"
    #[serde(rename="PARTNER_COST_TYPE_THIRD_PARTY_AD_SERVER")]
    PARTNERCOSTTYPETHIRDPARTYADSERVER,
    

    /// The cost is charged for using TrustMetrics. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_TRUST_METRICS"
    #[serde(rename="PARTNER_COST_TYPE_TRUST_METRICS")]
    PARTNERCOSTTYPETRUSTMETRICS,
    

    /// The cost is charged for using Vizu. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_VIZU"
    #[serde(rename="PARTNER_COST_TYPE_VIZU")]
    PARTNERCOSTTYPEVIZU,
    

    /// The cost is charged for using AdLingo. Billed through DV360.
    ///
    /// "PARTNER_COST_TYPE_ADLINGO_FEE"
    #[serde(rename="PARTNER_COST_TYPE_ADLINGO_FEE")]
    PARTNERCOSTTYPEADLINGOFEE,
    

    /// The cost is charged as custom fee 1. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_CUSTOM_FEE_1"
    #[serde(rename="PARTNER_COST_TYPE_CUSTOM_FEE_1")]
    PARTNERCOSTTYPECUSTOMFEE1,
    

    /// The cost is charged as custom fee 2. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_CUSTOM_FEE_2"
    #[serde(rename="PARTNER_COST_TYPE_CUSTOM_FEE_2")]
    PARTNERCOSTTYPECUSTOMFEE2,
    

    /// The cost is charged as custom fee 3. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_CUSTOM_FEE_3"
    #[serde(rename="PARTNER_COST_TYPE_CUSTOM_FEE_3")]
    PARTNERCOSTTYPECUSTOMFEE3,
    

    /// The cost is charged as custom fee 4. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_CUSTOM_FEE_4"
    #[serde(rename="PARTNER_COST_TYPE_CUSTOM_FEE_4")]
    PARTNERCOSTTYPECUSTOMFEE4,
    

    /// The cost is charged as custom fee 5. Billed by the partner.
    ///
    /// "PARTNER_COST_TYPE_CUSTOM_FEE_5"
    #[serde(rename="PARTNER_COST_TYPE_CUSTOM_FEE_5")]
    PARTNERCOSTTYPECUSTOMFEE5,
    

    /// The cost is charged for using Scibids. Billed through DV360. This type is currently only available to certain customers. Other customers attempting to use this type will receive an error.
    ///
    /// "PARTNER_COST_TYPE_SCIBIDS_FEE"
    #[serde(rename="PARTNER_COST_TYPE_SCIBIDS_FEE")]
    PARTNERCOSTTYPESCIBIDSFEE,
}

impl AsRef<str> for PartnerCostCostTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEUNSPECIFIED => "PARTNER_COST_TYPE_UNSPECIFIED",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEADLOOX => "PARTNER_COST_TYPE_ADLOOX",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEADLOOXPREBID => "PARTNER_COST_TYPE_ADLOOX_PREBID",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEADSAFE => "PARTNER_COST_TYPE_ADSAFE",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEADXPOSE => "PARTNER_COST_TYPE_ADXPOSE",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEAGGREGATEKNOWLEDGE => "PARTNER_COST_TYPE_AGGREGATE_KNOWLEDGE",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEAGENCYTRADINGDESK => "PARTNER_COST_TYPE_AGENCY_TRADING_DESK",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEDV360FEE => "PARTNER_COST_TYPE_DV360_FEE",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPECOMSCOREVCE => "PARTNER_COST_TYPE_COMSCORE_VCE",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEDATAMANAGEMENTPLATFORM => "PARTNER_COST_TYPE_DATA_MANAGEMENT_PLATFORM",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEDEFAULT => "PARTNER_COST_TYPE_DEFAULT",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEDOUBLEVERIFY => "PARTNER_COST_TYPE_DOUBLE_VERIFY",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEDOUBLEVERIFYPREBID => "PARTNER_COST_TYPE_DOUBLE_VERIFY_PREBID",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEEVIDON => "PARTNER_COST_TYPE_EVIDON",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEINTEGRALADSCIENCEVIDEO => "PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_VIDEO",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEINTEGRALADSCIENCEPREBID => "PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_PREBID",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEMEDIACOSTDATA => "PARTNER_COST_TYPE_MEDIA_COST_DATA",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEMOATVIDEO => "PARTNER_COST_TYPE_MOAT_VIDEO",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPENIELSENDAR => "PARTNER_COST_TYPE_NIELSEN_DAR",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPESHOPLOCAL => "PARTNER_COST_TYPE_SHOP_LOCAL",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPETERACENT => "PARTNER_COST_TYPE_TERACENT",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPETHIRDPARTYADSERVER => "PARTNER_COST_TYPE_THIRD_PARTY_AD_SERVER",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPETRUSTMETRICS => "PARTNER_COST_TYPE_TRUST_METRICS",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEVIZU => "PARTNER_COST_TYPE_VIZU",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPEADLINGOFEE => "PARTNER_COST_TYPE_ADLINGO_FEE",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPECUSTOMFEE1 => "PARTNER_COST_TYPE_CUSTOM_FEE_1",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPECUSTOMFEE2 => "PARTNER_COST_TYPE_CUSTOM_FEE_2",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPECUSTOMFEE3 => "PARTNER_COST_TYPE_CUSTOM_FEE_3",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPECUSTOMFEE4 => "PARTNER_COST_TYPE_CUSTOM_FEE_4",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPECUSTOMFEE5 => "PARTNER_COST_TYPE_CUSTOM_FEE_5",
            PartnerCostCostTypeEnum::PARTNERCOSTTYPESCIBIDSFEE => "PARTNER_COST_TYPE_SCIBIDS_FEE",
        }
    }
}

impl std::convert::TryFrom< &str> for PartnerCostCostTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTNER_COST_TYPE_UNSPECIFIED" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEUNSPECIFIED),
           "PARTNER_COST_TYPE_ADLOOX" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEADLOOX),
           "PARTNER_COST_TYPE_ADLOOX_PREBID" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEADLOOXPREBID),
           "PARTNER_COST_TYPE_ADSAFE" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEADSAFE),
           "PARTNER_COST_TYPE_ADXPOSE" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEADXPOSE),
           "PARTNER_COST_TYPE_AGGREGATE_KNOWLEDGE" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEAGGREGATEKNOWLEDGE),
           "PARTNER_COST_TYPE_AGENCY_TRADING_DESK" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEAGENCYTRADINGDESK),
           "PARTNER_COST_TYPE_DV360_FEE" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEDV360FEE),
           "PARTNER_COST_TYPE_COMSCORE_VCE" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPECOMSCOREVCE),
           "PARTNER_COST_TYPE_DATA_MANAGEMENT_PLATFORM" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEDATAMANAGEMENTPLATFORM),
           "PARTNER_COST_TYPE_DEFAULT" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEDEFAULT),
           "PARTNER_COST_TYPE_DOUBLE_VERIFY" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEDOUBLEVERIFY),
           "PARTNER_COST_TYPE_DOUBLE_VERIFY_PREBID" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEDOUBLEVERIFYPREBID),
           "PARTNER_COST_TYPE_EVIDON" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEEVIDON),
           "PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_VIDEO" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEINTEGRALADSCIENCEVIDEO),
           "PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_PREBID" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEINTEGRALADSCIENCEPREBID),
           "PARTNER_COST_TYPE_MEDIA_COST_DATA" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEMEDIACOSTDATA),
           "PARTNER_COST_TYPE_MOAT_VIDEO" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEMOATVIDEO),
           "PARTNER_COST_TYPE_NIELSEN_DAR" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPENIELSENDAR),
           "PARTNER_COST_TYPE_SHOP_LOCAL" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPESHOPLOCAL),
           "PARTNER_COST_TYPE_TERACENT" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPETERACENT),
           "PARTNER_COST_TYPE_THIRD_PARTY_AD_SERVER" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPETHIRDPARTYADSERVER),
           "PARTNER_COST_TYPE_TRUST_METRICS" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPETRUSTMETRICS),
           "PARTNER_COST_TYPE_VIZU" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEVIZU),
           "PARTNER_COST_TYPE_ADLINGO_FEE" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPEADLINGOFEE),
           "PARTNER_COST_TYPE_CUSTOM_FEE_1" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPECUSTOMFEE1),
           "PARTNER_COST_TYPE_CUSTOM_FEE_2" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPECUSTOMFEE2),
           "PARTNER_COST_TYPE_CUSTOM_FEE_3" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPECUSTOMFEE3),
           "PARTNER_COST_TYPE_CUSTOM_FEE_4" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPECUSTOMFEE4),
           "PARTNER_COST_TYPE_CUSTOM_FEE_5" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPECUSTOMFEE5),
           "PARTNER_COST_TYPE_SCIBIDS_FEE" => Ok(PartnerCostCostTypeEnum::PARTNERCOSTTYPESCIBIDSFEE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartnerCostCostTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartnerCostFeeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The fee type for this partner cost.
pub enum PartnerCostFeeTypeEnum {
    

    /// Value is not specified or is unknown in this version.
    ///
    /// "PARTNER_COST_FEE_TYPE_UNSPECIFIED"
    #[serde(rename="PARTNER_COST_FEE_TYPE_UNSPECIFIED")]
    PARTNERCOSTFEETYPEUNSPECIFIED,
    

    /// The partner cost is a fixed CPM fee. Not applicable when the partner cost cost_type is one of: * `PARTNER_COST_TYPE_MEDIA_COST_DATA` * `PARTNER_COST_TYPE_DV360_FEE`.
    ///
    /// "PARTNER_COST_FEE_TYPE_CPM_FEE"
    #[serde(rename="PARTNER_COST_FEE_TYPE_CPM_FEE")]
    PARTNERCOSTFEETYPECPMFEE,
    

    /// The partner cost is a percentage surcharge based on the media cost. Not applicable when the partner cost_type is one of: * `PARTNER_COST_TYPE_SHOP_LOCAL` * `PARTNER_COST_TYPE_TRUST_METRICS` * `PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_VIDEO` * `PARTNER_COST_TYPE_MOAT_VIDEO`.
    ///
    /// "PARTNER_COST_FEE_TYPE_MEDIA_FEE"
    #[serde(rename="PARTNER_COST_FEE_TYPE_MEDIA_FEE")]
    PARTNERCOSTFEETYPEMEDIAFEE,
}

impl AsRef<str> for PartnerCostFeeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartnerCostFeeTypeEnum::PARTNERCOSTFEETYPEUNSPECIFIED => "PARTNER_COST_FEE_TYPE_UNSPECIFIED",
            PartnerCostFeeTypeEnum::PARTNERCOSTFEETYPECPMFEE => "PARTNER_COST_FEE_TYPE_CPM_FEE",
            PartnerCostFeeTypeEnum::PARTNERCOSTFEETYPEMEDIAFEE => "PARTNER_COST_FEE_TYPE_MEDIA_FEE",
        }
    }
}

impl std::convert::TryFrom< &str> for PartnerCostFeeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTNER_COST_FEE_TYPE_UNSPECIFIED" => Ok(PartnerCostFeeTypeEnum::PARTNERCOSTFEETYPEUNSPECIFIED),
           "PARTNER_COST_FEE_TYPE_CPM_FEE" => Ok(PartnerCostFeeTypeEnum::PARTNERCOSTFEETYPECPMFEE),
           "PARTNER_COST_FEE_TYPE_MEDIA_FEE" => Ok(PartnerCostFeeTypeEnum::PARTNERCOSTFEETYPEMEDIAFEE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartnerCostFeeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartnerCostInvoiceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The invoice type for this partner cost. * Required when cost_type is one of: - `PARTNER_COST_TYPE_ADLOOX` - `PARTNER_COST_TYPE_DOUBLE_VERIFY` - `PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE`. * Output only for other types.
pub enum PartnerCostInvoiceTypeEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "PARTNER_COST_INVOICE_TYPE_UNSPECIFIED"
    #[serde(rename="PARTNER_COST_INVOICE_TYPE_UNSPECIFIED")]
    PARTNERCOSTINVOICETYPEUNSPECIFIED,
    

    /// Partner cost is billed through DV360.
    ///
    /// "PARTNER_COST_INVOICE_TYPE_DV360"
    #[serde(rename="PARTNER_COST_INVOICE_TYPE_DV360")]
    PARTNERCOSTINVOICETYPEDV360,
    

    /// Partner cost is billed by the partner.
    ///
    /// "PARTNER_COST_INVOICE_TYPE_PARTNER"
    #[serde(rename="PARTNER_COST_INVOICE_TYPE_PARTNER")]
    PARTNERCOSTINVOICETYPEPARTNER,
}

impl AsRef<str> for PartnerCostInvoiceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartnerCostInvoiceTypeEnum::PARTNERCOSTINVOICETYPEUNSPECIFIED => "PARTNER_COST_INVOICE_TYPE_UNSPECIFIED",
            PartnerCostInvoiceTypeEnum::PARTNERCOSTINVOICETYPEDV360 => "PARTNER_COST_INVOICE_TYPE_DV360",
            PartnerCostInvoiceTypeEnum::PARTNERCOSTINVOICETYPEPARTNER => "PARTNER_COST_INVOICE_TYPE_PARTNER",
        }
    }
}

impl std::convert::TryFrom< &str> for PartnerCostInvoiceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTNER_COST_INVOICE_TYPE_UNSPECIFIED" => Ok(PartnerCostInvoiceTypeEnum::PARTNERCOSTINVOICETYPEUNSPECIFIED),
           "PARTNER_COST_INVOICE_TYPE_DV360" => Ok(PartnerCostInvoiceTypeEnum::PARTNERCOSTINVOICETYPEDV360),
           "PARTNER_COST_INVOICE_TYPE_PARTNER" => Ok(PartnerCostInvoiceTypeEnum::PARTNERCOSTINVOICETYPEPARTNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartnerCostInvoiceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartnerRevenueModelMarkupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The markup type of the partner revenue model.
pub enum PartnerRevenueModelMarkupTypeEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "PARTNER_REVENUE_MODEL_MARKUP_TYPE_UNSPECIFIED"
    #[serde(rename="PARTNER_REVENUE_MODEL_MARKUP_TYPE_UNSPECIFIED")]
    PARTNERREVENUEMODELMARKUPTYPEUNSPECIFIED,
    

    /// Calculate the partner revenue based on a fixed CPM.
    ///
    /// "PARTNER_REVENUE_MODEL_MARKUP_TYPE_CPM"
    #[serde(rename="PARTNER_REVENUE_MODEL_MARKUP_TYPE_CPM")]
    PARTNERREVENUEMODELMARKUPTYPECPM,
    

    /// Calculate the partner revenue based on a percentage surcharge of its media cost.
    ///
    /// "PARTNER_REVENUE_MODEL_MARKUP_TYPE_MEDIA_COST_MARKUP"
    #[serde(rename="PARTNER_REVENUE_MODEL_MARKUP_TYPE_MEDIA_COST_MARKUP")]
    PARTNERREVENUEMODELMARKUPTYPEMEDIACOSTMARKUP,
    

    /// Calculate the partner revenue based on a percentage surcharge of its total media cost, which includes all partner costs and data costs.
    ///
    /// "PARTNER_REVENUE_MODEL_MARKUP_TYPE_TOTAL_MEDIA_COST_MARKUP"
    #[serde(rename="PARTNER_REVENUE_MODEL_MARKUP_TYPE_TOTAL_MEDIA_COST_MARKUP")]
    PARTNERREVENUEMODELMARKUPTYPETOTALMEDIACOSTMARKUP,
}

impl AsRef<str> for PartnerRevenueModelMarkupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartnerRevenueModelMarkupTypeEnum::PARTNERREVENUEMODELMARKUPTYPEUNSPECIFIED => "PARTNER_REVENUE_MODEL_MARKUP_TYPE_UNSPECIFIED",
            PartnerRevenueModelMarkupTypeEnum::PARTNERREVENUEMODELMARKUPTYPECPM => "PARTNER_REVENUE_MODEL_MARKUP_TYPE_CPM",
            PartnerRevenueModelMarkupTypeEnum::PARTNERREVENUEMODELMARKUPTYPEMEDIACOSTMARKUP => "PARTNER_REVENUE_MODEL_MARKUP_TYPE_MEDIA_COST_MARKUP",
            PartnerRevenueModelMarkupTypeEnum::PARTNERREVENUEMODELMARKUPTYPETOTALMEDIACOSTMARKUP => "PARTNER_REVENUE_MODEL_MARKUP_TYPE_TOTAL_MEDIA_COST_MARKUP",
        }
    }
}

impl std::convert::TryFrom< &str> for PartnerRevenueModelMarkupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTNER_REVENUE_MODEL_MARKUP_TYPE_UNSPECIFIED" => Ok(PartnerRevenueModelMarkupTypeEnum::PARTNERREVENUEMODELMARKUPTYPEUNSPECIFIED),
           "PARTNER_REVENUE_MODEL_MARKUP_TYPE_CPM" => Ok(PartnerRevenueModelMarkupTypeEnum::PARTNERREVENUEMODELMARKUPTYPECPM),
           "PARTNER_REVENUE_MODEL_MARKUP_TYPE_MEDIA_COST_MARKUP" => Ok(PartnerRevenueModelMarkupTypeEnum::PARTNERREVENUEMODELMARKUPTYPEMEDIACOSTMARKUP),
           "PARTNER_REVENUE_MODEL_MARKUP_TYPE_TOTAL_MEDIA_COST_MARKUP" => Ok(PartnerRevenueModelMarkupTypeEnum::PARTNERREVENUEMODELMARKUPTYPETOTALMEDIACOSTMARKUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartnerRevenueModelMarkupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PerformanceGoalPerformanceGoalTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the performance goal.
pub enum PerformanceGoalPerformanceGoalTypeEnum {
    

    /// Performance goal type is not specified or is unknown in this version.
    ///
    /// "PERFORMANCE_GOAL_TYPE_UNSPECIFIED"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_UNSPECIFIED")]
    PERFORMANCEGOALTYPEUNSPECIFIED,
    

    /// The performance goal is set in CPM (cost per mille).
    ///
    /// "PERFORMANCE_GOAL_TYPE_CPM"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_CPM")]
    PERFORMANCEGOALTYPECPM,
    

    /// The performance goal is set in CPC (cost per click).
    ///
    /// "PERFORMANCE_GOAL_TYPE_CPC"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_CPC")]
    PERFORMANCEGOALTYPECPC,
    

    /// The performance goal is set in CPA (cost per action).
    ///
    /// "PERFORMANCE_GOAL_TYPE_CPA"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_CPA")]
    PERFORMANCEGOALTYPECPA,
    

    /// The performance goal is set in CTR (click-through rate) percentage.
    ///
    /// "PERFORMANCE_GOAL_TYPE_CTR"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_CTR")]
    PERFORMANCEGOALTYPECTR,
    

    /// The performance goal is set in Viewability percentage.
    ///
    /// "PERFORMANCE_GOAL_TYPE_VIEWABILITY"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_VIEWABILITY")]
    PERFORMANCEGOALTYPEVIEWABILITY,
    

    /// The performance goal is set as CPIAVC (cost per impression audible and visible at completion).
    ///
    /// "PERFORMANCE_GOAL_TYPE_CPIAVC"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_CPIAVC")]
    PERFORMANCEGOALTYPECPIAVC,
    

    /// The performance goal is set in CPE (cost per engagement).
    ///
    /// "PERFORMANCE_GOAL_TYPE_CPE"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_CPE")]
    PERFORMANCEGOALTYPECPE,
    

    /// The performance goal is set in click conversion rate (conversions per click) percentage.
    ///
    /// "PERFORMANCE_GOAL_TYPE_CLICK_CVR"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_CLICK_CVR")]
    PERFORMANCEGOALTYPECLICKCVR,
    

    /// The performance goal is set in impression conversion rate (conversions per impression) percentage.
    ///
    /// "PERFORMANCE_GOAL_TYPE_IMPRESSION_CVR"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_IMPRESSION_CVR")]
    PERFORMANCEGOALTYPEIMPRESSIONCVR,
    

    /// The performance goal is set in VCPM (cost per thousand viewable impressions).
    ///
    /// "PERFORMANCE_GOAL_TYPE_VCPM"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_VCPM")]
    PERFORMANCEGOALTYPEVCPM,
    

    /// The performance goal is set in YouTube view rate (YouTube views per impression) percentage.
    ///
    /// "PERFORMANCE_GOAL_TYPE_VTR"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_VTR")]
    PERFORMANCEGOALTYPEVTR,
    

    /// The performance goal is set in audio completion rate (complete audio listens per impression) percentage.
    ///
    /// "PERFORMANCE_GOAL_TYPE_AUDIO_COMPLETION_RATE"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_AUDIO_COMPLETION_RATE")]
    PERFORMANCEGOALTYPEAUDIOCOMPLETIONRATE,
    

    /// The performance goal is set in video completion rate (complete video views per impression) percentage.
    ///
    /// "PERFORMANCE_GOAL_TYPE_VIDEO_COMPLETION_RATE"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_VIDEO_COMPLETION_RATE")]
    PERFORMANCEGOALTYPEVIDEOCOMPLETIONRATE,
    

    /// The performance goal is set to Other.
    ///
    /// "PERFORMANCE_GOAL_TYPE_OTHER"
    #[serde(rename="PERFORMANCE_GOAL_TYPE_OTHER")]
    PERFORMANCEGOALTYPEOTHER,
}

impl AsRef<str> for PerformanceGoalPerformanceGoalTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEUNSPECIFIED => "PERFORMANCE_GOAL_TYPE_UNSPECIFIED",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECPM => "PERFORMANCE_GOAL_TYPE_CPM",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECPC => "PERFORMANCE_GOAL_TYPE_CPC",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECPA => "PERFORMANCE_GOAL_TYPE_CPA",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECTR => "PERFORMANCE_GOAL_TYPE_CTR",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEVIEWABILITY => "PERFORMANCE_GOAL_TYPE_VIEWABILITY",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECPIAVC => "PERFORMANCE_GOAL_TYPE_CPIAVC",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECPE => "PERFORMANCE_GOAL_TYPE_CPE",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECLICKCVR => "PERFORMANCE_GOAL_TYPE_CLICK_CVR",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEIMPRESSIONCVR => "PERFORMANCE_GOAL_TYPE_IMPRESSION_CVR",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEVCPM => "PERFORMANCE_GOAL_TYPE_VCPM",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEVTR => "PERFORMANCE_GOAL_TYPE_VTR",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEAUDIOCOMPLETIONRATE => "PERFORMANCE_GOAL_TYPE_AUDIO_COMPLETION_RATE",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEVIDEOCOMPLETIONRATE => "PERFORMANCE_GOAL_TYPE_VIDEO_COMPLETION_RATE",
            PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEOTHER => "PERFORMANCE_GOAL_TYPE_OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for PerformanceGoalPerformanceGoalTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERFORMANCE_GOAL_TYPE_UNSPECIFIED" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEUNSPECIFIED),
           "PERFORMANCE_GOAL_TYPE_CPM" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECPM),
           "PERFORMANCE_GOAL_TYPE_CPC" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECPC),
           "PERFORMANCE_GOAL_TYPE_CPA" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECPA),
           "PERFORMANCE_GOAL_TYPE_CTR" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECTR),
           "PERFORMANCE_GOAL_TYPE_VIEWABILITY" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEVIEWABILITY),
           "PERFORMANCE_GOAL_TYPE_CPIAVC" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECPIAVC),
           "PERFORMANCE_GOAL_TYPE_CPE" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECPE),
           "PERFORMANCE_GOAL_TYPE_CLICK_CVR" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPECLICKCVR),
           "PERFORMANCE_GOAL_TYPE_IMPRESSION_CVR" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEIMPRESSIONCVR),
           "PERFORMANCE_GOAL_TYPE_VCPM" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEVCPM),
           "PERFORMANCE_GOAL_TYPE_VTR" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEVTR),
           "PERFORMANCE_GOAL_TYPE_AUDIO_COMPLETION_RATE" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEAUDIOCOMPLETIONRATE),
           "PERFORMANCE_GOAL_TYPE_VIDEO_COMPLETION_RATE" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEVIDEOCOMPLETIONRATE),
           "PERFORMANCE_GOAL_TYPE_OTHER" => Ok(PerformanceGoalPerformanceGoalTypeEnum::PERFORMANCEGOALTYPEOTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PerformanceGoalPerformanceGoalTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PerformanceGoalBidStrategyPerformanceGoalTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the performance goal that the bidding strategy will try to meet or beat. For line item level usage, the value must be one of: * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO`.
pub enum PerformanceGoalBidStrategyPerformanceGoalTypeEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_UNSPECIFIED"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_UNSPECIFIED")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPEUNSPECIFIED,
    

    /// Cost per action.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPECPA,
    

    /// Cost per click.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPECPC,
    

    /// Viewable CPM.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPEVIEWABLECPM,
    

    /// Custom bidding algorithm.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPECUSTOMALGO,
    

    /// Completed inview and audible views.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPECIVA,
    

    /// Inview time over 10 secs views.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPEIVOTEN,
    

    /// Viewable impressions.
    ///
    /// "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED"
    #[serde(rename="BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED")]
    BIDDINGSTRATEGYPERFORMANCEGOALTYPEAVVIEWED,
}

impl AsRef<str> for PerformanceGoalBidStrategyPerformanceGoalTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEUNSPECIFIED => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_UNSPECIFIED",
            PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECPA => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA",
            PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECPC => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC",
            PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEVIEWABLECPM => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM",
            PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECUSTOMALGO => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO",
            PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECIVA => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA",
            PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEIVOTEN => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN",
            PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEAVVIEWED => "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED",
        }
    }
}

impl std::convert::TryFrom< &str> for PerformanceGoalBidStrategyPerformanceGoalTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_UNSPECIFIED" => Ok(PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEUNSPECIFIED),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA" => Ok(PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECPA),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC" => Ok(PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECPC),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM" => Ok(PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEVIEWABLECPM),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO" => Ok(PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECUSTOMALGO),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA" => Ok(PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPECIVA),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN" => Ok(PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEIVOTEN),
           "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED" => Ok(PerformanceGoalBidStrategyPerformanceGoalTypeEnum::BIDDINGSTRATEGYPERFORMANCEGOALTYPEAVVIEWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PerformanceGoalBidStrategyPerformanceGoalTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The unit of distance by which the targeting radius is measured.
pub enum PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "DISTANCE_UNIT_UNSPECIFIED"
    #[serde(rename="DISTANCE_UNIT_UNSPECIFIED")]
    DISTANCEUNITUNSPECIFIED,
    

    /// Miles.
    ///
    /// "DISTANCE_UNIT_MILES"
    #[serde(rename="DISTANCE_UNIT_MILES")]
    DISTANCEUNITMILES,
    

    /// Kilometers.
    ///
    /// "DISTANCE_UNIT_KILOMETERS"
    #[serde(rename="DISTANCE_UNIT_KILOMETERS")]
    DISTANCEUNITKILOMETERS,
}

impl AsRef<str> for PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITUNSPECIFIED => "DISTANCE_UNIT_UNSPECIFIED",
            PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITMILES => "DISTANCE_UNIT_MILES",
            PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITKILOMETERS => "DISTANCE_UNIT_KILOMETERS",
        }
    }
}

impl std::convert::TryFrom< &str> for PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISTANCE_UNIT_UNSPECIFIED" => Ok(PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITUNSPECIFIED),
           "DISTANCE_UNIT_MILES" => Ok(PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITMILES),
           "DISTANCE_UNIT_KILOMETERS" => Ok(PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum::DISTANCEUNITKILOMETERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoiAssignedTargetingOptionDetailProximityRadiusUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PrismaConfigPrismaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The Prisma type.
pub enum PrismaConfigPrismaTypeEnum {
    

    /// Type is not specified or unknown in this version.
    ///
    /// "PRISMA_TYPE_UNSPECIFIED"
    #[serde(rename="PRISMA_TYPE_UNSPECIFIED")]
    PRISMATYPEUNSPECIFIED,
    

    /// Display type.
    ///
    /// "PRISMA_TYPE_DISPLAY"
    #[serde(rename="PRISMA_TYPE_DISPLAY")]
    PRISMATYPEDISPLAY,
    

    /// Search type.
    ///
    /// "PRISMA_TYPE_SEARCH"
    #[serde(rename="PRISMA_TYPE_SEARCH")]
    PRISMATYPESEARCH,
    

    /// Video type.
    ///
    /// "PRISMA_TYPE_VIDEO"
    #[serde(rename="PRISMA_TYPE_VIDEO")]
    PRISMATYPEVIDEO,
    

    /// Audio type.
    ///
    /// "PRISMA_TYPE_AUDIO"
    #[serde(rename="PRISMA_TYPE_AUDIO")]
    PRISMATYPEAUDIO,
    

    /// Social type.
    ///
    /// "PRISMA_TYPE_SOCIAL"
    #[serde(rename="PRISMA_TYPE_SOCIAL")]
    PRISMATYPESOCIAL,
    

    /// Fee type.
    ///
    /// "PRISMA_TYPE_FEE"
    #[serde(rename="PRISMA_TYPE_FEE")]
    PRISMATYPEFEE,
}

impl AsRef<str> for PrismaConfigPrismaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PrismaConfigPrismaTypeEnum::PRISMATYPEUNSPECIFIED => "PRISMA_TYPE_UNSPECIFIED",
            PrismaConfigPrismaTypeEnum::PRISMATYPEDISPLAY => "PRISMA_TYPE_DISPLAY",
            PrismaConfigPrismaTypeEnum::PRISMATYPESEARCH => "PRISMA_TYPE_SEARCH",
            PrismaConfigPrismaTypeEnum::PRISMATYPEVIDEO => "PRISMA_TYPE_VIDEO",
            PrismaConfigPrismaTypeEnum::PRISMATYPEAUDIO => "PRISMA_TYPE_AUDIO",
            PrismaConfigPrismaTypeEnum::PRISMATYPESOCIAL => "PRISMA_TYPE_SOCIAL",
            PrismaConfigPrismaTypeEnum::PRISMATYPEFEE => "PRISMA_TYPE_FEE",
        }
    }
}

impl std::convert::TryFrom< &str> for PrismaConfigPrismaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRISMA_TYPE_UNSPECIFIED" => Ok(PrismaConfigPrismaTypeEnum::PRISMATYPEUNSPECIFIED),
           "PRISMA_TYPE_DISPLAY" => Ok(PrismaConfigPrismaTypeEnum::PRISMATYPEDISPLAY),
           "PRISMA_TYPE_SEARCH" => Ok(PrismaConfigPrismaTypeEnum::PRISMATYPESEARCH),
           "PRISMA_TYPE_VIDEO" => Ok(PrismaConfigPrismaTypeEnum::PRISMATYPEVIDEO),
           "PRISMA_TYPE_AUDIO" => Ok(PrismaConfigPrismaTypeEnum::PRISMATYPEAUDIO),
           "PRISMA_TYPE_SOCIAL" => Ok(PrismaConfigPrismaTypeEnum::PRISMATYPESOCIAL),
           "PRISMA_TYPE_FEE" => Ok(PrismaConfigPrismaTypeEnum::PRISMATYPEFEE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PrismaConfigPrismaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Radius range for proximity location list. This represents the size of the area around a chosen location that will be targeted. `All` proximity location targeting under a single resource must have the same radius range value. Set this value to match any existing targeting. If updated, this field will change the radius range for all proximity targeting under the resource.
pub enum ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum {
    

    /// The targeted radius range is not specified or is unknown. Default value when radius range is not specified in this version. This enum is a placeholder for default value and does not represent a real radius range option.
    ///
    /// "PROXIMITY_RADIUS_RANGE_UNSPECIFIED"
    #[serde(rename="PROXIMITY_RADIUS_RANGE_UNSPECIFIED")]
    PROXIMITYRADIUSRANGEUNSPECIFIED,
    

    /// The targeted radius range is small.
    ///
    /// "PROXIMITY_RADIUS_RANGE_SMALL"
    #[serde(rename="PROXIMITY_RADIUS_RANGE_SMALL")]
    PROXIMITYRADIUSRANGESMALL,
    

    /// The targeted radius range is medium.
    ///
    /// "PROXIMITY_RADIUS_RANGE_MEDIUM"
    #[serde(rename="PROXIMITY_RADIUS_RANGE_MEDIUM")]
    PROXIMITYRADIUSRANGEMEDIUM,
    

    /// The targeted radius range is large.
    ///
    /// "PROXIMITY_RADIUS_RANGE_LARGE"
    #[serde(rename="PROXIMITY_RADIUS_RANGE_LARGE")]
    PROXIMITYRADIUSRANGELARGE,
}

impl AsRef<str> for ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum::PROXIMITYRADIUSRANGEUNSPECIFIED => "PROXIMITY_RADIUS_RANGE_UNSPECIFIED",
            ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum::PROXIMITYRADIUSRANGESMALL => "PROXIMITY_RADIUS_RANGE_SMALL",
            ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum::PROXIMITYRADIUSRANGEMEDIUM => "PROXIMITY_RADIUS_RANGE_MEDIUM",
            ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum::PROXIMITYRADIUSRANGELARGE => "PROXIMITY_RADIUS_RANGE_LARGE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROXIMITY_RADIUS_RANGE_UNSPECIFIED" => Ok(ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum::PROXIMITYRADIUSRANGEUNSPECIFIED),
           "PROXIMITY_RADIUS_RANGE_SMALL" => Ok(ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum::PROXIMITYRADIUSRANGESMALL),
           "PROXIMITY_RADIUS_RANGE_MEDIUM" => Ok(ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum::PROXIMITYRADIUSRANGEMEDIUM),
           "PROXIMITY_RADIUS_RANGE_LARGE" => Ok(ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum::PROXIMITYRADIUSRANGELARGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProximityLocationListAssignedTargetingOptionDetailProximityRadiusRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RateDetailInventorySourceRateTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The rate type. Acceptable values are `INVENTORY_SOURCE_RATE_TYPE_CPM_FIXED`, `INVENTORY_SOURCE_RATE_TYPE_CPM_FLOOR`, and `INVENTORY_SOURCE_RATE_TYPE_CPD`.
pub enum RateDetailInventorySourceRateTypeEnum {
    

    /// The rate type is not specified or is unknown in this version.
    ///
    /// "INVENTORY_SOURCE_RATE_TYPE_UNSPECIFIED"
    #[serde(rename="INVENTORY_SOURCE_RATE_TYPE_UNSPECIFIED")]
    INVENTORYSOURCERATETYPEUNSPECIFIED,
    

    /// The rate type is CPM (Fixed).
    ///
    /// "INVENTORY_SOURCE_RATE_TYPE_CPM_FIXED"
    #[serde(rename="INVENTORY_SOURCE_RATE_TYPE_CPM_FIXED")]
    INVENTORYSOURCERATETYPECPMFIXED,
    

    /// The rate type is CPM (Floor).
    ///
    /// "INVENTORY_SOURCE_RATE_TYPE_CPM_FLOOR"
    #[serde(rename="INVENTORY_SOURCE_RATE_TYPE_CPM_FLOOR")]
    INVENTORYSOURCERATETYPECPMFLOOR,
    

    /// The rate type is Cost per Day.
    ///
    /// "INVENTORY_SOURCE_RATE_TYPE_CPD"
    #[serde(rename="INVENTORY_SOURCE_RATE_TYPE_CPD")]
    INVENTORYSOURCERATETYPECPD,
    

    /// The rate type is Flat.
    ///
    /// "INVENTORY_SOURCE_RATE_TYPE_FLAT"
    #[serde(rename="INVENTORY_SOURCE_RATE_TYPE_FLAT")]
    INVENTORYSOURCERATETYPEFLAT,
}

impl AsRef<str> for RateDetailInventorySourceRateTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RateDetailInventorySourceRateTypeEnum::INVENTORYSOURCERATETYPEUNSPECIFIED => "INVENTORY_SOURCE_RATE_TYPE_UNSPECIFIED",
            RateDetailInventorySourceRateTypeEnum::INVENTORYSOURCERATETYPECPMFIXED => "INVENTORY_SOURCE_RATE_TYPE_CPM_FIXED",
            RateDetailInventorySourceRateTypeEnum::INVENTORYSOURCERATETYPECPMFLOOR => "INVENTORY_SOURCE_RATE_TYPE_CPM_FLOOR",
            RateDetailInventorySourceRateTypeEnum::INVENTORYSOURCERATETYPECPD => "INVENTORY_SOURCE_RATE_TYPE_CPD",
            RateDetailInventorySourceRateTypeEnum::INVENTORYSOURCERATETYPEFLAT => "INVENTORY_SOURCE_RATE_TYPE_FLAT",
        }
    }
}

impl std::convert::TryFrom< &str> for RateDetailInventorySourceRateTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVENTORY_SOURCE_RATE_TYPE_UNSPECIFIED" => Ok(RateDetailInventorySourceRateTypeEnum::INVENTORYSOURCERATETYPEUNSPECIFIED),
           "INVENTORY_SOURCE_RATE_TYPE_CPM_FIXED" => Ok(RateDetailInventorySourceRateTypeEnum::INVENTORYSOURCERATETYPECPMFIXED),
           "INVENTORY_SOURCE_RATE_TYPE_CPM_FLOOR" => Ok(RateDetailInventorySourceRateTypeEnum::INVENTORYSOURCERATETYPECPMFLOOR),
           "INVENTORY_SOURCE_RATE_TYPE_CPD" => Ok(RateDetailInventorySourceRateTypeEnum::INVENTORYSOURCERATETYPECPD),
           "INVENTORY_SOURCE_RATE_TYPE_FLAT" => Ok(RateDetailInventorySourceRateTypeEnum::INVENTORYSOURCERATETYPEFLAT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RateDetailInventorySourceRateTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReviewStatusInfoApprovalStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Represents the basic approval needed for a creative to begin serving. Summary of creative_and_landing_page_review_status and content_and_policy_review_status.
pub enum ReviewStatusInfoApprovalStatusEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "APPROVAL_STATUS_UNSPECIFIED"
    #[serde(rename="APPROVAL_STATUS_UNSPECIFIED")]
    APPROVALSTATUSUNSPECIFIED,
    

    /// The creative is still under review and not servable.
    ///
    /// "APPROVAL_STATUS_PENDING_NOT_SERVABLE"
    #[serde(rename="APPROVAL_STATUS_PENDING_NOT_SERVABLE")]
    APPROVALSTATUSPENDINGNOTSERVABLE,
    

    /// The creative has passed creative & landing page review and is servable, but is awaiting additional content & policy review.
    ///
    /// "APPROVAL_STATUS_PENDING_SERVABLE"
    #[serde(rename="APPROVAL_STATUS_PENDING_SERVABLE")]
    APPROVALSTATUSPENDINGSERVABLE,
    

    /// Both creative & landing page review and content & policy review are approved. The creative is servable.
    ///
    /// "APPROVAL_STATUS_APPROVED_SERVABLE"
    #[serde(rename="APPROVAL_STATUS_APPROVED_SERVABLE")]
    APPROVALSTATUSAPPROVEDSERVABLE,
    

    /// There is an issue with the creative that must be fixed before it can serve.
    ///
    /// "APPROVAL_STATUS_REJECTED_NOT_SERVABLE"
    #[serde(rename="APPROVAL_STATUS_REJECTED_NOT_SERVABLE")]
    APPROVALSTATUSREJECTEDNOTSERVABLE,
}

impl AsRef<str> for ReviewStatusInfoApprovalStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReviewStatusInfoApprovalStatusEnum::APPROVALSTATUSUNSPECIFIED => "APPROVAL_STATUS_UNSPECIFIED",
            ReviewStatusInfoApprovalStatusEnum::APPROVALSTATUSPENDINGNOTSERVABLE => "APPROVAL_STATUS_PENDING_NOT_SERVABLE",
            ReviewStatusInfoApprovalStatusEnum::APPROVALSTATUSPENDINGSERVABLE => "APPROVAL_STATUS_PENDING_SERVABLE",
            ReviewStatusInfoApprovalStatusEnum::APPROVALSTATUSAPPROVEDSERVABLE => "APPROVAL_STATUS_APPROVED_SERVABLE",
            ReviewStatusInfoApprovalStatusEnum::APPROVALSTATUSREJECTEDNOTSERVABLE => "APPROVAL_STATUS_REJECTED_NOT_SERVABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for ReviewStatusInfoApprovalStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APPROVAL_STATUS_UNSPECIFIED" => Ok(ReviewStatusInfoApprovalStatusEnum::APPROVALSTATUSUNSPECIFIED),
           "APPROVAL_STATUS_PENDING_NOT_SERVABLE" => Ok(ReviewStatusInfoApprovalStatusEnum::APPROVALSTATUSPENDINGNOTSERVABLE),
           "APPROVAL_STATUS_PENDING_SERVABLE" => Ok(ReviewStatusInfoApprovalStatusEnum::APPROVALSTATUSPENDINGSERVABLE),
           "APPROVAL_STATUS_APPROVED_SERVABLE" => Ok(ReviewStatusInfoApprovalStatusEnum::APPROVALSTATUSAPPROVEDSERVABLE),
           "APPROVAL_STATUS_REJECTED_NOT_SERVABLE" => Ok(ReviewStatusInfoApprovalStatusEnum::APPROVALSTATUSREJECTEDNOTSERVABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReviewStatusInfoApprovalStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReviewStatusInfoContentAndPolicyReviewStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Content and policy review status for the creative.
pub enum ReviewStatusInfoContentAndPolicyReviewStatusEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "REVIEW_STATUS_UNSPECIFIED"
    #[serde(rename="REVIEW_STATUS_UNSPECIFIED")]
    REVIEWSTATUSUNSPECIFIED,
    

    /// The creative is approved.
    ///
    /// "REVIEW_STATUS_APPROVED"
    #[serde(rename="REVIEW_STATUS_APPROVED")]
    REVIEWSTATUSAPPROVED,
    

    /// The creative is rejected.
    ///
    /// "REVIEW_STATUS_REJECTED"
    #[serde(rename="REVIEW_STATUS_REJECTED")]
    REVIEWSTATUSREJECTED,
    

    /// The creative is pending review.
    ///
    /// "REVIEW_STATUS_PENDING"
    #[serde(rename="REVIEW_STATUS_PENDING")]
    REVIEWSTATUSPENDING,
}

impl AsRef<str> for ReviewStatusInfoContentAndPolicyReviewStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReviewStatusInfoContentAndPolicyReviewStatusEnum::REVIEWSTATUSUNSPECIFIED => "REVIEW_STATUS_UNSPECIFIED",
            ReviewStatusInfoContentAndPolicyReviewStatusEnum::REVIEWSTATUSAPPROVED => "REVIEW_STATUS_APPROVED",
            ReviewStatusInfoContentAndPolicyReviewStatusEnum::REVIEWSTATUSREJECTED => "REVIEW_STATUS_REJECTED",
            ReviewStatusInfoContentAndPolicyReviewStatusEnum::REVIEWSTATUSPENDING => "REVIEW_STATUS_PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for ReviewStatusInfoContentAndPolicyReviewStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVIEW_STATUS_UNSPECIFIED" => Ok(ReviewStatusInfoContentAndPolicyReviewStatusEnum::REVIEWSTATUSUNSPECIFIED),
           "REVIEW_STATUS_APPROVED" => Ok(ReviewStatusInfoContentAndPolicyReviewStatusEnum::REVIEWSTATUSAPPROVED),
           "REVIEW_STATUS_REJECTED" => Ok(ReviewStatusInfoContentAndPolicyReviewStatusEnum::REVIEWSTATUSREJECTED),
           "REVIEW_STATUS_PENDING" => Ok(ReviewStatusInfoContentAndPolicyReviewStatusEnum::REVIEWSTATUSPENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReviewStatusInfoContentAndPolicyReviewStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Creative and landing page review status for the creative.
pub enum ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum {
    

    /// Type value is not specified or is unknown in this version.
    ///
    /// "REVIEW_STATUS_UNSPECIFIED"
    #[serde(rename="REVIEW_STATUS_UNSPECIFIED")]
    REVIEWSTATUSUNSPECIFIED,
    

    /// The creative is approved.
    ///
    /// "REVIEW_STATUS_APPROVED"
    #[serde(rename="REVIEW_STATUS_APPROVED")]
    REVIEWSTATUSAPPROVED,
    

    /// The creative is rejected.
    ///
    /// "REVIEW_STATUS_REJECTED"
    #[serde(rename="REVIEW_STATUS_REJECTED")]
    REVIEWSTATUSREJECTED,
    

    /// The creative is pending review.
    ///
    /// "REVIEW_STATUS_PENDING"
    #[serde(rename="REVIEW_STATUS_PENDING")]
    REVIEWSTATUSPENDING,
}

impl AsRef<str> for ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum::REVIEWSTATUSUNSPECIFIED => "REVIEW_STATUS_UNSPECIFIED",
            ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum::REVIEWSTATUSAPPROVED => "REVIEW_STATUS_APPROVED",
            ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum::REVIEWSTATUSREJECTED => "REVIEW_STATUS_REJECTED",
            ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum::REVIEWSTATUSPENDING => "REVIEW_STATUS_PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVIEW_STATUS_UNSPECIFIED" => Ok(ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum::REVIEWSTATUSUNSPECIFIED),
           "REVIEW_STATUS_APPROVED" => Ok(ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum::REVIEWSTATUSAPPROVED),
           "REVIEW_STATUS_REJECTED" => Ok(ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum::REVIEWSTATUSREJECTED),
           "REVIEW_STATUS_PENDING" => Ok(ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum::REVIEWSTATUSPENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ScriptErrorErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of error.
pub enum ScriptErrorErrorCodeEnum {
    

    /// The script error is not specified or is unknown in this version.
    ///
    /// "ERROR_CODE_UNSPECIFIED"
    #[serde(rename="ERROR_CODE_UNSPECIFIED")]
    ERRORCODEUNSPECIFIED,
    

    /// The script has a syntax error.
    ///
    /// "SYNTAX_ERROR"
    #[serde(rename="SYNTAX_ERROR")]
    SYNTAXERROR,
    

    /// The script uses deprecated syntax.
    ///
    /// "DEPRECATED_SYNTAX"
    #[serde(rename="DEPRECATED_SYNTAX")]
    DEPRECATEDSYNTAX,
    

    /// Internal errors were thrown while processing the script.
    ///
    /// "INTERNAL_ERROR"
    #[serde(rename="INTERNAL_ERROR")]
    INTERNALERROR,
}

impl AsRef<str> for ScriptErrorErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScriptErrorErrorCodeEnum::ERRORCODEUNSPECIFIED => "ERROR_CODE_UNSPECIFIED",
            ScriptErrorErrorCodeEnum::SYNTAXERROR => "SYNTAX_ERROR",
            ScriptErrorErrorCodeEnum::DEPRECATEDSYNTAX => "DEPRECATED_SYNTAX",
            ScriptErrorErrorCodeEnum::INTERNALERROR => "INTERNAL_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for ScriptErrorErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_CODE_UNSPECIFIED" => Ok(ScriptErrorErrorCodeEnum::ERRORCODEUNSPECIFIED),
           "SYNTAX_ERROR" => Ok(ScriptErrorErrorCodeEnum::SYNTAXERROR),
           "DEPRECATED_SYNTAX" => Ok(ScriptErrorErrorCodeEnum::DEPRECATEDSYNTAX),
           "INTERNAL_ERROR" => Ok(ScriptErrorErrorCodeEnum::INTERNALERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScriptErrorErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SdfConfigVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The version of SDF being used.
pub enum SdfConfigVersionEnum {
    

    /// SDF version value is not specified or is unknown in this version.
    ///
    /// "SDF_VERSION_UNSPECIFIED"
    #[serde(rename="SDF_VERSION_UNSPECIFIED")]
    SDFVERSIONUNSPECIFIED,
    

    /// SDF version 3.1
    ///
    /// "SDF_VERSION_3_1"
    #[serde(rename="SDF_VERSION_3_1")]
    SDFVERSION31,
    

    /// SDF version 4
    ///
    /// "SDF_VERSION_4"
    #[serde(rename="SDF_VERSION_4")]
    SDFVERSION4,
    

    /// SDF version 4.1
    ///
    /// "SDF_VERSION_4_1"
    #[serde(rename="SDF_VERSION_4_1")]
    SDFVERSION41,
    

    /// SDF version 4.2
    ///
    /// "SDF_VERSION_4_2"
    #[serde(rename="SDF_VERSION_4_2")]
    SDFVERSION42,
    

    /// SDF version 5.
    ///
    /// "SDF_VERSION_5"
    #[serde(rename="SDF_VERSION_5")]
    SDFVERSION5,
    

    /// SDF version 5.1
    ///
    /// "SDF_VERSION_5_1"
    #[serde(rename="SDF_VERSION_5_1")]
    SDFVERSION51,
    

    /// SDF version 5.2
    ///
    /// "SDF_VERSION_5_2"
    #[serde(rename="SDF_VERSION_5_2")]
    SDFVERSION52,
    

    /// SDF version 5.3
    ///
    /// "SDF_VERSION_5_3"
    #[serde(rename="SDF_VERSION_5_3")]
    SDFVERSION53,
    

    /// SDF version 5.4
    ///
    /// "SDF_VERSION_5_4"
    #[serde(rename="SDF_VERSION_5_4")]
    SDFVERSION54,
    

    /// SDF version 5.5
    ///
    /// "SDF_VERSION_5_5"
    #[serde(rename="SDF_VERSION_5_5")]
    SDFVERSION55,
    

    /// SDF version 6
    ///
    /// "SDF_VERSION_6"
    #[serde(rename="SDF_VERSION_6")]
    SDFVERSION6,
    

    /// SDF version 7. Read the [v7 migration guide](/display-video/api/structured-data-file/v7-migration-guide) before migrating to this version.
    ///
    /// "SDF_VERSION_7"
    #[serde(rename="SDF_VERSION_7")]
    SDFVERSION7,
}

impl AsRef<str> for SdfConfigVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SdfConfigVersionEnum::SDFVERSIONUNSPECIFIED => "SDF_VERSION_UNSPECIFIED",
            SdfConfigVersionEnum::SDFVERSION31 => "SDF_VERSION_3_1",
            SdfConfigVersionEnum::SDFVERSION4 => "SDF_VERSION_4",
            SdfConfigVersionEnum::SDFVERSION41 => "SDF_VERSION_4_1",
            SdfConfigVersionEnum::SDFVERSION42 => "SDF_VERSION_4_2",
            SdfConfigVersionEnum::SDFVERSION5 => "SDF_VERSION_5",
            SdfConfigVersionEnum::SDFVERSION51 => "SDF_VERSION_5_1",
            SdfConfigVersionEnum::SDFVERSION52 => "SDF_VERSION_5_2",
            SdfConfigVersionEnum::SDFVERSION53 => "SDF_VERSION_5_3",
            SdfConfigVersionEnum::SDFVERSION54 => "SDF_VERSION_5_4",
            SdfConfigVersionEnum::SDFVERSION55 => "SDF_VERSION_5_5",
            SdfConfigVersionEnum::SDFVERSION6 => "SDF_VERSION_6",
            SdfConfigVersionEnum::SDFVERSION7 => "SDF_VERSION_7",
        }
    }
}

impl std::convert::TryFrom< &str> for SdfConfigVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SDF_VERSION_UNSPECIFIED" => Ok(SdfConfigVersionEnum::SDFVERSIONUNSPECIFIED),
           "SDF_VERSION_3_1" => Ok(SdfConfigVersionEnum::SDFVERSION31),
           "SDF_VERSION_4" => Ok(SdfConfigVersionEnum::SDFVERSION4),
           "SDF_VERSION_4_1" => Ok(SdfConfigVersionEnum::SDFVERSION41),
           "SDF_VERSION_4_2" => Ok(SdfConfigVersionEnum::SDFVERSION42),
           "SDF_VERSION_5" => Ok(SdfConfigVersionEnum::SDFVERSION5),
           "SDF_VERSION_5_1" => Ok(SdfConfigVersionEnum::SDFVERSION51),
           "SDF_VERSION_5_2" => Ok(SdfConfigVersionEnum::SDFVERSION52),
           "SDF_VERSION_5_3" => Ok(SdfConfigVersionEnum::SDFVERSION53),
           "SDF_VERSION_5_4" => Ok(SdfConfigVersionEnum::SDFVERSION54),
           "SDF_VERSION_5_5" => Ok(SdfConfigVersionEnum::SDFVERSION55),
           "SDF_VERSION_6" => Ok(SdfConfigVersionEnum::SDFVERSION6),
           "SDF_VERSION_7" => Ok(SdfConfigVersionEnum::SDFVERSION7),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SdfConfigVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. An enum for the DV360 Sensitive category content classifier.
pub enum SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum {
    

    /// This enum is only a placeholder and doesn't specify a DV360 sensitive category.
    ///
    /// "SENSITIVE_CATEGORY_UNSPECIFIED"
    #[serde(rename="SENSITIVE_CATEGORY_UNSPECIFIED")]
    SENSITIVECATEGORYUNSPECIFIED,
    

    /// Adult or pornographic text, image, or video content.
    ///
    /// "SENSITIVE_CATEGORY_ADULT"
    #[serde(rename="SENSITIVE_CATEGORY_ADULT")]
    SENSITIVECATEGORYADULT,
    

    /// Content that may be construed as biased against individuals, groups, or organizations based on criteria such as race, religion, disability, sex, age, veteran status, sexual orientation, gender identity, or political affiliation. May also indicate discussion of such content, for instance, in an academic or journalistic context.
    ///
    /// "SENSITIVE_CATEGORY_DEROGATORY"
    #[serde(rename="SENSITIVE_CATEGORY_DEROGATORY")]
    SENSITIVECATEGORYDEROGATORY,
    

    /// Content related to audio, video, or software downloads.
    ///
    /// "SENSITIVE_CATEGORY_DOWNLOADS_SHARING"
    #[serde(rename="SENSITIVE_CATEGORY_DOWNLOADS_SHARING")]
    SENSITIVECATEGORYDOWNLOADSSHARING,
    

    /// Contains content related to personal weapons, including knives, guns, small firearms, and ammunition. Selecting either "weapons" or "sensitive social issues" will result in selecting both.
    ///
    /// "SENSITIVE_CATEGORY_WEAPONS"
    #[serde(rename="SENSITIVE_CATEGORY_WEAPONS")]
    SENSITIVECATEGORYWEAPONS,
    

    /// Contains content related to betting or wagering in a real-world or online setting.
    ///
    /// "SENSITIVE_CATEGORY_GAMBLING"
    #[serde(rename="SENSITIVE_CATEGORY_GAMBLING")]
    SENSITIVECATEGORYGAMBLING,
    

    /// Content which may be considered graphically violent, gory, gruesome, or shocking, such as street fighting videos, accident photos, descriptions of torture, etc.
    ///
    /// "SENSITIVE_CATEGORY_VIOLENCE"
    #[serde(rename="SENSITIVE_CATEGORY_VIOLENCE")]
    SENSITIVECATEGORYVIOLENCE,
    

    /// Adult content, as well as suggestive content that's not explicitly pornographic. This category includes all pages categorized as adult.
    ///
    /// "SENSITIVE_CATEGORY_SUGGESTIVE"
    #[serde(rename="SENSITIVE_CATEGORY_SUGGESTIVE")]
    SENSITIVECATEGORYSUGGESTIVE,
    

    /// Prominent use of words considered indecent, such as curse words and sexual slang. Pages with only very occasional usage, such as news sites that might include such words in a quotation, are not included.
    ///
    /// "SENSITIVE_CATEGORY_PROFANITY"
    #[serde(rename="SENSITIVE_CATEGORY_PROFANITY")]
    SENSITIVECATEGORYPROFANITY,
    

    /// Contains content related to alcoholic beverages, alcohol brands, recipes, etc.
    ///
    /// "SENSITIVE_CATEGORY_ALCOHOL"
    #[serde(rename="SENSITIVE_CATEGORY_ALCOHOL")]
    SENSITIVECATEGORYALCOHOL,
    

    /// Contains content related to the recreational use of legal or illegal drugs, as well as to drug paraphernalia or cultivation.
    ///
    /// "SENSITIVE_CATEGORY_DRUGS"
    #[serde(rename="SENSITIVE_CATEGORY_DRUGS")]
    SENSITIVECATEGORYDRUGS,
    

    /// Contains content related to tobacco and tobacco accessories, including lighters, humidors, ashtrays, etc.
    ///
    /// "SENSITIVE_CATEGORY_TOBACCO"
    #[serde(rename="SENSITIVE_CATEGORY_TOBACCO")]
    SENSITIVECATEGORYTOBACCO,
    

    /// Political news and media, including discussions of social, governmental, and public policy.
    ///
    /// "SENSITIVE_CATEGORY_POLITICS"
    #[serde(rename="SENSITIVE_CATEGORY_POLITICS")]
    SENSITIVECATEGORYPOLITICS,
    

    /// Content related to religious thought or beliefs.
    ///
    /// "SENSITIVE_CATEGORY_RELIGION"
    #[serde(rename="SENSITIVE_CATEGORY_RELIGION")]
    SENSITIVECATEGORYRELIGION,
    

    /// Content related to death, disasters, accidents, war, etc.
    ///
    /// "SENSITIVE_CATEGORY_TRAGEDY"
    #[serde(rename="SENSITIVE_CATEGORY_TRAGEDY")]
    SENSITIVECATEGORYTRAGEDY,
    

    /// Content related to motor vehicle, aviation or other transportation accidents.
    ///
    /// "SENSITIVE_CATEGORY_TRANSPORTATION_ACCIDENTS"
    #[serde(rename="SENSITIVE_CATEGORY_TRANSPORTATION_ACCIDENTS")]
    SENSITIVECATEGORYTRANSPORTATIONACCIDENTS,
    

    /// Issues that evoke strong, opposing views and spark debate. These include issues that are controversial in most countries and markets (such as abortion), as well as those that are controversial in specific countries and markets (such as immigration reform in the United States).
    ///
    /// "SENSITIVE_CATEGORY_SENSITIVE_SOCIAL_ISSUES"
    #[serde(rename="SENSITIVE_CATEGORY_SENSITIVE_SOCIAL_ISSUES")]
    SENSITIVECATEGORYSENSITIVESOCIALISSUES,
    

    /// Content which may be considered shocking or disturbing, such as violent news stories, stunts, or toilet humor.
    ///
    /// "SENSITIVE_CATEGORY_SHOCKING"
    #[serde(rename="SENSITIVE_CATEGORY_SHOCKING")]
    SENSITIVECATEGORYSHOCKING,
}

impl AsRef<str> for SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYUNSPECIFIED => "SENSITIVE_CATEGORY_UNSPECIFIED",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYADULT => "SENSITIVE_CATEGORY_ADULT",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDEROGATORY => "SENSITIVE_CATEGORY_DEROGATORY",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDOWNLOADSSHARING => "SENSITIVE_CATEGORY_DOWNLOADS_SHARING",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYWEAPONS => "SENSITIVE_CATEGORY_WEAPONS",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYGAMBLING => "SENSITIVE_CATEGORY_GAMBLING",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYVIOLENCE => "SENSITIVE_CATEGORY_VIOLENCE",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSUGGESTIVE => "SENSITIVE_CATEGORY_SUGGESTIVE",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYPROFANITY => "SENSITIVE_CATEGORY_PROFANITY",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYALCOHOL => "SENSITIVE_CATEGORY_ALCOHOL",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDRUGS => "SENSITIVE_CATEGORY_DRUGS",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTOBACCO => "SENSITIVE_CATEGORY_TOBACCO",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYPOLITICS => "SENSITIVE_CATEGORY_POLITICS",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYRELIGION => "SENSITIVE_CATEGORY_RELIGION",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTRAGEDY => "SENSITIVE_CATEGORY_TRAGEDY",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTRANSPORTATIONACCIDENTS => "SENSITIVE_CATEGORY_TRANSPORTATION_ACCIDENTS",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSENSITIVESOCIALISSUES => "SENSITIVE_CATEGORY_SENSITIVE_SOCIAL_ISSUES",
            SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSHOCKING => "SENSITIVE_CATEGORY_SHOCKING",
        }
    }
}

impl std::convert::TryFrom< &str> for SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SENSITIVE_CATEGORY_UNSPECIFIED" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYUNSPECIFIED),
           "SENSITIVE_CATEGORY_ADULT" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYADULT),
           "SENSITIVE_CATEGORY_DEROGATORY" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDEROGATORY),
           "SENSITIVE_CATEGORY_DOWNLOADS_SHARING" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDOWNLOADSSHARING),
           "SENSITIVE_CATEGORY_WEAPONS" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYWEAPONS),
           "SENSITIVE_CATEGORY_GAMBLING" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYGAMBLING),
           "SENSITIVE_CATEGORY_VIOLENCE" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYVIOLENCE),
           "SENSITIVE_CATEGORY_SUGGESTIVE" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSUGGESTIVE),
           "SENSITIVE_CATEGORY_PROFANITY" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYPROFANITY),
           "SENSITIVE_CATEGORY_ALCOHOL" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYALCOHOL),
           "SENSITIVE_CATEGORY_DRUGS" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDRUGS),
           "SENSITIVE_CATEGORY_TOBACCO" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTOBACCO),
           "SENSITIVE_CATEGORY_POLITICS" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYPOLITICS),
           "SENSITIVE_CATEGORY_RELIGION" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYRELIGION),
           "SENSITIVE_CATEGORY_TRAGEDY" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTRAGEDY),
           "SENSITIVE_CATEGORY_TRANSPORTATION_ACCIDENTS" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTRANSPORTATIONACCIDENTS),
           "SENSITIVE_CATEGORY_SENSITIVE_SOCIAL_ISSUES" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSENSITIVESOCIALISSUES),
           "SENSITIVE_CATEGORY_SHOCKING" => Ok(SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSHOCKING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SensitiveCategoryAssignedTargetingOptionDetailSensitiveCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. An enum for the DV360 Sensitive category content classifier.
pub enum SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum {
    

    /// This enum is only a placeholder and doesn't specify a DV360 sensitive category.
    ///
    /// "SENSITIVE_CATEGORY_UNSPECIFIED"
    #[serde(rename="SENSITIVE_CATEGORY_UNSPECIFIED")]
    SENSITIVECATEGORYUNSPECIFIED,
    

    /// Adult or pornographic text, image, or video content.
    ///
    /// "SENSITIVE_CATEGORY_ADULT"
    #[serde(rename="SENSITIVE_CATEGORY_ADULT")]
    SENSITIVECATEGORYADULT,
    

    /// Content that may be construed as biased against individuals, groups, or organizations based on criteria such as race, religion, disability, sex, age, veteran status, sexual orientation, gender identity, or political affiliation. May also indicate discussion of such content, for instance, in an academic or journalistic context.
    ///
    /// "SENSITIVE_CATEGORY_DEROGATORY"
    #[serde(rename="SENSITIVE_CATEGORY_DEROGATORY")]
    SENSITIVECATEGORYDEROGATORY,
    

    /// Content related to audio, video, or software downloads.
    ///
    /// "SENSITIVE_CATEGORY_DOWNLOADS_SHARING"
    #[serde(rename="SENSITIVE_CATEGORY_DOWNLOADS_SHARING")]
    SENSITIVECATEGORYDOWNLOADSSHARING,
    

    /// Contains content related to personal weapons, including knives, guns, small firearms, and ammunition. Selecting either "weapons" or "sensitive social issues" will result in selecting both.
    ///
    /// "SENSITIVE_CATEGORY_WEAPONS"
    #[serde(rename="SENSITIVE_CATEGORY_WEAPONS")]
    SENSITIVECATEGORYWEAPONS,
    

    /// Contains content related to betting or wagering in a real-world or online setting.
    ///
    /// "SENSITIVE_CATEGORY_GAMBLING"
    #[serde(rename="SENSITIVE_CATEGORY_GAMBLING")]
    SENSITIVECATEGORYGAMBLING,
    

    /// Content which may be considered graphically violent, gory, gruesome, or shocking, such as street fighting videos, accident photos, descriptions of torture, etc.
    ///
    /// "SENSITIVE_CATEGORY_VIOLENCE"
    #[serde(rename="SENSITIVE_CATEGORY_VIOLENCE")]
    SENSITIVECATEGORYVIOLENCE,
    

    /// Adult content, as well as suggestive content that's not explicitly pornographic. This category includes all pages categorized as adult.
    ///
    /// "SENSITIVE_CATEGORY_SUGGESTIVE"
    #[serde(rename="SENSITIVE_CATEGORY_SUGGESTIVE")]
    SENSITIVECATEGORYSUGGESTIVE,
    

    /// Prominent use of words considered indecent, such as curse words and sexual slang. Pages with only very occasional usage, such as news sites that might include such words in a quotation, are not included.
    ///
    /// "SENSITIVE_CATEGORY_PROFANITY"
    #[serde(rename="SENSITIVE_CATEGORY_PROFANITY")]
    SENSITIVECATEGORYPROFANITY,
    

    /// Contains content related to alcoholic beverages, alcohol brands, recipes, etc.
    ///
    /// "SENSITIVE_CATEGORY_ALCOHOL"
    #[serde(rename="SENSITIVE_CATEGORY_ALCOHOL")]
    SENSITIVECATEGORYALCOHOL,
    

    /// Contains content related to the recreational use of legal or illegal drugs, as well as to drug paraphernalia or cultivation.
    ///
    /// "SENSITIVE_CATEGORY_DRUGS"
    #[serde(rename="SENSITIVE_CATEGORY_DRUGS")]
    SENSITIVECATEGORYDRUGS,
    

    /// Contains content related to tobacco and tobacco accessories, including lighters, humidors, ashtrays, etc.
    ///
    /// "SENSITIVE_CATEGORY_TOBACCO"
    #[serde(rename="SENSITIVE_CATEGORY_TOBACCO")]
    SENSITIVECATEGORYTOBACCO,
    

    /// Political news and media, including discussions of social, governmental, and public policy.
    ///
    /// "SENSITIVE_CATEGORY_POLITICS"
    #[serde(rename="SENSITIVE_CATEGORY_POLITICS")]
    SENSITIVECATEGORYPOLITICS,
    

    /// Content related to religious thought or beliefs.
    ///
    /// "SENSITIVE_CATEGORY_RELIGION"
    #[serde(rename="SENSITIVE_CATEGORY_RELIGION")]
    SENSITIVECATEGORYRELIGION,
    

    /// Content related to death, disasters, accidents, war, etc.
    ///
    /// "SENSITIVE_CATEGORY_TRAGEDY"
    #[serde(rename="SENSITIVE_CATEGORY_TRAGEDY")]
    SENSITIVECATEGORYTRAGEDY,
    

    /// Content related to motor vehicle, aviation or other transportation accidents.
    ///
    /// "SENSITIVE_CATEGORY_TRANSPORTATION_ACCIDENTS"
    #[serde(rename="SENSITIVE_CATEGORY_TRANSPORTATION_ACCIDENTS")]
    SENSITIVECATEGORYTRANSPORTATIONACCIDENTS,
    

    /// Issues that evoke strong, opposing views and spark debate. These include issues that are controversial in most countries and markets (such as abortion), as well as those that are controversial in specific countries and markets (such as immigration reform in the United States).
    ///
    /// "SENSITIVE_CATEGORY_SENSITIVE_SOCIAL_ISSUES"
    #[serde(rename="SENSITIVE_CATEGORY_SENSITIVE_SOCIAL_ISSUES")]
    SENSITIVECATEGORYSENSITIVESOCIALISSUES,
    

    /// Content which may be considered shocking or disturbing, such as violent news stories, stunts, or toilet humor.
    ///
    /// "SENSITIVE_CATEGORY_SHOCKING"
    #[serde(rename="SENSITIVE_CATEGORY_SHOCKING")]
    SENSITIVECATEGORYSHOCKING,
}

impl AsRef<str> for SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYUNSPECIFIED => "SENSITIVE_CATEGORY_UNSPECIFIED",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYADULT => "SENSITIVE_CATEGORY_ADULT",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDEROGATORY => "SENSITIVE_CATEGORY_DEROGATORY",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDOWNLOADSSHARING => "SENSITIVE_CATEGORY_DOWNLOADS_SHARING",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYWEAPONS => "SENSITIVE_CATEGORY_WEAPONS",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYGAMBLING => "SENSITIVE_CATEGORY_GAMBLING",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYVIOLENCE => "SENSITIVE_CATEGORY_VIOLENCE",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSUGGESTIVE => "SENSITIVE_CATEGORY_SUGGESTIVE",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYPROFANITY => "SENSITIVE_CATEGORY_PROFANITY",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYALCOHOL => "SENSITIVE_CATEGORY_ALCOHOL",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDRUGS => "SENSITIVE_CATEGORY_DRUGS",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTOBACCO => "SENSITIVE_CATEGORY_TOBACCO",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYPOLITICS => "SENSITIVE_CATEGORY_POLITICS",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYRELIGION => "SENSITIVE_CATEGORY_RELIGION",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTRAGEDY => "SENSITIVE_CATEGORY_TRAGEDY",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTRANSPORTATIONACCIDENTS => "SENSITIVE_CATEGORY_TRANSPORTATION_ACCIDENTS",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSENSITIVESOCIALISSUES => "SENSITIVE_CATEGORY_SENSITIVE_SOCIAL_ISSUES",
            SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSHOCKING => "SENSITIVE_CATEGORY_SHOCKING",
        }
    }
}

impl std::convert::TryFrom< &str> for SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SENSITIVE_CATEGORY_UNSPECIFIED" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYUNSPECIFIED),
           "SENSITIVE_CATEGORY_ADULT" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYADULT),
           "SENSITIVE_CATEGORY_DEROGATORY" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDEROGATORY),
           "SENSITIVE_CATEGORY_DOWNLOADS_SHARING" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDOWNLOADSSHARING),
           "SENSITIVE_CATEGORY_WEAPONS" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYWEAPONS),
           "SENSITIVE_CATEGORY_GAMBLING" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYGAMBLING),
           "SENSITIVE_CATEGORY_VIOLENCE" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYVIOLENCE),
           "SENSITIVE_CATEGORY_SUGGESTIVE" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSUGGESTIVE),
           "SENSITIVE_CATEGORY_PROFANITY" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYPROFANITY),
           "SENSITIVE_CATEGORY_ALCOHOL" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYALCOHOL),
           "SENSITIVE_CATEGORY_DRUGS" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYDRUGS),
           "SENSITIVE_CATEGORY_TOBACCO" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTOBACCO),
           "SENSITIVE_CATEGORY_POLITICS" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYPOLITICS),
           "SENSITIVE_CATEGORY_RELIGION" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYRELIGION),
           "SENSITIVE_CATEGORY_TRAGEDY" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTRAGEDY),
           "SENSITIVE_CATEGORY_TRANSPORTATION_ACCIDENTS" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYTRANSPORTATIONACCIDENTS),
           "SENSITIVE_CATEGORY_SENSITIVE_SOCIAL_ISSUES" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSENSITIVESOCIALISSUES),
           "SENSITIVE_CATEGORY_SHOCKING" => Ok(SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum::SENSITIVECATEGORYSHOCKING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SensitiveCategoryTargetingOptionDetailSensitiveCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TargetingExpansionConfigTargetingExpansionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Whether optimized targeting is turned on. This field supports the following values: * `NO_EXPANSION`: optimized targeting is turned off * `LEAST_EXPANSION`: optimized targeting is turned on If this field is set to any other value, it will automatically be set to `LEAST_EXPANSION`. `NO_EXPANSION` will be the default value for the field and will be automatically assigned if you do not set the field.
pub enum TargetingExpansionConfigTargetingExpansionLevelEnum {
    

    /// The optimized targeting setting is not specified or is unknown in this version.
    ///
    /// "TARGETING_EXPANSION_LEVEL_UNSPECIFIED"
    #[serde(rename="TARGETING_EXPANSION_LEVEL_UNSPECIFIED")]
    TARGETINGEXPANSIONLEVELUNSPECIFIED,
    

    /// Optimized targeting is off.
    ///
    /// "NO_EXPANSION"
    #[serde(rename="NO_EXPANSION")]
    NOEXPANSION,
    

    /// Optimized targeting is on.
    ///
    /// "LEAST_EXPANSION"
    #[serde(rename="LEAST_EXPANSION")]
    LEASTEXPANSION,
    

    /// If used, will automatically be set to `LEAST_EXPANSION`.
    ///
    /// "SOME_EXPANSION"
    #[serde(rename="SOME_EXPANSION")]
    SOMEEXPANSION,
    

    /// If used, will automatically be set to `LEAST_EXPANSION`.
    ///
    /// "BALANCED_EXPANSION"
    #[serde(rename="BALANCED_EXPANSION")]
    BALANCEDEXPANSION,
    

    /// If used, will automatically be set to `LEAST_EXPANSION`.
    ///
    /// "MORE_EXPANSION"
    #[serde(rename="MORE_EXPANSION")]
    MOREEXPANSION,
    

    /// If used, will automatically be set to `LEAST_EXPANSION`.
    ///
    /// "MOST_EXPANSION"
    #[serde(rename="MOST_EXPANSION")]
    MOSTEXPANSION,
}

impl AsRef<str> for TargetingExpansionConfigTargetingExpansionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetingExpansionConfigTargetingExpansionLevelEnum::TARGETINGEXPANSIONLEVELUNSPECIFIED => "TARGETING_EXPANSION_LEVEL_UNSPECIFIED",
            TargetingExpansionConfigTargetingExpansionLevelEnum::NOEXPANSION => "NO_EXPANSION",
            TargetingExpansionConfigTargetingExpansionLevelEnum::LEASTEXPANSION => "LEAST_EXPANSION",
            TargetingExpansionConfigTargetingExpansionLevelEnum::SOMEEXPANSION => "SOME_EXPANSION",
            TargetingExpansionConfigTargetingExpansionLevelEnum::BALANCEDEXPANSION => "BALANCED_EXPANSION",
            TargetingExpansionConfigTargetingExpansionLevelEnum::MOREEXPANSION => "MORE_EXPANSION",
            TargetingExpansionConfigTargetingExpansionLevelEnum::MOSTEXPANSION => "MOST_EXPANSION",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetingExpansionConfigTargetingExpansionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_EXPANSION_LEVEL_UNSPECIFIED" => Ok(TargetingExpansionConfigTargetingExpansionLevelEnum::TARGETINGEXPANSIONLEVELUNSPECIFIED),
           "NO_EXPANSION" => Ok(TargetingExpansionConfigTargetingExpansionLevelEnum::NOEXPANSION),
           "LEAST_EXPANSION" => Ok(TargetingExpansionConfigTargetingExpansionLevelEnum::LEASTEXPANSION),
           "SOME_EXPANSION" => Ok(TargetingExpansionConfigTargetingExpansionLevelEnum::SOMEEXPANSION),
           "BALANCED_EXPANSION" => Ok(TargetingExpansionConfigTargetingExpansionLevelEnum::BALANCEDEXPANSION),
           "MORE_EXPANSION" => Ok(TargetingExpansionConfigTargetingExpansionLevelEnum::MOREEXPANSION),
           "MOST_EXPANSION" => Ok(TargetingExpansionConfigTargetingExpansionLevelEnum::MOSTEXPANSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetingExpansionConfigTargetingExpansionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TargetingOptionTargetingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of this targeting option.
pub enum TargetingOptionTargetingTypeEnum {
    

    /// Default value when type is not specified or is unknown in this version.
    ///
    /// "TARGETING_TYPE_UNSPECIFIED"
    #[serde(rename="TARGETING_TYPE_UNSPECIFIED")]
    TARGETINGTYPEUNSPECIFIED,
    

    /// Target a channel (a custom group of related websites or apps).
    ///
    /// "TARGETING_TYPE_CHANNEL"
    #[serde(rename="TARGETING_TYPE_CHANNEL")]
    TARGETINGTYPECHANNEL,
    

    /// Target an app category (for example, education or puzzle games).
    ///
    /// "TARGETING_TYPE_APP_CATEGORY"
    #[serde(rename="TARGETING_TYPE_APP_CATEGORY")]
    TARGETINGTYPEAPPCATEGORY,
    

    /// Target a specific app (for example, Angry Birds).
    ///
    /// "TARGETING_TYPE_APP"
    #[serde(rename="TARGETING_TYPE_APP")]
    TARGETINGTYPEAPP,
    

    /// Target a specific url (for example, quora.com).
    ///
    /// "TARGETING_TYPE_URL"
    #[serde(rename="TARGETING_TYPE_URL")]
    TARGETINGTYPEURL,
    

    /// Target ads during a chosen time period on a specific day.
    ///
    /// "TARGETING_TYPE_DAY_AND_TIME"
    #[serde(rename="TARGETING_TYPE_DAY_AND_TIME")]
    TARGETINGTYPEDAYANDTIME,
    

    /// Target ads to a specific age range (for example, 18-24).
    ///
    /// "TARGETING_TYPE_AGE_RANGE"
    #[serde(rename="TARGETING_TYPE_AGE_RANGE")]
    TARGETINGTYPEAGERANGE,
    

    /// Target ads to the specified regions on a regional location list.
    ///
    /// "TARGETING_TYPE_REGIONAL_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    TARGETINGTYPEREGIONALLOCATIONLIST,
    

    /// Target ads to the specified points of interest on a proximity location list.
    ///
    /// "TARGETING_TYPE_PROXIMITY_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    TARGETINGTYPEPROXIMITYLOCATIONLIST,
    

    /// Target ads to a specific gender (for example, female or male).
    ///
    /// "TARGETING_TYPE_GENDER"
    #[serde(rename="TARGETING_TYPE_GENDER")]
    TARGETINGTYPEGENDER,
    

    /// Target a specific video player size for video ads.
    ///
    /// "TARGETING_TYPE_VIDEO_PLAYER_SIZE"
    #[serde(rename="TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    TARGETINGTYPEVIDEOPLAYERSIZE,
    

    /// Target user rewarded content for video ads.
    ///
    /// "TARGETING_TYPE_USER_REWARDED_CONTENT"
    #[serde(rename="TARGETING_TYPE_USER_REWARDED_CONTENT")]
    TARGETINGTYPEUSERREWARDEDCONTENT,
    

    /// Target ads to a specific parental status (for example, parent or not a parent).
    ///
    /// "TARGETING_TYPE_PARENTAL_STATUS"
    #[serde(rename="TARGETING_TYPE_PARENTAL_STATUS")]
    TARGETINGTYPEPARENTALSTATUS,
    

    /// Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll).
    ///
    /// "TARGETING_TYPE_CONTENT_INSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    TARGETINGTYPECONTENTINSTREAMPOSITION,
    

    /// Target ads in a specific content outstream position.
    ///
    /// "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    TARGETINGTYPECONTENTOUTSTREAMPOSITION,
    

    /// Target ads to a specific device type (for example, tablet or connected TV).
    ///
    /// "TARGETING_TYPE_DEVICE_TYPE"
    #[serde(rename="TARGETING_TYPE_DEVICE_TYPE")]
    TARGETINGTYPEDEVICETYPE,
    

    /// Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time.
    ///
    /// "TARGETING_TYPE_AUDIENCE_GROUP"
    #[serde(rename="TARGETING_TYPE_AUDIENCE_GROUP")]
    TARGETINGTYPEAUDIENCEGROUP,
    

    /// Target ads to specific web browsers (for example, Chrome).
    ///
    /// "TARGETING_TYPE_BROWSER"
    #[serde(rename="TARGETING_TYPE_BROWSER")]
    TARGETINGTYPEBROWSER,
    

    /// Target ads to a specific household income range (for example, top 10%).
    ///
    /// "TARGETING_TYPE_HOUSEHOLD_INCOME"
    #[serde(rename="TARGETING_TYPE_HOUSEHOLD_INCOME")]
    TARGETINGTYPEHOUSEHOLDINCOME,
    

    /// Target ads in a specific on screen position.
    ///
    /// "TARGETING_TYPE_ON_SCREEN_POSITION"
    #[serde(rename="TARGETING_TYPE_ON_SCREEN_POSITION")]
    TARGETINGTYPEONSCREENPOSITION,
    

    /// Filter web sites through third party verification (for example, IAS or DoubleVerify).
    ///
    /// "TARGETING_TYPE_THIRD_PARTY_VERIFIER"
    #[serde(rename="TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    TARGETINGTYPETHIRDPARTYVERIFIER,
    

    /// Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences).
    ///
    /// "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION,
    

    /// Filter website content by sensitive categories (for example, adult).
    ///
    /// "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    TARGETINGTYPESENSITIVECATEGORYEXCLUSION,
    

    /// Target ads to a specific environment (for example, web or app).
    ///
    /// "TARGETING_TYPE_ENVIRONMENT"
    #[serde(rename="TARGETING_TYPE_ENVIRONMENT")]
    TARGETINGTYPEENVIRONMENT,
    

    /// Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange).
    ///
    /// "TARGETING_TYPE_CARRIER_AND_ISP"
    #[serde(rename="TARGETING_TYPE_CARRIER_AND_ISP")]
    TARGETINGTYPECARRIERANDISP,
    

    /// Target ads to a specific operating system (for example, macOS).
    ///
    /// "TARGETING_TYPE_OPERATING_SYSTEM"
    #[serde(rename="TARGETING_TYPE_OPERATING_SYSTEM")]
    TARGETINGTYPEOPERATINGSYSTEM,
    

    /// Target ads to a specific device make or model (for example, Roku or Samsung).
    ///
    /// "TARGETING_TYPE_DEVICE_MAKE_MODEL"
    #[serde(rename="TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    TARGETINGTYPEDEVICEMAKEMODEL,
    

    /// Target ads to a specific keyword (for example, dog or retriever).
    ///
    /// "TARGETING_TYPE_KEYWORD"
    #[serde(rename="TARGETING_TYPE_KEYWORD")]
    TARGETINGTYPEKEYWORD,
    

    /// Target ads to a specific negative keyword list.
    ///
    /// "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST"
    #[serde(rename="TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    TARGETINGTYPENEGATIVEKEYWORDLIST,
    

    /// Target ads to a specific viewability (for example, 80% viewable).
    ///
    /// "TARGETING_TYPE_VIEWABILITY"
    #[serde(rename="TARGETING_TYPE_VIEWABILITY")]
    TARGETINGTYPEVIEWABILITY,
    

    /// Target ads to a specific content category (for example, arts & entertainment).
    ///
    /// "TARGETING_TYPE_CATEGORY"
    #[serde(rename="TARGETING_TYPE_CATEGORY")]
    TARGETINGTYPECATEGORY,
    

    /// Purchase impressions from specific deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE")]
    TARGETINGTYPEINVENTORYSOURCE,
    

    /// Target ads to a specific language (for example, English or Japanese).
    ///
    /// "TARGETING_TYPE_LANGUAGE"
    #[serde(rename="TARGETING_TYPE_LANGUAGE")]
    TARGETINGTYPELANGUAGE,
    

    /// Target ads to ads.txt authorized sellers. If no targeting option of this type is assigned, the resource uses the "Authorized Direct Sellers and Resellers" option by default.
    ///
    /// "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS"
    #[serde(rename="TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    TARGETINGTYPEAUTHORIZEDSELLERSTATUS,
    

    /// Target ads to a specific regional location (for example, a city or state).
    ///
    /// "TARGETING_TYPE_GEO_REGION"
    #[serde(rename="TARGETING_TYPE_GEO_REGION")]
    TARGETINGTYPEGEOREGION,
    

    /// Purchase impressions from a group of deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE_GROUP"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    TARGETINGTYPEINVENTORYSOURCEGROUP,
    

    /// Purchase impressions from specific exchanges.
    ///
    /// "TARGETING_TYPE_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_EXCHANGE")]
    TARGETINGTYPEEXCHANGE,
    

    /// Purchase impressions from specific sub-exchanges.
    ///
    /// "TARGETING_TYPE_SUB_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_SUB_EXCHANGE")]
    TARGETINGTYPESUBEXCHANGE,
    

    /// Target ads around a specific point of interest, such as a notable building, a street address, or latitude/longitude coordinates.
    ///
    /// "TARGETING_TYPE_POI"
    #[serde(rename="TARGETING_TYPE_POI")]
    TARGETINGTYPEPOI,
    

    /// Target ads around locations of a business chain within a specific geo region.
    ///
    /// "TARGETING_TYPE_BUSINESS_CHAIN"
    #[serde(rename="TARGETING_TYPE_BUSINESS_CHAIN")]
    TARGETINGTYPEBUSINESSCHAIN,
    

    /// Target ads to a specific video content duration.
    ///
    /// "TARGETING_TYPE_CONTENT_DURATION"
    #[serde(rename="TARGETING_TYPE_CONTENT_DURATION")]
    TARGETINGTYPECONTENTDURATION,
    

    /// Target ads to a specific video content stream type.
    ///
    /// "TARGETING_TYPE_CONTENT_STREAM_TYPE"
    #[serde(rename="TARGETING_TYPE_CONTENT_STREAM_TYPE")]
    TARGETINGTYPECONTENTSTREAMTYPE,
    

    /// Target ads to a specific native content position.
    ///
    /// "TARGETING_TYPE_NATIVE_CONTENT_POSITION"
    #[serde(rename="TARGETING_TYPE_NATIVE_CONTENT_POSITION")]
    TARGETINGTYPENATIVECONTENTPOSITION,
    

    /// Target ads in an Open Measurement enabled inventory.
    ///
    /// "TARGETING_TYPE_OMID"
    #[serde(rename="TARGETING_TYPE_OMID")]
    TARGETINGTYPEOMID,
    

    /// Target ads to a specific audio content type.
    ///
    /// "TARGETING_TYPE_AUDIO_CONTENT_TYPE"
    #[serde(rename="TARGETING_TYPE_AUDIO_CONTENT_TYPE")]
    TARGETINGTYPEAUDIOCONTENTTYPE,
    

    /// Target ads to a specific content genre.
    ///
    /// "TARGETING_TYPE_CONTENT_GENRE"
    #[serde(rename="TARGETING_TYPE_CONTENT_GENRE")]
    TARGETINGTYPECONTENTGENRE,
}

impl AsRef<str> for TargetingOptionTargetingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED => "TARGETING_TYPE_UNSPECIFIED",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPECHANNEL => "TARGETING_TYPE_CHANNEL",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY => "TARGETING_TYPE_APP_CATEGORY",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEAPP => "TARGETING_TYPE_APP",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEURL => "TARGETING_TYPE_URL",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEDAYANDTIME => "TARGETING_TYPE_DAY_AND_TIME",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEAGERANGE => "TARGETING_TYPE_AGE_RANGE",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST => "TARGETING_TYPE_REGIONAL_LOCATION_LIST",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST => "TARGETING_TYPE_PROXIMITY_LOCATION_LIST",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEGENDER => "TARGETING_TYPE_GENDER",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE => "TARGETING_TYPE_VIDEO_PLAYER_SIZE",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT => "TARGETING_TYPE_USER_REWARDED_CONTENT",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS => "TARGETING_TYPE_PARENTAL_STATUS",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION => "TARGETING_TYPE_CONTENT_INSTREAM_POSITION",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION => "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEDEVICETYPE => "TARGETING_TYPE_DEVICE_TYPE",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP => "TARGETING_TYPE_AUDIENCE_GROUP",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEBROWSER => "TARGETING_TYPE_BROWSER",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME => "TARGETING_TYPE_HOUSEHOLD_INCOME",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION => "TARGETING_TYPE_ON_SCREEN_POSITION",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER => "TARGETING_TYPE_THIRD_PARTY_VERIFIER",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION => "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION => "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEENVIRONMENT => "TARGETING_TYPE_ENVIRONMENT",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPECARRIERANDISP => "TARGETING_TYPE_CARRIER_AND_ISP",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM => "TARGETING_TYPE_OPERATING_SYSTEM",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL => "TARGETING_TYPE_DEVICE_MAKE_MODEL",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEKEYWORD => "TARGETING_TYPE_KEYWORD",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST => "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEVIEWABILITY => "TARGETING_TYPE_VIEWABILITY",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPECATEGORY => "TARGETING_TYPE_CATEGORY",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE => "TARGETING_TYPE_INVENTORY_SOURCE",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPELANGUAGE => "TARGETING_TYPE_LANGUAGE",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS => "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEGEOREGION => "TARGETING_TYPE_GEO_REGION",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP => "TARGETING_TYPE_INVENTORY_SOURCE_GROUP",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEEXCHANGE => "TARGETING_TYPE_EXCHANGE",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE => "TARGETING_TYPE_SUB_EXCHANGE",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEPOI => "TARGETING_TYPE_POI",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN => "TARGETING_TYPE_BUSINESS_CHAIN",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTDURATION => "TARGETING_TYPE_CONTENT_DURATION",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE => "TARGETING_TYPE_CONTENT_STREAM_TYPE",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION => "TARGETING_TYPE_NATIVE_CONTENT_POSITION",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEOMID => "TARGETING_TYPE_OMID",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE => "TARGETING_TYPE_AUDIO_CONTENT_TYPE",
            TargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTGENRE => "TARGETING_TYPE_CONTENT_GENRE",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetingOptionTargetingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_TYPE_UNSPECIFIED" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED),
           "TARGETING_TYPE_CHANNEL" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPECHANNEL),
           "TARGETING_TYPE_APP_CATEGORY" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY),
           "TARGETING_TYPE_APP" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEAPP),
           "TARGETING_TYPE_URL" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEURL),
           "TARGETING_TYPE_DAY_AND_TIME" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEDAYANDTIME),
           "TARGETING_TYPE_AGE_RANGE" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEAGERANGE),
           "TARGETING_TYPE_REGIONAL_LOCATION_LIST" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST),
           "TARGETING_TYPE_PROXIMITY_LOCATION_LIST" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST),
           "TARGETING_TYPE_GENDER" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEGENDER),
           "TARGETING_TYPE_VIDEO_PLAYER_SIZE" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE),
           "TARGETING_TYPE_USER_REWARDED_CONTENT" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT),
           "TARGETING_TYPE_PARENTAL_STATUS" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS),
           "TARGETING_TYPE_CONTENT_INSTREAM_POSITION" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION),
           "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION),
           "TARGETING_TYPE_DEVICE_TYPE" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEDEVICETYPE),
           "TARGETING_TYPE_AUDIENCE_GROUP" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP),
           "TARGETING_TYPE_BROWSER" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEBROWSER),
           "TARGETING_TYPE_HOUSEHOLD_INCOME" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME),
           "TARGETING_TYPE_ON_SCREEN_POSITION" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION),
           "TARGETING_TYPE_THIRD_PARTY_VERIFIER" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER),
           "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION),
           "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION),
           "TARGETING_TYPE_ENVIRONMENT" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEENVIRONMENT),
           "TARGETING_TYPE_CARRIER_AND_ISP" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPECARRIERANDISP),
           "TARGETING_TYPE_OPERATING_SYSTEM" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM),
           "TARGETING_TYPE_DEVICE_MAKE_MODEL" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL),
           "TARGETING_TYPE_KEYWORD" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEKEYWORD),
           "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST),
           "TARGETING_TYPE_VIEWABILITY" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEVIEWABILITY),
           "TARGETING_TYPE_CATEGORY" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPECATEGORY),
           "TARGETING_TYPE_INVENTORY_SOURCE" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE),
           "TARGETING_TYPE_LANGUAGE" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPELANGUAGE),
           "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS),
           "TARGETING_TYPE_GEO_REGION" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEGEOREGION),
           "TARGETING_TYPE_INVENTORY_SOURCE_GROUP" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP),
           "TARGETING_TYPE_EXCHANGE" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEEXCHANGE),
           "TARGETING_TYPE_SUB_EXCHANGE" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE),
           "TARGETING_TYPE_POI" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEPOI),
           "TARGETING_TYPE_BUSINESS_CHAIN" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN),
           "TARGETING_TYPE_CONTENT_DURATION" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTDURATION),
           "TARGETING_TYPE_CONTENT_STREAM_TYPE" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE),
           "TARGETING_TYPE_NATIVE_CONTENT_POSITION" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION),
           "TARGETING_TYPE_OMID" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEOMID),
           "TARGETING_TYPE_AUDIO_CONTENT_TYPE" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE),
           "TARGETING_TYPE_CONTENT_GENRE" => Ok(TargetingOptionTargetingTypeEnum::TARGETINGTYPECONTENTGENRE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetingOptionTargetingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ThirdPartyUrlTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of interaction needs to be tracked by the tracking URL
pub enum ThirdPartyUrlTypeEnum {
    

    /// The type of third-party URL is unspecified or is unknown in this version.
    ///
    /// "THIRD_PARTY_URL_TYPE_UNSPECIFIED"
    #[serde(rename="THIRD_PARTY_URL_TYPE_UNSPECIFIED")]
    THIRDPARTYURLTYPEUNSPECIFIED,
    

    /// Used to count impressions of the creative after the audio or video buffering is complete.
    ///
    /// "THIRD_PARTY_URL_TYPE_IMPRESSION"
    #[serde(rename="THIRD_PARTY_URL_TYPE_IMPRESSION")]
    THIRDPARTYURLTYPEIMPRESSION,
    

    /// Used to track user clicks on the audio or video.
    ///
    /// "THIRD_PARTY_URL_TYPE_CLICK_TRACKING"
    #[serde(rename="THIRD_PARTY_URL_TYPE_CLICK_TRACKING")]
    THIRDPARTYURLTYPECLICKTRACKING,
    

    /// Used to track the number of times a user starts the audio or video.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_START"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_START")]
    THIRDPARTYURLTYPEAUDIOVIDEOSTART,
    

    /// Used to track the number of times the audio or video plays to 25% of its length.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_FIRST_QUARTILE"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_FIRST_QUARTILE")]
    THIRDPARTYURLTYPEAUDIOVIDEOFIRSTQUARTILE,
    

    /// Used to track the number of times the audio or video plays to 50% of its length.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_MIDPOINT"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_MIDPOINT")]
    THIRDPARTYURLTYPEAUDIOVIDEOMIDPOINT,
    

    /// Used to track the number of times the audio or video plays to 75% of its length.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_THIRD_QUARTILE"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_THIRD_QUARTILE")]
    THIRDPARTYURLTYPEAUDIOVIDEOTHIRDQUARTILE,
    

    /// Used to track the number of times the audio or video plays to the end.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_COMPLETE"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_COMPLETE")]
    THIRDPARTYURLTYPEAUDIOVIDEOCOMPLETE,
    

    /// Used to track the number of times a user mutes the audio or video.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_MUTE"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_MUTE")]
    THIRDPARTYURLTYPEAUDIOVIDEOMUTE,
    

    /// Used to track the number of times a user pauses the audio or video.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_PAUSE"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_PAUSE")]
    THIRDPARTYURLTYPEAUDIOVIDEOPAUSE,
    

    /// Used to track the number of times a user replays the audio or video.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_REWIND"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_REWIND")]
    THIRDPARTYURLTYPEAUDIOVIDEOREWIND,
    

    /// Used to track the number of times a user expands the player to full-screen size.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_FULLSCREEN"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_FULLSCREEN")]
    THIRDPARTYURLTYPEAUDIOVIDEOFULLSCREEN,
    

    /// Used to track the number of times a user stops the audio or video.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_STOP"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_STOP")]
    THIRDPARTYURLTYPEAUDIOVIDEOSTOP,
    

    /// Used to track the number of times a user performs a custom click, such as clicking on a video hot spot.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_CUSTOM"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_CUSTOM")]
    THIRDPARTYURLTYPEAUDIOVIDEOCUSTOM,
    

    /// Used to track the number of times the audio or video was skipped.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_SKIP"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_SKIP")]
    THIRDPARTYURLTYPEAUDIOVIDEOSKIP,
    

    /// Used to track the number of times the audio or video plays to an offset determined by the progress_offset.
    ///
    /// "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_PROGRESS"
    #[serde(rename="THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_PROGRESS")]
    THIRDPARTYURLTYPEAUDIOVIDEOPROGRESS,
}

impl AsRef<str> for ThirdPartyUrlTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEUNSPECIFIED => "THIRD_PARTY_URL_TYPE_UNSPECIFIED",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEIMPRESSION => "THIRD_PARTY_URL_TYPE_IMPRESSION",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPECLICKTRACKING => "THIRD_PARTY_URL_TYPE_CLICK_TRACKING",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOSTART => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_START",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOFIRSTQUARTILE => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_FIRST_QUARTILE",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOMIDPOINT => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_MIDPOINT",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOTHIRDQUARTILE => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_THIRD_QUARTILE",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOCOMPLETE => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_COMPLETE",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOMUTE => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_MUTE",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOPAUSE => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_PAUSE",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOREWIND => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_REWIND",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOFULLSCREEN => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_FULLSCREEN",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOSTOP => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_STOP",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOCUSTOM => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_CUSTOM",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOSKIP => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_SKIP",
            ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOPROGRESS => "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_PROGRESS",
        }
    }
}

impl std::convert::TryFrom< &str> for ThirdPartyUrlTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THIRD_PARTY_URL_TYPE_UNSPECIFIED" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEUNSPECIFIED),
           "THIRD_PARTY_URL_TYPE_IMPRESSION" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEIMPRESSION),
           "THIRD_PARTY_URL_TYPE_CLICK_TRACKING" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPECLICKTRACKING),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_START" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOSTART),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_FIRST_QUARTILE" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOFIRSTQUARTILE),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_MIDPOINT" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOMIDPOINT),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_THIRD_QUARTILE" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOTHIRDQUARTILE),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_COMPLETE" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOCOMPLETE),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_MUTE" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOMUTE),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_PAUSE" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOPAUSE),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_REWIND" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOREWIND),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_FULLSCREEN" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOFULLSCREEN),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_STOP" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOSTOP),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_CUSTOM" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOCUSTOM),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_SKIP" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOSKIP),
           "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_PROGRESS" => Ok(ThirdPartyUrlTypeEnum::THIRDPARTYURLTYPEAUDIOVIDEOPROGRESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThirdPartyUrlTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UniversalAdIdRegistryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The registry provides unique creative identifiers.
pub enum UniversalAdIdRegistryEnum {
    

    /// The Universal Ad registry is unspecified or is unknown in this version.
    ///
    /// "UNIVERSAL_AD_REGISTRY_UNSPECIFIED"
    #[serde(rename="UNIVERSAL_AD_REGISTRY_UNSPECIFIED")]
    UNIVERSALADREGISTRYUNSPECIFIED,
    

    /// Use a custom provider to provide the Universal Ad ID.
    ///
    /// "UNIVERSAL_AD_REGISTRY_OTHER"
    #[serde(rename="UNIVERSAL_AD_REGISTRY_OTHER")]
    UNIVERSALADREGISTRYOTHER,
    

    /// Use Ad-ID to provide the Universal Ad ID.
    ///
    /// "UNIVERSAL_AD_REGISTRY_AD_ID"
    #[serde(rename="UNIVERSAL_AD_REGISTRY_AD_ID")]
    UNIVERSALADREGISTRYADID,
    

    /// Use clearcast.co.uk to provide the Universal Ad ID.
    ///
    /// "UNIVERSAL_AD_REGISTRY_CLEARCAST"
    #[serde(rename="UNIVERSAL_AD_REGISTRY_CLEARCAST")]
    UNIVERSALADREGISTRYCLEARCAST,
    

    /// Use Display & Video 360 to provide the Universal Ad ID.
    ///
    /// "UNIVERSAL_AD_REGISTRY_DV360"
    #[serde(rename="UNIVERSAL_AD_REGISTRY_DV360")]
    UNIVERSALADREGISTRYDV360,
    

    /// Use Campaign Manager 360 to provide the Universal Ad ID.
    ///
    /// "UNIVERSAL_AD_REGISTRY_CM"
    #[serde(rename="UNIVERSAL_AD_REGISTRY_CM")]
    UNIVERSALADREGISTRYCM,
}

impl AsRef<str> for UniversalAdIdRegistryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYUNSPECIFIED => "UNIVERSAL_AD_REGISTRY_UNSPECIFIED",
            UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYOTHER => "UNIVERSAL_AD_REGISTRY_OTHER",
            UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYADID => "UNIVERSAL_AD_REGISTRY_AD_ID",
            UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYCLEARCAST => "UNIVERSAL_AD_REGISTRY_CLEARCAST",
            UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYDV360 => "UNIVERSAL_AD_REGISTRY_DV360",
            UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYCM => "UNIVERSAL_AD_REGISTRY_CM",
        }
    }
}

impl std::convert::TryFrom< &str> for UniversalAdIdRegistryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNIVERSAL_AD_REGISTRY_UNSPECIFIED" => Ok(UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYUNSPECIFIED),
           "UNIVERSAL_AD_REGISTRY_OTHER" => Ok(UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYOTHER),
           "UNIVERSAL_AD_REGISTRY_AD_ID" => Ok(UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYADID),
           "UNIVERSAL_AD_REGISTRY_CLEARCAST" => Ok(UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYCLEARCAST),
           "UNIVERSAL_AD_REGISTRY_DV360" => Ok(UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYDV360),
           "UNIVERSAL_AD_REGISTRY_CM" => Ok(UniversalAdIdRegistryEnum::UNIVERSALADREGISTRYCM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UniversalAdIdRegistryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. User rewarded content status for video ads.
pub enum UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum {
    

    /// User rewarded content is not specified or is unknown in this version.
    ///
    /// "USER_REWARDED_CONTENT_UNSPECIFIED"
    #[serde(rename="USER_REWARDED_CONTENT_UNSPECIFIED")]
    USERREWARDEDCONTENTUNSPECIFIED,
    

    /// Represents ads where the user will see a reward after viewing.
    ///
    /// "USER_REWARDED_CONTENT_USER_REWARDED"
    #[serde(rename="USER_REWARDED_CONTENT_USER_REWARDED")]
    USERREWARDEDCONTENTUSERREWARDED,
    

    /// Represents all other ads besides user-rewarded.
    ///
    /// "USER_REWARDED_CONTENT_NOT_USER_REWARDED"
    #[serde(rename="USER_REWARDED_CONTENT_NOT_USER_REWARDED")]
    USERREWARDEDCONTENTNOTUSERREWARDED,
}

impl AsRef<str> for UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTUNSPECIFIED => "USER_REWARDED_CONTENT_UNSPECIFIED",
            UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTUSERREWARDED => "USER_REWARDED_CONTENT_USER_REWARDED",
            UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTNOTUSERREWARDED => "USER_REWARDED_CONTENT_NOT_USER_REWARDED",
        }
    }
}

impl std::convert::TryFrom< &str> for UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_REWARDED_CONTENT_UNSPECIFIED" => Ok(UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTUNSPECIFIED),
           "USER_REWARDED_CONTENT_USER_REWARDED" => Ok(UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTUSERREWARDED),
           "USER_REWARDED_CONTENT_NOT_USER_REWARDED" => Ok(UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTNOTUSERREWARDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserRewardedContentAssignedTargetingOptionDetailUserRewardedContentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserRewardedContentTargetingOptionDetailUserRewardedContentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. User rewarded content status for video ads.
pub enum UserRewardedContentTargetingOptionDetailUserRewardedContentEnum {
    

    /// User rewarded content is not specified or is unknown in this version.
    ///
    /// "USER_REWARDED_CONTENT_UNSPECIFIED"
    #[serde(rename="USER_REWARDED_CONTENT_UNSPECIFIED")]
    USERREWARDEDCONTENTUNSPECIFIED,
    

    /// Represents ads where the user will see a reward after viewing.
    ///
    /// "USER_REWARDED_CONTENT_USER_REWARDED"
    #[serde(rename="USER_REWARDED_CONTENT_USER_REWARDED")]
    USERREWARDEDCONTENTUSERREWARDED,
    

    /// Represents all other ads besides user-rewarded.
    ///
    /// "USER_REWARDED_CONTENT_NOT_USER_REWARDED"
    #[serde(rename="USER_REWARDED_CONTENT_NOT_USER_REWARDED")]
    USERREWARDEDCONTENTNOTUSERREWARDED,
}

impl AsRef<str> for UserRewardedContentTargetingOptionDetailUserRewardedContentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserRewardedContentTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTUNSPECIFIED => "USER_REWARDED_CONTENT_UNSPECIFIED",
            UserRewardedContentTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTUSERREWARDED => "USER_REWARDED_CONTENT_USER_REWARDED",
            UserRewardedContentTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTNOTUSERREWARDED => "USER_REWARDED_CONTENT_NOT_USER_REWARDED",
        }
    }
}

impl std::convert::TryFrom< &str> for UserRewardedContentTargetingOptionDetailUserRewardedContentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_REWARDED_CONTENT_UNSPECIFIED" => Ok(UserRewardedContentTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTUNSPECIFIED),
           "USER_REWARDED_CONTENT_USER_REWARDED" => Ok(UserRewardedContentTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTUSERREWARDED),
           "USER_REWARDED_CONTENT_NOT_USER_REWARDED" => Ok(UserRewardedContentTargetingOptionDetailUserRewardedContentEnum::USERREWARDEDCONTENTNOTUSERREWARDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserRewardedContentTargetingOptionDetailUserRewardedContentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The video player size.
pub enum VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum {
    

    /// Video player size is not specified in this version. This enum is a place holder for a default value and does not represent a real video player size.
    ///
    /// "VIDEO_PLAYER_SIZE_UNSPECIFIED"
    #[serde(rename="VIDEO_PLAYER_SIZE_UNSPECIFIED")]
    VIDEOPLAYERSIZEUNSPECIFIED,
    

    /// The dimensions of the video player are less than 400×300 (desktop), or up to 20% of screen covered (mobile).
    ///
    /// "VIDEO_PLAYER_SIZE_SMALL"
    #[serde(rename="VIDEO_PLAYER_SIZE_SMALL")]
    VIDEOPLAYERSIZESMALL,
    

    /// The dimensions of the video player are between 400x300 and 1280x720 pixels (desktop), or 20% to 90% of the screen covered (mobile).
    ///
    /// "VIDEO_PLAYER_SIZE_LARGE"
    #[serde(rename="VIDEO_PLAYER_SIZE_LARGE")]
    VIDEOPLAYERSIZELARGE,
    

    /// The dimensions of the video player are 1280×720 or greater (desktop), or over 90% of the screen covered (mobile).
    ///
    /// "VIDEO_PLAYER_SIZE_HD"
    #[serde(rename="VIDEO_PLAYER_SIZE_HD")]
    VIDEOPLAYERSIZEHD,
    

    /// The dimensions of the video player are unknown.
    ///
    /// "VIDEO_PLAYER_SIZE_UNKNOWN"
    #[serde(rename="VIDEO_PLAYER_SIZE_UNKNOWN")]
    VIDEOPLAYERSIZEUNKNOWN,
}

impl AsRef<str> for VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEUNSPECIFIED => "VIDEO_PLAYER_SIZE_UNSPECIFIED",
            VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZESMALL => "VIDEO_PLAYER_SIZE_SMALL",
            VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZELARGE => "VIDEO_PLAYER_SIZE_LARGE",
            VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEHD => "VIDEO_PLAYER_SIZE_HD",
            VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEUNKNOWN => "VIDEO_PLAYER_SIZE_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIDEO_PLAYER_SIZE_UNSPECIFIED" => Ok(VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEUNSPECIFIED),
           "VIDEO_PLAYER_SIZE_SMALL" => Ok(VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZESMALL),
           "VIDEO_PLAYER_SIZE_LARGE" => Ok(VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZELARGE),
           "VIDEO_PLAYER_SIZE_HD" => Ok(VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEHD),
           "VIDEO_PLAYER_SIZE_UNKNOWN" => Ok(VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoPlayerSizeAssignedTargetingOptionDetailVideoPlayerSizeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The video player size.
pub enum VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum {
    

    /// Video player size is not specified in this version. This enum is a place holder for a default value and does not represent a real video player size.
    ///
    /// "VIDEO_PLAYER_SIZE_UNSPECIFIED"
    #[serde(rename="VIDEO_PLAYER_SIZE_UNSPECIFIED")]
    VIDEOPLAYERSIZEUNSPECIFIED,
    

    /// The dimensions of the video player are less than 400×300 (desktop), or up to 20% of screen covered (mobile).
    ///
    /// "VIDEO_PLAYER_SIZE_SMALL"
    #[serde(rename="VIDEO_PLAYER_SIZE_SMALL")]
    VIDEOPLAYERSIZESMALL,
    

    /// The dimensions of the video player are between 400x300 and 1280x720 pixels (desktop), or 20% to 90% of the screen covered (mobile).
    ///
    /// "VIDEO_PLAYER_SIZE_LARGE"
    #[serde(rename="VIDEO_PLAYER_SIZE_LARGE")]
    VIDEOPLAYERSIZELARGE,
    

    /// The dimensions of the video player are 1280×720 or greater (desktop), or over 90% of the screen covered (mobile).
    ///
    /// "VIDEO_PLAYER_SIZE_HD"
    #[serde(rename="VIDEO_PLAYER_SIZE_HD")]
    VIDEOPLAYERSIZEHD,
    

    /// The dimensions of the video player are unknown.
    ///
    /// "VIDEO_PLAYER_SIZE_UNKNOWN"
    #[serde(rename="VIDEO_PLAYER_SIZE_UNKNOWN")]
    VIDEOPLAYERSIZEUNKNOWN,
}

impl AsRef<str> for VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEUNSPECIFIED => "VIDEO_PLAYER_SIZE_UNSPECIFIED",
            VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZESMALL => "VIDEO_PLAYER_SIZE_SMALL",
            VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZELARGE => "VIDEO_PLAYER_SIZE_LARGE",
            VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEHD => "VIDEO_PLAYER_SIZE_HD",
            VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEUNKNOWN => "VIDEO_PLAYER_SIZE_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIDEO_PLAYER_SIZE_UNSPECIFIED" => Ok(VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEUNSPECIFIED),
           "VIDEO_PLAYER_SIZE_SMALL" => Ok(VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZESMALL),
           "VIDEO_PLAYER_SIZE_LARGE" => Ok(VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZELARGE),
           "VIDEO_PLAYER_SIZE_HD" => Ok(VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEHD),
           "VIDEO_PLAYER_SIZE_UNKNOWN" => Ok(VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum::VIDEOPLAYERSIZEUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoPlayerSizeTargetingOptionDetailVideoPlayerSizeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ViewabilityAssignedTargetingOptionDetailViewabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The predicted viewability percentage.
pub enum ViewabilityAssignedTargetingOptionDetailViewabilityEnum {
    

    /// Default value when viewability is not specified in this version. This enum is a placeholder for default value and does not represent a real viewability option.
    ///
    /// "VIEWABILITY_UNSPECIFIED"
    #[serde(rename="VIEWABILITY_UNSPECIFIED")]
    VIEWABILITYUNSPECIFIED,
    

    /// Bid only on impressions that are at least 10% likely to be viewable.
    ///
    /// "VIEWABILITY_10_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_10_PERCENT_OR_MORE")]
    VIEWABILITY10PERCENTORMORE,
    

    /// Bid only on impressions that are at least 20% likely to be viewable.
    ///
    /// "VIEWABILITY_20_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_20_PERCENT_OR_MORE")]
    VIEWABILITY20PERCENTORMORE,
    

    /// Bid only on impressions that are at least 30% likely to be viewable.
    ///
    /// "VIEWABILITY_30_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_30_PERCENT_OR_MORE")]
    VIEWABILITY30PERCENTORMORE,
    

    /// Bid only on impressions that are at least 40% likely to be viewable.
    ///
    /// "VIEWABILITY_40_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_40_PERCENT_OR_MORE")]
    VIEWABILITY40PERCENTORMORE,
    

    /// Bid only on impressions that are at least 50% likely to be viewable.
    ///
    /// "VIEWABILITY_50_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_50_PERCENT_OR_MORE")]
    VIEWABILITY50PERCENTORMORE,
    

    /// Bid only on impressions that are at least 60% likely to be viewable.
    ///
    /// "VIEWABILITY_60_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_60_PERCENT_OR_MORE")]
    VIEWABILITY60PERCENTORMORE,
    

    /// Bid only on impressions that are at least 70% likely to be viewable.
    ///
    /// "VIEWABILITY_70_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_70_PERCENT_OR_MORE")]
    VIEWABILITY70PERCENTORMORE,
    

    /// Bid only on impressions that are at least 80% likely to be viewable.
    ///
    /// "VIEWABILITY_80_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_80_PERCENT_OR_MORE")]
    VIEWABILITY80PERCENTORMORE,
    

    /// Bid only on impressions that are at least 90% likely to be viewable.
    ///
    /// "VIEWABILITY_90_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_90_PERCENT_OR_MORE")]
    VIEWABILITY90PERCENTORMORE,
}

impl AsRef<str> for ViewabilityAssignedTargetingOptionDetailViewabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITYUNSPECIFIED => "VIEWABILITY_UNSPECIFIED",
            ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY10PERCENTORMORE => "VIEWABILITY_10_PERCENT_OR_MORE",
            ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY20PERCENTORMORE => "VIEWABILITY_20_PERCENT_OR_MORE",
            ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY30PERCENTORMORE => "VIEWABILITY_30_PERCENT_OR_MORE",
            ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY40PERCENTORMORE => "VIEWABILITY_40_PERCENT_OR_MORE",
            ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY50PERCENTORMORE => "VIEWABILITY_50_PERCENT_OR_MORE",
            ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY60PERCENTORMORE => "VIEWABILITY_60_PERCENT_OR_MORE",
            ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY70PERCENTORMORE => "VIEWABILITY_70_PERCENT_OR_MORE",
            ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY80PERCENTORMORE => "VIEWABILITY_80_PERCENT_OR_MORE",
            ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY90PERCENTORMORE => "VIEWABILITY_90_PERCENT_OR_MORE",
        }
    }
}

impl std::convert::TryFrom< &str> for ViewabilityAssignedTargetingOptionDetailViewabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEWABILITY_UNSPECIFIED" => Ok(ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITYUNSPECIFIED),
           "VIEWABILITY_10_PERCENT_OR_MORE" => Ok(ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY10PERCENTORMORE),
           "VIEWABILITY_20_PERCENT_OR_MORE" => Ok(ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY20PERCENTORMORE),
           "VIEWABILITY_30_PERCENT_OR_MORE" => Ok(ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY30PERCENTORMORE),
           "VIEWABILITY_40_PERCENT_OR_MORE" => Ok(ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY40PERCENTORMORE),
           "VIEWABILITY_50_PERCENT_OR_MORE" => Ok(ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY50PERCENTORMORE),
           "VIEWABILITY_60_PERCENT_OR_MORE" => Ok(ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY60PERCENTORMORE),
           "VIEWABILITY_70_PERCENT_OR_MORE" => Ok(ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY70PERCENTORMORE),
           "VIEWABILITY_80_PERCENT_OR_MORE" => Ok(ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY80PERCENTORMORE),
           "VIEWABILITY_90_PERCENT_OR_MORE" => Ok(ViewabilityAssignedTargetingOptionDetailViewabilityEnum::VIEWABILITY90PERCENTORMORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ViewabilityAssignedTargetingOptionDetailViewabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ViewabilityTargetingOptionDetailViewabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The predicted viewability percentage.
pub enum ViewabilityTargetingOptionDetailViewabilityEnum {
    

    /// Default value when viewability is not specified in this version. This enum is a placeholder for default value and does not represent a real viewability option.
    ///
    /// "VIEWABILITY_UNSPECIFIED"
    #[serde(rename="VIEWABILITY_UNSPECIFIED")]
    VIEWABILITYUNSPECIFIED,
    

    /// Bid only on impressions that are at least 10% likely to be viewable.
    ///
    /// "VIEWABILITY_10_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_10_PERCENT_OR_MORE")]
    VIEWABILITY10PERCENTORMORE,
    

    /// Bid only on impressions that are at least 20% likely to be viewable.
    ///
    /// "VIEWABILITY_20_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_20_PERCENT_OR_MORE")]
    VIEWABILITY20PERCENTORMORE,
    

    /// Bid only on impressions that are at least 30% likely to be viewable.
    ///
    /// "VIEWABILITY_30_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_30_PERCENT_OR_MORE")]
    VIEWABILITY30PERCENTORMORE,
    

    /// Bid only on impressions that are at least 40% likely to be viewable.
    ///
    /// "VIEWABILITY_40_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_40_PERCENT_OR_MORE")]
    VIEWABILITY40PERCENTORMORE,
    

    /// Bid only on impressions that are at least 50% likely to be viewable.
    ///
    /// "VIEWABILITY_50_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_50_PERCENT_OR_MORE")]
    VIEWABILITY50PERCENTORMORE,
    

    /// Bid only on impressions that are at least 60% likely to be viewable.
    ///
    /// "VIEWABILITY_60_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_60_PERCENT_OR_MORE")]
    VIEWABILITY60PERCENTORMORE,
    

    /// Bid only on impressions that are at least 70% likely to be viewable.
    ///
    /// "VIEWABILITY_70_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_70_PERCENT_OR_MORE")]
    VIEWABILITY70PERCENTORMORE,
    

    /// Bid only on impressions that are at least 80% likely to be viewable.
    ///
    /// "VIEWABILITY_80_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_80_PERCENT_OR_MORE")]
    VIEWABILITY80PERCENTORMORE,
    

    /// Bid only on impressions that are at least 90% likely to be viewable.
    ///
    /// "VIEWABILITY_90_PERCENT_OR_MORE"
    #[serde(rename="VIEWABILITY_90_PERCENT_OR_MORE")]
    VIEWABILITY90PERCENTORMORE,
}

impl AsRef<str> for ViewabilityTargetingOptionDetailViewabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITYUNSPECIFIED => "VIEWABILITY_UNSPECIFIED",
            ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY10PERCENTORMORE => "VIEWABILITY_10_PERCENT_OR_MORE",
            ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY20PERCENTORMORE => "VIEWABILITY_20_PERCENT_OR_MORE",
            ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY30PERCENTORMORE => "VIEWABILITY_30_PERCENT_OR_MORE",
            ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY40PERCENTORMORE => "VIEWABILITY_40_PERCENT_OR_MORE",
            ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY50PERCENTORMORE => "VIEWABILITY_50_PERCENT_OR_MORE",
            ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY60PERCENTORMORE => "VIEWABILITY_60_PERCENT_OR_MORE",
            ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY70PERCENTORMORE => "VIEWABILITY_70_PERCENT_OR_MORE",
            ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY80PERCENTORMORE => "VIEWABILITY_80_PERCENT_OR_MORE",
            ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY90PERCENTORMORE => "VIEWABILITY_90_PERCENT_OR_MORE",
        }
    }
}

impl std::convert::TryFrom< &str> for ViewabilityTargetingOptionDetailViewabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEWABILITY_UNSPECIFIED" => Ok(ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITYUNSPECIFIED),
           "VIEWABILITY_10_PERCENT_OR_MORE" => Ok(ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY10PERCENTORMORE),
           "VIEWABILITY_20_PERCENT_OR_MORE" => Ok(ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY20PERCENTORMORE),
           "VIEWABILITY_30_PERCENT_OR_MORE" => Ok(ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY30PERCENTORMORE),
           "VIEWABILITY_40_PERCENT_OR_MORE" => Ok(ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY40PERCENTORMORE),
           "VIEWABILITY_50_PERCENT_OR_MORE" => Ok(ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY50PERCENTORMORE),
           "VIEWABILITY_60_PERCENT_OR_MORE" => Ok(ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY60PERCENTORMORE),
           "VIEWABILITY_70_PERCENT_OR_MORE" => Ok(ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY70PERCENTORMORE),
           "VIEWABILITY_80_PERCENT_OR_MORE" => Ok(ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY80PERCENTORMORE),
           "VIEWABILITY_90_PERCENT_OR_MORE" => Ok(ViewabilityTargetingOptionDetailViewabilityEnum::VIEWABILITY90PERCENTORMORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ViewabilityTargetingOptionDetailViewabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdvertiserTargetingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Identifies the type of assigned targeting options to list. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION` * `TARGETING_TYPE_YOUTUBE_VIDEO` * `TARGETING_TYPE_YOUTUBE_CHANNEL`
pub enum AdvertiserTargetingTypeEnum {
    

    /// Default value when type is not specified or is unknown in this version.
    ///
    /// "TARGETING_TYPE_UNSPECIFIED"
    #[serde(rename="TARGETING_TYPE_UNSPECIFIED")]
    TARGETINGTYPEUNSPECIFIED,
    

    /// Target a channel (a custom group of related websites or apps).
    ///
    /// "TARGETING_TYPE_CHANNEL"
    #[serde(rename="TARGETING_TYPE_CHANNEL")]
    TARGETINGTYPECHANNEL,
    

    /// Target an app category (for example, education or puzzle games).
    ///
    /// "TARGETING_TYPE_APP_CATEGORY"
    #[serde(rename="TARGETING_TYPE_APP_CATEGORY")]
    TARGETINGTYPEAPPCATEGORY,
    

    /// Target a specific app (for example, Angry Birds).
    ///
    /// "TARGETING_TYPE_APP"
    #[serde(rename="TARGETING_TYPE_APP")]
    TARGETINGTYPEAPP,
    

    /// Target a specific url (for example, quora.com).
    ///
    /// "TARGETING_TYPE_URL"
    #[serde(rename="TARGETING_TYPE_URL")]
    TARGETINGTYPEURL,
    

    /// Target ads during a chosen time period on a specific day.
    ///
    /// "TARGETING_TYPE_DAY_AND_TIME"
    #[serde(rename="TARGETING_TYPE_DAY_AND_TIME")]
    TARGETINGTYPEDAYANDTIME,
    

    /// Target ads to a specific age range (for example, 18-24).
    ///
    /// "TARGETING_TYPE_AGE_RANGE"
    #[serde(rename="TARGETING_TYPE_AGE_RANGE")]
    TARGETINGTYPEAGERANGE,
    

    /// Target ads to the specified regions on a regional location list.
    ///
    /// "TARGETING_TYPE_REGIONAL_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    TARGETINGTYPEREGIONALLOCATIONLIST,
    

    /// Target ads to the specified points of interest on a proximity location list.
    ///
    /// "TARGETING_TYPE_PROXIMITY_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    TARGETINGTYPEPROXIMITYLOCATIONLIST,
    

    /// Target ads to a specific gender (for example, female or male).
    ///
    /// "TARGETING_TYPE_GENDER"
    #[serde(rename="TARGETING_TYPE_GENDER")]
    TARGETINGTYPEGENDER,
    

    /// Target a specific video player size for video ads.
    ///
    /// "TARGETING_TYPE_VIDEO_PLAYER_SIZE"
    #[serde(rename="TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    TARGETINGTYPEVIDEOPLAYERSIZE,
    

    /// Target user rewarded content for video ads.
    ///
    /// "TARGETING_TYPE_USER_REWARDED_CONTENT"
    #[serde(rename="TARGETING_TYPE_USER_REWARDED_CONTENT")]
    TARGETINGTYPEUSERREWARDEDCONTENT,
    

    /// Target ads to a specific parental status (for example, parent or not a parent).
    ///
    /// "TARGETING_TYPE_PARENTAL_STATUS"
    #[serde(rename="TARGETING_TYPE_PARENTAL_STATUS")]
    TARGETINGTYPEPARENTALSTATUS,
    

    /// Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll).
    ///
    /// "TARGETING_TYPE_CONTENT_INSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    TARGETINGTYPECONTENTINSTREAMPOSITION,
    

    /// Target ads in a specific content outstream position.
    ///
    /// "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    TARGETINGTYPECONTENTOUTSTREAMPOSITION,
    

    /// Target ads to a specific device type (for example, tablet or connected TV).
    ///
    /// "TARGETING_TYPE_DEVICE_TYPE"
    #[serde(rename="TARGETING_TYPE_DEVICE_TYPE")]
    TARGETINGTYPEDEVICETYPE,
    

    /// Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time.
    ///
    /// "TARGETING_TYPE_AUDIENCE_GROUP"
    #[serde(rename="TARGETING_TYPE_AUDIENCE_GROUP")]
    TARGETINGTYPEAUDIENCEGROUP,
    

    /// Target ads to specific web browsers (for example, Chrome).
    ///
    /// "TARGETING_TYPE_BROWSER"
    #[serde(rename="TARGETING_TYPE_BROWSER")]
    TARGETINGTYPEBROWSER,
    

    /// Target ads to a specific household income range (for example, top 10%).
    ///
    /// "TARGETING_TYPE_HOUSEHOLD_INCOME"
    #[serde(rename="TARGETING_TYPE_HOUSEHOLD_INCOME")]
    TARGETINGTYPEHOUSEHOLDINCOME,
    

    /// Target ads in a specific on screen position.
    ///
    /// "TARGETING_TYPE_ON_SCREEN_POSITION"
    #[serde(rename="TARGETING_TYPE_ON_SCREEN_POSITION")]
    TARGETINGTYPEONSCREENPOSITION,
    

    /// Filter web sites through third party verification (for example, IAS or DoubleVerify).
    ///
    /// "TARGETING_TYPE_THIRD_PARTY_VERIFIER"
    #[serde(rename="TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    TARGETINGTYPETHIRDPARTYVERIFIER,
    

    /// Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences).
    ///
    /// "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION,
    

    /// Filter website content by sensitive categories (for example, adult).
    ///
    /// "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    TARGETINGTYPESENSITIVECATEGORYEXCLUSION,
    

    /// Target ads to a specific environment (for example, web or app).
    ///
    /// "TARGETING_TYPE_ENVIRONMENT"
    #[serde(rename="TARGETING_TYPE_ENVIRONMENT")]
    TARGETINGTYPEENVIRONMENT,
    

    /// Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange).
    ///
    /// "TARGETING_TYPE_CARRIER_AND_ISP"
    #[serde(rename="TARGETING_TYPE_CARRIER_AND_ISP")]
    TARGETINGTYPECARRIERANDISP,
    

    /// Target ads to a specific operating system (for example, macOS).
    ///
    /// "TARGETING_TYPE_OPERATING_SYSTEM"
    #[serde(rename="TARGETING_TYPE_OPERATING_SYSTEM")]
    TARGETINGTYPEOPERATINGSYSTEM,
    

    /// Target ads to a specific device make or model (for example, Roku or Samsung).
    ///
    /// "TARGETING_TYPE_DEVICE_MAKE_MODEL"
    #[serde(rename="TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    TARGETINGTYPEDEVICEMAKEMODEL,
    

    /// Target ads to a specific keyword (for example, dog or retriever).
    ///
    /// "TARGETING_TYPE_KEYWORD"
    #[serde(rename="TARGETING_TYPE_KEYWORD")]
    TARGETINGTYPEKEYWORD,
    

    /// Target ads to a specific negative keyword list.
    ///
    /// "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST"
    #[serde(rename="TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    TARGETINGTYPENEGATIVEKEYWORDLIST,
    

    /// Target ads to a specific viewability (for example, 80% viewable).
    ///
    /// "TARGETING_TYPE_VIEWABILITY"
    #[serde(rename="TARGETING_TYPE_VIEWABILITY")]
    TARGETINGTYPEVIEWABILITY,
    

    /// Target ads to a specific content category (for example, arts & entertainment).
    ///
    /// "TARGETING_TYPE_CATEGORY"
    #[serde(rename="TARGETING_TYPE_CATEGORY")]
    TARGETINGTYPECATEGORY,
    

    /// Purchase impressions from specific deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE")]
    TARGETINGTYPEINVENTORYSOURCE,
    

    /// Target ads to a specific language (for example, English or Japanese).
    ///
    /// "TARGETING_TYPE_LANGUAGE"
    #[serde(rename="TARGETING_TYPE_LANGUAGE")]
    TARGETINGTYPELANGUAGE,
    

    /// Target ads to ads.txt authorized sellers. If no targeting option of this type is assigned, the resource uses the "Authorized Direct Sellers and Resellers" option by default.
    ///
    /// "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS"
    #[serde(rename="TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    TARGETINGTYPEAUTHORIZEDSELLERSTATUS,
    

    /// Target ads to a specific regional location (for example, a city or state).
    ///
    /// "TARGETING_TYPE_GEO_REGION"
    #[serde(rename="TARGETING_TYPE_GEO_REGION")]
    TARGETINGTYPEGEOREGION,
    

    /// Purchase impressions from a group of deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE_GROUP"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    TARGETINGTYPEINVENTORYSOURCEGROUP,
    

    /// Purchase impressions from specific exchanges.
    ///
    /// "TARGETING_TYPE_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_EXCHANGE")]
    TARGETINGTYPEEXCHANGE,
    

    /// Purchase impressions from specific sub-exchanges.
    ///
    /// "TARGETING_TYPE_SUB_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_SUB_EXCHANGE")]
    TARGETINGTYPESUBEXCHANGE,
    

    /// Target ads around a specific point of interest, such as a notable building, a street address, or latitude/longitude coordinates.
    ///
    /// "TARGETING_TYPE_POI"
    #[serde(rename="TARGETING_TYPE_POI")]
    TARGETINGTYPEPOI,
    

    /// Target ads around locations of a business chain within a specific geo region.
    ///
    /// "TARGETING_TYPE_BUSINESS_CHAIN"
    #[serde(rename="TARGETING_TYPE_BUSINESS_CHAIN")]
    TARGETINGTYPEBUSINESSCHAIN,
    

    /// Target ads to a specific video content duration.
    ///
    /// "TARGETING_TYPE_CONTENT_DURATION"
    #[serde(rename="TARGETING_TYPE_CONTENT_DURATION")]
    TARGETINGTYPECONTENTDURATION,
    

    /// Target ads to a specific video content stream type.
    ///
    /// "TARGETING_TYPE_CONTENT_STREAM_TYPE"
    #[serde(rename="TARGETING_TYPE_CONTENT_STREAM_TYPE")]
    TARGETINGTYPECONTENTSTREAMTYPE,
    

    /// Target ads to a specific native content position.
    ///
    /// "TARGETING_TYPE_NATIVE_CONTENT_POSITION"
    #[serde(rename="TARGETING_TYPE_NATIVE_CONTENT_POSITION")]
    TARGETINGTYPENATIVECONTENTPOSITION,
    

    /// Target ads in an Open Measurement enabled inventory.
    ///
    /// "TARGETING_TYPE_OMID"
    #[serde(rename="TARGETING_TYPE_OMID")]
    TARGETINGTYPEOMID,
    

    /// Target ads to a specific audio content type.
    ///
    /// "TARGETING_TYPE_AUDIO_CONTENT_TYPE"
    #[serde(rename="TARGETING_TYPE_AUDIO_CONTENT_TYPE")]
    TARGETINGTYPEAUDIOCONTENTTYPE,
    

    /// Target ads to a specific content genre.
    ///
    /// "TARGETING_TYPE_CONTENT_GENRE"
    #[serde(rename="TARGETING_TYPE_CONTENT_GENRE")]
    TARGETINGTYPECONTENTGENRE,
}

impl AsRef<str> for AdvertiserTargetingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED => "TARGETING_TYPE_UNSPECIFIED",
            AdvertiserTargetingTypeEnum::TARGETINGTYPECHANNEL => "TARGETING_TYPE_CHANNEL",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY => "TARGETING_TYPE_APP_CATEGORY",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEAPP => "TARGETING_TYPE_APP",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEURL => "TARGETING_TYPE_URL",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEDAYANDTIME => "TARGETING_TYPE_DAY_AND_TIME",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEAGERANGE => "TARGETING_TYPE_AGE_RANGE",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST => "TARGETING_TYPE_REGIONAL_LOCATION_LIST",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST => "TARGETING_TYPE_PROXIMITY_LOCATION_LIST",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEGENDER => "TARGETING_TYPE_GENDER",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE => "TARGETING_TYPE_VIDEO_PLAYER_SIZE",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT => "TARGETING_TYPE_USER_REWARDED_CONTENT",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS => "TARGETING_TYPE_PARENTAL_STATUS",
            AdvertiserTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION => "TARGETING_TYPE_CONTENT_INSTREAM_POSITION",
            AdvertiserTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION => "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEDEVICETYPE => "TARGETING_TYPE_DEVICE_TYPE",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP => "TARGETING_TYPE_AUDIENCE_GROUP",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEBROWSER => "TARGETING_TYPE_BROWSER",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME => "TARGETING_TYPE_HOUSEHOLD_INCOME",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION => "TARGETING_TYPE_ON_SCREEN_POSITION",
            AdvertiserTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER => "TARGETING_TYPE_THIRD_PARTY_VERIFIER",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION => "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION",
            AdvertiserTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION => "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEENVIRONMENT => "TARGETING_TYPE_ENVIRONMENT",
            AdvertiserTargetingTypeEnum::TARGETINGTYPECARRIERANDISP => "TARGETING_TYPE_CARRIER_AND_ISP",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM => "TARGETING_TYPE_OPERATING_SYSTEM",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL => "TARGETING_TYPE_DEVICE_MAKE_MODEL",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEKEYWORD => "TARGETING_TYPE_KEYWORD",
            AdvertiserTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST => "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEVIEWABILITY => "TARGETING_TYPE_VIEWABILITY",
            AdvertiserTargetingTypeEnum::TARGETINGTYPECATEGORY => "TARGETING_TYPE_CATEGORY",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE => "TARGETING_TYPE_INVENTORY_SOURCE",
            AdvertiserTargetingTypeEnum::TARGETINGTYPELANGUAGE => "TARGETING_TYPE_LANGUAGE",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS => "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEGEOREGION => "TARGETING_TYPE_GEO_REGION",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP => "TARGETING_TYPE_INVENTORY_SOURCE_GROUP",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEEXCHANGE => "TARGETING_TYPE_EXCHANGE",
            AdvertiserTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE => "TARGETING_TYPE_SUB_EXCHANGE",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEPOI => "TARGETING_TYPE_POI",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN => "TARGETING_TYPE_BUSINESS_CHAIN",
            AdvertiserTargetingTypeEnum::TARGETINGTYPECONTENTDURATION => "TARGETING_TYPE_CONTENT_DURATION",
            AdvertiserTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE => "TARGETING_TYPE_CONTENT_STREAM_TYPE",
            AdvertiserTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION => "TARGETING_TYPE_NATIVE_CONTENT_POSITION",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEOMID => "TARGETING_TYPE_OMID",
            AdvertiserTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE => "TARGETING_TYPE_AUDIO_CONTENT_TYPE",
            AdvertiserTargetingTypeEnum::TARGETINGTYPECONTENTGENRE => "TARGETING_TYPE_CONTENT_GENRE",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserTargetingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_TYPE_UNSPECIFIED" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED),
           "TARGETING_TYPE_CHANNEL" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPECHANNEL),
           "TARGETING_TYPE_APP_CATEGORY" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY),
           "TARGETING_TYPE_APP" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEAPP),
           "TARGETING_TYPE_URL" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEURL),
           "TARGETING_TYPE_DAY_AND_TIME" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEDAYANDTIME),
           "TARGETING_TYPE_AGE_RANGE" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEAGERANGE),
           "TARGETING_TYPE_REGIONAL_LOCATION_LIST" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST),
           "TARGETING_TYPE_PROXIMITY_LOCATION_LIST" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST),
           "TARGETING_TYPE_GENDER" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEGENDER),
           "TARGETING_TYPE_VIDEO_PLAYER_SIZE" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE),
           "TARGETING_TYPE_USER_REWARDED_CONTENT" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT),
           "TARGETING_TYPE_PARENTAL_STATUS" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS),
           "TARGETING_TYPE_CONTENT_INSTREAM_POSITION" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION),
           "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION),
           "TARGETING_TYPE_DEVICE_TYPE" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEDEVICETYPE),
           "TARGETING_TYPE_AUDIENCE_GROUP" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP),
           "TARGETING_TYPE_BROWSER" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEBROWSER),
           "TARGETING_TYPE_HOUSEHOLD_INCOME" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME),
           "TARGETING_TYPE_ON_SCREEN_POSITION" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION),
           "TARGETING_TYPE_THIRD_PARTY_VERIFIER" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER),
           "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION),
           "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION),
           "TARGETING_TYPE_ENVIRONMENT" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEENVIRONMENT),
           "TARGETING_TYPE_CARRIER_AND_ISP" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPECARRIERANDISP),
           "TARGETING_TYPE_OPERATING_SYSTEM" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM),
           "TARGETING_TYPE_DEVICE_MAKE_MODEL" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL),
           "TARGETING_TYPE_KEYWORD" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEKEYWORD),
           "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST),
           "TARGETING_TYPE_VIEWABILITY" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEVIEWABILITY),
           "TARGETING_TYPE_CATEGORY" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPECATEGORY),
           "TARGETING_TYPE_INVENTORY_SOURCE" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE),
           "TARGETING_TYPE_LANGUAGE" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPELANGUAGE),
           "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS),
           "TARGETING_TYPE_GEO_REGION" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEGEOREGION),
           "TARGETING_TYPE_INVENTORY_SOURCE_GROUP" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP),
           "TARGETING_TYPE_EXCHANGE" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEEXCHANGE),
           "TARGETING_TYPE_SUB_EXCHANGE" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE),
           "TARGETING_TYPE_POI" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEPOI),
           "TARGETING_TYPE_BUSINESS_CHAIN" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN),
           "TARGETING_TYPE_CONTENT_DURATION" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPECONTENTDURATION),
           "TARGETING_TYPE_CONTENT_STREAM_TYPE" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE),
           "TARGETING_TYPE_NATIVE_CONTENT_POSITION" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION),
           "TARGETING_TYPE_OMID" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEOMID),
           "TARGETING_TYPE_AUDIO_CONTENT_TYPE" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE),
           "TARGETING_TYPE_CONTENT_GENRE" => Ok(AdvertiserTargetingTypeEnum::TARGETINGTYPECONTENTGENRE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserTargetingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdvertiserLoiSapinInvoiceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select type of invoice to retrieve for Loi Sapin advertisers. Only applicable to Loi Sapin advertisers. Will be ignored otherwise.
pub enum AdvertiserLoiSapinInvoiceTypeEnum {
    

    /// Value is not specified.
    ///
    /// "LOI_SAPIN_INVOICE_TYPE_UNSPECIFIED"
    #[serde(rename="LOI_SAPIN_INVOICE_TYPE_UNSPECIFIED")]
    LOISAPININVOICETYPEUNSPECIFIED,
    

    /// Invoices with Media cost.
    ///
    /// "LOI_SAPIN_INVOICE_TYPE_MEDIA"
    #[serde(rename="LOI_SAPIN_INVOICE_TYPE_MEDIA")]
    LOISAPININVOICETYPEMEDIA,
    

    /// Invoices with Platform fee.
    ///
    /// "LOI_SAPIN_INVOICE_TYPE_PLATFORM"
    #[serde(rename="LOI_SAPIN_INVOICE_TYPE_PLATFORM")]
    LOISAPININVOICETYPEPLATFORM,
}

impl AsRef<str> for AdvertiserLoiSapinInvoiceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserLoiSapinInvoiceTypeEnum::LOISAPININVOICETYPEUNSPECIFIED => "LOI_SAPIN_INVOICE_TYPE_UNSPECIFIED",
            AdvertiserLoiSapinInvoiceTypeEnum::LOISAPININVOICETYPEMEDIA => "LOI_SAPIN_INVOICE_TYPE_MEDIA",
            AdvertiserLoiSapinInvoiceTypeEnum::LOISAPININVOICETYPEPLATFORM => "LOI_SAPIN_INVOICE_TYPE_PLATFORM",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserLoiSapinInvoiceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOI_SAPIN_INVOICE_TYPE_UNSPECIFIED" => Ok(AdvertiserLoiSapinInvoiceTypeEnum::LOISAPININVOICETYPEUNSPECIFIED),
           "LOI_SAPIN_INVOICE_TYPE_MEDIA" => Ok(AdvertiserLoiSapinInvoiceTypeEnum::LOISAPININVOICETYPEMEDIA),
           "LOI_SAPIN_INVOICE_TYPE_PLATFORM" => Ok(AdvertiserLoiSapinInvoiceTypeEnum::LOISAPININVOICETYPEPLATFORM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserLoiSapinInvoiceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartnerTargetingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Identifies the type of assigned targeting options to list. Supported targeting types: * `TARGETING_TYPE_CHANNEL`
pub enum PartnerTargetingTypeEnum {
    

    /// Default value when type is not specified or is unknown in this version.
    ///
    /// "TARGETING_TYPE_UNSPECIFIED"
    #[serde(rename="TARGETING_TYPE_UNSPECIFIED")]
    TARGETINGTYPEUNSPECIFIED,
    

    /// Target a channel (a custom group of related websites or apps).
    ///
    /// "TARGETING_TYPE_CHANNEL"
    #[serde(rename="TARGETING_TYPE_CHANNEL")]
    TARGETINGTYPECHANNEL,
    

    /// Target an app category (for example, education or puzzle games).
    ///
    /// "TARGETING_TYPE_APP_CATEGORY"
    #[serde(rename="TARGETING_TYPE_APP_CATEGORY")]
    TARGETINGTYPEAPPCATEGORY,
    

    /// Target a specific app (for example, Angry Birds).
    ///
    /// "TARGETING_TYPE_APP"
    #[serde(rename="TARGETING_TYPE_APP")]
    TARGETINGTYPEAPP,
    

    /// Target a specific url (for example, quora.com).
    ///
    /// "TARGETING_TYPE_URL"
    #[serde(rename="TARGETING_TYPE_URL")]
    TARGETINGTYPEURL,
    

    /// Target ads during a chosen time period on a specific day.
    ///
    /// "TARGETING_TYPE_DAY_AND_TIME"
    #[serde(rename="TARGETING_TYPE_DAY_AND_TIME")]
    TARGETINGTYPEDAYANDTIME,
    

    /// Target ads to a specific age range (for example, 18-24).
    ///
    /// "TARGETING_TYPE_AGE_RANGE"
    #[serde(rename="TARGETING_TYPE_AGE_RANGE")]
    TARGETINGTYPEAGERANGE,
    

    /// Target ads to the specified regions on a regional location list.
    ///
    /// "TARGETING_TYPE_REGIONAL_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    TARGETINGTYPEREGIONALLOCATIONLIST,
    

    /// Target ads to the specified points of interest on a proximity location list.
    ///
    /// "TARGETING_TYPE_PROXIMITY_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    TARGETINGTYPEPROXIMITYLOCATIONLIST,
    

    /// Target ads to a specific gender (for example, female or male).
    ///
    /// "TARGETING_TYPE_GENDER"
    #[serde(rename="TARGETING_TYPE_GENDER")]
    TARGETINGTYPEGENDER,
    

    /// Target a specific video player size for video ads.
    ///
    /// "TARGETING_TYPE_VIDEO_PLAYER_SIZE"
    #[serde(rename="TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    TARGETINGTYPEVIDEOPLAYERSIZE,
    

    /// Target user rewarded content for video ads.
    ///
    /// "TARGETING_TYPE_USER_REWARDED_CONTENT"
    #[serde(rename="TARGETING_TYPE_USER_REWARDED_CONTENT")]
    TARGETINGTYPEUSERREWARDEDCONTENT,
    

    /// Target ads to a specific parental status (for example, parent or not a parent).
    ///
    /// "TARGETING_TYPE_PARENTAL_STATUS"
    #[serde(rename="TARGETING_TYPE_PARENTAL_STATUS")]
    TARGETINGTYPEPARENTALSTATUS,
    

    /// Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll).
    ///
    /// "TARGETING_TYPE_CONTENT_INSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    TARGETINGTYPECONTENTINSTREAMPOSITION,
    

    /// Target ads in a specific content outstream position.
    ///
    /// "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    TARGETINGTYPECONTENTOUTSTREAMPOSITION,
    

    /// Target ads to a specific device type (for example, tablet or connected TV).
    ///
    /// "TARGETING_TYPE_DEVICE_TYPE"
    #[serde(rename="TARGETING_TYPE_DEVICE_TYPE")]
    TARGETINGTYPEDEVICETYPE,
    

    /// Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time.
    ///
    /// "TARGETING_TYPE_AUDIENCE_GROUP"
    #[serde(rename="TARGETING_TYPE_AUDIENCE_GROUP")]
    TARGETINGTYPEAUDIENCEGROUP,
    

    /// Target ads to specific web browsers (for example, Chrome).
    ///
    /// "TARGETING_TYPE_BROWSER"
    #[serde(rename="TARGETING_TYPE_BROWSER")]
    TARGETINGTYPEBROWSER,
    

    /// Target ads to a specific household income range (for example, top 10%).
    ///
    /// "TARGETING_TYPE_HOUSEHOLD_INCOME"
    #[serde(rename="TARGETING_TYPE_HOUSEHOLD_INCOME")]
    TARGETINGTYPEHOUSEHOLDINCOME,
    

    /// Target ads in a specific on screen position.
    ///
    /// "TARGETING_TYPE_ON_SCREEN_POSITION"
    #[serde(rename="TARGETING_TYPE_ON_SCREEN_POSITION")]
    TARGETINGTYPEONSCREENPOSITION,
    

    /// Filter web sites through third party verification (for example, IAS or DoubleVerify).
    ///
    /// "TARGETING_TYPE_THIRD_PARTY_VERIFIER"
    #[serde(rename="TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    TARGETINGTYPETHIRDPARTYVERIFIER,
    

    /// Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences).
    ///
    /// "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION,
    

    /// Filter website content by sensitive categories (for example, adult).
    ///
    /// "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    TARGETINGTYPESENSITIVECATEGORYEXCLUSION,
    

    /// Target ads to a specific environment (for example, web or app).
    ///
    /// "TARGETING_TYPE_ENVIRONMENT"
    #[serde(rename="TARGETING_TYPE_ENVIRONMENT")]
    TARGETINGTYPEENVIRONMENT,
    

    /// Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange).
    ///
    /// "TARGETING_TYPE_CARRIER_AND_ISP"
    #[serde(rename="TARGETING_TYPE_CARRIER_AND_ISP")]
    TARGETINGTYPECARRIERANDISP,
    

    /// Target ads to a specific operating system (for example, macOS).
    ///
    /// "TARGETING_TYPE_OPERATING_SYSTEM"
    #[serde(rename="TARGETING_TYPE_OPERATING_SYSTEM")]
    TARGETINGTYPEOPERATINGSYSTEM,
    

    /// Target ads to a specific device make or model (for example, Roku or Samsung).
    ///
    /// "TARGETING_TYPE_DEVICE_MAKE_MODEL"
    #[serde(rename="TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    TARGETINGTYPEDEVICEMAKEMODEL,
    

    /// Target ads to a specific keyword (for example, dog or retriever).
    ///
    /// "TARGETING_TYPE_KEYWORD"
    #[serde(rename="TARGETING_TYPE_KEYWORD")]
    TARGETINGTYPEKEYWORD,
    

    /// Target ads to a specific negative keyword list.
    ///
    /// "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST"
    #[serde(rename="TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    TARGETINGTYPENEGATIVEKEYWORDLIST,
    

    /// Target ads to a specific viewability (for example, 80% viewable).
    ///
    /// "TARGETING_TYPE_VIEWABILITY"
    #[serde(rename="TARGETING_TYPE_VIEWABILITY")]
    TARGETINGTYPEVIEWABILITY,
    

    /// Target ads to a specific content category (for example, arts & entertainment).
    ///
    /// "TARGETING_TYPE_CATEGORY"
    #[serde(rename="TARGETING_TYPE_CATEGORY")]
    TARGETINGTYPECATEGORY,
    

    /// Purchase impressions from specific deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE")]
    TARGETINGTYPEINVENTORYSOURCE,
    

    /// Target ads to a specific language (for example, English or Japanese).
    ///
    /// "TARGETING_TYPE_LANGUAGE"
    #[serde(rename="TARGETING_TYPE_LANGUAGE")]
    TARGETINGTYPELANGUAGE,
    

    /// Target ads to ads.txt authorized sellers. If no targeting option of this type is assigned, the resource uses the "Authorized Direct Sellers and Resellers" option by default.
    ///
    /// "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS"
    #[serde(rename="TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    TARGETINGTYPEAUTHORIZEDSELLERSTATUS,
    

    /// Target ads to a specific regional location (for example, a city or state).
    ///
    /// "TARGETING_TYPE_GEO_REGION"
    #[serde(rename="TARGETING_TYPE_GEO_REGION")]
    TARGETINGTYPEGEOREGION,
    

    /// Purchase impressions from a group of deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE_GROUP"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    TARGETINGTYPEINVENTORYSOURCEGROUP,
    

    /// Purchase impressions from specific exchanges.
    ///
    /// "TARGETING_TYPE_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_EXCHANGE")]
    TARGETINGTYPEEXCHANGE,
    

    /// Purchase impressions from specific sub-exchanges.
    ///
    /// "TARGETING_TYPE_SUB_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_SUB_EXCHANGE")]
    TARGETINGTYPESUBEXCHANGE,
    

    /// Target ads around a specific point of interest, such as a notable building, a street address, or latitude/longitude coordinates.
    ///
    /// "TARGETING_TYPE_POI"
    #[serde(rename="TARGETING_TYPE_POI")]
    TARGETINGTYPEPOI,
    

    /// Target ads around locations of a business chain within a specific geo region.
    ///
    /// "TARGETING_TYPE_BUSINESS_CHAIN"
    #[serde(rename="TARGETING_TYPE_BUSINESS_CHAIN")]
    TARGETINGTYPEBUSINESSCHAIN,
    

    /// Target ads to a specific video content duration.
    ///
    /// "TARGETING_TYPE_CONTENT_DURATION"
    #[serde(rename="TARGETING_TYPE_CONTENT_DURATION")]
    TARGETINGTYPECONTENTDURATION,
    

    /// Target ads to a specific video content stream type.
    ///
    /// "TARGETING_TYPE_CONTENT_STREAM_TYPE"
    #[serde(rename="TARGETING_TYPE_CONTENT_STREAM_TYPE")]
    TARGETINGTYPECONTENTSTREAMTYPE,
    

    /// Target ads to a specific native content position.
    ///
    /// "TARGETING_TYPE_NATIVE_CONTENT_POSITION"
    #[serde(rename="TARGETING_TYPE_NATIVE_CONTENT_POSITION")]
    TARGETINGTYPENATIVECONTENTPOSITION,
    

    /// Target ads in an Open Measurement enabled inventory.
    ///
    /// "TARGETING_TYPE_OMID"
    #[serde(rename="TARGETING_TYPE_OMID")]
    TARGETINGTYPEOMID,
    

    /// Target ads to a specific audio content type.
    ///
    /// "TARGETING_TYPE_AUDIO_CONTENT_TYPE"
    #[serde(rename="TARGETING_TYPE_AUDIO_CONTENT_TYPE")]
    TARGETINGTYPEAUDIOCONTENTTYPE,
    

    /// Target ads to a specific content genre.
    ///
    /// "TARGETING_TYPE_CONTENT_GENRE"
    #[serde(rename="TARGETING_TYPE_CONTENT_GENRE")]
    TARGETINGTYPECONTENTGENRE,
}

impl AsRef<str> for PartnerTargetingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartnerTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED => "TARGETING_TYPE_UNSPECIFIED",
            PartnerTargetingTypeEnum::TARGETINGTYPECHANNEL => "TARGETING_TYPE_CHANNEL",
            PartnerTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY => "TARGETING_TYPE_APP_CATEGORY",
            PartnerTargetingTypeEnum::TARGETINGTYPEAPP => "TARGETING_TYPE_APP",
            PartnerTargetingTypeEnum::TARGETINGTYPEURL => "TARGETING_TYPE_URL",
            PartnerTargetingTypeEnum::TARGETINGTYPEDAYANDTIME => "TARGETING_TYPE_DAY_AND_TIME",
            PartnerTargetingTypeEnum::TARGETINGTYPEAGERANGE => "TARGETING_TYPE_AGE_RANGE",
            PartnerTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST => "TARGETING_TYPE_REGIONAL_LOCATION_LIST",
            PartnerTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST => "TARGETING_TYPE_PROXIMITY_LOCATION_LIST",
            PartnerTargetingTypeEnum::TARGETINGTYPEGENDER => "TARGETING_TYPE_GENDER",
            PartnerTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE => "TARGETING_TYPE_VIDEO_PLAYER_SIZE",
            PartnerTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT => "TARGETING_TYPE_USER_REWARDED_CONTENT",
            PartnerTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS => "TARGETING_TYPE_PARENTAL_STATUS",
            PartnerTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION => "TARGETING_TYPE_CONTENT_INSTREAM_POSITION",
            PartnerTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION => "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION",
            PartnerTargetingTypeEnum::TARGETINGTYPEDEVICETYPE => "TARGETING_TYPE_DEVICE_TYPE",
            PartnerTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP => "TARGETING_TYPE_AUDIENCE_GROUP",
            PartnerTargetingTypeEnum::TARGETINGTYPEBROWSER => "TARGETING_TYPE_BROWSER",
            PartnerTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME => "TARGETING_TYPE_HOUSEHOLD_INCOME",
            PartnerTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION => "TARGETING_TYPE_ON_SCREEN_POSITION",
            PartnerTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER => "TARGETING_TYPE_THIRD_PARTY_VERIFIER",
            PartnerTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION => "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION",
            PartnerTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION => "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION",
            PartnerTargetingTypeEnum::TARGETINGTYPEENVIRONMENT => "TARGETING_TYPE_ENVIRONMENT",
            PartnerTargetingTypeEnum::TARGETINGTYPECARRIERANDISP => "TARGETING_TYPE_CARRIER_AND_ISP",
            PartnerTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM => "TARGETING_TYPE_OPERATING_SYSTEM",
            PartnerTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL => "TARGETING_TYPE_DEVICE_MAKE_MODEL",
            PartnerTargetingTypeEnum::TARGETINGTYPEKEYWORD => "TARGETING_TYPE_KEYWORD",
            PartnerTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST => "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST",
            PartnerTargetingTypeEnum::TARGETINGTYPEVIEWABILITY => "TARGETING_TYPE_VIEWABILITY",
            PartnerTargetingTypeEnum::TARGETINGTYPECATEGORY => "TARGETING_TYPE_CATEGORY",
            PartnerTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE => "TARGETING_TYPE_INVENTORY_SOURCE",
            PartnerTargetingTypeEnum::TARGETINGTYPELANGUAGE => "TARGETING_TYPE_LANGUAGE",
            PartnerTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS => "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS",
            PartnerTargetingTypeEnum::TARGETINGTYPEGEOREGION => "TARGETING_TYPE_GEO_REGION",
            PartnerTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP => "TARGETING_TYPE_INVENTORY_SOURCE_GROUP",
            PartnerTargetingTypeEnum::TARGETINGTYPEEXCHANGE => "TARGETING_TYPE_EXCHANGE",
            PartnerTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE => "TARGETING_TYPE_SUB_EXCHANGE",
            PartnerTargetingTypeEnum::TARGETINGTYPEPOI => "TARGETING_TYPE_POI",
            PartnerTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN => "TARGETING_TYPE_BUSINESS_CHAIN",
            PartnerTargetingTypeEnum::TARGETINGTYPECONTENTDURATION => "TARGETING_TYPE_CONTENT_DURATION",
            PartnerTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE => "TARGETING_TYPE_CONTENT_STREAM_TYPE",
            PartnerTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION => "TARGETING_TYPE_NATIVE_CONTENT_POSITION",
            PartnerTargetingTypeEnum::TARGETINGTYPEOMID => "TARGETING_TYPE_OMID",
            PartnerTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE => "TARGETING_TYPE_AUDIO_CONTENT_TYPE",
            PartnerTargetingTypeEnum::TARGETINGTYPECONTENTGENRE => "TARGETING_TYPE_CONTENT_GENRE",
        }
    }
}

impl std::convert::TryFrom< &str> for PartnerTargetingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_TYPE_UNSPECIFIED" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED),
           "TARGETING_TYPE_CHANNEL" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPECHANNEL),
           "TARGETING_TYPE_APP_CATEGORY" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY),
           "TARGETING_TYPE_APP" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEAPP),
           "TARGETING_TYPE_URL" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEURL),
           "TARGETING_TYPE_DAY_AND_TIME" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEDAYANDTIME),
           "TARGETING_TYPE_AGE_RANGE" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEAGERANGE),
           "TARGETING_TYPE_REGIONAL_LOCATION_LIST" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST),
           "TARGETING_TYPE_PROXIMITY_LOCATION_LIST" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST),
           "TARGETING_TYPE_GENDER" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEGENDER),
           "TARGETING_TYPE_VIDEO_PLAYER_SIZE" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE),
           "TARGETING_TYPE_USER_REWARDED_CONTENT" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT),
           "TARGETING_TYPE_PARENTAL_STATUS" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS),
           "TARGETING_TYPE_CONTENT_INSTREAM_POSITION" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION),
           "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION),
           "TARGETING_TYPE_DEVICE_TYPE" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEDEVICETYPE),
           "TARGETING_TYPE_AUDIENCE_GROUP" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP),
           "TARGETING_TYPE_BROWSER" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEBROWSER),
           "TARGETING_TYPE_HOUSEHOLD_INCOME" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME),
           "TARGETING_TYPE_ON_SCREEN_POSITION" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION),
           "TARGETING_TYPE_THIRD_PARTY_VERIFIER" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER),
           "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION),
           "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION),
           "TARGETING_TYPE_ENVIRONMENT" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEENVIRONMENT),
           "TARGETING_TYPE_CARRIER_AND_ISP" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPECARRIERANDISP),
           "TARGETING_TYPE_OPERATING_SYSTEM" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM),
           "TARGETING_TYPE_DEVICE_MAKE_MODEL" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL),
           "TARGETING_TYPE_KEYWORD" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEKEYWORD),
           "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST),
           "TARGETING_TYPE_VIEWABILITY" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEVIEWABILITY),
           "TARGETING_TYPE_CATEGORY" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPECATEGORY),
           "TARGETING_TYPE_INVENTORY_SOURCE" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE),
           "TARGETING_TYPE_LANGUAGE" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPELANGUAGE),
           "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS),
           "TARGETING_TYPE_GEO_REGION" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEGEOREGION),
           "TARGETING_TYPE_INVENTORY_SOURCE_GROUP" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP),
           "TARGETING_TYPE_EXCHANGE" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEEXCHANGE),
           "TARGETING_TYPE_SUB_EXCHANGE" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE),
           "TARGETING_TYPE_POI" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEPOI),
           "TARGETING_TYPE_BUSINESS_CHAIN" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN),
           "TARGETING_TYPE_CONTENT_DURATION" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPECONTENTDURATION),
           "TARGETING_TYPE_CONTENT_STREAM_TYPE" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE),
           "TARGETING_TYPE_NATIVE_CONTENT_POSITION" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION),
           "TARGETING_TYPE_OMID" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEOMID),
           "TARGETING_TYPE_AUDIO_CONTENT_TYPE" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE),
           "TARGETING_TYPE_CONTENT_GENRE" => Ok(PartnerTargetingTypeEnum::TARGETINGTYPECONTENTGENRE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartnerTargetingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TargetingTypeTargetingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of targeting options to retrieve. Accepted values are: * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_POI` * `TARGETING_TYPE_BUSINESS_CHAIN`
pub enum TargetingTypeTargetingTypeEnum {
    

    /// Default value when type is not specified or is unknown in this version.
    ///
    /// "TARGETING_TYPE_UNSPECIFIED"
    #[serde(rename="TARGETING_TYPE_UNSPECIFIED")]
    TARGETINGTYPEUNSPECIFIED,
    

    /// Target a channel (a custom group of related websites or apps).
    ///
    /// "TARGETING_TYPE_CHANNEL"
    #[serde(rename="TARGETING_TYPE_CHANNEL")]
    TARGETINGTYPECHANNEL,
    

    /// Target an app category (for example, education or puzzle games).
    ///
    /// "TARGETING_TYPE_APP_CATEGORY"
    #[serde(rename="TARGETING_TYPE_APP_CATEGORY")]
    TARGETINGTYPEAPPCATEGORY,
    

    /// Target a specific app (for example, Angry Birds).
    ///
    /// "TARGETING_TYPE_APP"
    #[serde(rename="TARGETING_TYPE_APP")]
    TARGETINGTYPEAPP,
    

    /// Target a specific url (for example, quora.com).
    ///
    /// "TARGETING_TYPE_URL"
    #[serde(rename="TARGETING_TYPE_URL")]
    TARGETINGTYPEURL,
    

    /// Target ads during a chosen time period on a specific day.
    ///
    /// "TARGETING_TYPE_DAY_AND_TIME"
    #[serde(rename="TARGETING_TYPE_DAY_AND_TIME")]
    TARGETINGTYPEDAYANDTIME,
    

    /// Target ads to a specific age range (for example, 18-24).
    ///
    /// "TARGETING_TYPE_AGE_RANGE"
    #[serde(rename="TARGETING_TYPE_AGE_RANGE")]
    TARGETINGTYPEAGERANGE,
    

    /// Target ads to the specified regions on a regional location list.
    ///
    /// "TARGETING_TYPE_REGIONAL_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    TARGETINGTYPEREGIONALLOCATIONLIST,
    

    /// Target ads to the specified points of interest on a proximity location list.
    ///
    /// "TARGETING_TYPE_PROXIMITY_LOCATION_LIST"
    #[serde(rename="TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    TARGETINGTYPEPROXIMITYLOCATIONLIST,
    

    /// Target ads to a specific gender (for example, female or male).
    ///
    /// "TARGETING_TYPE_GENDER"
    #[serde(rename="TARGETING_TYPE_GENDER")]
    TARGETINGTYPEGENDER,
    

    /// Target a specific video player size for video ads.
    ///
    /// "TARGETING_TYPE_VIDEO_PLAYER_SIZE"
    #[serde(rename="TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    TARGETINGTYPEVIDEOPLAYERSIZE,
    

    /// Target user rewarded content for video ads.
    ///
    /// "TARGETING_TYPE_USER_REWARDED_CONTENT"
    #[serde(rename="TARGETING_TYPE_USER_REWARDED_CONTENT")]
    TARGETINGTYPEUSERREWARDEDCONTENT,
    

    /// Target ads to a specific parental status (for example, parent or not a parent).
    ///
    /// "TARGETING_TYPE_PARENTAL_STATUS"
    #[serde(rename="TARGETING_TYPE_PARENTAL_STATUS")]
    TARGETINGTYPEPARENTALSTATUS,
    

    /// Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll).
    ///
    /// "TARGETING_TYPE_CONTENT_INSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    TARGETINGTYPECONTENTINSTREAMPOSITION,
    

    /// Target ads in a specific content outstream position.
    ///
    /// "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION"
    #[serde(rename="TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    TARGETINGTYPECONTENTOUTSTREAMPOSITION,
    

    /// Target ads to a specific device type (for example, tablet or connected TV).
    ///
    /// "TARGETING_TYPE_DEVICE_TYPE"
    #[serde(rename="TARGETING_TYPE_DEVICE_TYPE")]
    TARGETINGTYPEDEVICETYPE,
    

    /// Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time.
    ///
    /// "TARGETING_TYPE_AUDIENCE_GROUP"
    #[serde(rename="TARGETING_TYPE_AUDIENCE_GROUP")]
    TARGETINGTYPEAUDIENCEGROUP,
    

    /// Target ads to specific web browsers (for example, Chrome).
    ///
    /// "TARGETING_TYPE_BROWSER"
    #[serde(rename="TARGETING_TYPE_BROWSER")]
    TARGETINGTYPEBROWSER,
    

    /// Target ads to a specific household income range (for example, top 10%).
    ///
    /// "TARGETING_TYPE_HOUSEHOLD_INCOME"
    #[serde(rename="TARGETING_TYPE_HOUSEHOLD_INCOME")]
    TARGETINGTYPEHOUSEHOLDINCOME,
    

    /// Target ads in a specific on screen position.
    ///
    /// "TARGETING_TYPE_ON_SCREEN_POSITION"
    #[serde(rename="TARGETING_TYPE_ON_SCREEN_POSITION")]
    TARGETINGTYPEONSCREENPOSITION,
    

    /// Filter web sites through third party verification (for example, IAS or DoubleVerify).
    ///
    /// "TARGETING_TYPE_THIRD_PARTY_VERIFIER"
    #[serde(rename="TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    TARGETINGTYPETHIRDPARTYVERIFIER,
    

    /// Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences).
    ///
    /// "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION,
    

    /// Filter website content by sensitive categories (for example, adult).
    ///
    /// "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION"
    #[serde(rename="TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    TARGETINGTYPESENSITIVECATEGORYEXCLUSION,
    

    /// Target ads to a specific environment (for example, web or app).
    ///
    /// "TARGETING_TYPE_ENVIRONMENT"
    #[serde(rename="TARGETING_TYPE_ENVIRONMENT")]
    TARGETINGTYPEENVIRONMENT,
    

    /// Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange).
    ///
    /// "TARGETING_TYPE_CARRIER_AND_ISP"
    #[serde(rename="TARGETING_TYPE_CARRIER_AND_ISP")]
    TARGETINGTYPECARRIERANDISP,
    

    /// Target ads to a specific operating system (for example, macOS).
    ///
    /// "TARGETING_TYPE_OPERATING_SYSTEM"
    #[serde(rename="TARGETING_TYPE_OPERATING_SYSTEM")]
    TARGETINGTYPEOPERATINGSYSTEM,
    

    /// Target ads to a specific device make or model (for example, Roku or Samsung).
    ///
    /// "TARGETING_TYPE_DEVICE_MAKE_MODEL"
    #[serde(rename="TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    TARGETINGTYPEDEVICEMAKEMODEL,
    

    /// Target ads to a specific keyword (for example, dog or retriever).
    ///
    /// "TARGETING_TYPE_KEYWORD"
    #[serde(rename="TARGETING_TYPE_KEYWORD")]
    TARGETINGTYPEKEYWORD,
    

    /// Target ads to a specific negative keyword list.
    ///
    /// "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST"
    #[serde(rename="TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    TARGETINGTYPENEGATIVEKEYWORDLIST,
    

    /// Target ads to a specific viewability (for example, 80% viewable).
    ///
    /// "TARGETING_TYPE_VIEWABILITY"
    #[serde(rename="TARGETING_TYPE_VIEWABILITY")]
    TARGETINGTYPEVIEWABILITY,
    

    /// Target ads to a specific content category (for example, arts & entertainment).
    ///
    /// "TARGETING_TYPE_CATEGORY"
    #[serde(rename="TARGETING_TYPE_CATEGORY")]
    TARGETINGTYPECATEGORY,
    

    /// Purchase impressions from specific deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE")]
    TARGETINGTYPEINVENTORYSOURCE,
    

    /// Target ads to a specific language (for example, English or Japanese).
    ///
    /// "TARGETING_TYPE_LANGUAGE"
    #[serde(rename="TARGETING_TYPE_LANGUAGE")]
    TARGETINGTYPELANGUAGE,
    

    /// Target ads to ads.txt authorized sellers. If no targeting option of this type is assigned, the resource uses the "Authorized Direct Sellers and Resellers" option by default.
    ///
    /// "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS"
    #[serde(rename="TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    TARGETINGTYPEAUTHORIZEDSELLERSTATUS,
    

    /// Target ads to a specific regional location (for example, a city or state).
    ///
    /// "TARGETING_TYPE_GEO_REGION"
    #[serde(rename="TARGETING_TYPE_GEO_REGION")]
    TARGETINGTYPEGEOREGION,
    

    /// Purchase impressions from a group of deals and auction packages.
    ///
    /// "TARGETING_TYPE_INVENTORY_SOURCE_GROUP"
    #[serde(rename="TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    TARGETINGTYPEINVENTORYSOURCEGROUP,
    

    /// Purchase impressions from specific exchanges.
    ///
    /// "TARGETING_TYPE_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_EXCHANGE")]
    TARGETINGTYPEEXCHANGE,
    

    /// Purchase impressions from specific sub-exchanges.
    ///
    /// "TARGETING_TYPE_SUB_EXCHANGE"
    #[serde(rename="TARGETING_TYPE_SUB_EXCHANGE")]
    TARGETINGTYPESUBEXCHANGE,
    

    /// Target ads around a specific point of interest, such as a notable building, a street address, or latitude/longitude coordinates.
    ///
    /// "TARGETING_TYPE_POI"
    #[serde(rename="TARGETING_TYPE_POI")]
    TARGETINGTYPEPOI,
    

    /// Target ads around locations of a business chain within a specific geo region.
    ///
    /// "TARGETING_TYPE_BUSINESS_CHAIN"
    #[serde(rename="TARGETING_TYPE_BUSINESS_CHAIN")]
    TARGETINGTYPEBUSINESSCHAIN,
    

    /// Target ads to a specific video content duration.
    ///
    /// "TARGETING_TYPE_CONTENT_DURATION"
    #[serde(rename="TARGETING_TYPE_CONTENT_DURATION")]
    TARGETINGTYPECONTENTDURATION,
    

    /// Target ads to a specific video content stream type.
    ///
    /// "TARGETING_TYPE_CONTENT_STREAM_TYPE"
    #[serde(rename="TARGETING_TYPE_CONTENT_STREAM_TYPE")]
    TARGETINGTYPECONTENTSTREAMTYPE,
    

    /// Target ads to a specific native content position.
    ///
    /// "TARGETING_TYPE_NATIVE_CONTENT_POSITION"
    #[serde(rename="TARGETING_TYPE_NATIVE_CONTENT_POSITION")]
    TARGETINGTYPENATIVECONTENTPOSITION,
    

    /// Target ads in an Open Measurement enabled inventory.
    ///
    /// "TARGETING_TYPE_OMID"
    #[serde(rename="TARGETING_TYPE_OMID")]
    TARGETINGTYPEOMID,
    

    /// Target ads to a specific audio content type.
    ///
    /// "TARGETING_TYPE_AUDIO_CONTENT_TYPE"
    #[serde(rename="TARGETING_TYPE_AUDIO_CONTENT_TYPE")]
    TARGETINGTYPEAUDIOCONTENTTYPE,
    

    /// Target ads to a specific content genre.
    ///
    /// "TARGETING_TYPE_CONTENT_GENRE"
    #[serde(rename="TARGETING_TYPE_CONTENT_GENRE")]
    TARGETINGTYPECONTENTGENRE,
}

impl AsRef<str> for TargetingTypeTargetingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED => "TARGETING_TYPE_UNSPECIFIED",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPECHANNEL => "TARGETING_TYPE_CHANNEL",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY => "TARGETING_TYPE_APP_CATEGORY",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEAPP => "TARGETING_TYPE_APP",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEURL => "TARGETING_TYPE_URL",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEDAYANDTIME => "TARGETING_TYPE_DAY_AND_TIME",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEAGERANGE => "TARGETING_TYPE_AGE_RANGE",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST => "TARGETING_TYPE_REGIONAL_LOCATION_LIST",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST => "TARGETING_TYPE_PROXIMITY_LOCATION_LIST",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEGENDER => "TARGETING_TYPE_GENDER",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE => "TARGETING_TYPE_VIDEO_PLAYER_SIZE",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT => "TARGETING_TYPE_USER_REWARDED_CONTENT",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS => "TARGETING_TYPE_PARENTAL_STATUS",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION => "TARGETING_TYPE_CONTENT_INSTREAM_POSITION",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION => "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEDEVICETYPE => "TARGETING_TYPE_DEVICE_TYPE",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP => "TARGETING_TYPE_AUDIENCE_GROUP",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEBROWSER => "TARGETING_TYPE_BROWSER",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME => "TARGETING_TYPE_HOUSEHOLD_INCOME",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION => "TARGETING_TYPE_ON_SCREEN_POSITION",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER => "TARGETING_TYPE_THIRD_PARTY_VERIFIER",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION => "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION => "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEENVIRONMENT => "TARGETING_TYPE_ENVIRONMENT",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPECARRIERANDISP => "TARGETING_TYPE_CARRIER_AND_ISP",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM => "TARGETING_TYPE_OPERATING_SYSTEM",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL => "TARGETING_TYPE_DEVICE_MAKE_MODEL",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEKEYWORD => "TARGETING_TYPE_KEYWORD",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST => "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEVIEWABILITY => "TARGETING_TYPE_VIEWABILITY",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPECATEGORY => "TARGETING_TYPE_CATEGORY",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE => "TARGETING_TYPE_INVENTORY_SOURCE",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPELANGUAGE => "TARGETING_TYPE_LANGUAGE",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS => "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEGEOREGION => "TARGETING_TYPE_GEO_REGION",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP => "TARGETING_TYPE_INVENTORY_SOURCE_GROUP",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEEXCHANGE => "TARGETING_TYPE_EXCHANGE",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE => "TARGETING_TYPE_SUB_EXCHANGE",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEPOI => "TARGETING_TYPE_POI",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN => "TARGETING_TYPE_BUSINESS_CHAIN",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPECONTENTDURATION => "TARGETING_TYPE_CONTENT_DURATION",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE => "TARGETING_TYPE_CONTENT_STREAM_TYPE",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION => "TARGETING_TYPE_NATIVE_CONTENT_POSITION",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEOMID => "TARGETING_TYPE_OMID",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE => "TARGETING_TYPE_AUDIO_CONTENT_TYPE",
            TargetingTypeTargetingTypeEnum::TARGETINGTYPECONTENTGENRE => "TARGETING_TYPE_CONTENT_GENRE",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetingTypeTargetingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_TYPE_UNSPECIFIED" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEUNSPECIFIED),
           "TARGETING_TYPE_CHANNEL" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPECHANNEL),
           "TARGETING_TYPE_APP_CATEGORY" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEAPPCATEGORY),
           "TARGETING_TYPE_APP" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEAPP),
           "TARGETING_TYPE_URL" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEURL),
           "TARGETING_TYPE_DAY_AND_TIME" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEDAYANDTIME),
           "TARGETING_TYPE_AGE_RANGE" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEAGERANGE),
           "TARGETING_TYPE_REGIONAL_LOCATION_LIST" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEREGIONALLOCATIONLIST),
           "TARGETING_TYPE_PROXIMITY_LOCATION_LIST" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEPROXIMITYLOCATIONLIST),
           "TARGETING_TYPE_GENDER" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEGENDER),
           "TARGETING_TYPE_VIDEO_PLAYER_SIZE" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEVIDEOPLAYERSIZE),
           "TARGETING_TYPE_USER_REWARDED_CONTENT" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEUSERREWARDEDCONTENT),
           "TARGETING_TYPE_PARENTAL_STATUS" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEPARENTALSTATUS),
           "TARGETING_TYPE_CONTENT_INSTREAM_POSITION" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPECONTENTINSTREAMPOSITION),
           "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPECONTENTOUTSTREAMPOSITION),
           "TARGETING_TYPE_DEVICE_TYPE" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEDEVICETYPE),
           "TARGETING_TYPE_AUDIENCE_GROUP" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEAUDIENCEGROUP),
           "TARGETING_TYPE_BROWSER" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEBROWSER),
           "TARGETING_TYPE_HOUSEHOLD_INCOME" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEHOUSEHOLDINCOME),
           "TARGETING_TYPE_ON_SCREEN_POSITION" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEONSCREENPOSITION),
           "TARGETING_TYPE_THIRD_PARTY_VERIFIER" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPETHIRDPARTYVERIFIER),
           "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEDIGITALCONTENTLABELEXCLUSION),
           "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPESENSITIVECATEGORYEXCLUSION),
           "TARGETING_TYPE_ENVIRONMENT" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEENVIRONMENT),
           "TARGETING_TYPE_CARRIER_AND_ISP" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPECARRIERANDISP),
           "TARGETING_TYPE_OPERATING_SYSTEM" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEOPERATINGSYSTEM),
           "TARGETING_TYPE_DEVICE_MAKE_MODEL" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEDEVICEMAKEMODEL),
           "TARGETING_TYPE_KEYWORD" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEKEYWORD),
           "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPENEGATIVEKEYWORDLIST),
           "TARGETING_TYPE_VIEWABILITY" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEVIEWABILITY),
           "TARGETING_TYPE_CATEGORY" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPECATEGORY),
           "TARGETING_TYPE_INVENTORY_SOURCE" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCE),
           "TARGETING_TYPE_LANGUAGE" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPELANGUAGE),
           "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEAUTHORIZEDSELLERSTATUS),
           "TARGETING_TYPE_GEO_REGION" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEGEOREGION),
           "TARGETING_TYPE_INVENTORY_SOURCE_GROUP" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEINVENTORYSOURCEGROUP),
           "TARGETING_TYPE_EXCHANGE" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEEXCHANGE),
           "TARGETING_TYPE_SUB_EXCHANGE" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPESUBEXCHANGE),
           "TARGETING_TYPE_POI" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEPOI),
           "TARGETING_TYPE_BUSINESS_CHAIN" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEBUSINESSCHAIN),
           "TARGETING_TYPE_CONTENT_DURATION" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPECONTENTDURATION),
           "TARGETING_TYPE_CONTENT_STREAM_TYPE" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPECONTENTSTREAMTYPE),
           "TARGETING_TYPE_NATIVE_CONTENT_POSITION" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPENATIVECONTENTPOSITION),
           "TARGETING_TYPE_OMID" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEOMID),
           "TARGETING_TYPE_AUDIO_CONTENT_TYPE" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPEAUDIOCONTENTTYPE),
           "TARGETING_TYPE_CONTENT_GENRE" => Ok(TargetingTypeTargetingTypeEnum::TARGETINGTYPECONTENTGENRE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetingTypeTargetingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


