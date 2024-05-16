use super::*;



// region AdvertisedIdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the identifier type.
Required.
pub enum AdvertisedIdTypeEnum {
    

    /// Do not use this value.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Eddystone, an open beacon format that supports Android and iOS devices
https://github.com/google/eddystone/wiki/Beacon-Specification
    ///
    /// "EDDYSTONE"
    #[serde(rename="EDDYSTONE")]
    EDDYSTONE,
    

    /// Apple iBeacon compatible beacon
    ///
    /// "IBEACON"
    #[serde(rename="IBEACON")]
    IBEACON,
    

    /// See http://altbeacon.org and/or https://github.com/AltBeacon/spec.
    ///
    /// "ALTBEACON"
    #[serde(rename="ALTBEACON")]
    ALTBEACON,
    

    /// Eddystone Ephemeral ID
    ///
    /// "EDDYSTONE_EID"
    #[serde(rename="EDDYSTONE_EID")]
    EDDYSTONEEID,
}

impl AsRef<str> for AdvertisedIdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertisedIdTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            AdvertisedIdTypeEnum::EDDYSTONE => "EDDYSTONE",
            AdvertisedIdTypeEnum::IBEACON => "IBEACON",
            AdvertisedIdTypeEnum::ALTBEACON => "ALTBEACON",
            AdvertisedIdTypeEnum::EDDYSTONEEID => "EDDYSTONE_EID",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertisedIdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(AdvertisedIdTypeEnum::TYPEUNSPECIFIED),
           "EDDYSTONE" => Ok(AdvertisedIdTypeEnum::EDDYSTONE),
           "IBEACON" => Ok(AdvertisedIdTypeEnum::IBEACON),
           "ALTBEACON" => Ok(AdvertisedIdTypeEnum::ALTBEACON),
           "EDDYSTONE_EID" => Ok(AdvertisedIdTypeEnum::EDDYSTONEEID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertisedIdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BeaconExpectedStabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Expected location stability. This is set when the beacon is registered or
updated, not automatically detected in any way.
Optional.
pub enum BeaconExpectedStabilityEnum {
    

    /// Do not use this value.
    ///
    /// "STABILITY_UNSPECIFIED"
    #[serde(rename="STABILITY_UNSPECIFIED")]
    STABILITYUNSPECIFIED,
    

    /// Not expected to move, for example a store's front door.
    ///
    /// "STABLE"
    #[serde(rename="STABLE")]
    STABLE,
    

    /// Usually stable but may move rarely, usually within a single place,
for example a store display.
    ///
    /// "PORTABLE"
    #[serde(rename="PORTABLE")]
    PORTABLE,
    

    /// Moves frequently, for example a personal item or food truck.
    ///
    /// "MOBILE"
    #[serde(rename="MOBILE")]
    MOBILE,
    

    /// Moves continuously in service, for example a bus or train.
    ///
    /// "ROVING"
    #[serde(rename="ROVING")]
    ROVING,
}

impl AsRef<str> for BeaconExpectedStabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BeaconExpectedStabilityEnum::STABILITYUNSPECIFIED => "STABILITY_UNSPECIFIED",
            BeaconExpectedStabilityEnum::STABLE => "STABLE",
            BeaconExpectedStabilityEnum::PORTABLE => "PORTABLE",
            BeaconExpectedStabilityEnum::MOBILE => "MOBILE",
            BeaconExpectedStabilityEnum::ROVING => "ROVING",
        }
    }
}

impl std::convert::TryFrom< &str> for BeaconExpectedStabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STABILITY_UNSPECIFIED" => Ok(BeaconExpectedStabilityEnum::STABILITYUNSPECIFIED),
           "STABLE" => Ok(BeaconExpectedStabilityEnum::STABLE),
           "PORTABLE" => Ok(BeaconExpectedStabilityEnum::PORTABLE),
           "MOBILE" => Ok(BeaconExpectedStabilityEnum::MOBILE),
           "ROVING" => Ok(BeaconExpectedStabilityEnum::ROVING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BeaconExpectedStabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BeaconStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current status of the beacon.
Required.
pub enum BeaconStatusEnum {
    

    /// Do not use this value.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The "normal" in-use state of a beacon.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Beacon should no longer be used for any purpose. This is irreversible.
    ///
    /// "DECOMMISSIONED"
    #[serde(rename="DECOMMISSIONED")]
    DECOMMISSIONED,
    

    /// The beacon should not be visible to mobile devices. This is reversible.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for BeaconStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BeaconStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            BeaconStatusEnum::ACTIVE => "ACTIVE",
            BeaconStatusEnum::DECOMMISSIONED => "DECOMMISSIONED",
            BeaconStatusEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for BeaconStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(BeaconStatusEnum::STATUSUNSPECIFIED),
           "ACTIVE" => Ok(BeaconStatusEnum::ACTIVE),
           "DECOMMISSIONED" => Ok(BeaconStatusEnum::DECOMMISSIONED),
           "INACTIVE" => Ok(BeaconStatusEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BeaconStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DiagnosticAlertsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An unordered list of Alerts that the beacon has.
pub enum DiagnosticAlertsEnum {
    

    /// Invalid value. Should never appear.
    ///
    /// "ALERT_UNSPECIFIED"
    #[serde(rename="ALERT_UNSPECIFIED")]
    ALERTUNSPECIFIED,
    

    /// The beacon has been reported far from its expected location (the beacon's
lat_lng field if populated, otherwise, if the beacon's place_id field is
present, the center of that place). This may indicate that the beacon has
been moved. This signal is not 100% accurate, but indicates that further
investigation is worthwhile.
    ///
    /// "WRONG_LOCATION"
    #[serde(rename="WRONG_LOCATION")]
    WRONGLOCATION,
    

    /// The battery level for the beacon is low enough that, given the beacon's
current use, its battery will run out with in the next 60 days. This
indicates that the battery should be replaced soon.
    ///
    /// "LOW_BATTERY"
    #[serde(rename="LOW_BATTERY")]
    LOWBATTERY,
    

    /// The beacon has been reported at a very low rate or not at all. This may
indicate that the beacon is broken or just that no one has gone near the
beacon in recent days. If this status appears unexpectedly, the beacon
owner should investigate further.
    ///
    /// "LOW_ACTIVITY"
    #[serde(rename="LOW_ACTIVITY")]
    LOWACTIVITY,
}

impl AsRef<str> for DiagnosticAlertsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DiagnosticAlertsEnum::ALERTUNSPECIFIED => "ALERT_UNSPECIFIED",
            DiagnosticAlertsEnum::WRONGLOCATION => "WRONG_LOCATION",
            DiagnosticAlertsEnum::LOWBATTERY => "LOW_BATTERY",
            DiagnosticAlertsEnum::LOWACTIVITY => "LOW_ACTIVITY",
        }
    }
}

