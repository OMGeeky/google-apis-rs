use super::*;



// region SasPortalDeviceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Device state.
pub enum SasPortalDeviceStateEnum {
    

    /// Unspecified state.
    ///
    /// "DEVICE_STATE_UNSPECIFIED"
    #[serde(rename="DEVICE_STATE_UNSPECIFIED")]
    DEVICESTATEUNSPECIFIED,
    

    /// Device created in the SAS Portal, however, not yet registered with SAS.
    ///
    /// "RESERVED"
    #[serde(rename="RESERVED")]
    RESERVED,
    

    /// Device registered with SAS.
    ///
    /// "REGISTERED"
    #[serde(rename="REGISTERED")]
    REGISTERED,
    

    /// Device de-registered with SAS.
    ///
    /// "DEREGISTERED"
    #[serde(rename="DEREGISTERED")]
    DEREGISTERED,
}

impl AsRef<str> for SasPortalDeviceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SasPortalDeviceStateEnum::DEVICESTATEUNSPECIFIED => "DEVICE_STATE_UNSPECIFIED",
            SasPortalDeviceStateEnum::RESERVED => "RESERVED",
            SasPortalDeviceStateEnum::REGISTERED => "REGISTERED",
            SasPortalDeviceStateEnum::DEREGISTERED => "DEREGISTERED",
        }
    }
}

impl std::convert::TryFrom< &str> for SasPortalDeviceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_STATE_UNSPECIFIED" => Ok(SasPortalDeviceStateEnum::DEVICESTATEUNSPECIFIED),
           "RESERVED" => Ok(SasPortalDeviceStateEnum::RESERVED),
           "REGISTERED" => Ok(SasPortalDeviceStateEnum::REGISTERED),
           "DEREGISTERED" => Ok(SasPortalDeviceStateEnum::DEREGISTERED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SasPortalDeviceStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SasPortalDeviceAirInterfaceRadioTechnologyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Conditional. This field specifies the radio access technology that is used for the CBSD.
pub enum SasPortalDeviceAirInterfaceRadioTechnologyEnum {
    
    /// "RADIO_TECHNOLOGY_UNSPECIFIED"
    #[serde(rename="RADIO_TECHNOLOGY_UNSPECIFIED")]
    RADIOTECHNOLOGYUNSPECIFIED,
    
    /// "E_UTRA"
    #[serde(rename="E_UTRA")]
    EUTRA,
    
    /// "CAMBIUM_NETWORKS"
    #[serde(rename="CAMBIUM_NETWORKS")]
    CAMBIUMNETWORKS,
    
    /// "FOUR_G_BBW_SAA_1"
    #[serde(rename="FOUR_G_BBW_SAA_1")]
    FOURGBBWSAA1,
    
    /// "NR"
    #[serde(rename="NR")]
    NR,
    
    /// "DOODLE_CBRS"
    #[serde(rename="DOODLE_CBRS")]
    DOODLECBRS,
    
    /// "CW"
    #[serde(rename="CW")]
    CW,
    
    /// "REDLINE"
    #[serde(rename="REDLINE")]
    REDLINE,
    
    /// "TARANA_WIRELESS"
    #[serde(rename="TARANA_WIRELESS")]
    TARANAWIRELESS,
}

impl AsRef<str> for SasPortalDeviceAirInterfaceRadioTechnologyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SasPortalDeviceAirInterfaceRadioTechnologyEnum::RADIOTECHNOLOGYUNSPECIFIED => "RADIO_TECHNOLOGY_UNSPECIFIED",
            SasPortalDeviceAirInterfaceRadioTechnologyEnum::EUTRA => "E_UTRA",
            SasPortalDeviceAirInterfaceRadioTechnologyEnum::CAMBIUMNETWORKS => "CAMBIUM_NETWORKS",
            SasPortalDeviceAirInterfaceRadioTechnologyEnum::FOURGBBWSAA1 => "FOUR_G_BBW_SAA_1",
            SasPortalDeviceAirInterfaceRadioTechnologyEnum::NR => "NR",
            SasPortalDeviceAirInterfaceRadioTechnologyEnum::DOODLECBRS => "DOODLE_CBRS",
            SasPortalDeviceAirInterfaceRadioTechnologyEnum::CW => "CW",
            SasPortalDeviceAirInterfaceRadioTechnologyEnum::REDLINE => "REDLINE",
            SasPortalDeviceAirInterfaceRadioTechnologyEnum::TARANAWIRELESS => "TARANA_WIRELESS",
        }
    }
}

