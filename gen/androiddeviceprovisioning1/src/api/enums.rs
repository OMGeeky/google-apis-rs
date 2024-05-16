use super::*;



// region ClaimDeviceRequestSectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The section type of the device's provisioning record.
pub enum ClaimDeviceRequestSectionTypeEnum {
    

    /// Unspecified section type.
    ///
    /// "SECTION_TYPE_UNSPECIFIED"
    #[serde(rename="SECTION_TYPE_UNSPECIFIED")]
    SECTIONTYPEUNSPECIFIED,
    

    /// SIM-lock section type.
    ///
    /// "SECTION_TYPE_SIM_LOCK"
    #[serde(rename="SECTION_TYPE_SIM_LOCK")]
    SECTIONTYPESIMLOCK,
    

    /// Zero-touch enrollment section type.
    ///
    /// "SECTION_TYPE_ZERO_TOUCH"
    #[serde(rename="SECTION_TYPE_ZERO_TOUCH")]
    SECTIONTYPEZEROTOUCH,
}

impl AsRef<str> for ClaimDeviceRequestSectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClaimDeviceRequestSectionTypeEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            ClaimDeviceRequestSectionTypeEnum::SECTIONTYPESIMLOCK => "SECTION_TYPE_SIM_LOCK",
            ClaimDeviceRequestSectionTypeEnum::SECTIONTYPEZEROTOUCH => "SECTION_TYPE_ZERO_TOUCH",
        }
    }
}