impl std::convert::TryFrom< &str> for DiagnosticAlertsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALERT_UNSPECIFIED" => Ok(DiagnosticAlertsEnum::ALERTUNSPECIFIED),
           "WRONG_LOCATION" => Ok(DiagnosticAlertsEnum::WRONGLOCATION),
           "LOW_BATTERY" => Ok(DiagnosticAlertsEnum::LOWBATTERY),
           "LOW_ACTIVITY" => Ok(DiagnosticAlertsEnum::LOWACTIVITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DiagnosticAlertsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NamespaceServingVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies what clients may receive attachments under this namespace
via `beaconinfo.getforobserved`.
pub enum NamespaceServingVisibilityEnum {
    

    /// Do not use this value.
    ///
    /// "VISIBILITY_UNSPECIFIED"
    #[serde(rename="VISIBILITY_UNSPECIFIED")]
    VISIBILITYUNSPECIFIED,
    

    /// Served only to the project that owns the namespace.
    ///
    /// "UNLISTED"
    #[serde(rename="UNLISTED")]
    UNLISTED,
    

    /// Any project can subscribe to attachments under the namespace.
    ///
    /// "PUBLIC"
    #[serde(rename="PUBLIC")]
    PUBLIC,
}

impl AsRef<str> for NamespaceServingVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NamespaceServingVisibilityEnum::VISIBILITYUNSPECIFIED => "VISIBILITY_UNSPECIFIED",
            NamespaceServingVisibilityEnum::UNLISTED => "UNLISTED",
            NamespaceServingVisibilityEnum::PUBLIC => "PUBLIC",
        }
    }
}

impl std::convert::TryFrom< &str> for NamespaceServingVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VISIBILITY_UNSPECIFIED" => Ok(NamespaceServingVisibilityEnum::VISIBILITYUNSPECIFIED),
           "UNLISTED" => Ok(NamespaceServingVisibilityEnum::UNLISTED),
           "PUBLIC" => Ok(NamespaceServingVisibilityEnum::PUBLIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NamespaceServingVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BeaconAlertFilterEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Requests only beacons that have the given alert. For example, to find
beacons that have low batteries use `alert_filter=LOW_BATTERY`.
pub enum BeaconAlertFilterEnum {
    

    /// no description found
    ///
    /// "ALERT_UNSPECIFIED"
    #[serde(rename="ALERT_UNSPECIFIED")]
    ALERTUNSPECIFIED,
    

    /// no description found
    ///
    /// "WRONG_LOCATION"
    #[serde(rename="WRONG_LOCATION")]
    WRONGLOCATION,
    

    /// no description found
    ///
    /// "LOW_BATTERY"
    #[serde(rename="LOW_BATTERY")]
    LOWBATTERY,
    

    /// no description found
    ///
    /// "LOW_ACTIVITY"
    #[serde(rename="LOW_ACTIVITY")]
    LOWACTIVITY,
}

impl AsRef<str> for BeaconAlertFilterEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BeaconAlertFilterEnum::ALERTUNSPECIFIED => "ALERT_UNSPECIFIED",
            BeaconAlertFilterEnum::WRONGLOCATION => "WRONG_LOCATION",
            BeaconAlertFilterEnum::LOWBATTERY => "LOW_BATTERY",
            BeaconAlertFilterEnum::LOWACTIVITY => "LOW_ACTIVITY",
        }
    }
}

impl std::convert::TryFrom< &str> for BeaconAlertFilterEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALERT_UNSPECIFIED" => Ok(BeaconAlertFilterEnum::ALERTUNSPECIFIED),
           "WRONG_LOCATION" => Ok(BeaconAlertFilterEnum::WRONGLOCATION),
           "LOW_BATTERY" => Ok(BeaconAlertFilterEnum::LOWBATTERY),
           "LOW_ACTIVITY" => Ok(BeaconAlertFilterEnum::LOWACTIVITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BeaconAlertFilterEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