impl std::convert::TryFrom< &str> for SasPortalDeviceAirInterfaceRadioTechnologyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RADIO_TECHNOLOGY_UNSPECIFIED" => Ok(SasPortalDeviceAirInterfaceRadioTechnologyEnum::RADIOTECHNOLOGYUNSPECIFIED),
           "E_UTRA" => Ok(SasPortalDeviceAirInterfaceRadioTechnologyEnum::EUTRA),
           "CAMBIUM_NETWORKS" => Ok(SasPortalDeviceAirInterfaceRadioTechnologyEnum::CAMBIUMNETWORKS),
           "FOUR_G_BBW_SAA_1" => Ok(SasPortalDeviceAirInterfaceRadioTechnologyEnum::FOURGBBWSAA1),
           "NR" => Ok(SasPortalDeviceAirInterfaceRadioTechnologyEnum::NR),
           "DOODLE_CBRS" => Ok(SasPortalDeviceAirInterfaceRadioTechnologyEnum::DOODLECBRS),
           "CW" => Ok(SasPortalDeviceAirInterfaceRadioTechnologyEnum::CW),
           "REDLINE" => Ok(SasPortalDeviceAirInterfaceRadioTechnologyEnum::REDLINE),
           "TARANA_WIRELESS" => Ok(SasPortalDeviceAirInterfaceRadioTechnologyEnum::TARANAWIRELESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SasPortalDeviceAirInterfaceRadioTechnologyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SasPortalDeviceConfigCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// FCC category of the device.
pub enum SasPortalDeviceConfigCategoryEnum {
    

    /// Unspecified device category.
    ///
    /// "DEVICE_CATEGORY_UNSPECIFIED"
    #[serde(rename="DEVICE_CATEGORY_UNSPECIFIED")]
    DEVICECATEGORYUNSPECIFIED,
    

    /// Category A.
    ///
    /// "DEVICE_CATEGORY_A"
    #[serde(rename="DEVICE_CATEGORY_A")]
    DEVICECATEGORYA,
    

    /// Category B.
    ///
    /// "DEVICE_CATEGORY_B"
    #[serde(rename="DEVICE_CATEGORY_B")]
    DEVICECATEGORYB,
}

impl AsRef<str> for SasPortalDeviceConfigCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SasPortalDeviceConfigCategoryEnum::DEVICECATEGORYUNSPECIFIED => "DEVICE_CATEGORY_UNSPECIFIED",
            SasPortalDeviceConfigCategoryEnum::DEVICECATEGORYA => "DEVICE_CATEGORY_A",
            SasPortalDeviceConfigCategoryEnum::DEVICECATEGORYB => "DEVICE_CATEGORY_B",
        }
    }
}