impl std::convert::TryFrom< &str> for ClaimDeviceRequestSectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(ClaimDeviceRequestSectionTypeEnum::SECTIONTYPEUNSPECIFIED),
           "SECTION_TYPE_SIM_LOCK" => Ok(ClaimDeviceRequestSectionTypeEnum::SECTIONTYPESIMLOCK),
           "SECTION_TYPE_ZERO_TOUCH" => Ok(ClaimDeviceRequestSectionTypeEnum::SECTIONTYPEZEROTOUCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClaimDeviceRequestSectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanyTermsStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Whether any user from the company has accepted the latest Terms of Service (ToS). See TermsStatus.
pub enum CompanyTermsStatusEnum {
    

    /// Default value. This value should never be set if the enum is present.
    ///
    /// "TERMS_STATUS_UNSPECIFIED"
    #[serde(rename="TERMS_STATUS_UNSPECIFIED")]
    TERMSSTATUSUNSPECIFIED,
    

    /// None of the company's users have accepted the ToS.
    ///
    /// "TERMS_STATUS_NOT_ACCEPTED"
    #[serde(rename="TERMS_STATUS_NOT_ACCEPTED")]
    TERMSSTATUSNOTACCEPTED,
    

    /// One (or more) of the company's users has accepted the ToS.
    ///
    /// "TERMS_STATUS_ACCEPTED"
    #[serde(rename="TERMS_STATUS_ACCEPTED")]
    TERMSSTATUSACCEPTED,
    

    /// None of the company's users has accepted the current ToS but at least one user accepted a previous ToS.
    ///
    /// "TERMS_STATUS_STALE"
    #[serde(rename="TERMS_STATUS_STALE")]
    TERMSSTATUSSTALE,
}

impl AsRef<str> for CompanyTermsStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanyTermsStatusEnum::TERMSSTATUSUNSPECIFIED => "TERMS_STATUS_UNSPECIFIED",
            CompanyTermsStatusEnum::TERMSSTATUSNOTACCEPTED => "TERMS_STATUS_NOT_ACCEPTED",
            CompanyTermsStatusEnum::TERMSSTATUSACCEPTED => "TERMS_STATUS_ACCEPTED",
            CompanyTermsStatusEnum::TERMSSTATUSSTALE => "TERMS_STATUS_STALE",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanyTermsStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TERMS_STATUS_UNSPECIFIED" => Ok(CompanyTermsStatusEnum::TERMSSTATUSUNSPECIFIED),
           "TERMS_STATUS_NOT_ACCEPTED" => Ok(CompanyTermsStatusEnum::TERMSSTATUSNOTACCEPTED),
           "TERMS_STATUS_ACCEPTED" => Ok(CompanyTermsStatusEnum::TERMSSTATUSACCEPTED),
           "TERMS_STATUS_STALE" => Ok(CompanyTermsStatusEnum::TERMSSTATUSSTALE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanyTermsStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceClaimAdditionalServiceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Additional service registered for the device.
pub enum DeviceClaimAdditionalServiceEnum {
    

    /// No additional service.
    ///
    /// "ADDITIONAL_SERVICE_UNSPECIFIED"
    #[serde(rename="ADDITIONAL_SERVICE_UNSPECIFIED")]
    ADDITIONALSERVICEUNSPECIFIED,
    

    /// Device protection service, also known as Android Enterprise Essentials. To claim a device with the device protection service you must enroll with the partnership team.
    ///
    /// "DEVICE_PROTECTION"
    #[serde(rename="DEVICE_PROTECTION")]
    DEVICEPROTECTION,
}

impl AsRef<str> for DeviceClaimAdditionalServiceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceClaimAdditionalServiceEnum::ADDITIONALSERVICEUNSPECIFIED => "ADDITIONAL_SERVICE_UNSPECIFIED",
            DeviceClaimAdditionalServiceEnum::DEVICEPROTECTION => "DEVICE_PROTECTION",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceClaimAdditionalServiceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADDITIONAL_SERVICE_UNSPECIFIED" => Ok(DeviceClaimAdditionalServiceEnum::ADDITIONALSERVICEUNSPECIFIED),
           "DEVICE_PROTECTION" => Ok(DeviceClaimAdditionalServiceEnum::DEVICEPROTECTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceClaimAdditionalServiceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceClaimSectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of claim made on the device.
pub enum DeviceClaimSectionTypeEnum {
    

    /// Unspecified section type.
    ///
    /// "SECTION_TYPE_UNSPECIFIED"
    #[serde(rename="SECTION_TYPE_UNSPECIFIED")]
    SECTIONTYPEUNSPECIFIED,
    

    /// SIM-lock section type.
    ///
    /// "SECTION_TYPE_SIM_LOCK"
    #[serde(rename="SECTION_TYPE_SIM_LOCK")]
    SECTIONTYPESIMLOCK,
    

    /// Zero-touch enrollment section type.
    ///
    /// "SECTION_TYPE_ZERO_TOUCH"
    #[serde(rename="SECTION_TYPE_ZERO_TOUCH")]
    SECTIONTYPEZEROTOUCH,
}

impl AsRef<str> for DeviceClaimSectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceClaimSectionTypeEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            DeviceClaimSectionTypeEnum::SECTIONTYPESIMLOCK => "SECTION_TYPE_SIM_LOCK",
            DeviceClaimSectionTypeEnum::SECTIONTYPEZEROTOUCH => "SECTION_TYPE_ZERO_TOUCH",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceClaimSectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(DeviceClaimSectionTypeEnum::SECTIONTYPEUNSPECIFIED),
           "SECTION_TYPE_SIM_LOCK" => Ok(DeviceClaimSectionTypeEnum::SECTIONTYPESIMLOCK),
           "SECTION_TYPE_ZERO_TOUCH" => Ok(DeviceClaimSectionTypeEnum::SECTIONTYPEZEROTOUCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceClaimSectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceIdentifierDeviceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the device
pub enum DeviceIdentifierDeviceTypeEnum {
    

    /// Device type is not specified.
    ///
    /// "DEVICE_TYPE_UNSPECIFIED"
    #[serde(rename="DEVICE_TYPE_UNSPECIFIED")]
    DEVICETYPEUNSPECIFIED,
    

    /// Android device
    ///
    /// "DEVICE_TYPE_ANDROID"
    #[serde(rename="DEVICE_TYPE_ANDROID")]
    DEVICETYPEANDROID,
    

    /// Chrome OS device
    ///
    /// "DEVICE_TYPE_CHROME_OS"
    #[serde(rename="DEVICE_TYPE_CHROME_OS")]
    DEVICETYPECHROMEOS,
}

impl AsRef<str> for DeviceIdentifierDeviceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceIdentifierDeviceTypeEnum::DEVICETYPEUNSPECIFIED => "DEVICE_TYPE_UNSPECIFIED",
            DeviceIdentifierDeviceTypeEnum::DEVICETYPEANDROID => "DEVICE_TYPE_ANDROID",
            DeviceIdentifierDeviceTypeEnum::DEVICETYPECHROMEOS => "DEVICE_TYPE_CHROME_OS",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceIdentifierDeviceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_TYPE_UNSPECIFIED" => Ok(DeviceIdentifierDeviceTypeEnum::DEVICETYPEUNSPECIFIED),
           "DEVICE_TYPE_ANDROID" => Ok(DeviceIdentifierDeviceTypeEnum::DEVICETYPEANDROID),
           "DEVICE_TYPE_CHROME_OS" => Ok(DeviceIdentifierDeviceTypeEnum::DEVICETYPECHROMEOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceIdentifierDeviceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FindDevicesByOwnerRequestSectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The section type of the device's provisioning record.
pub enum FindDevicesByOwnerRequestSectionTypeEnum {
    

    /// Unspecified section type.
    ///
    /// "SECTION_TYPE_UNSPECIFIED"
    #[serde(rename="SECTION_TYPE_UNSPECIFIED")]
    SECTIONTYPEUNSPECIFIED,
    

    /// SIM-lock section type.
    ///
    /// "SECTION_TYPE_SIM_LOCK"
    #[serde(rename="SECTION_TYPE_SIM_LOCK")]
    SECTIONTYPESIMLOCK,
    

    /// Zero-touch enrollment section type.
    ///
    /// "SECTION_TYPE_ZERO_TOUCH"
    #[serde(rename="SECTION_TYPE_ZERO_TOUCH")]
    SECTIONTYPEZEROTOUCH,
}

impl AsRef<str> for FindDevicesByOwnerRequestSectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FindDevicesByOwnerRequestSectionTypeEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            FindDevicesByOwnerRequestSectionTypeEnum::SECTIONTYPESIMLOCK => "SECTION_TYPE_SIM_LOCK",
            FindDevicesByOwnerRequestSectionTypeEnum::SECTIONTYPEZEROTOUCH => "SECTION_TYPE_ZERO_TOUCH",
        }
    }
}

impl std::convert::TryFrom< &str> for FindDevicesByOwnerRequestSectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(FindDevicesByOwnerRequestSectionTypeEnum::SECTIONTYPEUNSPECIFIED),
           "SECTION_TYPE_SIM_LOCK" => Ok(FindDevicesByOwnerRequestSectionTypeEnum::SECTIONTYPESIMLOCK),
           "SECTION_TYPE_ZERO_TOUCH" => Ok(FindDevicesByOwnerRequestSectionTypeEnum::SECTIONTYPEZEROTOUCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FindDevicesByOwnerRequestSectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GetDeviceSimLockStateResponseSimLockStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum GetDeviceSimLockStateResponseSimLockStateEnum {
    

    /// Invalid code. Shouldn't be used.
    ///
    /// "SIM_LOCK_STATE_UNSPECIFIED"
    #[serde(rename="SIM_LOCK_STATE_UNSPECIFIED")]
    SIMLOCKSTATEUNSPECIFIED,
    

    /// Device is not SIM locked.
    ///
    /// "UNLOCKED"
    #[serde(rename="UNLOCKED")]
    UNLOCKED,
    

    /// Device is SIM locked to the partner querying SIM lock state.
    ///
    /// "LOCKED_TO_PARTNER"
    #[serde(rename="LOCKED_TO_PARTNER")]
    LOCKEDTOPARTNER,
    

    /// Device is SIM locked to a different partner.
    ///
    /// "LOCKED_TO_OTHER_PARTNER"
    #[serde(rename="LOCKED_TO_OTHER_PARTNER")]
    LOCKEDTOOTHERPARTNER,
}

impl AsRef<str> for GetDeviceSimLockStateResponseSimLockStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GetDeviceSimLockStateResponseSimLockStateEnum::SIMLOCKSTATEUNSPECIFIED => "SIM_LOCK_STATE_UNSPECIFIED",
            GetDeviceSimLockStateResponseSimLockStateEnum::UNLOCKED => "UNLOCKED",
            GetDeviceSimLockStateResponseSimLockStateEnum::LOCKEDTOPARTNER => "LOCKED_TO_PARTNER",
            GetDeviceSimLockStateResponseSimLockStateEnum::LOCKEDTOOTHERPARTNER => "LOCKED_TO_OTHER_PARTNER",
        }
    }
}

