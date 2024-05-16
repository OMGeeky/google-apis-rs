use super::*;



// region GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Specifies the target version of memcached engine to upgrade to.
pub enum GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum {
    

    /// Memcache version is not specified by customer
    ///
    /// "MEMCACHE_VERSION_UNSPECIFIED"
    #[serde(rename="MEMCACHE_VERSION_UNSPECIFIED")]
    MEMCACHEVERSIONUNSPECIFIED,
    

    /// Memcached 1.5 version.
    ///
    /// "MEMCACHE_1_5"
    #[serde(rename="MEMCACHE_1_5")]
    MEMCACHE15,
    

    /// Memcached 1.6.15 version.
    ///
    /// "MEMCACHE_1_6_15"
    #[serde(rename="MEMCACHE_1_6_15")]
    MEMCACHE1615,
}

impl AsRef<str> for GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum::MEMCACHEVERSIONUNSPECIFIED => "MEMCACHE_VERSION_UNSPECIFIED",
            GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum::MEMCACHE15 => "MEMCACHE_1_5",
            GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum::MEMCACHE1615 => "MEMCACHE_1_6_15",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEMCACHE_VERSION_UNSPECIFIED" => Ok(GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum::MEMCACHEVERSIONUNSPECIFIED),
           "MEMCACHE_1_5" => Ok(GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum::MEMCACHE15),
           "MEMCACHE_1_6_15" => Ok(GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum::MEMCACHE1615),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMemcacheV1beta2UpgradeInstanceRequestMemcacheVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceMemcacheVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest supported major version is `MEMCACHE_1_5`. The minor version will be automatically determined by our system based on the latest supported minor version.
pub enum InstanceMemcacheVersionEnum {
    

    /// Memcache version is not specified by customer
    ///
    /// "MEMCACHE_VERSION_UNSPECIFIED"
    #[serde(rename="MEMCACHE_VERSION_UNSPECIFIED")]
    MEMCACHEVERSIONUNSPECIFIED,
    

    /// Memcached 1.5 version.
    ///
    /// "MEMCACHE_1_5"
    #[serde(rename="MEMCACHE_1_5")]
    MEMCACHE15,
    

    /// Memcached 1.6.15 version.
    ///
    /// "MEMCACHE_1_6_15"
    #[serde(rename="MEMCACHE_1_6_15")]
    MEMCACHE1615,
}

impl AsRef<str> for InstanceMemcacheVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceMemcacheVersionEnum::MEMCACHEVERSIONUNSPECIFIED => "MEMCACHE_VERSION_UNSPECIFIED",
            InstanceMemcacheVersionEnum::MEMCACHE15 => "MEMCACHE_1_5",
            InstanceMemcacheVersionEnum::MEMCACHE1615 => "MEMCACHE_1_6_15",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceMemcacheVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEMCACHE_VERSION_UNSPECIFIED" => Ok(InstanceMemcacheVersionEnum::MEMCACHEVERSIONUNSPECIFIED),
           "MEMCACHE_1_5" => Ok(InstanceMemcacheVersionEnum::MEMCACHE15),
           "MEMCACHE_1_6_15" => Ok(InstanceMemcacheVersionEnum::MEMCACHE1615),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceMemcacheVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of this Memcached instance.
pub enum InstanceStateEnum {
    

    /// State not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Memcached instance is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Memcached instance has been created and ready to be used.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Memcached instance is updating configuration such as maintenance policy and schedule.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// Memcached instance is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Memcached instance is going through maintenance, e.g. data plane rollout.
    ///
    /// "PERFORMING_MAINTENANCE"
    #[serde(rename="PERFORMING_MAINTENANCE")]
    PERFORMINGMAINTENANCE,
    

    /// Memcached instance is undergoing memcached engine version upgrade.
    ///
    /// "MEMCACHE_VERSION_UPGRADING"
    #[serde(rename="MEMCACHE_VERSION_UPGRADING")]
    MEMCACHEVERSIONUPGRADING,
}