impl std::convert::TryFrom< &str> for SasPortalDeviceConfigCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_CATEGORY_UNSPECIFIED" => Ok(SasPortalDeviceConfigCategoryEnum::DEVICECATEGORYUNSPECIFIED),
           "DEVICE_CATEGORY_A" => Ok(SasPortalDeviceConfigCategoryEnum::DEVICECATEGORYA),
           "DEVICE_CATEGORY_B" => Ok(SasPortalDeviceConfigCategoryEnum::DEVICECATEGORYB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SasPortalDeviceConfigCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SasPortalDeviceConfigMeasurementCapabilitiesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Measurement reporting capabilities of the device.
pub enum SasPortalDeviceConfigMeasurementCapabilitiesEnum {
    
    /// "MEASUREMENT_CAPABILITY_UNSPECIFIED"
    #[serde(rename="MEASUREMENT_CAPABILITY_UNSPECIFIED")]
    MEASUREMENTCAPABILITYUNSPECIFIED,
    
    /// "MEASUREMENT_CAPABILITY_RECEIVED_POWER_WITH_GRANT"
    #[serde(rename="MEASUREMENT_CAPABILITY_RECEIVED_POWER_WITH_GRANT")]
    MEASUREMENTCAPABILITYRECEIVEDPOWERWITHGRANT,
    
    /// "MEASUREMENT_CAPABILITY_RECEIVED_POWER_WITHOUT_GRANT"
    #[serde(rename="MEASUREMENT_CAPABILITY_RECEIVED_POWER_WITHOUT_GRANT")]
    MEASUREMENTCAPABILITYRECEIVEDPOWERWITHOUTGRANT,
}

impl AsRef<str> for SasPortalDeviceConfigMeasurementCapabilitiesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SasPortalDeviceConfigMeasurementCapabilitiesEnum::MEASUREMENTCAPABILITYUNSPECIFIED => "MEASUREMENT_CAPABILITY_UNSPECIFIED",
            SasPortalDeviceConfigMeasurementCapabilitiesEnum::MEASUREMENTCAPABILITYRECEIVEDPOWERWITHGRANT => "MEASUREMENT_CAPABILITY_RECEIVED_POWER_WITH_GRANT",
            SasPortalDeviceConfigMeasurementCapabilitiesEnum::MEASUREMENTCAPABILITYRECEIVEDPOWERWITHOUTGRANT => "MEASUREMENT_CAPABILITY_RECEIVED_POWER_WITHOUT_GRANT",
        }
    }
}

impl std::convert::TryFrom< &str> for SasPortalDeviceConfigMeasurementCapabilitiesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEASUREMENT_CAPABILITY_UNSPECIFIED" => Ok(SasPortalDeviceConfigMeasurementCapabilitiesEnum::MEASUREMENTCAPABILITYUNSPECIFIED),
           "MEASUREMENT_CAPABILITY_RECEIVED_POWER_WITH_GRANT" => Ok(SasPortalDeviceConfigMeasurementCapabilitiesEnum::MEASUREMENTCAPABILITYRECEIVEDPOWERWITHGRANT),
           "MEASUREMENT_CAPABILITY_RECEIVED_POWER_WITHOUT_GRANT" => Ok(SasPortalDeviceConfigMeasurementCapabilitiesEnum::MEASUREMENTCAPABILITYRECEIVEDPOWERWITHOUTGRANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SasPortalDeviceConfigMeasurementCapabilitiesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SasPortalDeviceConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the configuration.
pub enum SasPortalDeviceConfigStateEnum {
    
    /// "DEVICE_CONFIG_STATE_UNSPECIFIED"
    #[serde(rename="DEVICE_CONFIG_STATE_UNSPECIFIED")]
    DEVICECONFIGSTATEUNSPECIFIED,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    
    /// "FINAL"
    #[serde(rename="FINAL")]
    FINAL,
}

impl AsRef<str> for SasPortalDeviceConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SasPortalDeviceConfigStateEnum::DEVICECONFIGSTATEUNSPECIFIED => "DEVICE_CONFIG_STATE_UNSPECIFIED",
            SasPortalDeviceConfigStateEnum::DRAFT => "DRAFT",
            SasPortalDeviceConfigStateEnum::FINAL => "FINAL",
        }
    }
}