impl std::convert::TryFrom< &str> for GetDeviceSimLockStateResponseSimLockStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SIM_LOCK_STATE_UNSPECIFIED" => Ok(GetDeviceSimLockStateResponseSimLockStateEnum::SIMLOCKSTATEUNSPECIFIED),
           "UNLOCKED" => Ok(GetDeviceSimLockStateResponseSimLockStateEnum::UNLOCKED),
           "LOCKED_TO_PARTNER" => Ok(GetDeviceSimLockStateResponseSimLockStateEnum::LOCKEDTOPARTNER),
           "LOCKED_TO_OTHER_PARTNER" => Ok(GetDeviceSimLockStateResponseSimLockStateEnum::LOCKEDTOOTHERPARTNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GetDeviceSimLockStateResponseSimLockStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartnerClaimSectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The section type of the device's provisioning record.
pub enum PartnerClaimSectionTypeEnum {
    

    /// Unspecified section type.
    ///
    /// "SECTION_TYPE_UNSPECIFIED"
    #[serde(rename="SECTION_TYPE_UNSPECIFIED")]
    SECTIONTYPEUNSPECIFIED,
    

    /// SIM-lock section type.
    ///
    /// "SECTION_TYPE_SIM_LOCK"
    #[serde(rename="SECTION_TYPE_SIM_LOCK")]
    SECTIONTYPESIMLOCK,
    

    /// Zero-touch enrollment section type.
    ///
    /// "SECTION_TYPE_ZERO_TOUCH"
    #[serde(rename="SECTION_TYPE_ZERO_TOUCH")]
    SECTIONTYPEZEROTOUCH,
}

impl AsRef<str> for PartnerClaimSectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartnerClaimSectionTypeEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            PartnerClaimSectionTypeEnum::SECTIONTYPESIMLOCK => "SECTION_TYPE_SIM_LOCK",
            PartnerClaimSectionTypeEnum::SECTIONTYPEZEROTOUCH => "SECTION_TYPE_ZERO_TOUCH",
        }
    }
}