impl AsRef<str> for InstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            InstanceStateEnum::CREATING => "CREATING",
            InstanceStateEnum::READY => "READY",
            InstanceStateEnum::UPDATING => "UPDATING",
            InstanceStateEnum::DELETING => "DELETING",
            InstanceStateEnum::PERFORMINGMAINTENANCE => "PERFORMING_MAINTENANCE",
            InstanceStateEnum::MEMCACHEVERSIONUPGRADING => "MEMCACHE_VERSION_UPGRADING",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(InstanceStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(InstanceStateEnum::CREATING),
           "READY" => Ok(InstanceStateEnum::READY),
           "UPDATING" => Ok(InstanceStateEnum::UPDATING),
           "DELETING" => Ok(InstanceStateEnum::DELETING),
           "PERFORMING_MAINTENANCE" => Ok(InstanceStateEnum::PERFORMINGMAINTENANCE),
           "MEMCACHE_VERSION_UPGRADING" => Ok(InstanceStateEnum::MEMCACHEVERSIONUPGRADING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceMessageCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A code that correspond to one type of user-facing message.
pub enum InstanceMessageCodeEnum {
    

    /// Message Code not set.
    ///
    /// "CODE_UNSPECIFIED"
    #[serde(rename="CODE_UNSPECIFIED")]
    CODEUNSPECIFIED,
    

    /// Memcached nodes are distributed unevenly.
    ///
    /// "ZONE_DISTRIBUTION_UNBALANCED"
    #[serde(rename="ZONE_DISTRIBUTION_UNBALANCED")]
    ZONEDISTRIBUTIONUNBALANCED,
}

impl AsRef<str> for InstanceMessageCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceMessageCodeEnum::CODEUNSPECIFIED => "CODE_UNSPECIFIED",
            InstanceMessageCodeEnum::ZONEDISTRIBUTIONUNBALANCED => "ZONE_DISTRIBUTION_UNBALANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceMessageCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CODE_UNSPECIFIED" => Ok(InstanceMessageCodeEnum::CODEUNSPECIFIED),
           "ZONE_DISTRIBUTION_UNBALANCED" => Ok(InstanceMessageCodeEnum::ZONEDISTRIBUTIONUNBALANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceMessageCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NodeMemcacheVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Major version of memcached server running on this node, e.g. MEMCACHE_1_5
pub enum NodeMemcacheVersionEnum {
    

    /// Memcache version is not specified by customer
    ///
    /// "MEMCACHE_VERSION_UNSPECIFIED"
    #[serde(rename="MEMCACHE_VERSION_UNSPECIFIED")]
    MEMCACHEVERSIONUNSPECIFIED,
    

    /// Memcached 1.5 version.
    ///
    /// "MEMCACHE_1_5"
    #[serde(rename="MEMCACHE_1_5")]
    MEMCACHE15,
    

    /// Memcached 1.6.15 version.
    ///
    /// "MEMCACHE_1_6_15"
    #[serde(rename="MEMCACHE_1_6_15")]
    MEMCACHE1615,
}

impl AsRef<str> for NodeMemcacheVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodeMemcacheVersionEnum::MEMCACHEVERSIONUNSPECIFIED => "MEMCACHE_VERSION_UNSPECIFIED",
            NodeMemcacheVersionEnum::MEMCACHE15 => "MEMCACHE_1_5",
            NodeMemcacheVersionEnum::MEMCACHE1615 => "MEMCACHE_1_6_15",
        }
    }
}

impl std::convert::TryFrom< &str> for NodeMemcacheVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEMCACHE_VERSION_UNSPECIFIED" => Ok(NodeMemcacheVersionEnum::MEMCACHEVERSIONUNSPECIFIED),
           "MEMCACHE_1_5" => Ok(NodeMemcacheVersionEnum::MEMCACHE15),
           "MEMCACHE_1_6_15" => Ok(NodeMemcacheVersionEnum::MEMCACHE1615),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodeMemcacheVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NodeStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the Memcached node.
pub enum NodeStateEnum {
    