impl std::convert::TryFrom< &str> for SasPortalDeviceConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_CONFIG_STATE_UNSPECIFIED" => Ok(SasPortalDeviceConfigStateEnum::DEVICECONFIGSTATEUNSPECIFIED),
           "DRAFT" => Ok(SasPortalDeviceConfigStateEnum::DRAFT),
           "FINAL" => Ok(SasPortalDeviceConfigStateEnum::FINAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SasPortalDeviceConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SasPortalDeviceGrantChannelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of channel used.
pub enum SasPortalDeviceGrantChannelTypeEnum {
    
    /// "CHANNEL_TYPE_UNSPECIFIED"
    #[serde(rename="CHANNEL_TYPE_UNSPECIFIED")]
    CHANNELTYPEUNSPECIFIED,
    
    /// "CHANNEL_TYPE_GAA"
    #[serde(rename="CHANNEL_TYPE_GAA")]
    CHANNELTYPEGAA,
    
    /// "CHANNEL_TYPE_PAL"
    #[serde(rename="CHANNEL_TYPE_PAL")]
    CHANNELTYPEPAL,
}

impl AsRef<str> for SasPortalDeviceGrantChannelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SasPortalDeviceGrantChannelTypeEnum::CHANNELTYPEUNSPECIFIED => "CHANNEL_TYPE_UNSPECIFIED",
            SasPortalDeviceGrantChannelTypeEnum::CHANNELTYPEGAA => "CHANNEL_TYPE_GAA",
            SasPortalDeviceGrantChannelTypeEnum::CHANNELTYPEPAL => "CHANNEL_TYPE_PAL",
        }
    }
}

impl std::convert::TryFrom< &str> for SasPortalDeviceGrantChannelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHANNEL_TYPE_UNSPECIFIED" => Ok(SasPortalDeviceGrantChannelTypeEnum::CHANNELTYPEUNSPECIFIED),
           "CHANNEL_TYPE_GAA" => Ok(SasPortalDeviceGrantChannelTypeEnum::CHANNELTYPEGAA),
           "CHANNEL_TYPE_PAL" => Ok(SasPortalDeviceGrantChannelTypeEnum::CHANNELTYPEPAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SasPortalDeviceGrantChannelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SasPortalDeviceGrantStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the grant.
pub enum SasPortalDeviceGrantStateEnum {
    
    /// "GRANT_STATE_UNSPECIFIED"
    #[serde(rename="GRANT_STATE_UNSPECIFIED")]
    GRANTSTATEUNSPECIFIED,
    

    /// The grant has been granted but the device is not heartbeating on it.
    ///
    /// "GRANT_STATE_GRANTED"
    #[serde(rename="GRANT_STATE_GRANTED")]
    GRANTSTATEGRANTED,
    

    /// The grant has been terminated by the SAS.
    ///
    /// "GRANT_STATE_TERMINATED"
    #[serde(rename="GRANT_STATE_TERMINATED")]
    GRANTSTATETERMINATED,
    

    /// The grant has been suspended by the SAS.
    ///
    /// "GRANT_STATE_SUSPENDED"
    #[serde(rename="GRANT_STATE_SUSPENDED")]
    GRANTSTATESUSPENDED,
    

    /// The device is currently transmitting.
    ///
    /// "GRANT_STATE_AUTHORIZED"
    #[serde(rename="GRANT_STATE_AUTHORIZED")]
    GRANTSTATEAUTHORIZED,
    

    /// The grant has expired.
    ///
    /// "GRANT_STATE_EXPIRED"
    #[serde(rename="GRANT_STATE_EXPIRED")]
    GRANTSTATEEXPIRED,
}

impl AsRef<str> for SasPortalDeviceGrantStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SasPortalDeviceGrantStateEnum::GRANTSTATEUNSPECIFIED => "GRANT_STATE_UNSPECIFIED",
            SasPortalDeviceGrantStateEnum::GRANTSTATEGRANTED => "GRANT_STATE_GRANTED",
            SasPortalDeviceGrantStateEnum::GRANTSTATETERMINATED => "GRANT_STATE_TERMINATED",
            SasPortalDeviceGrantStateEnum::GRANTSTATESUSPENDED => "GRANT_STATE_SUSPENDED",
            SasPortalDeviceGrantStateEnum::GRANTSTATEAUTHORIZED => "GRANT_STATE_AUTHORIZED",
            SasPortalDeviceGrantStateEnum::GRANTSTATEEXPIRED => "GRANT_STATE_EXPIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for SasPortalDeviceGrantStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GRANT_STATE_UNSPECIFIED" => Ok(SasPortalDeviceGrantStateEnum::GRANTSTATEUNSPECIFIED),
           "GRANT_STATE_GRANTED" => Ok(SasPortalDeviceGrantStateEnum::GRANTSTATEGRANTED),
           "GRANT_STATE_TERMINATED" => Ok(SasPortalDeviceGrantStateEnum::GRANTSTATETERMINATED),
           "GRANT_STATE_SUSPENDED" => Ok(SasPortalDeviceGrantStateEnum::GRANTSTATESUSPENDED),
           "GRANT_STATE_AUTHORIZED" => Ok(SasPortalDeviceGrantStateEnum::GRANTSTATEAUTHORIZED),
           "GRANT_STATE_EXPIRED" => Ok(SasPortalDeviceGrantStateEnum::GRANTSTATEEXPIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SasPortalDeviceGrantStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SasPortalInstallationParamHeightTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how the height is measured.
pub enum SasPortalInstallationParamHeightTypeEnum {
    

    /// Unspecified height type.
    ///
    /// "HEIGHT_TYPE_UNSPECIFIED"
    #[serde(rename="HEIGHT_TYPE_UNSPECIFIED")]
    HEIGHTTYPEUNSPECIFIED,
    

    /// AGL height is measured relative to the ground level.
    ///
    /// "HEIGHT_TYPE_AGL"
    #[serde(rename="HEIGHT_TYPE_AGL")]
    HEIGHTTYPEAGL,
    

    /// AMSL height is measured relative to the mean sea level.
    ///
    /// "HEIGHT_TYPE_AMSL"
    #[serde(rename="HEIGHT_TYPE_AMSL")]
    HEIGHTTYPEAMSL,
}

impl AsRef<str> for SasPortalInstallationParamHeightTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SasPortalInstallationParamHeightTypeEnum::HEIGHTTYPEUNSPECIFIED => "HEIGHT_TYPE_UNSPECIFIED",
            SasPortalInstallationParamHeightTypeEnum::HEIGHTTYPEAGL => "HEIGHT_TYPE_AGL",
            SasPortalInstallationParamHeightTypeEnum::HEIGHTTYPEAMSL => "HEIGHT_TYPE_AMSL",
        }
    }
}

