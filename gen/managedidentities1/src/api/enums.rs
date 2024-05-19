use super::*;



// region BackupStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the backup.
pub enum BackupStateEnum {
    

    /// Not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Backup is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Backup has been created and validated.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Backup has been created but failed validation.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Backup is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for BackupStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            BackupStateEnum::CREATING => "CREATING",
            BackupStateEnum::ACTIVE => "ACTIVE",
            BackupStateEnum::FAILED => "FAILED",
            BackupStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(BackupStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(BackupStateEnum::CREATING),
           "ACTIVE" => Ok(BackupStateEnum::ACTIVE),
           "FAILED" => Ok(BackupStateEnum::FAILED),
           "DELETING" => Ok(BackupStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Indicates whether it’s an on-demand backup or scheduled.
pub enum BackupTypeEnum {
    

    /// Backup was manually created.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Backup was manually created.
    ///
    /// "ON_DEMAND"
    #[serde(rename="ON_DEMAND")]
    ONDEMAND,
    

    /// Backup was automatically created.
    ///
    /// "SCHEDULED"
    #[serde(rename="SCHEDULED")]
    SCHEDULED,
}

impl AsRef<str> for BackupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            BackupTypeEnum::ONDEMAND => "ON_DEMAND",
            BackupTypeEnum::SCHEDULED => "SCHEDULED",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(BackupTypeEnum::TYPEUNSPECIFIED),
           "ON_DEMAND" => Ok(BackupTypeEnum::ONDEMAND),
           "SCHEDULED" => Ok(BackupTypeEnum::SCHEDULED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CheckMigrationPermissionResponseStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of DomainMigration.
pub enum CheckMigrationPermissionResponseStateEnum {
    

    /// DomainMigration is in unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Domain Migration is Disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Domain Migration is Enabled.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// Domain Migration is not in valid state.
    ///
    /// "NEEDS_MAINTENANCE"
    #[serde(rename="NEEDS_MAINTENANCE")]
    NEEDSMAINTENANCE,
}

impl AsRef<str> for CheckMigrationPermissionResponseStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CheckMigrationPermissionResponseStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CheckMigrationPermissionResponseStateEnum::DISABLED => "DISABLED",
            CheckMigrationPermissionResponseStateEnum::ENABLED => "ENABLED",
            CheckMigrationPermissionResponseStateEnum::NEEDSMAINTENANCE => "NEEDS_MAINTENANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for CheckMigrationPermissionResponseStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CheckMigrationPermissionResponseStateEnum::STATEUNSPECIFIED),
           "DISABLED" => Ok(CheckMigrationPermissionResponseStateEnum::DISABLED),
           "ENABLED" => Ok(CheckMigrationPermissionResponseStateEnum::ENABLED),
           "NEEDS_MAINTENANCE" => Ok(CheckMigrationPermissionResponseStateEnum::NEEDSMAINTENANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CheckMigrationPermissionResponseStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DomainStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of this domain.
pub enum DomainStateEnum {
    

    /// Not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The domain is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The domain has been created and is fully usable.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The domain's configuration is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The domain is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The domain is being repaired and may be unusable. Details can be found in the `status_message` field.
    ///
    /// "REPAIRING"
    #[serde(rename="REPAIRING")]
    REPAIRING,
    

    /// The domain is undergoing maintenance.
    ///
    /// "PERFORMING_MAINTENANCE"
    #[serde(rename="PERFORMING_MAINTENANCE")]
    PERFORMINGMAINTENANCE,
    

    /// The domain is not serving requests.
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
}

impl AsRef<str> for DomainStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DomainStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            DomainStateEnum::CREATING => "CREATING",
            DomainStateEnum::READY => "READY",
            DomainStateEnum::UPDATING => "UPDATING",
            DomainStateEnum::DELETING => "DELETING",
            DomainStateEnum::REPAIRING => "REPAIRING",
            DomainStateEnum::PERFORMINGMAINTENANCE => "PERFORMING_MAINTENANCE",
            DomainStateEnum::UNAVAILABLE => "UNAVAILABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for DomainStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(DomainStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(DomainStateEnum::CREATING),
           "READY" => Ok(DomainStateEnum::READY),
           "UPDATING" => Ok(DomainStateEnum::UPDATING),
           "DELETING" => Ok(DomainStateEnum::DELETING),
           "REPAIRING" => Ok(DomainStateEnum::REPAIRING),
           "PERFORMING_MAINTENANCE" => Ok(DomainStateEnum::PERFORMINGMAINTENANCE),
           "UNAVAILABLE" => Ok(DomainStateEnum::UNAVAILABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DomainStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LDAPSSettingStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of this LDAPS settings.
pub enum LDAPSSettingStateEnum {
    

    /// Not Set
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The LDAPS setting is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The LDAPS setting is ready.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The LDAPS setting is not applied correctly.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for LDAPSSettingStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LDAPSSettingStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            LDAPSSettingStateEnum::UPDATING => "UPDATING",
            LDAPSSettingStateEnum::ACTIVE => "ACTIVE",
            LDAPSSettingStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for LDAPSSettingStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(LDAPSSettingStateEnum::STATEUNSPECIFIED),
           "UPDATING" => Ok(LDAPSSettingStateEnum::UPDATING),
           "ACTIVE" => Ok(LDAPSSettingStateEnum::ACTIVE),
           "FAILED" => Ok(LDAPSSettingStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LDAPSSettingStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OnPremDomainSIDDetailSidFilteringStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current SID filtering state.
pub enum OnPremDomainSIDDetailSidFilteringStateEnum {
    

    /// SID Filtering is in unspecified state.
    ///
    /// "SID_FILTERING_STATE_UNSPECIFIED"
    #[serde(rename="SID_FILTERING_STATE_UNSPECIFIED")]
    SIDFILTERINGSTATEUNSPECIFIED,
    

    /// SID Filtering is Enabled.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// SID Filtering is Disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for OnPremDomainSIDDetailSidFilteringStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OnPremDomainSIDDetailSidFilteringStateEnum::SIDFILTERINGSTATEUNSPECIFIED => "SID_FILTERING_STATE_UNSPECIFIED",
            OnPremDomainSIDDetailSidFilteringStateEnum::ENABLED => "ENABLED",
            OnPremDomainSIDDetailSidFilteringStateEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for OnPremDomainSIDDetailSidFilteringStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SID_FILTERING_STATE_UNSPECIFIED" => Ok(OnPremDomainSIDDetailSidFilteringStateEnum::SIDFILTERINGSTATEUNSPECIFIED),
           "ENABLED" => Ok(OnPremDomainSIDDetailSidFilteringStateEnum::ENABLED),
           "DISABLED" => Ok(OnPremDomainSIDDetailSidFilteringStateEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OnPremDomainSIDDetailSidFilteringStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PeeringStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of this Peering.
pub enum PeeringStateEnum {
    

    /// Not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Peering is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Peering is connected.
    ///
    /// "CONNECTED"
    #[serde(rename="CONNECTED")]
    CONNECTED,
    

    /// Peering is disconnected.
    ///
    /// "DISCONNECTED"
    #[serde(rename="DISCONNECTED")]
    DISCONNECTED,
    

    /// Peering is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for PeeringStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PeeringStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            PeeringStateEnum::CREATING => "CREATING",
            PeeringStateEnum::CONNECTED => "CONNECTED",
            PeeringStateEnum::DISCONNECTED => "DISCONNECTED",
            PeeringStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for PeeringStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(PeeringStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(PeeringStateEnum::CREATING),
           "CONNECTED" => Ok(PeeringStateEnum::CONNECTED),
           "DISCONNECTED" => Ok(PeeringStateEnum::DISCONNECTED),
           "DELETING" => Ok(PeeringStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PeeringStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SqlIntegrationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the SQL integration.
pub enum SqlIntegrationStateEnum {
    

    /// Not Set
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The SQL integration is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The SQL integration is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The SQL integration is ready.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
}

impl AsRef<str> for SqlIntegrationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SqlIntegrationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SqlIntegrationStateEnum::CREATING => "CREATING",
            SqlIntegrationStateEnum::DELETING => "DELETING",
            SqlIntegrationStateEnum::READY => "READY",
        }
    }
}

impl std::convert::TryFrom< &str> for SqlIntegrationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SqlIntegrationStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(SqlIntegrationStateEnum::CREATING),
           "DELETING" => Ok(SqlIntegrationStateEnum::DELETING),
           "READY" => Ok(SqlIntegrationStateEnum::READY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SqlIntegrationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TrustStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the trust.
pub enum TrustStateEnum {
    

    /// Not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The domain trust is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The domain trust is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The domain trust is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The domain trust is connected.
    ///
    /// "CONNECTED"
    #[serde(rename="CONNECTED")]
    CONNECTED,
    

    /// The domain trust is disconnected.
    ///
    /// "DISCONNECTED"
    #[serde(rename="DISCONNECTED")]
    DISCONNECTED,
}

impl AsRef<str> for TrustStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TrustStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            TrustStateEnum::CREATING => "CREATING",
            TrustStateEnum::UPDATING => "UPDATING",
            TrustStateEnum::DELETING => "DELETING",
            TrustStateEnum::CONNECTED => "CONNECTED",
            TrustStateEnum::DISCONNECTED => "DISCONNECTED",
        }
    }
}

impl std::convert::TryFrom< &str> for TrustStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(TrustStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(TrustStateEnum::CREATING),
           "UPDATING" => Ok(TrustStateEnum::UPDATING),
           "DELETING" => Ok(TrustStateEnum::DELETING),
           "CONNECTED" => Ok(TrustStateEnum::CONNECTED),
           "DISCONNECTED" => Ok(TrustStateEnum::DISCONNECTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TrustStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TrustTrustDirectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The trust direction, which decides if the current domain is trusted, trusting, or both.
pub enum TrustTrustDirectionEnum {
    

    /// Not set.
    ///
    /// "TRUST_DIRECTION_UNSPECIFIED"
    #[serde(rename="TRUST_DIRECTION_UNSPECIFIED")]
    TRUSTDIRECTIONUNSPECIFIED,
    

    /// The inbound direction represents the trusting side.
    ///
    /// "INBOUND"
    #[serde(rename="INBOUND")]
    INBOUND,
    

    /// The outboud direction represents the trusted side.
    ///
    /// "OUTBOUND"
    #[serde(rename="OUTBOUND")]
    OUTBOUND,
    

    /// The bidirectional direction represents the trusted / trusting side.
    ///
    /// "BIDIRECTIONAL"
    #[serde(rename="BIDIRECTIONAL")]
    BIDIRECTIONAL,
}

impl AsRef<str> for TrustTrustDirectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TrustTrustDirectionEnum::TRUSTDIRECTIONUNSPECIFIED => "TRUST_DIRECTION_UNSPECIFIED",
            TrustTrustDirectionEnum::INBOUND => "INBOUND",
            TrustTrustDirectionEnum::OUTBOUND => "OUTBOUND",
            TrustTrustDirectionEnum::BIDIRECTIONAL => "BIDIRECTIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for TrustTrustDirectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRUST_DIRECTION_UNSPECIFIED" => Ok(TrustTrustDirectionEnum::TRUSTDIRECTIONUNSPECIFIED),
           "INBOUND" => Ok(TrustTrustDirectionEnum::INBOUND),
           "OUTBOUND" => Ok(TrustTrustDirectionEnum::OUTBOUND),
           "BIDIRECTIONAL" => Ok(TrustTrustDirectionEnum::BIDIRECTIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TrustTrustDirectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TrustTrustTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of trust represented by the trust resource.
pub enum TrustTrustTypeEnum {
    

    /// Not set.
    ///
    /// "TRUST_TYPE_UNSPECIFIED"
    #[serde(rename="TRUST_TYPE_UNSPECIFIED")]
    TRUSTTYPEUNSPECIFIED,
    

    /// The forest trust.
    ///
    /// "FOREST"
    #[serde(rename="FOREST")]
    FOREST,
    

    /// The external domain trust.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
}

impl AsRef<str> for TrustTrustTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TrustTrustTypeEnum::TRUSTTYPEUNSPECIFIED => "TRUST_TYPE_UNSPECIFIED",
            TrustTrustTypeEnum::FOREST => "FOREST",
            TrustTrustTypeEnum::EXTERNAL => "EXTERNAL",
        }
    }
}

impl std::convert::TryFrom< &str> for TrustTrustTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRUST_TYPE_UNSPECIFIED" => Ok(TrustTrustTypeEnum::TRUSTTYPEUNSPECIFIED),
           "FOREST" => Ok(TrustTrustTypeEnum::FOREST),
           "EXTERNAL" => Ok(TrustTrustTypeEnum::EXTERNAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TrustTrustTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