    /// Node state is not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Node is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Node has been created and ready to be used.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Node is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Node is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for NodeStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodeStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            NodeStateEnum::CREATING => "CREATING",
            NodeStateEnum::READY => "READY",
            NodeStateEnum::DELETING => "DELETING",
            NodeStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for NodeStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(NodeStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(NodeStateEnum::CREATING),
           "READY" => Ok(NodeStateEnum::READY),
           "DELETING" => Ok(NodeStateEnum::DELETING),
           "UPDATING" => Ok(NodeStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodeStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RescheduleMaintenanceRequestRescheduleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. If reschedule type is SPECIFIC_TIME, must set up schedule_time as well.
pub enum RescheduleMaintenanceRequestRescheduleTypeEnum {
    

    /// Not set.
    ///
    /// "RESCHEDULE_TYPE_UNSPECIFIED"
    #[serde(rename="RESCHEDULE_TYPE_UNSPECIFIED")]
    RESCHEDULETYPEUNSPECIFIED,
    

    /// If the user wants to schedule the maintenance to happen now.
    ///
    /// "IMMEDIATE"
    #[serde(rename="IMMEDIATE")]
    IMMEDIATE,
    

    /// If the user wants to use the existing maintenance policy to find the next available window.
    ///
    /// "NEXT_AVAILABLE_WINDOW"
    #[serde(rename="NEXT_AVAILABLE_WINDOW")]
    NEXTAVAILABLEWINDOW,
    

    /// If the user wants to reschedule the maintenance to a specific time.
    ///
    /// "SPECIFIC_TIME"
    #[serde(rename="SPECIFIC_TIME")]
    SPECIFICTIME,
}

impl AsRef<str> for RescheduleMaintenanceRequestRescheduleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RescheduleMaintenanceRequestRescheduleTypeEnum::RESCHEDULETYPEUNSPECIFIED => "RESCHEDULE_TYPE_UNSPECIFIED",
            RescheduleMaintenanceRequestRescheduleTypeEnum::IMMEDIATE => "IMMEDIATE",
            RescheduleMaintenanceRequestRescheduleTypeEnum::NEXTAVAILABLEWINDOW => "NEXT_AVAILABLE_WINDOW",
            RescheduleMaintenanceRequestRescheduleTypeEnum::SPECIFICTIME => "SPECIFIC_TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for RescheduleMaintenanceRequestRescheduleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESCHEDULE_TYPE_UNSPECIFIED" => Ok(RescheduleMaintenanceRequestRescheduleTypeEnum::RESCHEDULETYPEUNSPECIFIED),
           "IMMEDIATE" => Ok(RescheduleMaintenanceRequestRescheduleTypeEnum::IMMEDIATE),
           "NEXT_AVAILABLE_WINDOW" => Ok(RescheduleMaintenanceRequestRescheduleTypeEnum::NEXTAVAILABLEWINDOW),
           "SPECIFIC_TIME" => Ok(RescheduleMaintenanceRequestRescheduleTypeEnum::SPECIFICTIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RescheduleMaintenanceRequestRescheduleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WeeklyMaintenanceWindowDayEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Allows to define schedule that runs specified day of the week.
pub enum WeeklyMaintenanceWindowDayEnum {
    

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

impl AsRef<str> for WeeklyMaintenanceWindowDayEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WeeklyMaintenanceWindowDayEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            WeeklyMaintenanceWindowDayEnum::MONDAY => "MONDAY",
            WeeklyMaintenanceWindowDayEnum::TUESDAY => "TUESDAY",
            WeeklyMaintenanceWindowDayEnum::WEDNESDAY => "WEDNESDAY",
            WeeklyMaintenanceWindowDayEnum::THURSDAY => "THURSDAY",
            WeeklyMaintenanceWindowDayEnum::FRIDAY => "FRIDAY",
            WeeklyMaintenanceWindowDayEnum::SATURDAY => "SATURDAY",
            WeeklyMaintenanceWindowDayEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for WeeklyMaintenanceWindowDayEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(WeeklyMaintenanceWindowDayEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(WeeklyMaintenanceWindowDayEnum::MONDAY),
           "TUESDAY" => Ok(WeeklyMaintenanceWindowDayEnum::TUESDAY),
           "WEDNESDAY" => Ok(WeeklyMaintenanceWindowDayEnum::WEDNESDAY),
           "THURSDAY" => Ok(WeeklyMaintenanceWindowDayEnum::THURSDAY),
           "FRIDAY" => Ok(WeeklyMaintenanceWindowDayEnum::FRIDAY),
           "SATURDAY" => Ok(WeeklyMaintenanceWindowDayEnum::SATURDAY),
           "SUNDAY" => Ok(WeeklyMaintenanceWindowDayEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WeeklyMaintenanceWindowDayEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