impl std::convert::TryFrom< &str> for SasPortalInstallationParamHeightTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEIGHT_TYPE_UNSPECIFIED" => Ok(SasPortalInstallationParamHeightTypeEnum::HEIGHTTYPEUNSPECIFIED),
           "HEIGHT_TYPE_AGL" => Ok(SasPortalInstallationParamHeightTypeEnum::HEIGHTTYPEAGL),
           "HEIGHT_TYPE_AMSL" => Ok(SasPortalInstallationParamHeightTypeEnum::HEIGHTTYPEAMSL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SasPortalInstallationParamHeightTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SasPortalNrqzValidationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the NRQZ validation info.
pub enum SasPortalNrqzValidationStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Draft state.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Final state.
    ///
    /// "FINAL"
    #[serde(rename="FINAL")]
    FINAL,
}

impl AsRef<str> for SasPortalNrqzValidationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SasPortalNrqzValidationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SasPortalNrqzValidationStateEnum::DRAFT => "DRAFT",
            SasPortalNrqzValidationStateEnum::FINAL => "FINAL",
        }
    }
}

impl std::convert::TryFrom< &str> for SasPortalNrqzValidationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SasPortalNrqzValidationStateEnum::STATEUNSPECIFIED),
           "DRAFT" => Ok(SasPortalNrqzValidationStateEnum::DRAFT),
           "FINAL" => Ok(SasPortalNrqzValidationStateEnum::FINAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SasPortalNrqzValidationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