impl std::convert::TryFrom< &str> for PartnerClaimSectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(PartnerClaimSectionTypeEnum::SECTIONTYPEUNSPECIFIED),
           "SECTION_TYPE_SIM_LOCK" => Ok(PartnerClaimSectionTypeEnum::SECTIONTYPESIMLOCK),
           "SECTION_TYPE_ZERO_TOUCH" => Ok(PartnerClaimSectionTypeEnum::SECTIONTYPEZEROTOUCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartnerClaimSectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartnerUnclaimSectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The section type of the device's provisioning record.
pub enum PartnerUnclaimSectionTypeEnum {
    

    /// Unspecified section type.
    ///
    /// "SECTION_TYPE_UNSPECIFIED"
    #[serde(rename="SECTION_TYPE_UNSPECIFIED")]
    SECTIONTYPEUNSPECIFIED,
    

    /// SIM-lock section type.
    ///
    /// "SECTION_TYPE_SIM_LOCK"
    #[serde(rename="SECTION_TYPE_SIM_LOCK")]
    SECTIONTYPESIMLOCK,
    

    /// Zero-touch enrollment section type.
    ///
    /// "SECTION_TYPE_ZERO_TOUCH"
    #[serde(rename="SECTION_TYPE_ZERO_TOUCH")]
    SECTIONTYPEZEROTOUCH,
}

impl AsRef<str> for PartnerUnclaimSectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartnerUnclaimSectionTypeEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            PartnerUnclaimSectionTypeEnum::SECTIONTYPESIMLOCK => "SECTION_TYPE_SIM_LOCK",
            PartnerUnclaimSectionTypeEnum::SECTIONTYPEZEROTOUCH => "SECTION_TYPE_ZERO_TOUCH",
        }
    }
}

impl std::convert::TryFrom< &str> for PartnerUnclaimSectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(PartnerUnclaimSectionTypeEnum::SECTIONTYPEUNSPECIFIED),
           "SECTION_TYPE_SIM_LOCK" => Ok(PartnerUnclaimSectionTypeEnum::SECTIONTYPESIMLOCK),
           "SECTION_TYPE_ZERO_TOUCH" => Ok(PartnerUnclaimSectionTypeEnum::SECTIONTYPEZEROTOUCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartnerUnclaimSectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UnclaimDeviceRequestSectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The section type of the device's provisioning record.
pub enum UnclaimDeviceRequestSectionTypeEnum {
    

    /// Unspecified section type.
    ///
    /// "SECTION_TYPE_UNSPECIFIED"
    #[serde(rename="SECTION_TYPE_UNSPECIFIED")]
    SECTIONTYPEUNSPECIFIED,
    

    /// SIM-lock section type.
    ///
    /// "SECTION_TYPE_SIM_LOCK"
    #[serde(rename="SECTION_TYPE_SIM_LOCK")]
    SECTIONTYPESIMLOCK,
    

    /// Zero-touch enrollment section type.
    ///
    /// "SECTION_TYPE_ZERO_TOUCH"
    #[serde(rename="SECTION_TYPE_ZERO_TOUCH")]
    SECTIONTYPEZEROTOUCH,
}

impl AsRef<str> for UnclaimDeviceRequestSectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UnclaimDeviceRequestSectionTypeEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            UnclaimDeviceRequestSectionTypeEnum::SECTIONTYPESIMLOCK => "SECTION_TYPE_SIM_LOCK",
            UnclaimDeviceRequestSectionTypeEnum::SECTIONTYPEZEROTOUCH => "SECTION_TYPE_ZERO_TOUCH",
        }
    }
}

impl std::convert::TryFrom< &str> for UnclaimDeviceRequestSectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(UnclaimDeviceRequestSectionTypeEnum::SECTIONTYPEUNSPECIFIED),
           "SECTION_TYPE_SIM_LOCK" => Ok(UnclaimDeviceRequestSectionTypeEnum::SECTIONTYPESIMLOCK),
           "SECTION_TYPE_ZERO_TOUCH" => Ok(UnclaimDeviceRequestSectionTypeEnum::SECTIONTYPEZEROTOUCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UnclaimDeviceRequestSectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


