use super::*;



// region AccountAccountProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Profile for this account. This is a read-only field that can be left blank.
pub enum AccountAccountProfileEnum {
    
    /// "ACCOUNT_PROFILE_BASIC"
    #[serde(rename="ACCOUNT_PROFILE_BASIC")]
    ACCOUNTPROFILEBASIC,
    
    /// "ACCOUNT_PROFILE_STANDARD"
    #[serde(rename="ACCOUNT_PROFILE_STANDARD")]
    ACCOUNTPROFILESTANDARD,
}

impl AsRef<str> for AccountAccountProfileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountAccountProfileEnum::ACCOUNTPROFILEBASIC => "ACCOUNT_PROFILE_BASIC",
            AccountAccountProfileEnum::ACCOUNTPROFILESTANDARD => "ACCOUNT_PROFILE_STANDARD",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountAccountProfileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCOUNT_PROFILE_BASIC" => Ok(AccountAccountProfileEnum::ACCOUNTPROFILEBASIC),
           "ACCOUNT_PROFILE_STANDARD" => Ok(AccountAccountProfileEnum::ACCOUNTPROFILESTANDARD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountAccountProfileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountActiveAdsLimitTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Maximum number of active ads allowed for this account.
pub enum AccountActiveAdsLimitTierEnum {
    
    /// "ACTIVE_ADS_TIER_40K"
    #[serde(rename="ACTIVE_ADS_TIER_40K")]
    ACTIVEADSTIER40K,
    
    /// "ACTIVE_ADS_TIER_75K"
    #[serde(rename="ACTIVE_ADS_TIER_75K")]
    ACTIVEADSTIER75K,
    
    /// "ACTIVE_ADS_TIER_100K"
    #[serde(rename="ACTIVE_ADS_TIER_100K")]
    ACTIVEADSTIER100K,
    
    /// "ACTIVE_ADS_TIER_200K"
    #[serde(rename="ACTIVE_ADS_TIER_200K")]
    ACTIVEADSTIER200K,
    
    /// "ACTIVE_ADS_TIER_300K"
    #[serde(rename="ACTIVE_ADS_TIER_300K")]
    ACTIVEADSTIER300K,
    
    /// "ACTIVE_ADS_TIER_500K"
    #[serde(rename="ACTIVE_ADS_TIER_500K")]
    ACTIVEADSTIER500K,
    
    /// "ACTIVE_ADS_TIER_750K"
    #[serde(rename="ACTIVE_ADS_TIER_750K")]
    ACTIVEADSTIER750K,
    
    /// "ACTIVE_ADS_TIER_1M"
    #[serde(rename="ACTIVE_ADS_TIER_1M")]
    ACTIVEADSTIER1M,
}

impl AsRef<str> for AccountActiveAdsLimitTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountActiveAdsLimitTierEnum::ACTIVEADSTIER40K => "ACTIVE_ADS_TIER_40K",
            AccountActiveAdsLimitTierEnum::ACTIVEADSTIER75K => "ACTIVE_ADS_TIER_75K",
            AccountActiveAdsLimitTierEnum::ACTIVEADSTIER100K => "ACTIVE_ADS_TIER_100K",
            AccountActiveAdsLimitTierEnum::ACTIVEADSTIER200K => "ACTIVE_ADS_TIER_200K",
            AccountActiveAdsLimitTierEnum::ACTIVEADSTIER300K => "ACTIVE_ADS_TIER_300K",
            AccountActiveAdsLimitTierEnum::ACTIVEADSTIER500K => "ACTIVE_ADS_TIER_500K",
            AccountActiveAdsLimitTierEnum::ACTIVEADSTIER750K => "ACTIVE_ADS_TIER_750K",
            AccountActiveAdsLimitTierEnum::ACTIVEADSTIER1M => "ACTIVE_ADS_TIER_1M",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountActiveAdsLimitTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTIVE_ADS_TIER_40K" => Ok(AccountActiveAdsLimitTierEnum::ACTIVEADSTIER40K),
           "ACTIVE_ADS_TIER_75K" => Ok(AccountActiveAdsLimitTierEnum::ACTIVEADSTIER75K),
           "ACTIVE_ADS_TIER_100K" => Ok(AccountActiveAdsLimitTierEnum::ACTIVEADSTIER100K),
           "ACTIVE_ADS_TIER_200K" => Ok(AccountActiveAdsLimitTierEnum::ACTIVEADSTIER200K),
           "ACTIVE_ADS_TIER_300K" => Ok(AccountActiveAdsLimitTierEnum::ACTIVEADSTIER300K),
           "ACTIVE_ADS_TIER_500K" => Ok(AccountActiveAdsLimitTierEnum::ACTIVEADSTIER500K),
           "ACTIVE_ADS_TIER_750K" => Ok(AccountActiveAdsLimitTierEnum::ACTIVEADSTIER750K),
           "ACTIVE_ADS_TIER_1M" => Ok(AccountActiveAdsLimitTierEnum::ACTIVEADSTIER1M),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountActiveAdsLimitTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountActiveAdSummaryActiveAdsLimitTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Maximum number of active ads allowed for the account.
pub enum AccountActiveAdSummaryActiveAdsLimitTierEnum {
    
    /// "ACTIVE_ADS_TIER_40K"
    #[serde(rename="ACTIVE_ADS_TIER_40K")]
    ACTIVEADSTIER40K,
    
    /// "ACTIVE_ADS_TIER_75K"
    #[serde(rename="ACTIVE_ADS_TIER_75K")]
    ACTIVEADSTIER75K,
    
    /// "ACTIVE_ADS_TIER_100K"
    #[serde(rename="ACTIVE_ADS_TIER_100K")]
    ACTIVEADSTIER100K,
    
    /// "ACTIVE_ADS_TIER_200K"
    #[serde(rename="ACTIVE_ADS_TIER_200K")]
    ACTIVEADSTIER200K,
    
    /// "ACTIVE_ADS_TIER_300K"
    #[serde(rename="ACTIVE_ADS_TIER_300K")]
    ACTIVEADSTIER300K,
    
    /// "ACTIVE_ADS_TIER_500K"
    #[serde(rename="ACTIVE_ADS_TIER_500K")]
    ACTIVEADSTIER500K,
    
    /// "ACTIVE_ADS_TIER_750K"
    #[serde(rename="ACTIVE_ADS_TIER_750K")]
    ACTIVEADSTIER750K,
    
    /// "ACTIVE_ADS_TIER_1M"
    #[serde(rename="ACTIVE_ADS_TIER_1M")]
    ACTIVEADSTIER1M,
}

impl AsRef<str> for AccountActiveAdSummaryActiveAdsLimitTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER40K => "ACTIVE_ADS_TIER_40K",
            AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER75K => "ACTIVE_ADS_TIER_75K",
            AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER100K => "ACTIVE_ADS_TIER_100K",
            AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER200K => "ACTIVE_ADS_TIER_200K",
            AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER300K => "ACTIVE_ADS_TIER_300K",
            AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER500K => "ACTIVE_ADS_TIER_500K",
            AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER750K => "ACTIVE_ADS_TIER_750K",
            AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER1M => "ACTIVE_ADS_TIER_1M",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountActiveAdSummaryActiveAdsLimitTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTIVE_ADS_TIER_40K" => Ok(AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER40K),
           "ACTIVE_ADS_TIER_75K" => Ok(AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER75K),
           "ACTIVE_ADS_TIER_100K" => Ok(AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER100K),
           "ACTIVE_ADS_TIER_200K" => Ok(AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER200K),
           "ACTIVE_ADS_TIER_300K" => Ok(AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER300K),
           "ACTIVE_ADS_TIER_500K" => Ok(AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER500K),
           "ACTIVE_ADS_TIER_750K" => Ok(AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER750K),
           "ACTIVE_ADS_TIER_1M" => Ok(AccountActiveAdSummaryActiveAdsLimitTierEnum::ACTIVEADSTIER1M),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountActiveAdSummaryActiveAdsLimitTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountPermissionAccountProfilesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Account profiles associated with this account permission. Possible values are: - "ACCOUNT_PROFILE_BASIC" - "ACCOUNT_PROFILE_STANDARD" 
pub enum AccountPermissionAccountProfilesEnum {
    
    /// "ACCOUNT_PROFILE_BASIC"
    #[serde(rename="ACCOUNT_PROFILE_BASIC")]
    ACCOUNTPROFILEBASIC,
    
    /// "ACCOUNT_PROFILE_STANDARD"
    #[serde(rename="ACCOUNT_PROFILE_STANDARD")]
    ACCOUNTPROFILESTANDARD,
}

impl AsRef<str> for AccountPermissionAccountProfilesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountPermissionAccountProfilesEnum::ACCOUNTPROFILEBASIC => "ACCOUNT_PROFILE_BASIC",
            AccountPermissionAccountProfilesEnum::ACCOUNTPROFILESTANDARD => "ACCOUNT_PROFILE_STANDARD",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountPermissionAccountProfilesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCOUNT_PROFILE_BASIC" => Ok(AccountPermissionAccountProfilesEnum::ACCOUNTPROFILEBASIC),
           "ACCOUNT_PROFILE_STANDARD" => Ok(AccountPermissionAccountProfilesEnum::ACCOUNTPROFILESTANDARD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountPermissionAccountProfilesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountPermissionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Administrative level required to enable this account permission.
pub enum AccountPermissionLevelEnum {
    
    /// "USER"
    #[serde(rename="USER")]
    USER,
    
    /// "ADMINISTRATOR"
    #[serde(rename="ADMINISTRATOR")]
    ADMINISTRATOR,
}

impl AsRef<str> for AccountPermissionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountPermissionLevelEnum::USER => "USER",
            AccountPermissionLevelEnum::ADMINISTRATOR => "ADMINISTRATOR",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountPermissionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER" => Ok(AccountPermissionLevelEnum::USER),
           "ADMINISTRATOR" => Ok(AccountPermissionLevelEnum::ADMINISTRATOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountPermissionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountUserProfileTraffickerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Trafficker type of this user profile. This is a read-only field.
pub enum AccountUserProfileTraffickerTypeEnum {
    
    /// "INTERNAL_NON_TRAFFICKER"
    #[serde(rename="INTERNAL_NON_TRAFFICKER")]
    INTERNALNONTRAFFICKER,
    
    /// "INTERNAL_TRAFFICKER"
    #[serde(rename="INTERNAL_TRAFFICKER")]
    INTERNALTRAFFICKER,
    
    /// "EXTERNAL_TRAFFICKER"
    #[serde(rename="EXTERNAL_TRAFFICKER")]
    EXTERNALTRAFFICKER,
}

impl AsRef<str> for AccountUserProfileTraffickerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountUserProfileTraffickerTypeEnum::INTERNALNONTRAFFICKER => "INTERNAL_NON_TRAFFICKER",
            AccountUserProfileTraffickerTypeEnum::INTERNALTRAFFICKER => "INTERNAL_TRAFFICKER",
            AccountUserProfileTraffickerTypeEnum::EXTERNALTRAFFICKER => "EXTERNAL_TRAFFICKER",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountUserProfileTraffickerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTERNAL_NON_TRAFFICKER" => Ok(AccountUserProfileTraffickerTypeEnum::INTERNALNONTRAFFICKER),
           "INTERNAL_TRAFFICKER" => Ok(AccountUserProfileTraffickerTypeEnum::INTERNALTRAFFICKER),
           "EXTERNAL_TRAFFICKER" => Ok(AccountUserProfileTraffickerTypeEnum::EXTERNALTRAFFICKER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountUserProfileTraffickerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountUserProfileUserAccessTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// User type of the user profile. This is a read-only field that can be left blank.
pub enum AccountUserProfileUserAccessTypeEnum {
    
    /// "NORMAL_USER"
    #[serde(rename="NORMAL_USER")]
    NORMALUSER,
    
    /// "SUPER_USER"
    #[serde(rename="SUPER_USER")]
    SUPERUSER,
    
    /// "INTERNAL_ADMINISTRATOR"
    #[serde(rename="INTERNAL_ADMINISTRATOR")]
    INTERNALADMINISTRATOR,
    
    /// "READ_ONLY_SUPER_USER"
    #[serde(rename="READ_ONLY_SUPER_USER")]
    READONLYSUPERUSER,
}

impl AsRef<str> for AccountUserProfileUserAccessTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountUserProfileUserAccessTypeEnum::NORMALUSER => "NORMAL_USER",
            AccountUserProfileUserAccessTypeEnum::SUPERUSER => "SUPER_USER",
            AccountUserProfileUserAccessTypeEnum::INTERNALADMINISTRATOR => "INTERNAL_ADMINISTRATOR",
            AccountUserProfileUserAccessTypeEnum::READONLYSUPERUSER => "READ_ONLY_SUPER_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountUserProfileUserAccessTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NORMAL_USER" => Ok(AccountUserProfileUserAccessTypeEnum::NORMALUSER),
           "SUPER_USER" => Ok(AccountUserProfileUserAccessTypeEnum::SUPERUSER),
           "INTERNAL_ADMINISTRATOR" => Ok(AccountUserProfileUserAccessTypeEnum::INTERNALADMINISTRATOR),
           "READ_ONLY_SUPER_USER" => Ok(AccountUserProfileUserAccessTypeEnum::READONLYSUPERUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountUserProfileUserAccessTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdCompatibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Compatibility of this ad. Applicable when type is AD_SERVING_DEFAULT_AD. DISPLAY and DISPLAY_INTERSTITIAL refer to either rendering on desktop or on mobile devices or in mobile apps for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are only used for existing default ads. New mobile placements must be assigned DISPLAY or DISPLAY_INTERSTITIAL and default ads created for those placements will be limited to those compatibility types. IN_STREAM_VIDEO refers to rendering in-stream video ads developed with the VAST standard.
pub enum AdCompatibilityEnum {
    
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    
    /// "DISPLAY_INTERSTITIAL"
    #[serde(rename="DISPLAY_INTERSTITIAL")]
    DISPLAYINTERSTITIAL,
    
    /// "APP"
    #[serde(rename="APP")]
    APP,
    
    /// "APP_INTERSTITIAL"
    #[serde(rename="APP_INTERSTITIAL")]
    APPINTERSTITIAL,
    
    /// "IN_STREAM_VIDEO"
    #[serde(rename="IN_STREAM_VIDEO")]
    INSTREAMVIDEO,
    
    /// "IN_STREAM_AUDIO"
    #[serde(rename="IN_STREAM_AUDIO")]
    INSTREAMAUDIO,
}

impl AsRef<str> for AdCompatibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdCompatibilityEnum::DISPLAY => "DISPLAY",
            AdCompatibilityEnum::DISPLAYINTERSTITIAL => "DISPLAY_INTERSTITIAL",
            AdCompatibilityEnum::APP => "APP",
            AdCompatibilityEnum::APPINTERSTITIAL => "APP_INTERSTITIAL",
            AdCompatibilityEnum::INSTREAMVIDEO => "IN_STREAM_VIDEO",
            AdCompatibilityEnum::INSTREAMAUDIO => "IN_STREAM_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for AdCompatibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISPLAY" => Ok(AdCompatibilityEnum::DISPLAY),
           "DISPLAY_INTERSTITIAL" => Ok(AdCompatibilityEnum::DISPLAYINTERSTITIAL),
           "APP" => Ok(AdCompatibilityEnum::APP),
           "APP_INTERSTITIAL" => Ok(AdCompatibilityEnum::APPINTERSTITIAL),
           "IN_STREAM_VIDEO" => Ok(AdCompatibilityEnum::INSTREAMVIDEO),
           "IN_STREAM_AUDIO" => Ok(AdCompatibilityEnum::INSTREAMAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdCompatibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of ad. This is a required field on insertion. Note that default ads ( AD_SERVING_DEFAULT_AD) cannot be created directly (see Creative resource).
pub enum AdTypeEnum {
    
    /// "AD_SERVING_STANDARD_AD"
    #[serde(rename="AD_SERVING_STANDARD_AD")]
    ADSERVINGSTANDARDAD,
    
    /// "AD_SERVING_DEFAULT_AD"
    #[serde(rename="AD_SERVING_DEFAULT_AD")]
    ADSERVINGDEFAULTAD,
    
    /// "AD_SERVING_CLICK_TRACKER"
    #[serde(rename="AD_SERVING_CLICK_TRACKER")]
    ADSERVINGCLICKTRACKER,
    
    /// "AD_SERVING_TRACKING"
    #[serde(rename="AD_SERVING_TRACKING")]
    ADSERVINGTRACKING,
    
    /// "AD_SERVING_BRAND_SAFE_AD"
    #[serde(rename="AD_SERVING_BRAND_SAFE_AD")]
    ADSERVINGBRANDSAFEAD,
}

impl AsRef<str> for AdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdTypeEnum::ADSERVINGSTANDARDAD => "AD_SERVING_STANDARD_AD",
            AdTypeEnum::ADSERVINGDEFAULTAD => "AD_SERVING_DEFAULT_AD",
            AdTypeEnum::ADSERVINGCLICKTRACKER => "AD_SERVING_CLICK_TRACKER",
            AdTypeEnum::ADSERVINGTRACKING => "AD_SERVING_TRACKING",
            AdTypeEnum::ADSERVINGBRANDSAFEAD => "AD_SERVING_BRAND_SAFE_AD",
        }
    }
}

impl std::convert::TryFrom< &str> for AdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AD_SERVING_STANDARD_AD" => Ok(AdTypeEnum::ADSERVINGSTANDARDAD),
           "AD_SERVING_DEFAULT_AD" => Ok(AdTypeEnum::ADSERVINGDEFAULTAD),
           "AD_SERVING_CLICK_TRACKER" => Ok(AdTypeEnum::ADSERVINGCLICKTRACKER),
           "AD_SERVING_TRACKING" => Ok(AdTypeEnum::ADSERVINGTRACKING),
           "AD_SERVING_BRAND_SAFE_AD" => Ok(AdTypeEnum::ADSERVINGBRANDSAFEAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdSlotCompatibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Ad slot compatibility. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop, mobile devices or in mobile apps for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard.
pub enum AdSlotCompatibilityEnum {
    
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    
    /// "DISPLAY_INTERSTITIAL"
    #[serde(rename="DISPLAY_INTERSTITIAL")]
    DISPLAYINTERSTITIAL,
    
    /// "APP"
    #[serde(rename="APP")]
    APP,
    
    /// "APP_INTERSTITIAL"
    #[serde(rename="APP_INTERSTITIAL")]
    APPINTERSTITIAL,
    
    /// "IN_STREAM_VIDEO"
    #[serde(rename="IN_STREAM_VIDEO")]
    INSTREAMVIDEO,
    
    /// "IN_STREAM_AUDIO"
    #[serde(rename="IN_STREAM_AUDIO")]
    INSTREAMAUDIO,
}

impl AsRef<str> for AdSlotCompatibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdSlotCompatibilityEnum::DISPLAY => "DISPLAY",
            AdSlotCompatibilityEnum::DISPLAYINTERSTITIAL => "DISPLAY_INTERSTITIAL",
            AdSlotCompatibilityEnum::APP => "APP",
            AdSlotCompatibilityEnum::APPINTERSTITIAL => "APP_INTERSTITIAL",
            AdSlotCompatibilityEnum::INSTREAMVIDEO => "IN_STREAM_VIDEO",
            AdSlotCompatibilityEnum::INSTREAMAUDIO => "IN_STREAM_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for AdSlotCompatibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISPLAY" => Ok(AdSlotCompatibilityEnum::DISPLAY),
           "DISPLAY_INTERSTITIAL" => Ok(AdSlotCompatibilityEnum::DISPLAYINTERSTITIAL),
           "APP" => Ok(AdSlotCompatibilityEnum::APP),
           "APP_INTERSTITIAL" => Ok(AdSlotCompatibilityEnum::APPINTERSTITIAL),
           "IN_STREAM_VIDEO" => Ok(AdSlotCompatibilityEnum::INSTREAMVIDEO),
           "IN_STREAM_AUDIO" => Ok(AdSlotCompatibilityEnum::INSTREAMAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdSlotCompatibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdSlotPaymentSourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Payment source type of this ad slot.
pub enum AdSlotPaymentSourceTypeEnum {
    
    /// "PLANNING_PAYMENT_SOURCE_TYPE_AGENCY_PAID"
    #[serde(rename="PLANNING_PAYMENT_SOURCE_TYPE_AGENCY_PAID")]
    PLANNINGPAYMENTSOURCETYPEAGENCYPAID,
    
    /// "PLANNING_PAYMENT_SOURCE_TYPE_PUBLISHER_PAID"
    #[serde(rename="PLANNING_PAYMENT_SOURCE_TYPE_PUBLISHER_PAID")]
    PLANNINGPAYMENTSOURCETYPEPUBLISHERPAID,
}

impl AsRef<str> for AdSlotPaymentSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdSlotPaymentSourceTypeEnum::PLANNINGPAYMENTSOURCETYPEAGENCYPAID => "PLANNING_PAYMENT_SOURCE_TYPE_AGENCY_PAID",
            AdSlotPaymentSourceTypeEnum::PLANNINGPAYMENTSOURCETYPEPUBLISHERPAID => "PLANNING_PAYMENT_SOURCE_TYPE_PUBLISHER_PAID",
        }
    }
}

impl std::convert::TryFrom< &str> for AdSlotPaymentSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLANNING_PAYMENT_SOURCE_TYPE_AGENCY_PAID" => Ok(AdSlotPaymentSourceTypeEnum::PLANNINGPAYMENTSOURCETYPEAGENCYPAID),
           "PLANNING_PAYMENT_SOURCE_TYPE_PUBLISHER_PAID" => Ok(AdSlotPaymentSourceTypeEnum::PLANNINGPAYMENTSOURCETYPEPUBLISHERPAID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdSlotPaymentSourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdvertiserStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of this advertiser.
pub enum AdvertiserStatusEnum {
    
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    
    /// "ON_HOLD"
    #[serde(rename="ON_HOLD")]
    ONHOLD,
}

impl AsRef<str> for AdvertiserStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserStatusEnum::APPROVED => "APPROVED",
            AdvertiserStatusEnum::ONHOLD => "ON_HOLD",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APPROVED" => Ok(AdvertiserStatusEnum::APPROVED),
           "ON_HOLD" => Ok(AdvertiserStatusEnum::ONHOLD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConversionErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The error code.
pub enum ConversionErrorCodeEnum {
    
    /// "INVALID_ARGUMENT"
    #[serde(rename="INVALID_ARGUMENT")]
    INVALIDARGUMENT,
    
    /// "INTERNAL"
    #[serde(rename="INTERNAL")]
    INTERNAL,
    
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
}

impl AsRef<str> for ConversionErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConversionErrorCodeEnum::INVALIDARGUMENT => "INVALID_ARGUMENT",
            ConversionErrorCodeEnum::INTERNAL => "INTERNAL",
            ConversionErrorCodeEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            ConversionErrorCodeEnum::NOTFOUND => "NOT_FOUND",
        }
    }
}

impl std::convert::TryFrom< &str> for ConversionErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVALID_ARGUMENT" => Ok(ConversionErrorCodeEnum::INVALIDARGUMENT),
           "INTERNAL" => Ok(ConversionErrorCodeEnum::INTERNAL),
           "PERMISSION_DENIED" => Ok(ConversionErrorCodeEnum::PERMISSIONDENIED),
           "NOT_FOUND" => Ok(ConversionErrorCodeEnum::NOTFOUND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConversionErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeArtworkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of artwork used for the creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
pub enum CreativeArtworkTypeEnum {
    
    /// "ARTWORK_TYPE_FLASH"
    #[serde(rename="ARTWORK_TYPE_FLASH")]
    ARTWORKTYPEFLASH,
    
    /// "ARTWORK_TYPE_HTML5"
    #[serde(rename="ARTWORK_TYPE_HTML5")]
    ARTWORKTYPEHTML5,
    
    /// "ARTWORK_TYPE_MIXED"
    #[serde(rename="ARTWORK_TYPE_MIXED")]
    ARTWORKTYPEMIXED,
    
    /// "ARTWORK_TYPE_IMAGE"
    #[serde(rename="ARTWORK_TYPE_IMAGE")]
    ARTWORKTYPEIMAGE,
}

impl AsRef<str> for CreativeArtworkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeArtworkTypeEnum::ARTWORKTYPEFLASH => "ARTWORK_TYPE_FLASH",
            CreativeArtworkTypeEnum::ARTWORKTYPEHTML5 => "ARTWORK_TYPE_HTML5",
            CreativeArtworkTypeEnum::ARTWORKTYPEMIXED => "ARTWORK_TYPE_MIXED",
            CreativeArtworkTypeEnum::ARTWORKTYPEIMAGE => "ARTWORK_TYPE_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeArtworkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ARTWORK_TYPE_FLASH" => Ok(CreativeArtworkTypeEnum::ARTWORKTYPEFLASH),
           "ARTWORK_TYPE_HTML5" => Ok(CreativeArtworkTypeEnum::ARTWORKTYPEHTML5),
           "ARTWORK_TYPE_MIXED" => Ok(CreativeArtworkTypeEnum::ARTWORKTYPEMIXED),
           "ARTWORK_TYPE_IMAGE" => Ok(CreativeArtworkTypeEnum::ARTWORKTYPEIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeArtworkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAuthoringSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Source application where creative was authored. Presently, only DBM authored creatives will have this field set. Applicable to all creative types.
pub enum CreativeAuthoringSourceEnum {
    
    /// "CREATIVE_AUTHORING_SOURCE_DCM"
    #[serde(rename="CREATIVE_AUTHORING_SOURCE_DCM")]
    CREATIVEAUTHORINGSOURCEDCM,
    
    /// "CREATIVE_AUTHORING_SOURCE_DBM"
    #[serde(rename="CREATIVE_AUTHORING_SOURCE_DBM")]
    CREATIVEAUTHORINGSOURCEDBM,
    
    /// "CREATIVE_AUTHORING_SOURCE_STUDIO"
    #[serde(rename="CREATIVE_AUTHORING_SOURCE_STUDIO")]
    CREATIVEAUTHORINGSOURCESTUDIO,
    
    /// "CREATIVE_AUTHORING_SOURCE_GWD"
    #[serde(rename="CREATIVE_AUTHORING_SOURCE_GWD")]
    CREATIVEAUTHORINGSOURCEGWD,
}

impl AsRef<str> for CreativeAuthoringSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAuthoringSourceEnum::CREATIVEAUTHORINGSOURCEDCM => "CREATIVE_AUTHORING_SOURCE_DCM",
            CreativeAuthoringSourceEnum::CREATIVEAUTHORINGSOURCEDBM => "CREATIVE_AUTHORING_SOURCE_DBM",
            CreativeAuthoringSourceEnum::CREATIVEAUTHORINGSOURCESTUDIO => "CREATIVE_AUTHORING_SOURCE_STUDIO",
            CreativeAuthoringSourceEnum::CREATIVEAUTHORINGSOURCEGWD => "CREATIVE_AUTHORING_SOURCE_GWD",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAuthoringSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_AUTHORING_SOURCE_DCM" => Ok(CreativeAuthoringSourceEnum::CREATIVEAUTHORINGSOURCEDCM),
           "CREATIVE_AUTHORING_SOURCE_DBM" => Ok(CreativeAuthoringSourceEnum::CREATIVEAUTHORINGSOURCEDBM),
           "CREATIVE_AUTHORING_SOURCE_STUDIO" => Ok(CreativeAuthoringSourceEnum::CREATIVEAUTHORINGSOURCESTUDIO),
           "CREATIVE_AUTHORING_SOURCE_GWD" => Ok(CreativeAuthoringSourceEnum::CREATIVEAUTHORINGSOURCEGWD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAuthoringSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAuthoringToolEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Authoring tool for HTML5 banner creatives. This is a read-only field. Applicable to the following creative types: HTML5_BANNER.
pub enum CreativeAuthoringToolEnum {
    
    /// "NINJA"
    #[serde(rename="NINJA")]
    NINJA,
    
    /// "SWIFFY"
    #[serde(rename="SWIFFY")]
    SWIFFY,
}

impl AsRef<str> for CreativeAuthoringToolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAuthoringToolEnum::NINJA => "NINJA",
            CreativeAuthoringToolEnum::SWIFFY => "SWIFFY",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAuthoringToolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NINJA" => Ok(CreativeAuthoringToolEnum::NINJA),
           "SWIFFY" => Ok(CreativeAuthoringToolEnum::SWIFFY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAuthoringToolEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeBackupImageFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of feature dependencies that will cause a backup image to be served if the browser that serves the ad does not support them. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative asset correctly. This field is initially auto-generated to contain all features detected by Campaign Manager for all the assets of this creative and can then be modified by the client. To reset this field, copy over all the creativeAssets' detected features. Applicable to the following creative types: HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
pub enum CreativeBackupImageFeaturesEnum {
    
    /// "CSS_FONT_FACE"
    #[serde(rename="CSS_FONT_FACE")]
    CSSFONTFACE,
    
    /// "CSS_BACKGROUND_SIZE"
    #[serde(rename="CSS_BACKGROUND_SIZE")]
    CSSBACKGROUNDSIZE,
    
    /// "CSS_BORDER_IMAGE"
    #[serde(rename="CSS_BORDER_IMAGE")]
    CSSBORDERIMAGE,
    
    /// "CSS_BORDER_RADIUS"
    #[serde(rename="CSS_BORDER_RADIUS")]
    CSSBORDERRADIUS,
    
    /// "CSS_BOX_SHADOW"
    #[serde(rename="CSS_BOX_SHADOW")]
    CSSBOXSHADOW,
    
    /// "CSS_FLEX_BOX"
    #[serde(rename="CSS_FLEX_BOX")]
    CSSFLEXBOX,
    
    /// "CSS_HSLA"
    #[serde(rename="CSS_HSLA")]
    CSSHSLA,
    
    /// "CSS_MULTIPLE_BGS"
    #[serde(rename="CSS_MULTIPLE_BGS")]
    CSSMULTIPLEBGS,
    
    /// "CSS_OPACITY"
    #[serde(rename="CSS_OPACITY")]
    CSSOPACITY,
    
    /// "CSS_RGBA"
    #[serde(rename="CSS_RGBA")]
    CSSRGBA,
    
    /// "CSS_TEXT_SHADOW"
    #[serde(rename="CSS_TEXT_SHADOW")]
    CSSTEXTSHADOW,
    
    /// "CSS_ANIMATIONS"
    #[serde(rename="CSS_ANIMATIONS")]
    CSSANIMATIONS,
    
    /// "CSS_COLUMNS"
    #[serde(rename="CSS_COLUMNS")]
    CSSCOLUMNS,
    
    /// "CSS_GENERATED_CONTENT"
    #[serde(rename="CSS_GENERATED_CONTENT")]
    CSSGENERATEDCONTENT,
    
    /// "CSS_GRADIENTS"
    #[serde(rename="CSS_GRADIENTS")]
    CSSGRADIENTS,
    
    /// "CSS_REFLECTIONS"
    #[serde(rename="CSS_REFLECTIONS")]
    CSSREFLECTIONS,
    
    /// "CSS_TRANSFORMS"
    #[serde(rename="CSS_TRANSFORMS")]
    CSSTRANSFORMS,
    
    /// "CSS_TRANSFORMS3D"
    #[serde(rename="CSS_TRANSFORMS3D")]
    CSSTRANSFORMS3D,
    
    /// "CSS_TRANSITIONS"
    #[serde(rename="CSS_TRANSITIONS")]
    CSSTRANSITIONS,
    
    /// "APPLICATION_CACHE"
    #[serde(rename="APPLICATION_CACHE")]
    APPLICATIONCACHE,
    
    /// "CANVAS"
    #[serde(rename="CANVAS")]
    CANVAS,
    
    /// "CANVAS_TEXT"
    #[serde(rename="CANVAS_TEXT")]
    CANVASTEXT,
    
    /// "DRAG_AND_DROP"
    #[serde(rename="DRAG_AND_DROP")]
    DRAGANDDROP,
    
    /// "HASH_CHANGE"
    #[serde(rename="HASH_CHANGE")]
    HASHCHANGE,
    
    /// "HISTORY"
    #[serde(rename="HISTORY")]
    HISTORY,
    
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
    
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    
    /// "INDEXED_DB"
    #[serde(rename="INDEXED_DB")]
    INDEXEDDB,
    
    /// "INPUT_ATTR_AUTOCOMPLETE"
    #[serde(rename="INPUT_ATTR_AUTOCOMPLETE")]
    INPUTATTRAUTOCOMPLETE,
    
    /// "INPUT_ATTR_AUTOFOCUS"
    #[serde(rename="INPUT_ATTR_AUTOFOCUS")]
    INPUTATTRAUTOFOCUS,
    
    /// "INPUT_ATTR_LIST"
    #[serde(rename="INPUT_ATTR_LIST")]
    INPUTATTRLIST,
    
    /// "INPUT_ATTR_PLACEHOLDER"
    #[serde(rename="INPUT_ATTR_PLACEHOLDER")]
    INPUTATTRPLACEHOLDER,
    
    /// "INPUT_ATTR_MAX"
    #[serde(rename="INPUT_ATTR_MAX")]
    INPUTATTRMAX,
    
    /// "INPUT_ATTR_MIN"
    #[serde(rename="INPUT_ATTR_MIN")]
    INPUTATTRMIN,
    
    /// "INPUT_ATTR_MULTIPLE"
    #[serde(rename="INPUT_ATTR_MULTIPLE")]
    INPUTATTRMULTIPLE,
    
    /// "INPUT_ATTR_PATTERN"
    #[serde(rename="INPUT_ATTR_PATTERN")]
    INPUTATTRPATTERN,
    
    /// "INPUT_ATTR_REQUIRED"
    #[serde(rename="INPUT_ATTR_REQUIRED")]
    INPUTATTRREQUIRED,
    
    /// "INPUT_ATTR_STEP"
    #[serde(rename="INPUT_ATTR_STEP")]
    INPUTATTRSTEP,
    
    /// "INPUT_TYPE_SEARCH"
    #[serde(rename="INPUT_TYPE_SEARCH")]
    INPUTTYPESEARCH,
    
    /// "INPUT_TYPE_TEL"
    #[serde(rename="INPUT_TYPE_TEL")]
    INPUTTYPETEL,
    
    /// "INPUT_TYPE_URL"
    #[serde(rename="INPUT_TYPE_URL")]
    INPUTTYPEURL,
    
    /// "INPUT_TYPE_EMAIL"
    #[serde(rename="INPUT_TYPE_EMAIL")]
    INPUTTYPEEMAIL,
    
    /// "INPUT_TYPE_DATETIME"
    #[serde(rename="INPUT_TYPE_DATETIME")]
    INPUTTYPEDATETIME,
    
    /// "INPUT_TYPE_DATE"
    #[serde(rename="INPUT_TYPE_DATE")]
    INPUTTYPEDATE,
    
    /// "INPUT_TYPE_MONTH"
    #[serde(rename="INPUT_TYPE_MONTH")]
    INPUTTYPEMONTH,
    
    /// "INPUT_TYPE_WEEK"
    #[serde(rename="INPUT_TYPE_WEEK")]
    INPUTTYPEWEEK,
    
    /// "INPUT_TYPE_TIME"
    #[serde(rename="INPUT_TYPE_TIME")]
    INPUTTYPETIME,
    
    /// "INPUT_TYPE_DATETIME_LOCAL"
    #[serde(rename="INPUT_TYPE_DATETIME_LOCAL")]
    INPUTTYPEDATETIMELOCAL,
    
    /// "INPUT_TYPE_NUMBER"
    #[serde(rename="INPUT_TYPE_NUMBER")]
    INPUTTYPENUMBER,
    
    /// "INPUT_TYPE_RANGE"
    #[serde(rename="INPUT_TYPE_RANGE")]
    INPUTTYPERANGE,
    
    /// "INPUT_TYPE_COLOR"
    #[serde(rename="INPUT_TYPE_COLOR")]
    INPUTTYPECOLOR,
    
    /// "LOCAL_STORAGE"
    #[serde(rename="LOCAL_STORAGE")]
    LOCALSTORAGE,
    
    /// "POST_MESSAGE"
    #[serde(rename="POST_MESSAGE")]
    POSTMESSAGE,
    
    /// "SESSION_STORAGE"
    #[serde(rename="SESSION_STORAGE")]
    SESSIONSTORAGE,
    
    /// "WEB_SOCKETS"
    #[serde(rename="WEB_SOCKETS")]
    WEBSOCKETS,
    
    /// "WEB_SQL_DATABASE"
    #[serde(rename="WEB_SQL_DATABASE")]
    WEBSQLDATABASE,
    
    /// "WEB_WORKERS"
    #[serde(rename="WEB_WORKERS")]
    WEBWORKERS,
    
    /// "GEO_LOCATION"
    #[serde(rename="GEO_LOCATION")]
    GEOLOCATION,
    
    /// "INLINE_SVG"
    #[serde(rename="INLINE_SVG")]
    INLINESVG,
    
    /// "SMIL"
    #[serde(rename="SMIL")]
    SMIL,
    
    /// "SVG_HREF"
    #[serde(rename="SVG_HREF")]
    SVGHREF,
    
    /// "SVG_CLIP_PATHS"
    #[serde(rename="SVG_CLIP_PATHS")]
    SVGCLIPPATHS,
    
    /// "TOUCH"
    #[serde(rename="TOUCH")]
    TOUCH,
    
    /// "WEBGL"
    #[serde(rename="WEBGL")]
    WEBGL,
    
    /// "SVG_FILTERS"
    #[serde(rename="SVG_FILTERS")]
    SVGFILTERS,
    
    /// "SVG_FE_IMAGE"
    #[serde(rename="SVG_FE_IMAGE")]
    SVGFEIMAGE,
}

impl AsRef<str> for CreativeBackupImageFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeBackupImageFeaturesEnum::CSSFONTFACE => "CSS_FONT_FACE",
            CreativeBackupImageFeaturesEnum::CSSBACKGROUNDSIZE => "CSS_BACKGROUND_SIZE",
            CreativeBackupImageFeaturesEnum::CSSBORDERIMAGE => "CSS_BORDER_IMAGE",
            CreativeBackupImageFeaturesEnum::CSSBORDERRADIUS => "CSS_BORDER_RADIUS",
            CreativeBackupImageFeaturesEnum::CSSBOXSHADOW => "CSS_BOX_SHADOW",
            CreativeBackupImageFeaturesEnum::CSSFLEXBOX => "CSS_FLEX_BOX",
            CreativeBackupImageFeaturesEnum::CSSHSLA => "CSS_HSLA",
            CreativeBackupImageFeaturesEnum::CSSMULTIPLEBGS => "CSS_MULTIPLE_BGS",
            CreativeBackupImageFeaturesEnum::CSSOPACITY => "CSS_OPACITY",
            CreativeBackupImageFeaturesEnum::CSSRGBA => "CSS_RGBA",
            CreativeBackupImageFeaturesEnum::CSSTEXTSHADOW => "CSS_TEXT_SHADOW",
            CreativeBackupImageFeaturesEnum::CSSANIMATIONS => "CSS_ANIMATIONS",
            CreativeBackupImageFeaturesEnum::CSSCOLUMNS => "CSS_COLUMNS",
            CreativeBackupImageFeaturesEnum::CSSGENERATEDCONTENT => "CSS_GENERATED_CONTENT",
            CreativeBackupImageFeaturesEnum::CSSGRADIENTS => "CSS_GRADIENTS",
            CreativeBackupImageFeaturesEnum::CSSREFLECTIONS => "CSS_REFLECTIONS",
            CreativeBackupImageFeaturesEnum::CSSTRANSFORMS => "CSS_TRANSFORMS",
            CreativeBackupImageFeaturesEnum::CSSTRANSFORMS3D => "CSS_TRANSFORMS3D",
            CreativeBackupImageFeaturesEnum::CSSTRANSITIONS => "CSS_TRANSITIONS",
            CreativeBackupImageFeaturesEnum::APPLICATIONCACHE => "APPLICATION_CACHE",
            CreativeBackupImageFeaturesEnum::CANVAS => "CANVAS",
            CreativeBackupImageFeaturesEnum::CANVASTEXT => "CANVAS_TEXT",
            CreativeBackupImageFeaturesEnum::DRAGANDDROP => "DRAG_AND_DROP",
            CreativeBackupImageFeaturesEnum::HASHCHANGE => "HASH_CHANGE",
            CreativeBackupImageFeaturesEnum::HISTORY => "HISTORY",
            CreativeBackupImageFeaturesEnum::AUDIO => "AUDIO",
            CreativeBackupImageFeaturesEnum::VIDEO => "VIDEO",
            CreativeBackupImageFeaturesEnum::INDEXEDDB => "INDEXED_DB",
            CreativeBackupImageFeaturesEnum::INPUTATTRAUTOCOMPLETE => "INPUT_ATTR_AUTOCOMPLETE",
            CreativeBackupImageFeaturesEnum::INPUTATTRAUTOFOCUS => "INPUT_ATTR_AUTOFOCUS",
            CreativeBackupImageFeaturesEnum::INPUTATTRLIST => "INPUT_ATTR_LIST",
            CreativeBackupImageFeaturesEnum::INPUTATTRPLACEHOLDER => "INPUT_ATTR_PLACEHOLDER",
            CreativeBackupImageFeaturesEnum::INPUTATTRMAX => "INPUT_ATTR_MAX",
            CreativeBackupImageFeaturesEnum::INPUTATTRMIN => "INPUT_ATTR_MIN",
            CreativeBackupImageFeaturesEnum::INPUTATTRMULTIPLE => "INPUT_ATTR_MULTIPLE",
            CreativeBackupImageFeaturesEnum::INPUTATTRPATTERN => "INPUT_ATTR_PATTERN",
            CreativeBackupImageFeaturesEnum::INPUTATTRREQUIRED => "INPUT_ATTR_REQUIRED",
            CreativeBackupImageFeaturesEnum::INPUTATTRSTEP => "INPUT_ATTR_STEP",
            CreativeBackupImageFeaturesEnum::INPUTTYPESEARCH => "INPUT_TYPE_SEARCH",
            CreativeBackupImageFeaturesEnum::INPUTTYPETEL => "INPUT_TYPE_TEL",
            CreativeBackupImageFeaturesEnum::INPUTTYPEURL => "INPUT_TYPE_URL",
            CreativeBackupImageFeaturesEnum::INPUTTYPEEMAIL => "INPUT_TYPE_EMAIL",
            CreativeBackupImageFeaturesEnum::INPUTTYPEDATETIME => "INPUT_TYPE_DATETIME",
            CreativeBackupImageFeaturesEnum::INPUTTYPEDATE => "INPUT_TYPE_DATE",
            CreativeBackupImageFeaturesEnum::INPUTTYPEMONTH => "INPUT_TYPE_MONTH",
            CreativeBackupImageFeaturesEnum::INPUTTYPEWEEK => "INPUT_TYPE_WEEK",
            CreativeBackupImageFeaturesEnum::INPUTTYPETIME => "INPUT_TYPE_TIME",
            CreativeBackupImageFeaturesEnum::INPUTTYPEDATETIMELOCAL => "INPUT_TYPE_DATETIME_LOCAL",
            CreativeBackupImageFeaturesEnum::INPUTTYPENUMBER => "INPUT_TYPE_NUMBER",
            CreativeBackupImageFeaturesEnum::INPUTTYPERANGE => "INPUT_TYPE_RANGE",
            CreativeBackupImageFeaturesEnum::INPUTTYPECOLOR => "INPUT_TYPE_COLOR",
            CreativeBackupImageFeaturesEnum::LOCALSTORAGE => "LOCAL_STORAGE",
            CreativeBackupImageFeaturesEnum::POSTMESSAGE => "POST_MESSAGE",
            CreativeBackupImageFeaturesEnum::SESSIONSTORAGE => "SESSION_STORAGE",
            CreativeBackupImageFeaturesEnum::WEBSOCKETS => "WEB_SOCKETS",
            CreativeBackupImageFeaturesEnum::WEBSQLDATABASE => "WEB_SQL_DATABASE",
            CreativeBackupImageFeaturesEnum::WEBWORKERS => "WEB_WORKERS",
            CreativeBackupImageFeaturesEnum::GEOLOCATION => "GEO_LOCATION",
            CreativeBackupImageFeaturesEnum::INLINESVG => "INLINE_SVG",
            CreativeBackupImageFeaturesEnum::SMIL => "SMIL",
            CreativeBackupImageFeaturesEnum::SVGHREF => "SVG_HREF",
            CreativeBackupImageFeaturesEnum::SVGCLIPPATHS => "SVG_CLIP_PATHS",
            CreativeBackupImageFeaturesEnum::TOUCH => "TOUCH",
            CreativeBackupImageFeaturesEnum::WEBGL => "WEBGL",
            CreativeBackupImageFeaturesEnum::SVGFILTERS => "SVG_FILTERS",
            CreativeBackupImageFeaturesEnum::SVGFEIMAGE => "SVG_FE_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeBackupImageFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CSS_FONT_FACE" => Ok(CreativeBackupImageFeaturesEnum::CSSFONTFACE),
           "CSS_BACKGROUND_SIZE" => Ok(CreativeBackupImageFeaturesEnum::CSSBACKGROUNDSIZE),
           "CSS_BORDER_IMAGE" => Ok(CreativeBackupImageFeaturesEnum::CSSBORDERIMAGE),
           "CSS_BORDER_RADIUS" => Ok(CreativeBackupImageFeaturesEnum::CSSBORDERRADIUS),
           "CSS_BOX_SHADOW" => Ok(CreativeBackupImageFeaturesEnum::CSSBOXSHADOW),
           "CSS_FLEX_BOX" => Ok(CreativeBackupImageFeaturesEnum::CSSFLEXBOX),
           "CSS_HSLA" => Ok(CreativeBackupImageFeaturesEnum::CSSHSLA),
           "CSS_MULTIPLE_BGS" => Ok(CreativeBackupImageFeaturesEnum::CSSMULTIPLEBGS),
           "CSS_OPACITY" => Ok(CreativeBackupImageFeaturesEnum::CSSOPACITY),
           "CSS_RGBA" => Ok(CreativeBackupImageFeaturesEnum::CSSRGBA),
           "CSS_TEXT_SHADOW" => Ok(CreativeBackupImageFeaturesEnum::CSSTEXTSHADOW),
           "CSS_ANIMATIONS" => Ok(CreativeBackupImageFeaturesEnum::CSSANIMATIONS),
           "CSS_COLUMNS" => Ok(CreativeBackupImageFeaturesEnum::CSSCOLUMNS),
           "CSS_GENERATED_CONTENT" => Ok(CreativeBackupImageFeaturesEnum::CSSGENERATEDCONTENT),
           "CSS_GRADIENTS" => Ok(CreativeBackupImageFeaturesEnum::CSSGRADIENTS),
           "CSS_REFLECTIONS" => Ok(CreativeBackupImageFeaturesEnum::CSSREFLECTIONS),
           "CSS_TRANSFORMS" => Ok(CreativeBackupImageFeaturesEnum::CSSTRANSFORMS),
           "CSS_TRANSFORMS3D" => Ok(CreativeBackupImageFeaturesEnum::CSSTRANSFORMS3D),
           "CSS_TRANSITIONS" => Ok(CreativeBackupImageFeaturesEnum::CSSTRANSITIONS),
           "APPLICATION_CACHE" => Ok(CreativeBackupImageFeaturesEnum::APPLICATIONCACHE),
           "CANVAS" => Ok(CreativeBackupImageFeaturesEnum::CANVAS),
           "CANVAS_TEXT" => Ok(CreativeBackupImageFeaturesEnum::CANVASTEXT),
           "DRAG_AND_DROP" => Ok(CreativeBackupImageFeaturesEnum::DRAGANDDROP),
           "HASH_CHANGE" => Ok(CreativeBackupImageFeaturesEnum::HASHCHANGE),
           "HISTORY" => Ok(CreativeBackupImageFeaturesEnum::HISTORY),
           "AUDIO" => Ok(CreativeBackupImageFeaturesEnum::AUDIO),
           "VIDEO" => Ok(CreativeBackupImageFeaturesEnum::VIDEO),
           "INDEXED_DB" => Ok(CreativeBackupImageFeaturesEnum::INDEXEDDB),
           "INPUT_ATTR_AUTOCOMPLETE" => Ok(CreativeBackupImageFeaturesEnum::INPUTATTRAUTOCOMPLETE),
           "INPUT_ATTR_AUTOFOCUS" => Ok(CreativeBackupImageFeaturesEnum::INPUTATTRAUTOFOCUS),
           "INPUT_ATTR_LIST" => Ok(CreativeBackupImageFeaturesEnum::INPUTATTRLIST),
           "INPUT_ATTR_PLACEHOLDER" => Ok(CreativeBackupImageFeaturesEnum::INPUTATTRPLACEHOLDER),
           "INPUT_ATTR_MAX" => Ok(CreativeBackupImageFeaturesEnum::INPUTATTRMAX),
           "INPUT_ATTR_MIN" => Ok(CreativeBackupImageFeaturesEnum::INPUTATTRMIN),
           "INPUT_ATTR_MULTIPLE" => Ok(CreativeBackupImageFeaturesEnum::INPUTATTRMULTIPLE),
           "INPUT_ATTR_PATTERN" => Ok(CreativeBackupImageFeaturesEnum::INPUTATTRPATTERN),
           "INPUT_ATTR_REQUIRED" => Ok(CreativeBackupImageFeaturesEnum::INPUTATTRREQUIRED),
           "INPUT_ATTR_STEP" => Ok(CreativeBackupImageFeaturesEnum::INPUTATTRSTEP),
           "INPUT_TYPE_SEARCH" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPESEARCH),
           "INPUT_TYPE_TEL" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPETEL),
           "INPUT_TYPE_URL" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPEURL),
           "INPUT_TYPE_EMAIL" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPEEMAIL),
           "INPUT_TYPE_DATETIME" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPEDATETIME),
           "INPUT_TYPE_DATE" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPEDATE),
           "INPUT_TYPE_MONTH" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPEMONTH),
           "INPUT_TYPE_WEEK" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPEWEEK),
           "INPUT_TYPE_TIME" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPETIME),
           "INPUT_TYPE_DATETIME_LOCAL" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPEDATETIMELOCAL),
           "INPUT_TYPE_NUMBER" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPENUMBER),
           "INPUT_TYPE_RANGE" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPERANGE),
           "INPUT_TYPE_COLOR" => Ok(CreativeBackupImageFeaturesEnum::INPUTTYPECOLOR),
           "LOCAL_STORAGE" => Ok(CreativeBackupImageFeaturesEnum::LOCALSTORAGE),
           "POST_MESSAGE" => Ok(CreativeBackupImageFeaturesEnum::POSTMESSAGE),
           "SESSION_STORAGE" => Ok(CreativeBackupImageFeaturesEnum::SESSIONSTORAGE),
           "WEB_SOCKETS" => Ok(CreativeBackupImageFeaturesEnum::WEBSOCKETS),
           "WEB_SQL_DATABASE" => Ok(CreativeBackupImageFeaturesEnum::WEBSQLDATABASE),
           "WEB_WORKERS" => Ok(CreativeBackupImageFeaturesEnum::WEBWORKERS),
           "GEO_LOCATION" => Ok(CreativeBackupImageFeaturesEnum::GEOLOCATION),
           "INLINE_SVG" => Ok(CreativeBackupImageFeaturesEnum::INLINESVG),
           "SMIL" => Ok(CreativeBackupImageFeaturesEnum::SMIL),
           "SVG_HREF" => Ok(CreativeBackupImageFeaturesEnum::SVGHREF),
           "SVG_CLIP_PATHS" => Ok(CreativeBackupImageFeaturesEnum::SVGCLIPPATHS),
           "TOUCH" => Ok(CreativeBackupImageFeaturesEnum::TOUCH),
           "WEBGL" => Ok(CreativeBackupImageFeaturesEnum::WEBGL),
           "SVG_FILTERS" => Ok(CreativeBackupImageFeaturesEnum::SVGFILTERS),
           "SVG_FE_IMAGE" => Ok(CreativeBackupImageFeaturesEnum::SVGFEIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeBackupImageFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeCompatibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Compatibilities associated with this creative. This is a read-only field. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop or on mobile devices or in mobile apps for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. Only pre-existing creatives may have these compatibilities since new creatives will either be assigned DISPLAY or DISPLAY_INTERSTITIAL instead. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard. IN_STREAM_AUDIO refers to rendering in in-stream audio ads developed with the VAST standard. Applicable to all creative types. Acceptable values are: - "APP" - "APP_INTERSTITIAL" - "IN_STREAM_VIDEO" - "IN_STREAM_AUDIO" - "DISPLAY" - "DISPLAY_INTERSTITIAL" 
pub enum CreativeCompatibilityEnum {
    
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    
    /// "DISPLAY_INTERSTITIAL"
    #[serde(rename="DISPLAY_INTERSTITIAL")]
    DISPLAYINTERSTITIAL,
    
    /// "APP"
    #[serde(rename="APP")]
    APP,
    
    /// "APP_INTERSTITIAL"
    #[serde(rename="APP_INTERSTITIAL")]
    APPINTERSTITIAL,
    
    /// "IN_STREAM_VIDEO"
    #[serde(rename="IN_STREAM_VIDEO")]
    INSTREAMVIDEO,
    
    /// "IN_STREAM_AUDIO"
    #[serde(rename="IN_STREAM_AUDIO")]
    INSTREAMAUDIO,
}

impl AsRef<str> for CreativeCompatibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeCompatibilityEnum::DISPLAY => "DISPLAY",
            CreativeCompatibilityEnum::DISPLAYINTERSTITIAL => "DISPLAY_INTERSTITIAL",
            CreativeCompatibilityEnum::APP => "APP",
            CreativeCompatibilityEnum::APPINTERSTITIAL => "APP_INTERSTITIAL",
            CreativeCompatibilityEnum::INSTREAMVIDEO => "IN_STREAM_VIDEO",
            CreativeCompatibilityEnum::INSTREAMAUDIO => "IN_STREAM_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeCompatibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISPLAY" => Ok(CreativeCompatibilityEnum::DISPLAY),
           "DISPLAY_INTERSTITIAL" => Ok(CreativeCompatibilityEnum::DISPLAYINTERSTITIAL),
           "APP" => Ok(CreativeCompatibilityEnum::APP),
           "APP_INTERSTITIAL" => Ok(CreativeCompatibilityEnum::APPINTERSTITIAL),
           "IN_STREAM_VIDEO" => Ok(CreativeCompatibilityEnum::INSTREAMVIDEO),
           "IN_STREAM_AUDIO" => Ok(CreativeCompatibilityEnum::INSTREAMAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeCompatibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this creative. This is a required field. Applicable to all creative types. *Note:* FLASH_INPAGE, HTML5_BANNER, and IMAGE are only used for existing creatives. New creatives should use DISPLAY as a replacement for these types.
pub enum CreativeTypeEnum {
    
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    
    /// "DISPLAY_REDIRECT"
    #[serde(rename="DISPLAY_REDIRECT")]
    DISPLAYREDIRECT,
    
    /// "CUSTOM_DISPLAY"
    #[serde(rename="CUSTOM_DISPLAY")]
    CUSTOMDISPLAY,
    
    /// "INTERNAL_REDIRECT"
    #[serde(rename="INTERNAL_REDIRECT")]
    INTERNALREDIRECT,
    
    /// "CUSTOM_DISPLAY_INTERSTITIAL"
    #[serde(rename="CUSTOM_DISPLAY_INTERSTITIAL")]
    CUSTOMDISPLAYINTERSTITIAL,
    
    /// "INTERSTITIAL_INTERNAL_REDIRECT"
    #[serde(rename="INTERSTITIAL_INTERNAL_REDIRECT")]
    INTERSTITIALINTERNALREDIRECT,
    
    /// "TRACKING_TEXT"
    #[serde(rename="TRACKING_TEXT")]
    TRACKINGTEXT,
    
    /// "RICH_MEDIA_DISPLAY_BANNER"
    #[serde(rename="RICH_MEDIA_DISPLAY_BANNER")]
    RICHMEDIADISPLAYBANNER,
    
    /// "RICH_MEDIA_INPAGE_FLOATING"
    #[serde(rename="RICH_MEDIA_INPAGE_FLOATING")]
    RICHMEDIAINPAGEFLOATING,
    
    /// "RICH_MEDIA_IM_EXPAND"
    #[serde(rename="RICH_MEDIA_IM_EXPAND")]
    RICHMEDIAIMEXPAND,
    
    /// "RICH_MEDIA_DISPLAY_EXPANDING"
    #[serde(rename="RICH_MEDIA_DISPLAY_EXPANDING")]
    RICHMEDIADISPLAYEXPANDING,
    
    /// "RICH_MEDIA_DISPLAY_INTERSTITIAL"
    #[serde(rename="RICH_MEDIA_DISPLAY_INTERSTITIAL")]
    RICHMEDIADISPLAYINTERSTITIAL,
    
    /// "RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL"
    #[serde(rename="RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL")]
    RICHMEDIADISPLAYMULTIFLOATINGINTERSTITIAL,
    
    /// "RICH_MEDIA_MOBILE_IN_APP"
    #[serde(rename="RICH_MEDIA_MOBILE_IN_APP")]
    RICHMEDIAMOBILEINAPP,
    
    /// "FLASH_INPAGE"
    #[serde(rename="FLASH_INPAGE")]
    FLASHINPAGE,
    
    /// "INSTREAM_VIDEO"
    #[serde(rename="INSTREAM_VIDEO")]
    INSTREAMVIDEO,
    
    /// "VPAID_LINEAR_VIDEO"
    #[serde(rename="VPAID_LINEAR_VIDEO")]
    VPAIDLINEARVIDEO,
    
    /// "VPAID_NON_LINEAR_VIDEO"
    #[serde(rename="VPAID_NON_LINEAR_VIDEO")]
    VPAIDNONLINEARVIDEO,
    
    /// "INSTREAM_VIDEO_REDIRECT"
    #[serde(rename="INSTREAM_VIDEO_REDIRECT")]
    INSTREAMVIDEOREDIRECT,
    
    /// "RICH_MEDIA_PEEL_DOWN"
    #[serde(rename="RICH_MEDIA_PEEL_DOWN")]
    RICHMEDIAPEELDOWN,
    
    /// "HTML5_BANNER"
    #[serde(rename="HTML5_BANNER")]
    HTML5BANNER,
    
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    
    /// "DISPLAY_IMAGE_GALLERY"
    #[serde(rename="DISPLAY_IMAGE_GALLERY")]
    DISPLAYIMAGEGALLERY,
    
    /// "BRAND_SAFE_DEFAULT_INSTREAM_VIDEO"
    #[serde(rename="BRAND_SAFE_DEFAULT_INSTREAM_VIDEO")]
    BRANDSAFEDEFAULTINSTREAMVIDEO,
    
    /// "INSTREAM_AUDIO"
    #[serde(rename="INSTREAM_AUDIO")]
    INSTREAMAUDIO,
}

impl AsRef<str> for CreativeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeTypeEnum::IMAGE => "IMAGE",
            CreativeTypeEnum::DISPLAYREDIRECT => "DISPLAY_REDIRECT",
            CreativeTypeEnum::CUSTOMDISPLAY => "CUSTOM_DISPLAY",
            CreativeTypeEnum::INTERNALREDIRECT => "INTERNAL_REDIRECT",
            CreativeTypeEnum::CUSTOMDISPLAYINTERSTITIAL => "CUSTOM_DISPLAY_INTERSTITIAL",
            CreativeTypeEnum::INTERSTITIALINTERNALREDIRECT => "INTERSTITIAL_INTERNAL_REDIRECT",
            CreativeTypeEnum::TRACKINGTEXT => "TRACKING_TEXT",
            CreativeTypeEnum::RICHMEDIADISPLAYBANNER => "RICH_MEDIA_DISPLAY_BANNER",
            CreativeTypeEnum::RICHMEDIAINPAGEFLOATING => "RICH_MEDIA_INPAGE_FLOATING",
            CreativeTypeEnum::RICHMEDIAIMEXPAND => "RICH_MEDIA_IM_EXPAND",
            CreativeTypeEnum::RICHMEDIADISPLAYEXPANDING => "RICH_MEDIA_DISPLAY_EXPANDING",
            CreativeTypeEnum::RICHMEDIADISPLAYINTERSTITIAL => "RICH_MEDIA_DISPLAY_INTERSTITIAL",
            CreativeTypeEnum::RICHMEDIADISPLAYMULTIFLOATINGINTERSTITIAL => "RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL",
            CreativeTypeEnum::RICHMEDIAMOBILEINAPP => "RICH_MEDIA_MOBILE_IN_APP",
            CreativeTypeEnum::FLASHINPAGE => "FLASH_INPAGE",
            CreativeTypeEnum::INSTREAMVIDEO => "INSTREAM_VIDEO",
            CreativeTypeEnum::VPAIDLINEARVIDEO => "VPAID_LINEAR_VIDEO",
            CreativeTypeEnum::VPAIDNONLINEARVIDEO => "VPAID_NON_LINEAR_VIDEO",
            CreativeTypeEnum::INSTREAMVIDEOREDIRECT => "INSTREAM_VIDEO_REDIRECT",
            CreativeTypeEnum::RICHMEDIAPEELDOWN => "RICH_MEDIA_PEEL_DOWN",
            CreativeTypeEnum::HTML5BANNER => "HTML5_BANNER",
            CreativeTypeEnum::DISPLAY => "DISPLAY",
            CreativeTypeEnum::DISPLAYIMAGEGALLERY => "DISPLAY_IMAGE_GALLERY",
            CreativeTypeEnum::BRANDSAFEDEFAULTINSTREAMVIDEO => "BRAND_SAFE_DEFAULT_INSTREAM_VIDEO",
            CreativeTypeEnum::INSTREAMAUDIO => "INSTREAM_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMAGE" => Ok(CreativeTypeEnum::IMAGE),
           "DISPLAY_REDIRECT" => Ok(CreativeTypeEnum::DISPLAYREDIRECT),
           "CUSTOM_DISPLAY" => Ok(CreativeTypeEnum::CUSTOMDISPLAY),
           "INTERNAL_REDIRECT" => Ok(CreativeTypeEnum::INTERNALREDIRECT),
           "CUSTOM_DISPLAY_INTERSTITIAL" => Ok(CreativeTypeEnum::CUSTOMDISPLAYINTERSTITIAL),
           "INTERSTITIAL_INTERNAL_REDIRECT" => Ok(CreativeTypeEnum::INTERSTITIALINTERNALREDIRECT),
           "TRACKING_TEXT" => Ok(CreativeTypeEnum::TRACKINGTEXT),
           "RICH_MEDIA_DISPLAY_BANNER" => Ok(CreativeTypeEnum::RICHMEDIADISPLAYBANNER),
           "RICH_MEDIA_INPAGE_FLOATING" => Ok(CreativeTypeEnum::RICHMEDIAINPAGEFLOATING),
           "RICH_MEDIA_IM_EXPAND" => Ok(CreativeTypeEnum::RICHMEDIAIMEXPAND),
           "RICH_MEDIA_DISPLAY_EXPANDING" => Ok(CreativeTypeEnum::RICHMEDIADISPLAYEXPANDING),
           "RICH_MEDIA_DISPLAY_INTERSTITIAL" => Ok(CreativeTypeEnum::RICHMEDIADISPLAYINTERSTITIAL),
           "RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL" => Ok(CreativeTypeEnum::RICHMEDIADISPLAYMULTIFLOATINGINTERSTITIAL),
           "RICH_MEDIA_MOBILE_IN_APP" => Ok(CreativeTypeEnum::RICHMEDIAMOBILEINAPP),
           "FLASH_INPAGE" => Ok(CreativeTypeEnum::FLASHINPAGE),
           "INSTREAM_VIDEO" => Ok(CreativeTypeEnum::INSTREAMVIDEO),
           "VPAID_LINEAR_VIDEO" => Ok(CreativeTypeEnum::VPAIDLINEARVIDEO),
           "VPAID_NON_LINEAR_VIDEO" => Ok(CreativeTypeEnum::VPAIDNONLINEARVIDEO),
           "INSTREAM_VIDEO_REDIRECT" => Ok(CreativeTypeEnum::INSTREAMVIDEOREDIRECT),
           "RICH_MEDIA_PEEL_DOWN" => Ok(CreativeTypeEnum::RICHMEDIAPEELDOWN),
           "HTML5_BANNER" => Ok(CreativeTypeEnum::HTML5BANNER),
           "DISPLAY" => Ok(CreativeTypeEnum::DISPLAY),
           "DISPLAY_IMAGE_GALLERY" => Ok(CreativeTypeEnum::DISPLAYIMAGEGALLERY),
           "BRAND_SAFE_DEFAULT_INSTREAM_VIDEO" => Ok(CreativeTypeEnum::BRANDSAFEDEFAULTINSTREAMVIDEO),
           "INSTREAM_AUDIO" => Ok(CreativeTypeEnum::INSTREAMAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Possible alignments for an asset. This is a read-only field. Applicable to the following creative types: RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL .
pub enum CreativeAssetAlignmentEnum {
    
    /// "ALIGNMENT_TOP"
    #[serde(rename="ALIGNMENT_TOP")]
    ALIGNMENTTOP,
    
    /// "ALIGNMENT_RIGHT"
    #[serde(rename="ALIGNMENT_RIGHT")]
    ALIGNMENTRIGHT,
    
    /// "ALIGNMENT_BOTTOM"
    #[serde(rename="ALIGNMENT_BOTTOM")]
    ALIGNMENTBOTTOM,
    
    /// "ALIGNMENT_LEFT"
    #[serde(rename="ALIGNMENT_LEFT")]
    ALIGNMENTLEFT,
}

impl AsRef<str> for CreativeAssetAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetAlignmentEnum::ALIGNMENTTOP => "ALIGNMENT_TOP",
            CreativeAssetAlignmentEnum::ALIGNMENTRIGHT => "ALIGNMENT_RIGHT",
            CreativeAssetAlignmentEnum::ALIGNMENTBOTTOM => "ALIGNMENT_BOTTOM",
            CreativeAssetAlignmentEnum::ALIGNMENTLEFT => "ALIGNMENT_LEFT",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIGNMENT_TOP" => Ok(CreativeAssetAlignmentEnum::ALIGNMENTTOP),
           "ALIGNMENT_RIGHT" => Ok(CreativeAssetAlignmentEnum::ALIGNMENTRIGHT),
           "ALIGNMENT_BOTTOM" => Ok(CreativeAssetAlignmentEnum::ALIGNMENTBOTTOM),
           "ALIGNMENT_LEFT" => Ok(CreativeAssetAlignmentEnum::ALIGNMENTLEFT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetArtworkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Artwork type of rich media creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA.
pub enum CreativeAssetArtworkTypeEnum {
    
    /// "ARTWORK_TYPE_FLASH"
    #[serde(rename="ARTWORK_TYPE_FLASH")]
    ARTWORKTYPEFLASH,
    
    /// "ARTWORK_TYPE_HTML5"
    #[serde(rename="ARTWORK_TYPE_HTML5")]
    ARTWORKTYPEHTML5,
    
    /// "ARTWORK_TYPE_MIXED"
    #[serde(rename="ARTWORK_TYPE_MIXED")]
    ARTWORKTYPEMIXED,
    
    /// "ARTWORK_TYPE_IMAGE"
    #[serde(rename="ARTWORK_TYPE_IMAGE")]
    ARTWORKTYPEIMAGE,
}

impl AsRef<str> for CreativeAssetArtworkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetArtworkTypeEnum::ARTWORKTYPEFLASH => "ARTWORK_TYPE_FLASH",
            CreativeAssetArtworkTypeEnum::ARTWORKTYPEHTML5 => "ARTWORK_TYPE_HTML5",
            CreativeAssetArtworkTypeEnum::ARTWORKTYPEMIXED => "ARTWORK_TYPE_MIXED",
            CreativeAssetArtworkTypeEnum::ARTWORKTYPEIMAGE => "ARTWORK_TYPE_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetArtworkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ARTWORK_TYPE_FLASH" => Ok(CreativeAssetArtworkTypeEnum::ARTWORKTYPEFLASH),
           "ARTWORK_TYPE_HTML5" => Ok(CreativeAssetArtworkTypeEnum::ARTWORKTYPEHTML5),
           "ARTWORK_TYPE_MIXED" => Ok(CreativeAssetArtworkTypeEnum::ARTWORKTYPEMIXED),
           "ARTWORK_TYPE_IMAGE" => Ok(CreativeAssetArtworkTypeEnum::ARTWORKTYPEIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetArtworkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetChildAssetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rich media child asset type. This is a read-only field. Applicable to the following creative types: all VPAID.
pub enum CreativeAssetChildAssetTypeEnum {
    
    /// "CHILD_ASSET_TYPE_FLASH"
    #[serde(rename="CHILD_ASSET_TYPE_FLASH")]
    CHILDASSETTYPEFLASH,
    
    /// "CHILD_ASSET_TYPE_VIDEO"
    #[serde(rename="CHILD_ASSET_TYPE_VIDEO")]
    CHILDASSETTYPEVIDEO,
    
    /// "CHILD_ASSET_TYPE_IMAGE"
    #[serde(rename="CHILD_ASSET_TYPE_IMAGE")]
    CHILDASSETTYPEIMAGE,
    
    /// "CHILD_ASSET_TYPE_DATA"
    #[serde(rename="CHILD_ASSET_TYPE_DATA")]
    CHILDASSETTYPEDATA,
}

impl AsRef<str> for CreativeAssetChildAssetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetChildAssetTypeEnum::CHILDASSETTYPEFLASH => "CHILD_ASSET_TYPE_FLASH",
            CreativeAssetChildAssetTypeEnum::CHILDASSETTYPEVIDEO => "CHILD_ASSET_TYPE_VIDEO",
            CreativeAssetChildAssetTypeEnum::CHILDASSETTYPEIMAGE => "CHILD_ASSET_TYPE_IMAGE",
            CreativeAssetChildAssetTypeEnum::CHILDASSETTYPEDATA => "CHILD_ASSET_TYPE_DATA",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetChildAssetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHILD_ASSET_TYPE_FLASH" => Ok(CreativeAssetChildAssetTypeEnum::CHILDASSETTYPEFLASH),
           "CHILD_ASSET_TYPE_VIDEO" => Ok(CreativeAssetChildAssetTypeEnum::CHILDASSETTYPEVIDEO),
           "CHILD_ASSET_TYPE_IMAGE" => Ok(CreativeAssetChildAssetTypeEnum::CHILDASSETTYPEIMAGE),
           "CHILD_ASSET_TYPE_DATA" => Ok(CreativeAssetChildAssetTypeEnum::CHILDASSETTYPEDATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetChildAssetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetDetectedFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of feature dependencies for the creative asset that are detected by Campaign Manager. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative correctly. This is a read-only, auto-generated field. Applicable to the following creative types: HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
pub enum CreativeAssetDetectedFeaturesEnum {
    
    /// "CSS_FONT_FACE"
    #[serde(rename="CSS_FONT_FACE")]
    CSSFONTFACE,
    
    /// "CSS_BACKGROUND_SIZE"
    #[serde(rename="CSS_BACKGROUND_SIZE")]
    CSSBACKGROUNDSIZE,
    
    /// "CSS_BORDER_IMAGE"
    #[serde(rename="CSS_BORDER_IMAGE")]
    CSSBORDERIMAGE,
    
    /// "CSS_BORDER_RADIUS"
    #[serde(rename="CSS_BORDER_RADIUS")]
    CSSBORDERRADIUS,
    
    /// "CSS_BOX_SHADOW"
    #[serde(rename="CSS_BOX_SHADOW")]
    CSSBOXSHADOW,
    
    /// "CSS_FLEX_BOX"
    #[serde(rename="CSS_FLEX_BOX")]
    CSSFLEXBOX,
    
    /// "CSS_HSLA"
    #[serde(rename="CSS_HSLA")]
    CSSHSLA,
    
    /// "CSS_MULTIPLE_BGS"
    #[serde(rename="CSS_MULTIPLE_BGS")]
    CSSMULTIPLEBGS,
    
    /// "CSS_OPACITY"
    #[serde(rename="CSS_OPACITY")]
    CSSOPACITY,
    
    /// "CSS_RGBA"
    #[serde(rename="CSS_RGBA")]
    CSSRGBA,
    
    /// "CSS_TEXT_SHADOW"
    #[serde(rename="CSS_TEXT_SHADOW")]
    CSSTEXTSHADOW,
    
    /// "CSS_ANIMATIONS"
    #[serde(rename="CSS_ANIMATIONS")]
    CSSANIMATIONS,
    
    /// "CSS_COLUMNS"
    #[serde(rename="CSS_COLUMNS")]
    CSSCOLUMNS,
    
    /// "CSS_GENERATED_CONTENT"
    #[serde(rename="CSS_GENERATED_CONTENT")]
    CSSGENERATEDCONTENT,
    
    /// "CSS_GRADIENTS"
    #[serde(rename="CSS_GRADIENTS")]
    CSSGRADIENTS,
    
    /// "CSS_REFLECTIONS"
    #[serde(rename="CSS_REFLECTIONS")]
    CSSREFLECTIONS,
    
    /// "CSS_TRANSFORMS"
    #[serde(rename="CSS_TRANSFORMS")]
    CSSTRANSFORMS,
    
    /// "CSS_TRANSFORMS3D"
    #[serde(rename="CSS_TRANSFORMS3D")]
    CSSTRANSFORMS3D,
    
    /// "CSS_TRANSITIONS"
    #[serde(rename="CSS_TRANSITIONS")]
    CSSTRANSITIONS,
    
    /// "APPLICATION_CACHE"
    #[serde(rename="APPLICATION_CACHE")]
    APPLICATIONCACHE,
    
    /// "CANVAS"
    #[serde(rename="CANVAS")]
    CANVAS,
    
    /// "CANVAS_TEXT"
    #[serde(rename="CANVAS_TEXT")]
    CANVASTEXT,
    
    /// "DRAG_AND_DROP"
    #[serde(rename="DRAG_AND_DROP")]
    DRAGANDDROP,
    
    /// "HASH_CHANGE"
    #[serde(rename="HASH_CHANGE")]
    HASHCHANGE,
    
    /// "HISTORY"
    #[serde(rename="HISTORY")]
    HISTORY,
    
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
    
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    
    /// "INDEXED_DB"
    #[serde(rename="INDEXED_DB")]
    INDEXEDDB,
    
    /// "INPUT_ATTR_AUTOCOMPLETE"
    #[serde(rename="INPUT_ATTR_AUTOCOMPLETE")]
    INPUTATTRAUTOCOMPLETE,
    
    /// "INPUT_ATTR_AUTOFOCUS"
    #[serde(rename="INPUT_ATTR_AUTOFOCUS")]
    INPUTATTRAUTOFOCUS,
    
    /// "INPUT_ATTR_LIST"
    #[serde(rename="INPUT_ATTR_LIST")]
    INPUTATTRLIST,
    
    /// "INPUT_ATTR_PLACEHOLDER"
    #[serde(rename="INPUT_ATTR_PLACEHOLDER")]
    INPUTATTRPLACEHOLDER,
    
    /// "INPUT_ATTR_MAX"
    #[serde(rename="INPUT_ATTR_MAX")]
    INPUTATTRMAX,
    
    /// "INPUT_ATTR_MIN"
    #[serde(rename="INPUT_ATTR_MIN")]
    INPUTATTRMIN,
    
    /// "INPUT_ATTR_MULTIPLE"
    #[serde(rename="INPUT_ATTR_MULTIPLE")]
    INPUTATTRMULTIPLE,
    
    /// "INPUT_ATTR_PATTERN"
    #[serde(rename="INPUT_ATTR_PATTERN")]
    INPUTATTRPATTERN,
    
    /// "INPUT_ATTR_REQUIRED"
    #[serde(rename="INPUT_ATTR_REQUIRED")]
    INPUTATTRREQUIRED,
    
    /// "INPUT_ATTR_STEP"
    #[serde(rename="INPUT_ATTR_STEP")]
    INPUTATTRSTEP,
    
    /// "INPUT_TYPE_SEARCH"
    #[serde(rename="INPUT_TYPE_SEARCH")]
    INPUTTYPESEARCH,
    
    /// "INPUT_TYPE_TEL"
    #[serde(rename="INPUT_TYPE_TEL")]
    INPUTTYPETEL,
    
    /// "INPUT_TYPE_URL"
    #[serde(rename="INPUT_TYPE_URL")]
    INPUTTYPEURL,
    
    /// "INPUT_TYPE_EMAIL"
    #[serde(rename="INPUT_TYPE_EMAIL")]
    INPUTTYPEEMAIL,
    
    /// "INPUT_TYPE_DATETIME"
    #[serde(rename="INPUT_TYPE_DATETIME")]
    INPUTTYPEDATETIME,
    
    /// "INPUT_TYPE_DATE"
    #[serde(rename="INPUT_TYPE_DATE")]
    INPUTTYPEDATE,
    
    /// "INPUT_TYPE_MONTH"
    #[serde(rename="INPUT_TYPE_MONTH")]
    INPUTTYPEMONTH,
    
    /// "INPUT_TYPE_WEEK"
    #[serde(rename="INPUT_TYPE_WEEK")]
    INPUTTYPEWEEK,
    
    /// "INPUT_TYPE_TIME"
    #[serde(rename="INPUT_TYPE_TIME")]
    INPUTTYPETIME,
    
    /// "INPUT_TYPE_DATETIME_LOCAL"
    #[serde(rename="INPUT_TYPE_DATETIME_LOCAL")]
    INPUTTYPEDATETIMELOCAL,
    
    /// "INPUT_TYPE_NUMBER"
    #[serde(rename="INPUT_TYPE_NUMBER")]
    INPUTTYPENUMBER,
    
    /// "INPUT_TYPE_RANGE"
    #[serde(rename="INPUT_TYPE_RANGE")]
    INPUTTYPERANGE,
    
    /// "INPUT_TYPE_COLOR"
    #[serde(rename="INPUT_TYPE_COLOR")]
    INPUTTYPECOLOR,
    
    /// "LOCAL_STORAGE"
    #[serde(rename="LOCAL_STORAGE")]
    LOCALSTORAGE,
    
    /// "POST_MESSAGE"
    #[serde(rename="POST_MESSAGE")]
    POSTMESSAGE,
    
    /// "SESSION_STORAGE"
    #[serde(rename="SESSION_STORAGE")]
    SESSIONSTORAGE,
    
    /// "WEB_SOCKETS"
    #[serde(rename="WEB_SOCKETS")]
    WEBSOCKETS,
    
    /// "WEB_SQL_DATABASE"
    #[serde(rename="WEB_SQL_DATABASE")]
    WEBSQLDATABASE,
    
    /// "WEB_WORKERS"
    #[serde(rename="WEB_WORKERS")]
    WEBWORKERS,
    
    /// "GEO_LOCATION"
    #[serde(rename="GEO_LOCATION")]
    GEOLOCATION,
    
    /// "INLINE_SVG"
    #[serde(rename="INLINE_SVG")]
    INLINESVG,
    
    /// "SMIL"
    #[serde(rename="SMIL")]
    SMIL,
    
    /// "SVG_HREF"
    #[serde(rename="SVG_HREF")]
    SVGHREF,
    
    /// "SVG_CLIP_PATHS"
    #[serde(rename="SVG_CLIP_PATHS")]
    SVGCLIPPATHS,
    
    /// "TOUCH"
    #[serde(rename="TOUCH")]
    TOUCH,
    
    /// "WEBGL"
    #[serde(rename="WEBGL")]
    WEBGL,
    
    /// "SVG_FILTERS"
    #[serde(rename="SVG_FILTERS")]
    SVGFILTERS,
    
    /// "SVG_FE_IMAGE"
    #[serde(rename="SVG_FE_IMAGE")]
    SVGFEIMAGE,
}

impl AsRef<str> for CreativeAssetDetectedFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetDetectedFeaturesEnum::CSSFONTFACE => "CSS_FONT_FACE",
            CreativeAssetDetectedFeaturesEnum::CSSBACKGROUNDSIZE => "CSS_BACKGROUND_SIZE",
            CreativeAssetDetectedFeaturesEnum::CSSBORDERIMAGE => "CSS_BORDER_IMAGE",
            CreativeAssetDetectedFeaturesEnum::CSSBORDERRADIUS => "CSS_BORDER_RADIUS",
            CreativeAssetDetectedFeaturesEnum::CSSBOXSHADOW => "CSS_BOX_SHADOW",
            CreativeAssetDetectedFeaturesEnum::CSSFLEXBOX => "CSS_FLEX_BOX",
            CreativeAssetDetectedFeaturesEnum::CSSHSLA => "CSS_HSLA",
            CreativeAssetDetectedFeaturesEnum::CSSMULTIPLEBGS => "CSS_MULTIPLE_BGS",
            CreativeAssetDetectedFeaturesEnum::CSSOPACITY => "CSS_OPACITY",
            CreativeAssetDetectedFeaturesEnum::CSSRGBA => "CSS_RGBA",
            CreativeAssetDetectedFeaturesEnum::CSSTEXTSHADOW => "CSS_TEXT_SHADOW",
            CreativeAssetDetectedFeaturesEnum::CSSANIMATIONS => "CSS_ANIMATIONS",
            CreativeAssetDetectedFeaturesEnum::CSSCOLUMNS => "CSS_COLUMNS",
            CreativeAssetDetectedFeaturesEnum::CSSGENERATEDCONTENT => "CSS_GENERATED_CONTENT",
            CreativeAssetDetectedFeaturesEnum::CSSGRADIENTS => "CSS_GRADIENTS",
            CreativeAssetDetectedFeaturesEnum::CSSREFLECTIONS => "CSS_REFLECTIONS",
            CreativeAssetDetectedFeaturesEnum::CSSTRANSFORMS => "CSS_TRANSFORMS",
            CreativeAssetDetectedFeaturesEnum::CSSTRANSFORMS3D => "CSS_TRANSFORMS3D",
            CreativeAssetDetectedFeaturesEnum::CSSTRANSITIONS => "CSS_TRANSITIONS",
            CreativeAssetDetectedFeaturesEnum::APPLICATIONCACHE => "APPLICATION_CACHE",
            CreativeAssetDetectedFeaturesEnum::CANVAS => "CANVAS",
            CreativeAssetDetectedFeaturesEnum::CANVASTEXT => "CANVAS_TEXT",
            CreativeAssetDetectedFeaturesEnum::DRAGANDDROP => "DRAG_AND_DROP",
            CreativeAssetDetectedFeaturesEnum::HASHCHANGE => "HASH_CHANGE",
            CreativeAssetDetectedFeaturesEnum::HISTORY => "HISTORY",
            CreativeAssetDetectedFeaturesEnum::AUDIO => "AUDIO",
            CreativeAssetDetectedFeaturesEnum::VIDEO => "VIDEO",
            CreativeAssetDetectedFeaturesEnum::INDEXEDDB => "INDEXED_DB",
            CreativeAssetDetectedFeaturesEnum::INPUTATTRAUTOCOMPLETE => "INPUT_ATTR_AUTOCOMPLETE",
            CreativeAssetDetectedFeaturesEnum::INPUTATTRAUTOFOCUS => "INPUT_ATTR_AUTOFOCUS",
            CreativeAssetDetectedFeaturesEnum::INPUTATTRLIST => "INPUT_ATTR_LIST",
            CreativeAssetDetectedFeaturesEnum::INPUTATTRPLACEHOLDER => "INPUT_ATTR_PLACEHOLDER",
            CreativeAssetDetectedFeaturesEnum::INPUTATTRMAX => "INPUT_ATTR_MAX",
            CreativeAssetDetectedFeaturesEnum::INPUTATTRMIN => "INPUT_ATTR_MIN",
            CreativeAssetDetectedFeaturesEnum::INPUTATTRMULTIPLE => "INPUT_ATTR_MULTIPLE",
            CreativeAssetDetectedFeaturesEnum::INPUTATTRPATTERN => "INPUT_ATTR_PATTERN",
            CreativeAssetDetectedFeaturesEnum::INPUTATTRREQUIRED => "INPUT_ATTR_REQUIRED",
            CreativeAssetDetectedFeaturesEnum::INPUTATTRSTEP => "INPUT_ATTR_STEP",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPESEARCH => "INPUT_TYPE_SEARCH",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPETEL => "INPUT_TYPE_TEL",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPEURL => "INPUT_TYPE_URL",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPEEMAIL => "INPUT_TYPE_EMAIL",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPEDATETIME => "INPUT_TYPE_DATETIME",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPEDATE => "INPUT_TYPE_DATE",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPEMONTH => "INPUT_TYPE_MONTH",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPEWEEK => "INPUT_TYPE_WEEK",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPETIME => "INPUT_TYPE_TIME",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPEDATETIMELOCAL => "INPUT_TYPE_DATETIME_LOCAL",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPENUMBER => "INPUT_TYPE_NUMBER",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPERANGE => "INPUT_TYPE_RANGE",
            CreativeAssetDetectedFeaturesEnum::INPUTTYPECOLOR => "INPUT_TYPE_COLOR",
            CreativeAssetDetectedFeaturesEnum::LOCALSTORAGE => "LOCAL_STORAGE",
            CreativeAssetDetectedFeaturesEnum::POSTMESSAGE => "POST_MESSAGE",
            CreativeAssetDetectedFeaturesEnum::SESSIONSTORAGE => "SESSION_STORAGE",
            CreativeAssetDetectedFeaturesEnum::WEBSOCKETS => "WEB_SOCKETS",
            CreativeAssetDetectedFeaturesEnum::WEBSQLDATABASE => "WEB_SQL_DATABASE",
            CreativeAssetDetectedFeaturesEnum::WEBWORKERS => "WEB_WORKERS",
            CreativeAssetDetectedFeaturesEnum::GEOLOCATION => "GEO_LOCATION",
            CreativeAssetDetectedFeaturesEnum::INLINESVG => "INLINE_SVG",
            CreativeAssetDetectedFeaturesEnum::SMIL => "SMIL",
            CreativeAssetDetectedFeaturesEnum::SVGHREF => "SVG_HREF",
            CreativeAssetDetectedFeaturesEnum::SVGCLIPPATHS => "SVG_CLIP_PATHS",
            CreativeAssetDetectedFeaturesEnum::TOUCH => "TOUCH",
            CreativeAssetDetectedFeaturesEnum::WEBGL => "WEBGL",
            CreativeAssetDetectedFeaturesEnum::SVGFILTERS => "SVG_FILTERS",
            CreativeAssetDetectedFeaturesEnum::SVGFEIMAGE => "SVG_FE_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetDetectedFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CSS_FONT_FACE" => Ok(CreativeAssetDetectedFeaturesEnum::CSSFONTFACE),
           "CSS_BACKGROUND_SIZE" => Ok(CreativeAssetDetectedFeaturesEnum::CSSBACKGROUNDSIZE),
           "CSS_BORDER_IMAGE" => Ok(CreativeAssetDetectedFeaturesEnum::CSSBORDERIMAGE),
           "CSS_BORDER_RADIUS" => Ok(CreativeAssetDetectedFeaturesEnum::CSSBORDERRADIUS),
           "CSS_BOX_SHADOW" => Ok(CreativeAssetDetectedFeaturesEnum::CSSBOXSHADOW),
           "CSS_FLEX_BOX" => Ok(CreativeAssetDetectedFeaturesEnum::CSSFLEXBOX),
           "CSS_HSLA" => Ok(CreativeAssetDetectedFeaturesEnum::CSSHSLA),
           "CSS_MULTIPLE_BGS" => Ok(CreativeAssetDetectedFeaturesEnum::CSSMULTIPLEBGS),
           "CSS_OPACITY" => Ok(CreativeAssetDetectedFeaturesEnum::CSSOPACITY),
           "CSS_RGBA" => Ok(CreativeAssetDetectedFeaturesEnum::CSSRGBA),
           "CSS_TEXT_SHADOW" => Ok(CreativeAssetDetectedFeaturesEnum::CSSTEXTSHADOW),
           "CSS_ANIMATIONS" => Ok(CreativeAssetDetectedFeaturesEnum::CSSANIMATIONS),
           "CSS_COLUMNS" => Ok(CreativeAssetDetectedFeaturesEnum::CSSCOLUMNS),
           "CSS_GENERATED_CONTENT" => Ok(CreativeAssetDetectedFeaturesEnum::CSSGENERATEDCONTENT),
           "CSS_GRADIENTS" => Ok(CreativeAssetDetectedFeaturesEnum::CSSGRADIENTS),
           "CSS_REFLECTIONS" => Ok(CreativeAssetDetectedFeaturesEnum::CSSREFLECTIONS),
           "CSS_TRANSFORMS" => Ok(CreativeAssetDetectedFeaturesEnum::CSSTRANSFORMS),
           "CSS_TRANSFORMS3D" => Ok(CreativeAssetDetectedFeaturesEnum::CSSTRANSFORMS3D),
           "CSS_TRANSITIONS" => Ok(CreativeAssetDetectedFeaturesEnum::CSSTRANSITIONS),
           "APPLICATION_CACHE" => Ok(CreativeAssetDetectedFeaturesEnum::APPLICATIONCACHE),
           "CANVAS" => Ok(CreativeAssetDetectedFeaturesEnum::CANVAS),
           "CANVAS_TEXT" => Ok(CreativeAssetDetectedFeaturesEnum::CANVASTEXT),
           "DRAG_AND_DROP" => Ok(CreativeAssetDetectedFeaturesEnum::DRAGANDDROP),
           "HASH_CHANGE" => Ok(CreativeAssetDetectedFeaturesEnum::HASHCHANGE),
           "HISTORY" => Ok(CreativeAssetDetectedFeaturesEnum::HISTORY),
           "AUDIO" => Ok(CreativeAssetDetectedFeaturesEnum::AUDIO),
           "VIDEO" => Ok(CreativeAssetDetectedFeaturesEnum::VIDEO),
           "INDEXED_DB" => Ok(CreativeAssetDetectedFeaturesEnum::INDEXEDDB),
           "INPUT_ATTR_AUTOCOMPLETE" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTATTRAUTOCOMPLETE),
           "INPUT_ATTR_AUTOFOCUS" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTATTRAUTOFOCUS),
           "INPUT_ATTR_LIST" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTATTRLIST),
           "INPUT_ATTR_PLACEHOLDER" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTATTRPLACEHOLDER),
           "INPUT_ATTR_MAX" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTATTRMAX),
           "INPUT_ATTR_MIN" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTATTRMIN),
           "INPUT_ATTR_MULTIPLE" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTATTRMULTIPLE),
           "INPUT_ATTR_PATTERN" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTATTRPATTERN),
           "INPUT_ATTR_REQUIRED" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTATTRREQUIRED),
           "INPUT_ATTR_STEP" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTATTRSTEP),
           "INPUT_TYPE_SEARCH" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPESEARCH),
           "INPUT_TYPE_TEL" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPETEL),
           "INPUT_TYPE_URL" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPEURL),
           "INPUT_TYPE_EMAIL" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPEEMAIL),
           "INPUT_TYPE_DATETIME" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPEDATETIME),
           "INPUT_TYPE_DATE" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPEDATE),
           "INPUT_TYPE_MONTH" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPEMONTH),
           "INPUT_TYPE_WEEK" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPEWEEK),
           "INPUT_TYPE_TIME" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPETIME),
           "INPUT_TYPE_DATETIME_LOCAL" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPEDATETIMELOCAL),
           "INPUT_TYPE_NUMBER" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPENUMBER),
           "INPUT_TYPE_RANGE" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPERANGE),
           "INPUT_TYPE_COLOR" => Ok(CreativeAssetDetectedFeaturesEnum::INPUTTYPECOLOR),
           "LOCAL_STORAGE" => Ok(CreativeAssetDetectedFeaturesEnum::LOCALSTORAGE),
           "POST_MESSAGE" => Ok(CreativeAssetDetectedFeaturesEnum::POSTMESSAGE),
           "SESSION_STORAGE" => Ok(CreativeAssetDetectedFeaturesEnum::SESSIONSTORAGE),
           "WEB_SOCKETS" => Ok(CreativeAssetDetectedFeaturesEnum::WEBSOCKETS),
           "WEB_SQL_DATABASE" => Ok(CreativeAssetDetectedFeaturesEnum::WEBSQLDATABASE),
           "WEB_WORKERS" => Ok(CreativeAssetDetectedFeaturesEnum::WEBWORKERS),
           "GEO_LOCATION" => Ok(CreativeAssetDetectedFeaturesEnum::GEOLOCATION),
           "INLINE_SVG" => Ok(CreativeAssetDetectedFeaturesEnum::INLINESVG),
           "SMIL" => Ok(CreativeAssetDetectedFeaturesEnum::SMIL),
           "SVG_HREF" => Ok(CreativeAssetDetectedFeaturesEnum::SVGHREF),
           "SVG_CLIP_PATHS" => Ok(CreativeAssetDetectedFeaturesEnum::SVGCLIPPATHS),
           "TOUCH" => Ok(CreativeAssetDetectedFeaturesEnum::TOUCH),
           "WEBGL" => Ok(CreativeAssetDetectedFeaturesEnum::WEBGL),
           "SVG_FILTERS" => Ok(CreativeAssetDetectedFeaturesEnum::SVGFILTERS),
           "SVG_FE_IMAGE" => Ok(CreativeAssetDetectedFeaturesEnum::SVGFEIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetDetectedFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetDisplayTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of rich media asset. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA.
pub enum CreativeAssetDisplayTypeEnum {
    
    /// "ASSET_DISPLAY_TYPE_INPAGE"
    #[serde(rename="ASSET_DISPLAY_TYPE_INPAGE")]
    ASSETDISPLAYTYPEINPAGE,
    
    /// "ASSET_DISPLAY_TYPE_FLOATING"
    #[serde(rename="ASSET_DISPLAY_TYPE_FLOATING")]
    ASSETDISPLAYTYPEFLOATING,
    
    /// "ASSET_DISPLAY_TYPE_OVERLAY"
    #[serde(rename="ASSET_DISPLAY_TYPE_OVERLAY")]
    ASSETDISPLAYTYPEOVERLAY,
    
    /// "ASSET_DISPLAY_TYPE_EXPANDING"
    #[serde(rename="ASSET_DISPLAY_TYPE_EXPANDING")]
    ASSETDISPLAYTYPEEXPANDING,
    
    /// "ASSET_DISPLAY_TYPE_FLASH_IN_FLASH"
    #[serde(rename="ASSET_DISPLAY_TYPE_FLASH_IN_FLASH")]
    ASSETDISPLAYTYPEFLASHINFLASH,
    
    /// "ASSET_DISPLAY_TYPE_FLASH_IN_FLASH_EXPANDING"
    #[serde(rename="ASSET_DISPLAY_TYPE_FLASH_IN_FLASH_EXPANDING")]
    ASSETDISPLAYTYPEFLASHINFLASHEXPANDING,
    
    /// "ASSET_DISPLAY_TYPE_PEEL_DOWN"
    #[serde(rename="ASSET_DISPLAY_TYPE_PEEL_DOWN")]
    ASSETDISPLAYTYPEPEELDOWN,
    
    /// "ASSET_DISPLAY_TYPE_VPAID_LINEAR"
    #[serde(rename="ASSET_DISPLAY_TYPE_VPAID_LINEAR")]
    ASSETDISPLAYTYPEVPAIDLINEAR,
    
    /// "ASSET_DISPLAY_TYPE_VPAID_NON_LINEAR"
    #[serde(rename="ASSET_DISPLAY_TYPE_VPAID_NON_LINEAR")]
    ASSETDISPLAYTYPEVPAIDNONLINEAR,
    
    /// "ASSET_DISPLAY_TYPE_BACKDROP"
    #[serde(rename="ASSET_DISPLAY_TYPE_BACKDROP")]
    ASSETDISPLAYTYPEBACKDROP,
}

impl AsRef<str> for CreativeAssetDisplayTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEINPAGE => "ASSET_DISPLAY_TYPE_INPAGE",
            CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEFLOATING => "ASSET_DISPLAY_TYPE_FLOATING",
            CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEOVERLAY => "ASSET_DISPLAY_TYPE_OVERLAY",
            CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEEXPANDING => "ASSET_DISPLAY_TYPE_EXPANDING",
            CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEFLASHINFLASH => "ASSET_DISPLAY_TYPE_FLASH_IN_FLASH",
            CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEFLASHINFLASHEXPANDING => "ASSET_DISPLAY_TYPE_FLASH_IN_FLASH_EXPANDING",
            CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEPEELDOWN => "ASSET_DISPLAY_TYPE_PEEL_DOWN",
            CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEVPAIDLINEAR => "ASSET_DISPLAY_TYPE_VPAID_LINEAR",
            CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEVPAIDNONLINEAR => "ASSET_DISPLAY_TYPE_VPAID_NON_LINEAR",
            CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEBACKDROP => "ASSET_DISPLAY_TYPE_BACKDROP",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetDisplayTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSET_DISPLAY_TYPE_INPAGE" => Ok(CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEINPAGE),
           "ASSET_DISPLAY_TYPE_FLOATING" => Ok(CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEFLOATING),
           "ASSET_DISPLAY_TYPE_OVERLAY" => Ok(CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEOVERLAY),
           "ASSET_DISPLAY_TYPE_EXPANDING" => Ok(CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEEXPANDING),
           "ASSET_DISPLAY_TYPE_FLASH_IN_FLASH" => Ok(CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEFLASHINFLASH),
           "ASSET_DISPLAY_TYPE_FLASH_IN_FLASH_EXPANDING" => Ok(CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEFLASHINFLASHEXPANDING),
           "ASSET_DISPLAY_TYPE_PEEL_DOWN" => Ok(CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEPEELDOWN),
           "ASSET_DISPLAY_TYPE_VPAID_LINEAR" => Ok(CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEVPAIDLINEAR),
           "ASSET_DISPLAY_TYPE_VPAID_NON_LINEAR" => Ok(CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEVPAIDNONLINEAR),
           "ASSET_DISPLAY_TYPE_BACKDROP" => Ok(CreativeAssetDisplayTypeEnum::ASSETDISPLAYTYPEBACKDROP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetDisplayTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetDurationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Duration type for which an asset will be displayed. Applicable to the following creative types: all RICH_MEDIA.
pub enum CreativeAssetDurationTypeEnum {
    
    /// "ASSET_DURATION_TYPE_AUTO"
    #[serde(rename="ASSET_DURATION_TYPE_AUTO")]
    ASSETDURATIONTYPEAUTO,
    
    /// "ASSET_DURATION_TYPE_NONE"
    #[serde(rename="ASSET_DURATION_TYPE_NONE")]
    ASSETDURATIONTYPENONE,
    
    /// "ASSET_DURATION_TYPE_CUSTOM"
    #[serde(rename="ASSET_DURATION_TYPE_CUSTOM")]
    ASSETDURATIONTYPECUSTOM,
}

impl AsRef<str> for CreativeAssetDurationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetDurationTypeEnum::ASSETDURATIONTYPEAUTO => "ASSET_DURATION_TYPE_AUTO",
            CreativeAssetDurationTypeEnum::ASSETDURATIONTYPENONE => "ASSET_DURATION_TYPE_NONE",
            CreativeAssetDurationTypeEnum::ASSETDURATIONTYPECUSTOM => "ASSET_DURATION_TYPE_CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetDurationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSET_DURATION_TYPE_AUTO" => Ok(CreativeAssetDurationTypeEnum::ASSETDURATIONTYPEAUTO),
           "ASSET_DURATION_TYPE_NONE" => Ok(CreativeAssetDurationTypeEnum::ASSETDURATIONTYPENONE),
           "ASSET_DURATION_TYPE_CUSTOM" => Ok(CreativeAssetDurationTypeEnum::ASSETDURATIONTYPECUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetDurationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetOrientationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Orientation of video asset. This is a read-only, auto-generated field.
pub enum CreativeAssetOrientationEnum {
    
    /// "LANDSCAPE"
    #[serde(rename="LANDSCAPE")]
    LANDSCAPE,
    
    /// "PORTRAIT"
    #[serde(rename="PORTRAIT")]
    PORTRAIT,
    
    /// "SQUARE"
    #[serde(rename="SQUARE")]
    SQUARE,
}

impl AsRef<str> for CreativeAssetOrientationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetOrientationEnum::LANDSCAPE => "LANDSCAPE",
            CreativeAssetOrientationEnum::PORTRAIT => "PORTRAIT",
            CreativeAssetOrientationEnum::SQUARE => "SQUARE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetOrientationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LANDSCAPE" => Ok(CreativeAssetOrientationEnum::LANDSCAPE),
           "PORTRAIT" => Ok(CreativeAssetOrientationEnum::PORTRAIT),
           "SQUARE" => Ok(CreativeAssetOrientationEnum::SQUARE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetOrientationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetPositionLeftUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Offset left unit for an asset. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA.
pub enum CreativeAssetPositionLeftUnitEnum {
    
    /// "OFFSET_UNIT_PIXEL"
    #[serde(rename="OFFSET_UNIT_PIXEL")]
    OFFSETUNITPIXEL,
    
    /// "OFFSET_UNIT_PERCENT"
    #[serde(rename="OFFSET_UNIT_PERCENT")]
    OFFSETUNITPERCENT,
    
    /// "OFFSET_UNIT_PIXEL_FROM_CENTER"
    #[serde(rename="OFFSET_UNIT_PIXEL_FROM_CENTER")]
    OFFSETUNITPIXELFROMCENTER,
}

impl AsRef<str> for CreativeAssetPositionLeftUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetPositionLeftUnitEnum::OFFSETUNITPIXEL => "OFFSET_UNIT_PIXEL",
            CreativeAssetPositionLeftUnitEnum::OFFSETUNITPERCENT => "OFFSET_UNIT_PERCENT",
            CreativeAssetPositionLeftUnitEnum::OFFSETUNITPIXELFROMCENTER => "OFFSET_UNIT_PIXEL_FROM_CENTER",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetPositionLeftUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFFSET_UNIT_PIXEL" => Ok(CreativeAssetPositionLeftUnitEnum::OFFSETUNITPIXEL),
           "OFFSET_UNIT_PERCENT" => Ok(CreativeAssetPositionLeftUnitEnum::OFFSETUNITPERCENT),
           "OFFSET_UNIT_PIXEL_FROM_CENTER" => Ok(CreativeAssetPositionLeftUnitEnum::OFFSETUNITPIXELFROMCENTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetPositionLeftUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetPositionTopUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Offset top unit for an asset. This is a read-only field if the asset displayType is ASSET_DISPLAY_TYPE_OVERLAY. Applicable to the following creative types: all RICH_MEDIA.
pub enum CreativeAssetPositionTopUnitEnum {
    
    /// "OFFSET_UNIT_PIXEL"
    #[serde(rename="OFFSET_UNIT_PIXEL")]
    OFFSETUNITPIXEL,
    
    /// "OFFSET_UNIT_PERCENT"
    #[serde(rename="OFFSET_UNIT_PERCENT")]
    OFFSETUNITPERCENT,
    
    /// "OFFSET_UNIT_PIXEL_FROM_CENTER"
    #[serde(rename="OFFSET_UNIT_PIXEL_FROM_CENTER")]
    OFFSETUNITPIXELFROMCENTER,
}

impl AsRef<str> for CreativeAssetPositionTopUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetPositionTopUnitEnum::OFFSETUNITPIXEL => "OFFSET_UNIT_PIXEL",
            CreativeAssetPositionTopUnitEnum::OFFSETUNITPERCENT => "OFFSET_UNIT_PERCENT",
            CreativeAssetPositionTopUnitEnum::OFFSETUNITPIXELFROMCENTER => "OFFSET_UNIT_PIXEL_FROM_CENTER",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetPositionTopUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFFSET_UNIT_PIXEL" => Ok(CreativeAssetPositionTopUnitEnum::OFFSETUNITPIXEL),
           "OFFSET_UNIT_PERCENT" => Ok(CreativeAssetPositionTopUnitEnum::OFFSETUNITPERCENT),
           "OFFSET_UNIT_PIXEL_FROM_CENTER" => Ok(CreativeAssetPositionTopUnitEnum::OFFSETUNITPIXELFROMCENTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetPositionTopUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Role of the asset in relation to creative. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT. This is a required field. PRIMARY applies to DISPLAY, FLASH_INPAGE, HTML5_BANNER, IMAGE, DISPLAY_IMAGE_GALLERY, all RICH_MEDIA (which may contain multiple primary assets), and all VPAID creatives. BACKUP_IMAGE applies to FLASH_INPAGE, HTML5_BANNER, all RICH_MEDIA, and all VPAID creatives. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE. ADDITIONAL_IMAGE and ADDITIONAL_FLASH apply to FLASH_INPAGE creatives. OTHER refers to assets from sources other than Campaign Manager, such as Studio uploaded assets, applicable to all RICH_MEDIA and all VPAID creatives. PARENT_VIDEO refers to videos uploaded by the user in Campaign Manager and is applicable to INSTREAM_VIDEO and VPAID_LINEAR_VIDEO creatives. TRANSCODED_VIDEO refers to videos transcoded by Campaign Manager from PARENT_VIDEO assets and is applicable to INSTREAM_VIDEO and VPAID_LINEAR_VIDEO creatives. ALTERNATE_VIDEO refers to the Campaign Manager representation of child asset videos from Studio, and is applicable to VPAID_LINEAR_VIDEO creatives. These cannot be added or removed within Campaign Manager. For VPAID_LINEAR_VIDEO creatives, PARENT_VIDEO, TRANSCODED_VIDEO and ALTERNATE_VIDEO assets that are marked active serve as backup in case the VPAID creative cannot be served. Only PARENT_VIDEO assets can be added or removed for an INSTREAM_VIDEO or VPAID_LINEAR_VIDEO creative. PARENT_AUDIO refers to audios uploaded by the user in Campaign Manager and is applicable to INSTREAM_AUDIO creatives. TRANSCODED_AUDIO refers to audios transcoded by Campaign Manager from PARENT_AUDIO assets and is applicable to INSTREAM_AUDIO creatives. 
pub enum CreativeAssetRoleEnum {
    
    /// "PRIMARY"
    #[serde(rename="PRIMARY")]
    PRIMARY,
    
    /// "BACKUP_IMAGE"
    #[serde(rename="BACKUP_IMAGE")]
    BACKUPIMAGE,
    
    /// "ADDITIONAL_IMAGE"
    #[serde(rename="ADDITIONAL_IMAGE")]
    ADDITIONALIMAGE,
    
    /// "ADDITIONAL_FLASH"
    #[serde(rename="ADDITIONAL_FLASH")]
    ADDITIONALFLASH,
    
    /// "PARENT_VIDEO"
    #[serde(rename="PARENT_VIDEO")]
    PARENTVIDEO,
    
    /// "TRANSCODED_VIDEO"
    #[serde(rename="TRANSCODED_VIDEO")]
    TRANSCODEDVIDEO,
    
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
    
    /// "ALTERNATE_VIDEO"
    #[serde(rename="ALTERNATE_VIDEO")]
    ALTERNATEVIDEO,
    
    /// "PARENT_AUDIO"
    #[serde(rename="PARENT_AUDIO")]
    PARENTAUDIO,
    
    /// "TRANSCODED_AUDIO"
    #[serde(rename="TRANSCODED_AUDIO")]
    TRANSCODEDAUDIO,
}

impl AsRef<str> for CreativeAssetRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetRoleEnum::PRIMARY => "PRIMARY",
            CreativeAssetRoleEnum::BACKUPIMAGE => "BACKUP_IMAGE",
            CreativeAssetRoleEnum::ADDITIONALIMAGE => "ADDITIONAL_IMAGE",
            CreativeAssetRoleEnum::ADDITIONALFLASH => "ADDITIONAL_FLASH",
            CreativeAssetRoleEnum::PARENTVIDEO => "PARENT_VIDEO",
            CreativeAssetRoleEnum::TRANSCODEDVIDEO => "TRANSCODED_VIDEO",
            CreativeAssetRoleEnum::OTHER => "OTHER",
            CreativeAssetRoleEnum::ALTERNATEVIDEO => "ALTERNATE_VIDEO",
            CreativeAssetRoleEnum::PARENTAUDIO => "PARENT_AUDIO",
            CreativeAssetRoleEnum::TRANSCODEDAUDIO => "TRANSCODED_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIMARY" => Ok(CreativeAssetRoleEnum::PRIMARY),
           "BACKUP_IMAGE" => Ok(CreativeAssetRoleEnum::BACKUPIMAGE),
           "ADDITIONAL_IMAGE" => Ok(CreativeAssetRoleEnum::ADDITIONALIMAGE),
           "ADDITIONAL_FLASH" => Ok(CreativeAssetRoleEnum::ADDITIONALFLASH),
           "PARENT_VIDEO" => Ok(CreativeAssetRoleEnum::PARENTVIDEO),
           "TRANSCODED_VIDEO" => Ok(CreativeAssetRoleEnum::TRANSCODEDVIDEO),
           "OTHER" => Ok(CreativeAssetRoleEnum::OTHER),
           "ALTERNATE_VIDEO" => Ok(CreativeAssetRoleEnum::ALTERNATEVIDEO),
           "PARENT_AUDIO" => Ok(CreativeAssetRoleEnum::PARENTAUDIO),
           "TRANSCODED_AUDIO" => Ok(CreativeAssetRoleEnum::TRANSCODEDAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetStartTimeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Initial wait time type before making the asset visible. Applicable to the following creative types: all RICH_MEDIA.
pub enum CreativeAssetStartTimeTypeEnum {
    
    /// "ASSET_START_TIME_TYPE_NONE"
    #[serde(rename="ASSET_START_TIME_TYPE_NONE")]
    ASSETSTARTTIMETYPENONE,
    
    /// "ASSET_START_TIME_TYPE_CUSTOM"
    #[serde(rename="ASSET_START_TIME_TYPE_CUSTOM")]
    ASSETSTARTTIMETYPECUSTOM,
}

impl AsRef<str> for CreativeAssetStartTimeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetStartTimeTypeEnum::ASSETSTARTTIMETYPENONE => "ASSET_START_TIME_TYPE_NONE",
            CreativeAssetStartTimeTypeEnum::ASSETSTARTTIMETYPECUSTOM => "ASSET_START_TIME_TYPE_CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetStartTimeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSET_START_TIME_TYPE_NONE" => Ok(CreativeAssetStartTimeTypeEnum::ASSETSTARTTIMETYPENONE),
           "ASSET_START_TIME_TYPE_CUSTOM" => Ok(CreativeAssetStartTimeTypeEnum::ASSETSTARTTIMETYPECUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetStartTimeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetWindowModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Window mode options for flash assets. Applicable to the following creative types: FLASH_INPAGE, RICH_MEDIA_DISPLAY_EXPANDING, RICH_MEDIA_IM_EXPAND, RICH_MEDIA_DISPLAY_BANNER, and RICH_MEDIA_INPAGE_FLOATING.
pub enum CreativeAssetWindowModeEnum {
    
    /// "OPAQUE"
    #[serde(rename="OPAQUE")]
    OPAQUE,
    
    /// "WINDOW"
    #[serde(rename="WINDOW")]
    WINDOW,
    
    /// "TRANSPARENT"
    #[serde(rename="TRANSPARENT")]
    TRANSPARENT,
}

impl AsRef<str> for CreativeAssetWindowModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetWindowModeEnum::OPAQUE => "OPAQUE",
            CreativeAssetWindowModeEnum::WINDOW => "WINDOW",
            CreativeAssetWindowModeEnum::TRANSPARENT => "TRANSPARENT",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetWindowModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPAQUE" => Ok(CreativeAssetWindowModeEnum::OPAQUE),
           "WINDOW" => Ok(CreativeAssetWindowModeEnum::WINDOW),
           "TRANSPARENT" => Ok(CreativeAssetWindowModeEnum::TRANSPARENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetWindowModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetIdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of asset to upload. This is a required field. FLASH and IMAGE are no longer supported for new uploads. All image assets should use HTML_IMAGE.
pub enum CreativeAssetIdTypeEnum {
    
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    
    /// "FLASH"
    #[serde(rename="FLASH")]
    FLASH,
    
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    
    /// "HTML"
    #[serde(rename="HTML")]
    HTML,
    
    /// "HTML_IMAGE"
    #[serde(rename="HTML_IMAGE")]
    HTMLIMAGE,
    
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
}

impl AsRef<str> for CreativeAssetIdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetIdTypeEnum::IMAGE => "IMAGE",
            CreativeAssetIdTypeEnum::FLASH => "FLASH",
            CreativeAssetIdTypeEnum::VIDEO => "VIDEO",
            CreativeAssetIdTypeEnum::HTML => "HTML",
            CreativeAssetIdTypeEnum::HTMLIMAGE => "HTML_IMAGE",
            CreativeAssetIdTypeEnum::AUDIO => "AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetIdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMAGE" => Ok(CreativeAssetIdTypeEnum::IMAGE),
           "FLASH" => Ok(CreativeAssetIdTypeEnum::FLASH),
           "VIDEO" => Ok(CreativeAssetIdTypeEnum::VIDEO),
           "HTML" => Ok(CreativeAssetIdTypeEnum::HTML),
           "HTML_IMAGE" => Ok(CreativeAssetIdTypeEnum::HTMLIMAGE),
           "AUDIO" => Ok(CreativeAssetIdTypeEnum::AUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetIdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetMetadataDetectedFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of feature dependencies for the creative asset that are detected by Campaign Manager. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative correctly. This is a read-only, auto-generated field.
pub enum CreativeAssetMetadataDetectedFeaturesEnum {
    
    /// "CSS_FONT_FACE"
    #[serde(rename="CSS_FONT_FACE")]
    CSSFONTFACE,
    
    /// "CSS_BACKGROUND_SIZE"
    #[serde(rename="CSS_BACKGROUND_SIZE")]
    CSSBACKGROUNDSIZE,
    
    /// "CSS_BORDER_IMAGE"
    #[serde(rename="CSS_BORDER_IMAGE")]
    CSSBORDERIMAGE,
    
    /// "CSS_BORDER_RADIUS"
    #[serde(rename="CSS_BORDER_RADIUS")]
    CSSBORDERRADIUS,
    
    /// "CSS_BOX_SHADOW"
    #[serde(rename="CSS_BOX_SHADOW")]
    CSSBOXSHADOW,
    
    /// "CSS_FLEX_BOX"
    #[serde(rename="CSS_FLEX_BOX")]
    CSSFLEXBOX,
    
    /// "CSS_HSLA"
    #[serde(rename="CSS_HSLA")]
    CSSHSLA,
    
    /// "CSS_MULTIPLE_BGS"
    #[serde(rename="CSS_MULTIPLE_BGS")]
    CSSMULTIPLEBGS,
    
    /// "CSS_OPACITY"
    #[serde(rename="CSS_OPACITY")]
    CSSOPACITY,
    
    /// "CSS_RGBA"
    #[serde(rename="CSS_RGBA")]
    CSSRGBA,
    
    /// "CSS_TEXT_SHADOW"
    #[serde(rename="CSS_TEXT_SHADOW")]
    CSSTEXTSHADOW,
    
    /// "CSS_ANIMATIONS"
    #[serde(rename="CSS_ANIMATIONS")]
    CSSANIMATIONS,
    
    /// "CSS_COLUMNS"
    #[serde(rename="CSS_COLUMNS")]
    CSSCOLUMNS,
    
    /// "CSS_GENERATED_CONTENT"
    #[serde(rename="CSS_GENERATED_CONTENT")]
    CSSGENERATEDCONTENT,
    
    /// "CSS_GRADIENTS"
    #[serde(rename="CSS_GRADIENTS")]
    CSSGRADIENTS,
    
    /// "CSS_REFLECTIONS"
    #[serde(rename="CSS_REFLECTIONS")]
    CSSREFLECTIONS,
    
    /// "CSS_TRANSFORMS"
    #[serde(rename="CSS_TRANSFORMS")]
    CSSTRANSFORMS,
    
    /// "CSS_TRANSFORMS3D"
    #[serde(rename="CSS_TRANSFORMS3D")]
    CSSTRANSFORMS3D,
    
    /// "CSS_TRANSITIONS"
    #[serde(rename="CSS_TRANSITIONS")]
    CSSTRANSITIONS,
    
    /// "APPLICATION_CACHE"
    #[serde(rename="APPLICATION_CACHE")]
    APPLICATIONCACHE,
    
    /// "CANVAS"
    #[serde(rename="CANVAS")]
    CANVAS,
    
    /// "CANVAS_TEXT"
    #[serde(rename="CANVAS_TEXT")]
    CANVASTEXT,
    
    /// "DRAG_AND_DROP"
    #[serde(rename="DRAG_AND_DROP")]
    DRAGANDDROP,
    
    /// "HASH_CHANGE"
    #[serde(rename="HASH_CHANGE")]
    HASHCHANGE,
    
    /// "HISTORY"
    #[serde(rename="HISTORY")]
    HISTORY,
    
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
    
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    
    /// "INDEXED_DB"
    #[serde(rename="INDEXED_DB")]
    INDEXEDDB,
    
    /// "INPUT_ATTR_AUTOCOMPLETE"
    #[serde(rename="INPUT_ATTR_AUTOCOMPLETE")]
    INPUTATTRAUTOCOMPLETE,
    
    /// "INPUT_ATTR_AUTOFOCUS"
    #[serde(rename="INPUT_ATTR_AUTOFOCUS")]
    INPUTATTRAUTOFOCUS,
    
    /// "INPUT_ATTR_LIST"
    #[serde(rename="INPUT_ATTR_LIST")]
    INPUTATTRLIST,
    
    /// "INPUT_ATTR_PLACEHOLDER"
    #[serde(rename="INPUT_ATTR_PLACEHOLDER")]
    INPUTATTRPLACEHOLDER,
    
    /// "INPUT_ATTR_MAX"
    #[serde(rename="INPUT_ATTR_MAX")]
    INPUTATTRMAX,
    
    /// "INPUT_ATTR_MIN"
    #[serde(rename="INPUT_ATTR_MIN")]
    INPUTATTRMIN,
    
    /// "INPUT_ATTR_MULTIPLE"
    #[serde(rename="INPUT_ATTR_MULTIPLE")]
    INPUTATTRMULTIPLE,
    
    /// "INPUT_ATTR_PATTERN"
    #[serde(rename="INPUT_ATTR_PATTERN")]
    INPUTATTRPATTERN,
    
    /// "INPUT_ATTR_REQUIRED"
    #[serde(rename="INPUT_ATTR_REQUIRED")]
    INPUTATTRREQUIRED,
    
    /// "INPUT_ATTR_STEP"
    #[serde(rename="INPUT_ATTR_STEP")]
    INPUTATTRSTEP,
    
    /// "INPUT_TYPE_SEARCH"
    #[serde(rename="INPUT_TYPE_SEARCH")]
    INPUTTYPESEARCH,
    
    /// "INPUT_TYPE_TEL"
    #[serde(rename="INPUT_TYPE_TEL")]
    INPUTTYPETEL,
    
    /// "INPUT_TYPE_URL"
    #[serde(rename="INPUT_TYPE_URL")]
    INPUTTYPEURL,
    
    /// "INPUT_TYPE_EMAIL"
    #[serde(rename="INPUT_TYPE_EMAIL")]
    INPUTTYPEEMAIL,
    
    /// "INPUT_TYPE_DATETIME"
    #[serde(rename="INPUT_TYPE_DATETIME")]
    INPUTTYPEDATETIME,
    
    /// "INPUT_TYPE_DATE"
    #[serde(rename="INPUT_TYPE_DATE")]
    INPUTTYPEDATE,
    
    /// "INPUT_TYPE_MONTH"
    #[serde(rename="INPUT_TYPE_MONTH")]
    INPUTTYPEMONTH,
    
    /// "INPUT_TYPE_WEEK"
    #[serde(rename="INPUT_TYPE_WEEK")]
    INPUTTYPEWEEK,
    
    /// "INPUT_TYPE_TIME"
    #[serde(rename="INPUT_TYPE_TIME")]
    INPUTTYPETIME,
    
    /// "INPUT_TYPE_DATETIME_LOCAL"
    #[serde(rename="INPUT_TYPE_DATETIME_LOCAL")]
    INPUTTYPEDATETIMELOCAL,
    
    /// "INPUT_TYPE_NUMBER"
    #[serde(rename="INPUT_TYPE_NUMBER")]
    INPUTTYPENUMBER,
    
    /// "INPUT_TYPE_RANGE"
    #[serde(rename="INPUT_TYPE_RANGE")]
    INPUTTYPERANGE,
    
    /// "INPUT_TYPE_COLOR"
    #[serde(rename="INPUT_TYPE_COLOR")]
    INPUTTYPECOLOR,
    
    /// "LOCAL_STORAGE"
    #[serde(rename="LOCAL_STORAGE")]
    LOCALSTORAGE,
    
    /// "POST_MESSAGE"
    #[serde(rename="POST_MESSAGE")]
    POSTMESSAGE,
    
    /// "SESSION_STORAGE"
    #[serde(rename="SESSION_STORAGE")]
    SESSIONSTORAGE,
    
    /// "WEB_SOCKETS"
    #[serde(rename="WEB_SOCKETS")]
    WEBSOCKETS,
    
    /// "WEB_SQL_DATABASE"
    #[serde(rename="WEB_SQL_DATABASE")]
    WEBSQLDATABASE,
    
    /// "WEB_WORKERS"
    #[serde(rename="WEB_WORKERS")]
    WEBWORKERS,
    
    /// "GEO_LOCATION"
    #[serde(rename="GEO_LOCATION")]
    GEOLOCATION,
    
    /// "INLINE_SVG"
    #[serde(rename="INLINE_SVG")]
    INLINESVG,
    
    /// "SMIL"
    #[serde(rename="SMIL")]
    SMIL,
    
    /// "SVG_HREF"
    #[serde(rename="SVG_HREF")]
    SVGHREF,
    
    /// "SVG_CLIP_PATHS"
    #[serde(rename="SVG_CLIP_PATHS")]
    SVGCLIPPATHS,
    
    /// "TOUCH"
    #[serde(rename="TOUCH")]
    TOUCH,
    
    /// "WEBGL"
    #[serde(rename="WEBGL")]
    WEBGL,
    
    /// "SVG_FILTERS"
    #[serde(rename="SVG_FILTERS")]
    SVGFILTERS,
    
    /// "SVG_FE_IMAGE"
    #[serde(rename="SVG_FE_IMAGE")]
    SVGFEIMAGE,
}

impl AsRef<str> for CreativeAssetMetadataDetectedFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetMetadataDetectedFeaturesEnum::CSSFONTFACE => "CSS_FONT_FACE",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSBACKGROUNDSIZE => "CSS_BACKGROUND_SIZE",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSBORDERIMAGE => "CSS_BORDER_IMAGE",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSBORDERRADIUS => "CSS_BORDER_RADIUS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSBOXSHADOW => "CSS_BOX_SHADOW",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSFLEXBOX => "CSS_FLEX_BOX",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSHSLA => "CSS_HSLA",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSMULTIPLEBGS => "CSS_MULTIPLE_BGS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSOPACITY => "CSS_OPACITY",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSRGBA => "CSS_RGBA",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSTEXTSHADOW => "CSS_TEXT_SHADOW",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSANIMATIONS => "CSS_ANIMATIONS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSCOLUMNS => "CSS_COLUMNS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSGENERATEDCONTENT => "CSS_GENERATED_CONTENT",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSGRADIENTS => "CSS_GRADIENTS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSREFLECTIONS => "CSS_REFLECTIONS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSFORMS => "CSS_TRANSFORMS",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSFORMS3D => "CSS_TRANSFORMS3D",
            CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSITIONS => "CSS_TRANSITIONS",
            CreativeAssetMetadataDetectedFeaturesEnum::APPLICATIONCACHE => "APPLICATION_CACHE",
            CreativeAssetMetadataDetectedFeaturesEnum::CANVAS => "CANVAS",
            CreativeAssetMetadataDetectedFeaturesEnum::CANVASTEXT => "CANVAS_TEXT",
            CreativeAssetMetadataDetectedFeaturesEnum::DRAGANDDROP => "DRAG_AND_DROP",
            CreativeAssetMetadataDetectedFeaturesEnum::HASHCHANGE => "HASH_CHANGE",
            CreativeAssetMetadataDetectedFeaturesEnum::HISTORY => "HISTORY",
            CreativeAssetMetadataDetectedFeaturesEnum::AUDIO => "AUDIO",
            CreativeAssetMetadataDetectedFeaturesEnum::VIDEO => "VIDEO",
            CreativeAssetMetadataDetectedFeaturesEnum::INDEXEDDB => "INDEXED_DB",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRAUTOCOMPLETE => "INPUT_ATTR_AUTOCOMPLETE",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRAUTOFOCUS => "INPUT_ATTR_AUTOFOCUS",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRLIST => "INPUT_ATTR_LIST",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRPLACEHOLDER => "INPUT_ATTR_PLACEHOLDER",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMAX => "INPUT_ATTR_MAX",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMIN => "INPUT_ATTR_MIN",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMULTIPLE => "INPUT_ATTR_MULTIPLE",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRPATTERN => "INPUT_ATTR_PATTERN",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRREQUIRED => "INPUT_ATTR_REQUIRED",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRSTEP => "INPUT_ATTR_STEP",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPESEARCH => "INPUT_TYPE_SEARCH",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPETEL => "INPUT_TYPE_TEL",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEURL => "INPUT_TYPE_URL",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEEMAIL => "INPUT_TYPE_EMAIL",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATETIME => "INPUT_TYPE_DATETIME",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATE => "INPUT_TYPE_DATE",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEMONTH => "INPUT_TYPE_MONTH",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEWEEK => "INPUT_TYPE_WEEK",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPETIME => "INPUT_TYPE_TIME",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATETIMELOCAL => "INPUT_TYPE_DATETIME_LOCAL",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPENUMBER => "INPUT_TYPE_NUMBER",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPERANGE => "INPUT_TYPE_RANGE",
            CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPECOLOR => "INPUT_TYPE_COLOR",
            CreativeAssetMetadataDetectedFeaturesEnum::LOCALSTORAGE => "LOCAL_STORAGE",
            CreativeAssetMetadataDetectedFeaturesEnum::POSTMESSAGE => "POST_MESSAGE",
            CreativeAssetMetadataDetectedFeaturesEnum::SESSIONSTORAGE => "SESSION_STORAGE",
            CreativeAssetMetadataDetectedFeaturesEnum::WEBSOCKETS => "WEB_SOCKETS",
            CreativeAssetMetadataDetectedFeaturesEnum::WEBSQLDATABASE => "WEB_SQL_DATABASE",
            CreativeAssetMetadataDetectedFeaturesEnum::WEBWORKERS => "WEB_WORKERS",
            CreativeAssetMetadataDetectedFeaturesEnum::GEOLOCATION => "GEO_LOCATION",
            CreativeAssetMetadataDetectedFeaturesEnum::INLINESVG => "INLINE_SVG",
            CreativeAssetMetadataDetectedFeaturesEnum::SMIL => "SMIL",
            CreativeAssetMetadataDetectedFeaturesEnum::SVGHREF => "SVG_HREF",
            CreativeAssetMetadataDetectedFeaturesEnum::SVGCLIPPATHS => "SVG_CLIP_PATHS",
            CreativeAssetMetadataDetectedFeaturesEnum::TOUCH => "TOUCH",
            CreativeAssetMetadataDetectedFeaturesEnum::WEBGL => "WEBGL",
            CreativeAssetMetadataDetectedFeaturesEnum::SVGFILTERS => "SVG_FILTERS",
            CreativeAssetMetadataDetectedFeaturesEnum::SVGFEIMAGE => "SVG_FE_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetMetadataDetectedFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CSS_FONT_FACE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSFONTFACE),
           "CSS_BACKGROUND_SIZE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSBACKGROUNDSIZE),
           "CSS_BORDER_IMAGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSBORDERIMAGE),
           "CSS_BORDER_RADIUS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSBORDERRADIUS),
           "CSS_BOX_SHADOW" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSBOXSHADOW),
           "CSS_FLEX_BOX" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSFLEXBOX),
           "CSS_HSLA" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSHSLA),
           "CSS_MULTIPLE_BGS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSMULTIPLEBGS),
           "CSS_OPACITY" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSOPACITY),
           "CSS_RGBA" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSRGBA),
           "CSS_TEXT_SHADOW" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSTEXTSHADOW),
           "CSS_ANIMATIONS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSANIMATIONS),
           "CSS_COLUMNS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSCOLUMNS),
           "CSS_GENERATED_CONTENT" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSGENERATEDCONTENT),
           "CSS_GRADIENTS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSGRADIENTS),
           "CSS_REFLECTIONS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSREFLECTIONS),
           "CSS_TRANSFORMS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSFORMS),
           "CSS_TRANSFORMS3D" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSFORMS3D),
           "CSS_TRANSITIONS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CSSTRANSITIONS),
           "APPLICATION_CACHE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::APPLICATIONCACHE),
           "CANVAS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CANVAS),
           "CANVAS_TEXT" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::CANVASTEXT),
           "DRAG_AND_DROP" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::DRAGANDDROP),
           "HASH_CHANGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::HASHCHANGE),
           "HISTORY" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::HISTORY),
           "AUDIO" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::AUDIO),
           "VIDEO" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::VIDEO),
           "INDEXED_DB" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INDEXEDDB),
           "INPUT_ATTR_AUTOCOMPLETE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRAUTOCOMPLETE),
           "INPUT_ATTR_AUTOFOCUS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRAUTOFOCUS),
           "INPUT_ATTR_LIST" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRLIST),
           "INPUT_ATTR_PLACEHOLDER" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRPLACEHOLDER),
           "INPUT_ATTR_MAX" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMAX),
           "INPUT_ATTR_MIN" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMIN),
           "INPUT_ATTR_MULTIPLE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRMULTIPLE),
           "INPUT_ATTR_PATTERN" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRPATTERN),
           "INPUT_ATTR_REQUIRED" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRREQUIRED),
           "INPUT_ATTR_STEP" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTATTRSTEP),
           "INPUT_TYPE_SEARCH" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPESEARCH),
           "INPUT_TYPE_TEL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPETEL),
           "INPUT_TYPE_URL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEURL),
           "INPUT_TYPE_EMAIL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEEMAIL),
           "INPUT_TYPE_DATETIME" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATETIME),
           "INPUT_TYPE_DATE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATE),
           "INPUT_TYPE_MONTH" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEMONTH),
           "INPUT_TYPE_WEEK" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEWEEK),
           "INPUT_TYPE_TIME" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPETIME),
           "INPUT_TYPE_DATETIME_LOCAL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPEDATETIMELOCAL),
           "INPUT_TYPE_NUMBER" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPENUMBER),
           "INPUT_TYPE_RANGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPERANGE),
           "INPUT_TYPE_COLOR" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INPUTTYPECOLOR),
           "LOCAL_STORAGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::LOCALSTORAGE),
           "POST_MESSAGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::POSTMESSAGE),
           "SESSION_STORAGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SESSIONSTORAGE),
           "WEB_SOCKETS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::WEBSOCKETS),
           "WEB_SQL_DATABASE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::WEBSQLDATABASE),
           "WEB_WORKERS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::WEBWORKERS),
           "GEO_LOCATION" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::GEOLOCATION),
           "INLINE_SVG" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::INLINESVG),
           "SMIL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SMIL),
           "SVG_HREF" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SVGHREF),
           "SVG_CLIP_PATHS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SVGCLIPPATHS),
           "TOUCH" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::TOUCH),
           "WEBGL" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::WEBGL),
           "SVG_FILTERS" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SVGFILTERS),
           "SVG_FE_IMAGE" => Ok(CreativeAssetMetadataDetectedFeaturesEnum::SVGFEIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetMetadataDetectedFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAssetMetadataWarnedValidationRulesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rules validated during code generation that generated a warning. This is a read-only, auto-generated field. Possible values are: - "ADMOB_REFERENCED" - "ASSET_FORMAT_UNSUPPORTED_DCM" - "ASSET_INVALID" - "CLICK_TAG_HARD_CODED" - "CLICK_TAG_INVALID" - "CLICK_TAG_IN_GWD" - "CLICK_TAG_MISSING" - "CLICK_TAG_MORE_THAN_ONE" - "CLICK_TAG_NON_TOP_LEVEL" - "COMPONENT_UNSUPPORTED_DCM" - "ENABLER_UNSUPPORTED_METHOD_DCM" - "EXTERNAL_FILE_REFERENCED" - "FILE_DETAIL_EMPTY" - "FILE_TYPE_INVALID" - "GWD_PROPERTIES_INVALID" - "HTML5_FEATURE_UNSUPPORTED" - "LINKED_FILE_NOT_FOUND" - "MAX_FLASH_VERSION_11" - "MRAID_REFERENCED" - "NOT_SSL_COMPLIANT" - "ORPHANED_ASSET" - "PRIMARY_HTML_MISSING" - "SVG_INVALID" - "ZIP_INVALID" 
pub enum CreativeAssetMetadataWarnedValidationRulesEnum {
    
    /// "CLICK_TAG_NON_TOP_LEVEL"
    #[serde(rename="CLICK_TAG_NON_TOP_LEVEL")]
    CLICKTAGNONTOPLEVEL,
    
    /// "CLICK_TAG_MISSING"
    #[serde(rename="CLICK_TAG_MISSING")]
    CLICKTAGMISSING,
    
    /// "CLICK_TAG_MORE_THAN_ONE"
    #[serde(rename="CLICK_TAG_MORE_THAN_ONE")]
    CLICKTAGMORETHANONE,
    
    /// "CLICK_TAG_INVALID"
    #[serde(rename="CLICK_TAG_INVALID")]
    CLICKTAGINVALID,
    
    /// "ORPHANED_ASSET"
    #[serde(rename="ORPHANED_ASSET")]
    ORPHANEDASSET,
    
    /// "PRIMARY_HTML_MISSING"
    #[serde(rename="PRIMARY_HTML_MISSING")]
    PRIMARYHTMLMISSING,
    
    /// "EXTERNAL_FILE_REFERENCED"
    #[serde(rename="EXTERNAL_FILE_REFERENCED")]
    EXTERNALFILEREFERENCED,
    
    /// "MRAID_REFERENCED"
    #[serde(rename="MRAID_REFERENCED")]
    MRAIDREFERENCED,
    
    /// "ADMOB_REFERENCED"
    #[serde(rename="ADMOB_REFERENCED")]
    ADMOBREFERENCED,
    
    /// "FILE_TYPE_INVALID"
    #[serde(rename="FILE_TYPE_INVALID")]
    FILETYPEINVALID,
    
    /// "ZIP_INVALID"
    #[serde(rename="ZIP_INVALID")]
    ZIPINVALID,
    
    /// "LINKED_FILE_NOT_FOUND"
    #[serde(rename="LINKED_FILE_NOT_FOUND")]
    LINKEDFILENOTFOUND,
    
    /// "MAX_FLASH_VERSION_11"
    #[serde(rename="MAX_FLASH_VERSION_11")]
    MAXFLASHVERSION11,
    
    /// "NOT_SSL_COMPLIANT"
    #[serde(rename="NOT_SSL_COMPLIANT")]
    NOTSSLCOMPLIANT,
    
    /// "FILE_DETAIL_EMPTY"
    #[serde(rename="FILE_DETAIL_EMPTY")]
    FILEDETAILEMPTY,
    
    /// "ASSET_INVALID"
    #[serde(rename="ASSET_INVALID")]
    ASSETINVALID,
    
    /// "GWD_PROPERTIES_INVALID"
    #[serde(rename="GWD_PROPERTIES_INVALID")]
    GWDPROPERTIESINVALID,
    
    /// "ENABLER_UNSUPPORTED_METHOD_DCM"
    #[serde(rename="ENABLER_UNSUPPORTED_METHOD_DCM")]
    ENABLERUNSUPPORTEDMETHODDCM,
    
    /// "ASSET_FORMAT_UNSUPPORTED_DCM"
    #[serde(rename="ASSET_FORMAT_UNSUPPORTED_DCM")]
    ASSETFORMATUNSUPPORTEDDCM,
    
    /// "COMPONENT_UNSUPPORTED_DCM"
    #[serde(rename="COMPONENT_UNSUPPORTED_DCM")]
    COMPONENTUNSUPPORTEDDCM,
    
    /// "HTML5_FEATURE_UNSUPPORTED"
    #[serde(rename="HTML5_FEATURE_UNSUPPORTED")]
    HTML5FEATUREUNSUPPORTED,
    
    /// "CLICK_TAG_IN_GWD"
    #[serde(rename="CLICK_TAG_IN_GWD")]
    CLICKTAGINGWD,
    
    /// "CLICK_TAG_HARD_CODED"
    #[serde(rename="CLICK_TAG_HARD_CODED")]
    CLICKTAGHARDCODED,
    
    /// "SVG_INVALID"
    #[serde(rename="SVG_INVALID")]
    SVGINVALID,
    
    /// "CLICK_TAG_IN_RICH_MEDIA"
    #[serde(rename="CLICK_TAG_IN_RICH_MEDIA")]
    CLICKTAGINRICHMEDIA,
}

impl AsRef<str> for CreativeAssetMetadataWarnedValidationRulesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGNONTOPLEVEL => "CLICK_TAG_NON_TOP_LEVEL",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGMISSING => "CLICK_TAG_MISSING",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGMORETHANONE => "CLICK_TAG_MORE_THAN_ONE",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINVALID => "CLICK_TAG_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::ORPHANEDASSET => "ORPHANED_ASSET",
            CreativeAssetMetadataWarnedValidationRulesEnum::PRIMARYHTMLMISSING => "PRIMARY_HTML_MISSING",
            CreativeAssetMetadataWarnedValidationRulesEnum::EXTERNALFILEREFERENCED => "EXTERNAL_FILE_REFERENCED",
            CreativeAssetMetadataWarnedValidationRulesEnum::MRAIDREFERENCED => "MRAID_REFERENCED",
            CreativeAssetMetadataWarnedValidationRulesEnum::ADMOBREFERENCED => "ADMOB_REFERENCED",
            CreativeAssetMetadataWarnedValidationRulesEnum::FILETYPEINVALID => "FILE_TYPE_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::ZIPINVALID => "ZIP_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::LINKEDFILENOTFOUND => "LINKED_FILE_NOT_FOUND",
            CreativeAssetMetadataWarnedValidationRulesEnum::MAXFLASHVERSION11 => "MAX_FLASH_VERSION_11",
            CreativeAssetMetadataWarnedValidationRulesEnum::NOTSSLCOMPLIANT => "NOT_SSL_COMPLIANT",
            CreativeAssetMetadataWarnedValidationRulesEnum::FILEDETAILEMPTY => "FILE_DETAIL_EMPTY",
            CreativeAssetMetadataWarnedValidationRulesEnum::ASSETINVALID => "ASSET_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::GWDPROPERTIESINVALID => "GWD_PROPERTIES_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::ENABLERUNSUPPORTEDMETHODDCM => "ENABLER_UNSUPPORTED_METHOD_DCM",
            CreativeAssetMetadataWarnedValidationRulesEnum::ASSETFORMATUNSUPPORTEDDCM => "ASSET_FORMAT_UNSUPPORTED_DCM",
            CreativeAssetMetadataWarnedValidationRulesEnum::COMPONENTUNSUPPORTEDDCM => "COMPONENT_UNSUPPORTED_DCM",
            CreativeAssetMetadataWarnedValidationRulesEnum::HTML5FEATUREUNSUPPORTED => "HTML5_FEATURE_UNSUPPORTED",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINGWD => "CLICK_TAG_IN_GWD",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGHARDCODED => "CLICK_TAG_HARD_CODED",
            CreativeAssetMetadataWarnedValidationRulesEnum::SVGINVALID => "SVG_INVALID",
            CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINRICHMEDIA => "CLICK_TAG_IN_RICH_MEDIA",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAssetMetadataWarnedValidationRulesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLICK_TAG_NON_TOP_LEVEL" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGNONTOPLEVEL),
           "CLICK_TAG_MISSING" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGMISSING),
           "CLICK_TAG_MORE_THAN_ONE" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGMORETHANONE),
           "CLICK_TAG_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINVALID),
           "ORPHANED_ASSET" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ORPHANEDASSET),
           "PRIMARY_HTML_MISSING" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::PRIMARYHTMLMISSING),
           "EXTERNAL_FILE_REFERENCED" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::EXTERNALFILEREFERENCED),
           "MRAID_REFERENCED" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::MRAIDREFERENCED),
           "ADMOB_REFERENCED" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ADMOBREFERENCED),
           "FILE_TYPE_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::FILETYPEINVALID),
           "ZIP_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ZIPINVALID),
           "LINKED_FILE_NOT_FOUND" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::LINKEDFILENOTFOUND),
           "MAX_FLASH_VERSION_11" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::MAXFLASHVERSION11),
           "NOT_SSL_COMPLIANT" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::NOTSSLCOMPLIANT),
           "FILE_DETAIL_EMPTY" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::FILEDETAILEMPTY),
           "ASSET_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ASSETINVALID),
           "GWD_PROPERTIES_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::GWDPROPERTIESINVALID),
           "ENABLER_UNSUPPORTED_METHOD_DCM" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ENABLERUNSUPPORTEDMETHODDCM),
           "ASSET_FORMAT_UNSUPPORTED_DCM" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::ASSETFORMATUNSUPPORTEDDCM),
           "COMPONENT_UNSUPPORTED_DCM" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::COMPONENTUNSUPPORTEDDCM),
           "HTML5_FEATURE_UNSUPPORTED" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::HTML5FEATUREUNSUPPORTED),
           "CLICK_TAG_IN_GWD" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINGWD),
           "CLICK_TAG_HARD_CODED" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGHARDCODED),
           "SVG_INVALID" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::SVGINVALID),
           "CLICK_TAG_IN_RICH_MEDIA" => Ok(CreativeAssetMetadataWarnedValidationRulesEnum::CLICKTAGINRICHMEDIA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAssetMetadataWarnedValidationRulesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeCustomEventAdvertiserCustomEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the event. This is a read-only field.
pub enum CreativeCustomEventAdvertiserCustomEventTypeEnum {
    
    /// "ADVERTISER_EVENT_TIMER"
    #[serde(rename="ADVERTISER_EVENT_TIMER")]
    ADVERTISEREVENTTIMER,
    
    /// "ADVERTISER_EVENT_EXIT"
    #[serde(rename="ADVERTISER_EVENT_EXIT")]
    ADVERTISEREVENTEXIT,
    
    /// "ADVERTISER_EVENT_COUNTER"
    #[serde(rename="ADVERTISER_EVENT_COUNTER")]
    ADVERTISEREVENTCOUNTER,
}

impl AsRef<str> for CreativeCustomEventAdvertiserCustomEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTTIMER => "ADVERTISER_EVENT_TIMER",
            CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTEXIT => "ADVERTISER_EVENT_EXIT",
            CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTCOUNTER => "ADVERTISER_EVENT_COUNTER",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeCustomEventAdvertiserCustomEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADVERTISER_EVENT_TIMER" => Ok(CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTTIMER),
           "ADVERTISER_EVENT_EXIT" => Ok(CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTEXIT),
           "ADVERTISER_EVENT_COUNTER" => Ok(CreativeCustomEventAdvertiserCustomEventTypeEnum::ADVERTISEREVENTCOUNTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeCustomEventAdvertiserCustomEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeCustomEventArtworkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Artwork type used by the creative.This is a read-only field.
pub enum CreativeCustomEventArtworkTypeEnum {
    
    /// "ARTWORK_TYPE_FLASH"
    #[serde(rename="ARTWORK_TYPE_FLASH")]
    ARTWORKTYPEFLASH,
    
    /// "ARTWORK_TYPE_HTML5"
    #[serde(rename="ARTWORK_TYPE_HTML5")]
    ARTWORKTYPEHTML5,
    
    /// "ARTWORK_TYPE_MIXED"
    #[serde(rename="ARTWORK_TYPE_MIXED")]
    ARTWORKTYPEMIXED,
    
    /// "ARTWORK_TYPE_IMAGE"
    #[serde(rename="ARTWORK_TYPE_IMAGE")]
    ARTWORKTYPEIMAGE,
}

impl AsRef<str> for CreativeCustomEventArtworkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEFLASH => "ARTWORK_TYPE_FLASH",
            CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEHTML5 => "ARTWORK_TYPE_HTML5",
            CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEMIXED => "ARTWORK_TYPE_MIXED",
            CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEIMAGE => "ARTWORK_TYPE_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeCustomEventArtworkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ARTWORK_TYPE_FLASH" => Ok(CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEFLASH),
           "ARTWORK_TYPE_HTML5" => Ok(CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEHTML5),
           "ARTWORK_TYPE_MIXED" => Ok(CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEMIXED),
           "ARTWORK_TYPE_IMAGE" => Ok(CreativeCustomEventArtworkTypeEnum::ARTWORKTYPEIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeCustomEventArtworkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeCustomEventTargetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Target type used by the event.
pub enum CreativeCustomEventTargetTypeEnum {
    
    /// "TARGET_BLANK"
    #[serde(rename="TARGET_BLANK")]
    TARGETBLANK,
    
    /// "TARGET_TOP"
    #[serde(rename="TARGET_TOP")]
    TARGETTOP,
    
    /// "TARGET_SELF"
    #[serde(rename="TARGET_SELF")]
    TARGETSELF,
    
    /// "TARGET_PARENT"
    #[serde(rename="TARGET_PARENT")]
    TARGETPARENT,
    
    /// "TARGET_POPUP"
    #[serde(rename="TARGET_POPUP")]
    TARGETPOPUP,
}

impl AsRef<str> for CreativeCustomEventTargetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeCustomEventTargetTypeEnum::TARGETBLANK => "TARGET_BLANK",
            CreativeCustomEventTargetTypeEnum::TARGETTOP => "TARGET_TOP",
            CreativeCustomEventTargetTypeEnum::TARGETSELF => "TARGET_SELF",
            CreativeCustomEventTargetTypeEnum::TARGETPARENT => "TARGET_PARENT",
            CreativeCustomEventTargetTypeEnum::TARGETPOPUP => "TARGET_POPUP",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeCustomEventTargetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGET_BLANK" => Ok(CreativeCustomEventTargetTypeEnum::TARGETBLANK),
           "TARGET_TOP" => Ok(CreativeCustomEventTargetTypeEnum::TARGETTOP),
           "TARGET_SELF" => Ok(CreativeCustomEventTargetTypeEnum::TARGETSELF),
           "TARGET_PARENT" => Ok(CreativeCustomEventTargetTypeEnum::TARGETPARENT),
           "TARGET_POPUP" => Ok(CreativeCustomEventTargetTypeEnum::TARGETPOPUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeCustomEventTargetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeGroupAssignmentCreativeGroupNumberEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Creative group number of the creative group assignment.
pub enum CreativeGroupAssignmentCreativeGroupNumberEnum {
    
    /// "CREATIVE_GROUP_ONE"
    #[serde(rename="CREATIVE_GROUP_ONE")]
    CREATIVEGROUPONE,
    
    /// "CREATIVE_GROUP_TWO"
    #[serde(rename="CREATIVE_GROUP_TWO")]
    CREATIVEGROUPTWO,
}

impl AsRef<str> for CreativeGroupAssignmentCreativeGroupNumberEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeGroupAssignmentCreativeGroupNumberEnum::CREATIVEGROUPONE => "CREATIVE_GROUP_ONE",
            CreativeGroupAssignmentCreativeGroupNumberEnum::CREATIVEGROUPTWO => "CREATIVE_GROUP_TWO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeGroupAssignmentCreativeGroupNumberEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_GROUP_ONE" => Ok(CreativeGroupAssignmentCreativeGroupNumberEnum::CREATIVEGROUPONE),
           "CREATIVE_GROUP_TWO" => Ok(CreativeGroupAssignmentCreativeGroupNumberEnum::CREATIVEGROUPTWO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeGroupAssignmentCreativeGroupNumberEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeOptimizationConfigurationOptimizationModelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optimization model for this configuration.
pub enum CreativeOptimizationConfigurationOptimizationModelEnum {
    
    /// "CLICK"
    #[serde(rename="CLICK")]
    CLICK,
    
    /// "POST_CLICK"
    #[serde(rename="POST_CLICK")]
    POSTCLICK,
    
    /// "POST_IMPRESSION"
    #[serde(rename="POST_IMPRESSION")]
    POSTIMPRESSION,
    
    /// "POST_CLICK_AND_IMPRESSION"
    #[serde(rename="POST_CLICK_AND_IMPRESSION")]
    POSTCLICKANDIMPRESSION,
    
    /// "VIDEO_COMPLETION"
    #[serde(rename="VIDEO_COMPLETION")]
    VIDEOCOMPLETION,
}

impl AsRef<str> for CreativeOptimizationConfigurationOptimizationModelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeOptimizationConfigurationOptimizationModelEnum::CLICK => "CLICK",
            CreativeOptimizationConfigurationOptimizationModelEnum::POSTCLICK => "POST_CLICK",
            CreativeOptimizationConfigurationOptimizationModelEnum::POSTIMPRESSION => "POST_IMPRESSION",
            CreativeOptimizationConfigurationOptimizationModelEnum::POSTCLICKANDIMPRESSION => "POST_CLICK_AND_IMPRESSION",
            CreativeOptimizationConfigurationOptimizationModelEnum::VIDEOCOMPLETION => "VIDEO_COMPLETION",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeOptimizationConfigurationOptimizationModelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLICK" => Ok(CreativeOptimizationConfigurationOptimizationModelEnum::CLICK),
           "POST_CLICK" => Ok(CreativeOptimizationConfigurationOptimizationModelEnum::POSTCLICK),
           "POST_IMPRESSION" => Ok(CreativeOptimizationConfigurationOptimizationModelEnum::POSTIMPRESSION),
           "POST_CLICK_AND_IMPRESSION" => Ok(CreativeOptimizationConfigurationOptimizationModelEnum::POSTCLICKANDIMPRESSION),
           "VIDEO_COMPLETION" => Ok(CreativeOptimizationConfigurationOptimizationModelEnum::VIDEOCOMPLETION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeOptimizationConfigurationOptimizationModelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeRotationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of creative rotation. Can be used to specify whether to use sequential or random rotation.
pub enum CreativeRotationTypeEnum {
    
    /// "CREATIVE_ROTATION_TYPE_SEQUENTIAL"
    #[serde(rename="CREATIVE_ROTATION_TYPE_SEQUENTIAL")]
    CREATIVEROTATIONTYPESEQUENTIAL,
    
    /// "CREATIVE_ROTATION_TYPE_RANDOM"
    #[serde(rename="CREATIVE_ROTATION_TYPE_RANDOM")]
    CREATIVEROTATIONTYPERANDOM,
}

impl AsRef<str> for CreativeRotationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRotationTypeEnum::CREATIVEROTATIONTYPESEQUENTIAL => "CREATIVE_ROTATION_TYPE_SEQUENTIAL",
            CreativeRotationTypeEnum::CREATIVEROTATIONTYPERANDOM => "CREATIVE_ROTATION_TYPE_RANDOM",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRotationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_ROTATION_TYPE_SEQUENTIAL" => Ok(CreativeRotationTypeEnum::CREATIVEROTATIONTYPESEQUENTIAL),
           "CREATIVE_ROTATION_TYPE_RANDOM" => Ok(CreativeRotationTypeEnum::CREATIVEROTATIONTYPERANDOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeRotationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeRotationWeightCalculationStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Strategy for calculating weights. Used with CREATIVE_ROTATION_TYPE_RANDOM.
pub enum CreativeRotationWeightCalculationStrategyEnum {
    
    /// "WEIGHT_STRATEGY_EQUAL"
    #[serde(rename="WEIGHT_STRATEGY_EQUAL")]
    WEIGHTSTRATEGYEQUAL,
    
    /// "WEIGHT_STRATEGY_CUSTOM"
    #[serde(rename="WEIGHT_STRATEGY_CUSTOM")]
    WEIGHTSTRATEGYCUSTOM,
    
    /// "WEIGHT_STRATEGY_HIGHEST_CTR"
    #[serde(rename="WEIGHT_STRATEGY_HIGHEST_CTR")]
    WEIGHTSTRATEGYHIGHESTCTR,
    
    /// "WEIGHT_STRATEGY_OPTIMIZED"
    #[serde(rename="WEIGHT_STRATEGY_OPTIMIZED")]
    WEIGHTSTRATEGYOPTIMIZED,
}

impl AsRef<str> for CreativeRotationWeightCalculationStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRotationWeightCalculationStrategyEnum::WEIGHTSTRATEGYEQUAL => "WEIGHT_STRATEGY_EQUAL",
            CreativeRotationWeightCalculationStrategyEnum::WEIGHTSTRATEGYCUSTOM => "WEIGHT_STRATEGY_CUSTOM",
            CreativeRotationWeightCalculationStrategyEnum::WEIGHTSTRATEGYHIGHESTCTR => "WEIGHT_STRATEGY_HIGHEST_CTR",
            CreativeRotationWeightCalculationStrategyEnum::WEIGHTSTRATEGYOPTIMIZED => "WEIGHT_STRATEGY_OPTIMIZED",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRotationWeightCalculationStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEIGHT_STRATEGY_EQUAL" => Ok(CreativeRotationWeightCalculationStrategyEnum::WEIGHTSTRATEGYEQUAL),
           "WEIGHT_STRATEGY_CUSTOM" => Ok(CreativeRotationWeightCalculationStrategyEnum::WEIGHTSTRATEGYCUSTOM),
           "WEIGHT_STRATEGY_HIGHEST_CTR" => Ok(CreativeRotationWeightCalculationStrategyEnum::WEIGHTSTRATEGYHIGHESTCTR),
           "WEIGHT_STRATEGY_OPTIMIZED" => Ok(CreativeRotationWeightCalculationStrategyEnum::WEIGHTSTRATEGYOPTIMIZED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeRotationWeightCalculationStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomFloodlightVariableTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of custom floodlight variable to supply a value for. These map to the "u[1-20]=" in the tags.
pub enum CustomFloodlightVariableTypeEnum {
    
    /// "U1"
    #[serde(rename="U1")]
    U1,
    
    /// "U2"
    #[serde(rename="U2")]
    U2,
    
    /// "U3"
    #[serde(rename="U3")]
    U3,
    
    /// "U4"
    #[serde(rename="U4")]
    U4,
    
    /// "U5"
    #[serde(rename="U5")]
    U5,
    
    /// "U6"
    #[serde(rename="U6")]
    U6,
    
    /// "U7"
    #[serde(rename="U7")]
    U7,
    
    /// "U8"
    #[serde(rename="U8")]
    U8,
    
    /// "U9"
    #[serde(rename="U9")]
    U9,
    
    /// "U10"
    #[serde(rename="U10")]
    U10,
    
    /// "U11"
    #[serde(rename="U11")]
    U11,
    
    /// "U12"
    #[serde(rename="U12")]
    U12,
    
    /// "U13"
    #[serde(rename="U13")]
    U13,
    
    /// "U14"
    #[serde(rename="U14")]
    U14,
    
    /// "U15"
    #[serde(rename="U15")]
    U15,
    
    /// "U16"
    #[serde(rename="U16")]
    U16,
    
    /// "U17"
    #[serde(rename="U17")]
    U17,
    
    /// "U18"
    #[serde(rename="U18")]
    U18,
    
    /// "U19"
    #[serde(rename="U19")]
    U19,
    
    /// "U20"
    #[serde(rename="U20")]
    U20,
    
    /// "U21"
    #[serde(rename="U21")]
    U21,
    
    /// "U22"
    #[serde(rename="U22")]
    U22,
    
    /// "U23"
    #[serde(rename="U23")]
    U23,
    
    /// "U24"
    #[serde(rename="U24")]
    U24,
    
    /// "U25"
    #[serde(rename="U25")]
    U25,
    
    /// "U26"
    #[serde(rename="U26")]
    U26,
    
    /// "U27"
    #[serde(rename="U27")]
    U27,
    
    /// "U28"
    #[serde(rename="U28")]
    U28,
    
    /// "U29"
    #[serde(rename="U29")]
    U29,
    
    /// "U30"
    #[serde(rename="U30")]
    U30,
    
    /// "U31"
    #[serde(rename="U31")]
    U31,
    
    /// "U32"
    #[serde(rename="U32")]
    U32,
    
    /// "U33"
    #[serde(rename="U33")]
    U33,
    
    /// "U34"
    #[serde(rename="U34")]
    U34,
    
    /// "U35"
    #[serde(rename="U35")]
    U35,
    
    /// "U36"
    #[serde(rename="U36")]
    U36,
    
    /// "U37"
    #[serde(rename="U37")]
    U37,
    
    /// "U38"
    #[serde(rename="U38")]
    U38,
    
    /// "U39"
    #[serde(rename="U39")]
    U39,
    
    /// "U40"
    #[serde(rename="U40")]
    U40,
    
    /// "U41"
    #[serde(rename="U41")]
    U41,
    
    /// "U42"
    #[serde(rename="U42")]
    U42,
    
    /// "U43"
    #[serde(rename="U43")]
    U43,
    
    /// "U44"
    #[serde(rename="U44")]
    U44,
    
    /// "U45"
    #[serde(rename="U45")]
    U45,
    
    /// "U46"
    #[serde(rename="U46")]
    U46,
    
    /// "U47"
    #[serde(rename="U47")]
    U47,
    
    /// "U48"
    #[serde(rename="U48")]
    U48,
    
    /// "U49"
    #[serde(rename="U49")]
    U49,
    
    /// "U50"
    #[serde(rename="U50")]
    U50,
    
    /// "U51"
    #[serde(rename="U51")]
    U51,
    
    /// "U52"
    #[serde(rename="U52")]
    U52,
    
    /// "U53"
    #[serde(rename="U53")]
    U53,
    
    /// "U54"
    #[serde(rename="U54")]
    U54,
    
    /// "U55"
    #[serde(rename="U55")]
    U55,
    
    /// "U56"
    #[serde(rename="U56")]
    U56,
    
    /// "U57"
    #[serde(rename="U57")]
    U57,
    
    /// "U58"
    #[serde(rename="U58")]
    U58,
    
    /// "U59"
    #[serde(rename="U59")]
    U59,
    
    /// "U60"
    #[serde(rename="U60")]
    U60,
    
    /// "U61"
    #[serde(rename="U61")]
    U61,
    
    /// "U62"
    #[serde(rename="U62")]
    U62,
    
    /// "U63"
    #[serde(rename="U63")]
    U63,
    
    /// "U64"
    #[serde(rename="U64")]
    U64,
    
    /// "U65"
    #[serde(rename="U65")]
    U65,
    
    /// "U66"
    #[serde(rename="U66")]
    U66,
    
    /// "U67"
    #[serde(rename="U67")]
    U67,
    
    /// "U68"
    #[serde(rename="U68")]
    U68,
    
    /// "U69"
    #[serde(rename="U69")]
    U69,
    
    /// "U70"
    #[serde(rename="U70")]
    U70,
    
    /// "U71"
    #[serde(rename="U71")]
    U71,
    
    /// "U72"
    #[serde(rename="U72")]
    U72,
    
    /// "U73"
    #[serde(rename="U73")]
    U73,
    
    /// "U74"
    #[serde(rename="U74")]
    U74,
    
    /// "U75"
    #[serde(rename="U75")]
    U75,
    
    /// "U76"
    #[serde(rename="U76")]
    U76,
    
    /// "U77"
    #[serde(rename="U77")]
    U77,
    
    /// "U78"
    #[serde(rename="U78")]
    U78,
    
    /// "U79"
    #[serde(rename="U79")]
    U79,
    
    /// "U80"
    #[serde(rename="U80")]
    U80,
    
    /// "U81"
    #[serde(rename="U81")]
    U81,
    
    /// "U82"
    #[serde(rename="U82")]
    U82,
    
    /// "U83"
    #[serde(rename="U83")]
    U83,
    
    /// "U84"
    #[serde(rename="U84")]
    U84,
    
    /// "U85"
    #[serde(rename="U85")]
    U85,
    
    /// "U86"
    #[serde(rename="U86")]
    U86,
    
    /// "U87"
    #[serde(rename="U87")]
    U87,
    
    /// "U88"
    #[serde(rename="U88")]
    U88,
    
    /// "U89"
    #[serde(rename="U89")]
    U89,
    
    /// "U90"
    #[serde(rename="U90")]
    U90,
    
    /// "U91"
    #[serde(rename="U91")]
    U91,
    
    /// "U92"
    #[serde(rename="U92")]
    U92,
    
    /// "U93"
    #[serde(rename="U93")]
    U93,
    
    /// "U94"
    #[serde(rename="U94")]
    U94,
    
    /// "U95"
    #[serde(rename="U95")]
    U95,
    
    /// "U96"
    #[serde(rename="U96")]
    U96,
    
    /// "U97"
    #[serde(rename="U97")]
    U97,
    
    /// "U98"
    #[serde(rename="U98")]
    U98,
    
    /// "U99"
    #[serde(rename="U99")]
    U99,
    
    /// "U100"
    #[serde(rename="U100")]
    U100,
}

impl AsRef<str> for CustomFloodlightVariableTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomFloodlightVariableTypeEnum::U1 => "U1",
            CustomFloodlightVariableTypeEnum::U2 => "U2",
            CustomFloodlightVariableTypeEnum::U3 => "U3",
            CustomFloodlightVariableTypeEnum::U4 => "U4",
            CustomFloodlightVariableTypeEnum::U5 => "U5",
            CustomFloodlightVariableTypeEnum::U6 => "U6",
            CustomFloodlightVariableTypeEnum::U7 => "U7",
            CustomFloodlightVariableTypeEnum::U8 => "U8",
            CustomFloodlightVariableTypeEnum::U9 => "U9",
            CustomFloodlightVariableTypeEnum::U10 => "U10",
            CustomFloodlightVariableTypeEnum::U11 => "U11",
            CustomFloodlightVariableTypeEnum::U12 => "U12",
            CustomFloodlightVariableTypeEnum::U13 => "U13",
            CustomFloodlightVariableTypeEnum::U14 => "U14",
            CustomFloodlightVariableTypeEnum::U15 => "U15",
            CustomFloodlightVariableTypeEnum::U16 => "U16",
            CustomFloodlightVariableTypeEnum::U17 => "U17",
            CustomFloodlightVariableTypeEnum::U18 => "U18",
            CustomFloodlightVariableTypeEnum::U19 => "U19",
            CustomFloodlightVariableTypeEnum::U20 => "U20",
            CustomFloodlightVariableTypeEnum::U21 => "U21",
            CustomFloodlightVariableTypeEnum::U22 => "U22",
            CustomFloodlightVariableTypeEnum::U23 => "U23",
            CustomFloodlightVariableTypeEnum::U24 => "U24",
            CustomFloodlightVariableTypeEnum::U25 => "U25",
            CustomFloodlightVariableTypeEnum::U26 => "U26",
            CustomFloodlightVariableTypeEnum::U27 => "U27",
            CustomFloodlightVariableTypeEnum::U28 => "U28",
            CustomFloodlightVariableTypeEnum::U29 => "U29",
            CustomFloodlightVariableTypeEnum::U30 => "U30",
            CustomFloodlightVariableTypeEnum::U31 => "U31",
            CustomFloodlightVariableTypeEnum::U32 => "U32",
            CustomFloodlightVariableTypeEnum::U33 => "U33",
            CustomFloodlightVariableTypeEnum::U34 => "U34",
            CustomFloodlightVariableTypeEnum::U35 => "U35",
            CustomFloodlightVariableTypeEnum::U36 => "U36",
            CustomFloodlightVariableTypeEnum::U37 => "U37",
            CustomFloodlightVariableTypeEnum::U38 => "U38",
            CustomFloodlightVariableTypeEnum::U39 => "U39",
            CustomFloodlightVariableTypeEnum::U40 => "U40",
            CustomFloodlightVariableTypeEnum::U41 => "U41",
            CustomFloodlightVariableTypeEnum::U42 => "U42",
            CustomFloodlightVariableTypeEnum::U43 => "U43",
            CustomFloodlightVariableTypeEnum::U44 => "U44",
            CustomFloodlightVariableTypeEnum::U45 => "U45",
            CustomFloodlightVariableTypeEnum::U46 => "U46",
            CustomFloodlightVariableTypeEnum::U47 => "U47",
            CustomFloodlightVariableTypeEnum::U48 => "U48",
            CustomFloodlightVariableTypeEnum::U49 => "U49",
            CustomFloodlightVariableTypeEnum::U50 => "U50",
            CustomFloodlightVariableTypeEnum::U51 => "U51",
            CustomFloodlightVariableTypeEnum::U52 => "U52",
            CustomFloodlightVariableTypeEnum::U53 => "U53",
            CustomFloodlightVariableTypeEnum::U54 => "U54",
            CustomFloodlightVariableTypeEnum::U55 => "U55",
            CustomFloodlightVariableTypeEnum::U56 => "U56",
            CustomFloodlightVariableTypeEnum::U57 => "U57",
            CustomFloodlightVariableTypeEnum::U58 => "U58",
            CustomFloodlightVariableTypeEnum::U59 => "U59",
            CustomFloodlightVariableTypeEnum::U60 => "U60",
            CustomFloodlightVariableTypeEnum::U61 => "U61",
            CustomFloodlightVariableTypeEnum::U62 => "U62",
            CustomFloodlightVariableTypeEnum::U63 => "U63",
            CustomFloodlightVariableTypeEnum::U64 => "U64",
            CustomFloodlightVariableTypeEnum::U65 => "U65",
            CustomFloodlightVariableTypeEnum::U66 => "U66",
            CustomFloodlightVariableTypeEnum::U67 => "U67",
            CustomFloodlightVariableTypeEnum::U68 => "U68",
            CustomFloodlightVariableTypeEnum::U69 => "U69",
            CustomFloodlightVariableTypeEnum::U70 => "U70",
            CustomFloodlightVariableTypeEnum::U71 => "U71",
            CustomFloodlightVariableTypeEnum::U72 => "U72",
            CustomFloodlightVariableTypeEnum::U73 => "U73",
            CustomFloodlightVariableTypeEnum::U74 => "U74",
            CustomFloodlightVariableTypeEnum::U75 => "U75",
            CustomFloodlightVariableTypeEnum::U76 => "U76",
            CustomFloodlightVariableTypeEnum::U77 => "U77",
            CustomFloodlightVariableTypeEnum::U78 => "U78",
            CustomFloodlightVariableTypeEnum::U79 => "U79",
            CustomFloodlightVariableTypeEnum::U80 => "U80",
            CustomFloodlightVariableTypeEnum::U81 => "U81",
            CustomFloodlightVariableTypeEnum::U82 => "U82",
            CustomFloodlightVariableTypeEnum::U83 => "U83",
            CustomFloodlightVariableTypeEnum::U84 => "U84",
            CustomFloodlightVariableTypeEnum::U85 => "U85",
            CustomFloodlightVariableTypeEnum::U86 => "U86",
            CustomFloodlightVariableTypeEnum::U87 => "U87",
            CustomFloodlightVariableTypeEnum::U88 => "U88",
            CustomFloodlightVariableTypeEnum::U89 => "U89",
            CustomFloodlightVariableTypeEnum::U90 => "U90",
            CustomFloodlightVariableTypeEnum::U91 => "U91",
            CustomFloodlightVariableTypeEnum::U92 => "U92",
            CustomFloodlightVariableTypeEnum::U93 => "U93",
            CustomFloodlightVariableTypeEnum::U94 => "U94",
            CustomFloodlightVariableTypeEnum::U95 => "U95",
            CustomFloodlightVariableTypeEnum::U96 => "U96",
            CustomFloodlightVariableTypeEnum::U97 => "U97",
            CustomFloodlightVariableTypeEnum::U98 => "U98",
            CustomFloodlightVariableTypeEnum::U99 => "U99",
            CustomFloodlightVariableTypeEnum::U100 => "U100",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomFloodlightVariableTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "U1" => Ok(CustomFloodlightVariableTypeEnum::U1),
           "U2" => Ok(CustomFloodlightVariableTypeEnum::U2),
           "U3" => Ok(CustomFloodlightVariableTypeEnum::U3),
           "U4" => Ok(CustomFloodlightVariableTypeEnum::U4),
           "U5" => Ok(CustomFloodlightVariableTypeEnum::U5),
           "U6" => Ok(CustomFloodlightVariableTypeEnum::U6),
           "U7" => Ok(CustomFloodlightVariableTypeEnum::U7),
           "U8" => Ok(CustomFloodlightVariableTypeEnum::U8),
           "U9" => Ok(CustomFloodlightVariableTypeEnum::U9),
           "U10" => Ok(CustomFloodlightVariableTypeEnum::U10),
           "U11" => Ok(CustomFloodlightVariableTypeEnum::U11),
           "U12" => Ok(CustomFloodlightVariableTypeEnum::U12),
           "U13" => Ok(CustomFloodlightVariableTypeEnum::U13),
           "U14" => Ok(CustomFloodlightVariableTypeEnum::U14),
           "U15" => Ok(CustomFloodlightVariableTypeEnum::U15),
           "U16" => Ok(CustomFloodlightVariableTypeEnum::U16),
           "U17" => Ok(CustomFloodlightVariableTypeEnum::U17),
           "U18" => Ok(CustomFloodlightVariableTypeEnum::U18),
           "U19" => Ok(CustomFloodlightVariableTypeEnum::U19),
           "U20" => Ok(CustomFloodlightVariableTypeEnum::U20),
           "U21" => Ok(CustomFloodlightVariableTypeEnum::U21),
           "U22" => Ok(CustomFloodlightVariableTypeEnum::U22),
           "U23" => Ok(CustomFloodlightVariableTypeEnum::U23),
           "U24" => Ok(CustomFloodlightVariableTypeEnum::U24),
           "U25" => Ok(CustomFloodlightVariableTypeEnum::U25),
           "U26" => Ok(CustomFloodlightVariableTypeEnum::U26),
           "U27" => Ok(CustomFloodlightVariableTypeEnum::U27),
           "U28" => Ok(CustomFloodlightVariableTypeEnum::U28),
           "U29" => Ok(CustomFloodlightVariableTypeEnum::U29),
           "U30" => Ok(CustomFloodlightVariableTypeEnum::U30),
           "U31" => Ok(CustomFloodlightVariableTypeEnum::U31),
           "U32" => Ok(CustomFloodlightVariableTypeEnum::U32),
           "U33" => Ok(CustomFloodlightVariableTypeEnum::U33),
           "U34" => Ok(CustomFloodlightVariableTypeEnum::U34),
           "U35" => Ok(CustomFloodlightVariableTypeEnum::U35),
           "U36" => Ok(CustomFloodlightVariableTypeEnum::U36),
           "U37" => Ok(CustomFloodlightVariableTypeEnum::U37),
           "U38" => Ok(CustomFloodlightVariableTypeEnum::U38),
           "U39" => Ok(CustomFloodlightVariableTypeEnum::U39),
           "U40" => Ok(CustomFloodlightVariableTypeEnum::U40),
           "U41" => Ok(CustomFloodlightVariableTypeEnum::U41),
           "U42" => Ok(CustomFloodlightVariableTypeEnum::U42),
           "U43" => Ok(CustomFloodlightVariableTypeEnum::U43),
           "U44" => Ok(CustomFloodlightVariableTypeEnum::U44),
           "U45" => Ok(CustomFloodlightVariableTypeEnum::U45),
           "U46" => Ok(CustomFloodlightVariableTypeEnum::U46),
           "U47" => Ok(CustomFloodlightVariableTypeEnum::U47),
           "U48" => Ok(CustomFloodlightVariableTypeEnum::U48),
           "U49" => Ok(CustomFloodlightVariableTypeEnum::U49),
           "U50" => Ok(CustomFloodlightVariableTypeEnum::U50),
           "U51" => Ok(CustomFloodlightVariableTypeEnum::U51),
           "U52" => Ok(CustomFloodlightVariableTypeEnum::U52),
           "U53" => Ok(CustomFloodlightVariableTypeEnum::U53),
           "U54" => Ok(CustomFloodlightVariableTypeEnum::U54),
           "U55" => Ok(CustomFloodlightVariableTypeEnum::U55),
           "U56" => Ok(CustomFloodlightVariableTypeEnum::U56),
           "U57" => Ok(CustomFloodlightVariableTypeEnum::U57),
           "U58" => Ok(CustomFloodlightVariableTypeEnum::U58),
           "U59" => Ok(CustomFloodlightVariableTypeEnum::U59),
           "U60" => Ok(CustomFloodlightVariableTypeEnum::U60),
           "U61" => Ok(CustomFloodlightVariableTypeEnum::U61),
           "U62" => Ok(CustomFloodlightVariableTypeEnum::U62),
           "U63" => Ok(CustomFloodlightVariableTypeEnum::U63),
           "U64" => Ok(CustomFloodlightVariableTypeEnum::U64),
           "U65" => Ok(CustomFloodlightVariableTypeEnum::U65),
           "U66" => Ok(CustomFloodlightVariableTypeEnum::U66),
           "U67" => Ok(CustomFloodlightVariableTypeEnum::U67),
           "U68" => Ok(CustomFloodlightVariableTypeEnum::U68),
           "U69" => Ok(CustomFloodlightVariableTypeEnum::U69),
           "U70" => Ok(CustomFloodlightVariableTypeEnum::U70),
           "U71" => Ok(CustomFloodlightVariableTypeEnum::U71),
           "U72" => Ok(CustomFloodlightVariableTypeEnum::U72),
           "U73" => Ok(CustomFloodlightVariableTypeEnum::U73),
           "U74" => Ok(CustomFloodlightVariableTypeEnum::U74),
           "U75" => Ok(CustomFloodlightVariableTypeEnum::U75),
           "U76" => Ok(CustomFloodlightVariableTypeEnum::U76),
           "U77" => Ok(CustomFloodlightVariableTypeEnum::U77),
           "U78" => Ok(CustomFloodlightVariableTypeEnum::U78),
           "U79" => Ok(CustomFloodlightVariableTypeEnum::U79),
           "U80" => Ok(CustomFloodlightVariableTypeEnum::U80),
           "U81" => Ok(CustomFloodlightVariableTypeEnum::U81),
           "U82" => Ok(CustomFloodlightVariableTypeEnum::U82),
           "U83" => Ok(CustomFloodlightVariableTypeEnum::U83),
           "U84" => Ok(CustomFloodlightVariableTypeEnum::U84),
           "U85" => Ok(CustomFloodlightVariableTypeEnum::U85),
           "U86" => Ok(CustomFloodlightVariableTypeEnum::U86),
           "U87" => Ok(CustomFloodlightVariableTypeEnum::U87),
           "U88" => Ok(CustomFloodlightVariableTypeEnum::U88),
           "U89" => Ok(CustomFloodlightVariableTypeEnum::U89),
           "U90" => Ok(CustomFloodlightVariableTypeEnum::U90),
           "U91" => Ok(CustomFloodlightVariableTypeEnum::U91),
           "U92" => Ok(CustomFloodlightVariableTypeEnum::U92),
           "U93" => Ok(CustomFloodlightVariableTypeEnum::U93),
           "U94" => Ok(CustomFloodlightVariableTypeEnum::U94),
           "U95" => Ok(CustomFloodlightVariableTypeEnum::U95),
           "U96" => Ok(CustomFloodlightVariableTypeEnum::U96),
           "U97" => Ok(CustomFloodlightVariableTypeEnum::U97),
           "U98" => Ok(CustomFloodlightVariableTypeEnum::U98),
           "U99" => Ok(CustomFloodlightVariableTypeEnum::U99),
           "U100" => Ok(CustomFloodlightVariableTypeEnum::U100),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomFloodlightVariableTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DateRangeRelativeDateRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The date range relative to the date of when the report is run.
pub enum DateRangeRelativeDateRangeEnum {
    
    /// "TODAY"
    #[serde(rename="TODAY")]
    TODAY,
    
    /// "YESTERDAY"
    #[serde(rename="YESTERDAY")]
    YESTERDAY,
    
    /// "WEEK_TO_DATE"
    #[serde(rename="WEEK_TO_DATE")]
    WEEKTODATE,
    
    /// "MONTH_TO_DATE"
    #[serde(rename="MONTH_TO_DATE")]
    MONTHTODATE,
    
    /// "QUARTER_TO_DATE"
    #[serde(rename="QUARTER_TO_DATE")]
    QUARTERTODATE,
    
    /// "YEAR_TO_DATE"
    #[serde(rename="YEAR_TO_DATE")]
    YEARTODATE,
    
    /// "PREVIOUS_WEEK"
    #[serde(rename="PREVIOUS_WEEK")]
    PREVIOUSWEEK,
    
    /// "PREVIOUS_MONTH"
    #[serde(rename="PREVIOUS_MONTH")]
    PREVIOUSMONTH,
    
    /// "PREVIOUS_QUARTER"
    #[serde(rename="PREVIOUS_QUARTER")]
    PREVIOUSQUARTER,
    
    /// "PREVIOUS_YEAR"
    #[serde(rename="PREVIOUS_YEAR")]
    PREVIOUSYEAR,
    
    /// "LAST_7_DAYS"
    #[serde(rename="LAST_7_DAYS")]
    LAST7DAYS,
    
    /// "LAST_30_DAYS"
    #[serde(rename="LAST_30_DAYS")]
    LAST30DAYS,
    
    /// "LAST_90_DAYS"
    #[serde(rename="LAST_90_DAYS")]
    LAST90DAYS,
    
    /// "LAST_365_DAYS"
    #[serde(rename="LAST_365_DAYS")]
    LAST365DAYS,
    
    /// "LAST_24_MONTHS"
    #[serde(rename="LAST_24_MONTHS")]
    LAST24MONTHS,
    
    /// "LAST_14_DAYS"
    #[serde(rename="LAST_14_DAYS")]
    LAST14DAYS,
    
    /// "LAST_60_DAYS"
    #[serde(rename="LAST_60_DAYS")]
    LAST60DAYS,
}

impl AsRef<str> for DateRangeRelativeDateRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DateRangeRelativeDateRangeEnum::TODAY => "TODAY",
            DateRangeRelativeDateRangeEnum::YESTERDAY => "YESTERDAY",
            DateRangeRelativeDateRangeEnum::WEEKTODATE => "WEEK_TO_DATE",
            DateRangeRelativeDateRangeEnum::MONTHTODATE => "MONTH_TO_DATE",
            DateRangeRelativeDateRangeEnum::QUARTERTODATE => "QUARTER_TO_DATE",
            DateRangeRelativeDateRangeEnum::YEARTODATE => "YEAR_TO_DATE",
            DateRangeRelativeDateRangeEnum::PREVIOUSWEEK => "PREVIOUS_WEEK",
            DateRangeRelativeDateRangeEnum::PREVIOUSMONTH => "PREVIOUS_MONTH",
            DateRangeRelativeDateRangeEnum::PREVIOUSQUARTER => "PREVIOUS_QUARTER",
            DateRangeRelativeDateRangeEnum::PREVIOUSYEAR => "PREVIOUS_YEAR",
            DateRangeRelativeDateRangeEnum::LAST7DAYS => "LAST_7_DAYS",
            DateRangeRelativeDateRangeEnum::LAST30DAYS => "LAST_30_DAYS",
            DateRangeRelativeDateRangeEnum::LAST90DAYS => "LAST_90_DAYS",
            DateRangeRelativeDateRangeEnum::LAST365DAYS => "LAST_365_DAYS",
            DateRangeRelativeDateRangeEnum::LAST24MONTHS => "LAST_24_MONTHS",
            DateRangeRelativeDateRangeEnum::LAST14DAYS => "LAST_14_DAYS",
            DateRangeRelativeDateRangeEnum::LAST60DAYS => "LAST_60_DAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for DateRangeRelativeDateRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TODAY" => Ok(DateRangeRelativeDateRangeEnum::TODAY),
           "YESTERDAY" => Ok(DateRangeRelativeDateRangeEnum::YESTERDAY),
           "WEEK_TO_DATE" => Ok(DateRangeRelativeDateRangeEnum::WEEKTODATE),
           "MONTH_TO_DATE" => Ok(DateRangeRelativeDateRangeEnum::MONTHTODATE),
           "QUARTER_TO_DATE" => Ok(DateRangeRelativeDateRangeEnum::QUARTERTODATE),
           "YEAR_TO_DATE" => Ok(DateRangeRelativeDateRangeEnum::YEARTODATE),
           "PREVIOUS_WEEK" => Ok(DateRangeRelativeDateRangeEnum::PREVIOUSWEEK),
           "PREVIOUS_MONTH" => Ok(DateRangeRelativeDateRangeEnum::PREVIOUSMONTH),
           "PREVIOUS_QUARTER" => Ok(DateRangeRelativeDateRangeEnum::PREVIOUSQUARTER),
           "PREVIOUS_YEAR" => Ok(DateRangeRelativeDateRangeEnum::PREVIOUSYEAR),
           "LAST_7_DAYS" => Ok(DateRangeRelativeDateRangeEnum::LAST7DAYS),
           "LAST_30_DAYS" => Ok(DateRangeRelativeDateRangeEnum::LAST30DAYS),
           "LAST_90_DAYS" => Ok(DateRangeRelativeDateRangeEnum::LAST90DAYS),
           "LAST_365_DAYS" => Ok(DateRangeRelativeDateRangeEnum::LAST365DAYS),
           "LAST_24_MONTHS" => Ok(DateRangeRelativeDateRangeEnum::LAST24MONTHS),
           "LAST_14_DAYS" => Ok(DateRangeRelativeDateRangeEnum::LAST14DAYS),
           "LAST_60_DAYS" => Ok(DateRangeRelativeDateRangeEnum::LAST60DAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DateRangeRelativeDateRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DayPartTargetingDaysOfWeekEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Days of the week when the ad will serve. Acceptable values are: - "SUNDAY" - "MONDAY" - "TUESDAY" - "WEDNESDAY" - "THURSDAY" - "FRIDAY" - "SATURDAY" 
pub enum DayPartTargetingDaysOfWeekEnum {
    
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for DayPartTargetingDaysOfWeekEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DayPartTargetingDaysOfWeekEnum::MONDAY => "MONDAY",
            DayPartTargetingDaysOfWeekEnum::TUESDAY => "TUESDAY",
            DayPartTargetingDaysOfWeekEnum::WEDNESDAY => "WEDNESDAY",
            DayPartTargetingDaysOfWeekEnum::THURSDAY => "THURSDAY",
            DayPartTargetingDaysOfWeekEnum::FRIDAY => "FRIDAY",
            DayPartTargetingDaysOfWeekEnum::SATURDAY => "SATURDAY",
            DayPartTargetingDaysOfWeekEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for DayPartTargetingDaysOfWeekEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MONDAY" => Ok(DayPartTargetingDaysOfWeekEnum::MONDAY),
           "TUESDAY" => Ok(DayPartTargetingDaysOfWeekEnum::TUESDAY),
           "WEDNESDAY" => Ok(DayPartTargetingDaysOfWeekEnum::WEDNESDAY),
           "THURSDAY" => Ok(DayPartTargetingDaysOfWeekEnum::THURSDAY),
           "FRIDAY" => Ok(DayPartTargetingDaysOfWeekEnum::FRIDAY),
           "SATURDAY" => Ok(DayPartTargetingDaysOfWeekEnum::SATURDAY),
           "SUNDAY" => Ok(DayPartTargetingDaysOfWeekEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DayPartTargetingDaysOfWeekEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeliverySchedulePriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Serving priority of an ad, with respect to other ads. The lower the priority number, the greater the priority with which it is served.
pub enum DeliverySchedulePriorityEnum {
    
    /// "AD_PRIORITY_01"
    #[serde(rename="AD_PRIORITY_01")]
    ADPRIORITY01,
    
    /// "AD_PRIORITY_02"
    #[serde(rename="AD_PRIORITY_02")]
    ADPRIORITY02,
    
    /// "AD_PRIORITY_03"
    #[serde(rename="AD_PRIORITY_03")]
    ADPRIORITY03,
    
    /// "AD_PRIORITY_04"
    #[serde(rename="AD_PRIORITY_04")]
    ADPRIORITY04,
    
    /// "AD_PRIORITY_05"
    #[serde(rename="AD_PRIORITY_05")]
    ADPRIORITY05,
    
    /// "AD_PRIORITY_06"
    #[serde(rename="AD_PRIORITY_06")]
    ADPRIORITY06,
    
    /// "AD_PRIORITY_07"
    #[serde(rename="AD_PRIORITY_07")]
    ADPRIORITY07,
    
    /// "AD_PRIORITY_08"
    #[serde(rename="AD_PRIORITY_08")]
    ADPRIORITY08,
    
    /// "AD_PRIORITY_09"
    #[serde(rename="AD_PRIORITY_09")]
    ADPRIORITY09,
    
    /// "AD_PRIORITY_10"
    #[serde(rename="AD_PRIORITY_10")]
    ADPRIORITY10,
    
    /// "AD_PRIORITY_11"
    #[serde(rename="AD_PRIORITY_11")]
    ADPRIORITY11,
    
    /// "AD_PRIORITY_12"
    #[serde(rename="AD_PRIORITY_12")]
    ADPRIORITY12,
    
    /// "AD_PRIORITY_13"
    #[serde(rename="AD_PRIORITY_13")]
    ADPRIORITY13,
    
    /// "AD_PRIORITY_14"
    #[serde(rename="AD_PRIORITY_14")]
    ADPRIORITY14,
    
    /// "AD_PRIORITY_15"
    #[serde(rename="AD_PRIORITY_15")]
    ADPRIORITY15,
    
    /// "AD_PRIORITY_16"
    #[serde(rename="AD_PRIORITY_16")]
    ADPRIORITY16,
}

impl AsRef<str> for DeliverySchedulePriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeliverySchedulePriorityEnum::ADPRIORITY01 => "AD_PRIORITY_01",
            DeliverySchedulePriorityEnum::ADPRIORITY02 => "AD_PRIORITY_02",
            DeliverySchedulePriorityEnum::ADPRIORITY03 => "AD_PRIORITY_03",
            DeliverySchedulePriorityEnum::ADPRIORITY04 => "AD_PRIORITY_04",
            DeliverySchedulePriorityEnum::ADPRIORITY05 => "AD_PRIORITY_05",
            DeliverySchedulePriorityEnum::ADPRIORITY06 => "AD_PRIORITY_06",
            DeliverySchedulePriorityEnum::ADPRIORITY07 => "AD_PRIORITY_07",
            DeliverySchedulePriorityEnum::ADPRIORITY08 => "AD_PRIORITY_08",
            DeliverySchedulePriorityEnum::ADPRIORITY09 => "AD_PRIORITY_09",
            DeliverySchedulePriorityEnum::ADPRIORITY10 => "AD_PRIORITY_10",
            DeliverySchedulePriorityEnum::ADPRIORITY11 => "AD_PRIORITY_11",
            DeliverySchedulePriorityEnum::ADPRIORITY12 => "AD_PRIORITY_12",
            DeliverySchedulePriorityEnum::ADPRIORITY13 => "AD_PRIORITY_13",
            DeliverySchedulePriorityEnum::ADPRIORITY14 => "AD_PRIORITY_14",
            DeliverySchedulePriorityEnum::ADPRIORITY15 => "AD_PRIORITY_15",
            DeliverySchedulePriorityEnum::ADPRIORITY16 => "AD_PRIORITY_16",
        }
    }
}

impl std::convert::TryFrom< &str> for DeliverySchedulePriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AD_PRIORITY_01" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY01),
           "AD_PRIORITY_02" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY02),
           "AD_PRIORITY_03" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY03),
           "AD_PRIORITY_04" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY04),
           "AD_PRIORITY_05" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY05),
           "AD_PRIORITY_06" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY06),
           "AD_PRIORITY_07" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY07),
           "AD_PRIORITY_08" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY08),
           "AD_PRIORITY_09" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY09),
           "AD_PRIORITY_10" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY10),
           "AD_PRIORITY_11" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY11),
           "AD_PRIORITY_12" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY12),
           "AD_PRIORITY_13" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY13),
           "AD_PRIORITY_14" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY14),
           "AD_PRIORITY_15" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY15),
           "AD_PRIORITY_16" => Ok(DeliverySchedulePriorityEnum::ADPRIORITY16),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeliverySchedulePriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DimensionValueMatchTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how the 'value' field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, '*' is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions ('dfa:paidSearch*') allow a matchType other than EXACT.
pub enum DimensionValueMatchTypeEnum {
    
    /// "EXACT"
    #[serde(rename="EXACT")]
    EXACT,
    
    /// "BEGINS_WITH"
    #[serde(rename="BEGINS_WITH")]
    BEGINSWITH,
    
    /// "CONTAINS"
    #[serde(rename="CONTAINS")]
    CONTAINS,
    
    /// "WILDCARD_EXPRESSION"
    #[serde(rename="WILDCARD_EXPRESSION")]
    WILDCARDEXPRESSION,
}

impl AsRef<str> for DimensionValueMatchTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DimensionValueMatchTypeEnum::EXACT => "EXACT",
            DimensionValueMatchTypeEnum::BEGINSWITH => "BEGINS_WITH",
            DimensionValueMatchTypeEnum::CONTAINS => "CONTAINS",
            DimensionValueMatchTypeEnum::WILDCARDEXPRESSION => "WILDCARD_EXPRESSION",
        }
    }
}

impl std::convert::TryFrom< &str> for DimensionValueMatchTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXACT" => Ok(DimensionValueMatchTypeEnum::EXACT),
           "BEGINS_WITH" => Ok(DimensionValueMatchTypeEnum::BEGINSWITH),
           "CONTAINS" => Ok(DimensionValueMatchTypeEnum::CONTAINS),
           "WILDCARD_EXPRESSION" => Ok(DimensionValueMatchTypeEnum::WILDCARDEXPRESSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DimensionValueMatchTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DirectorySiteInpageTagFormatsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tag types for regular placements. Acceptable values are: - "STANDARD" - "IFRAME_JAVASCRIPT_INPAGE" - "INTERNAL_REDIRECT_INPAGE" - "JAVASCRIPT_INPAGE" 
pub enum DirectorySiteInpageTagFormatsEnum {
    
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    
    /// "IFRAME_JAVASCRIPT_INPAGE"
    #[serde(rename="IFRAME_JAVASCRIPT_INPAGE")]
    IFRAMEJAVASCRIPTINPAGE,
    
    /// "INTERNAL_REDIRECT_INPAGE"
    #[serde(rename="INTERNAL_REDIRECT_INPAGE")]
    INTERNALREDIRECTINPAGE,
    
    /// "JAVASCRIPT_INPAGE"
    #[serde(rename="JAVASCRIPT_INPAGE")]
    JAVASCRIPTINPAGE,
}

impl AsRef<str> for DirectorySiteInpageTagFormatsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DirectorySiteInpageTagFormatsEnum::STANDARD => "STANDARD",
            DirectorySiteInpageTagFormatsEnum::IFRAMEJAVASCRIPTINPAGE => "IFRAME_JAVASCRIPT_INPAGE",
            DirectorySiteInpageTagFormatsEnum::INTERNALREDIRECTINPAGE => "INTERNAL_REDIRECT_INPAGE",
            DirectorySiteInpageTagFormatsEnum::JAVASCRIPTINPAGE => "JAVASCRIPT_INPAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for DirectorySiteInpageTagFormatsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STANDARD" => Ok(DirectorySiteInpageTagFormatsEnum::STANDARD),
           "IFRAME_JAVASCRIPT_INPAGE" => Ok(DirectorySiteInpageTagFormatsEnum::IFRAMEJAVASCRIPTINPAGE),
           "INTERNAL_REDIRECT_INPAGE" => Ok(DirectorySiteInpageTagFormatsEnum::INTERNALREDIRECTINPAGE),
           "JAVASCRIPT_INPAGE" => Ok(DirectorySiteInpageTagFormatsEnum::JAVASCRIPTINPAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DirectorySiteInpageTagFormatsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DirectorySiteInterstitialTagFormatsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tag types for interstitial placements. Acceptable values are: - "IFRAME_JAVASCRIPT_INTERSTITIAL" - "INTERNAL_REDIRECT_INTERSTITIAL" - "JAVASCRIPT_INTERSTITIAL" 
pub enum DirectorySiteInterstitialTagFormatsEnum {
    
    /// "IFRAME_JAVASCRIPT_INTERSTITIAL"
    #[serde(rename="IFRAME_JAVASCRIPT_INTERSTITIAL")]
    IFRAMEJAVASCRIPTINTERSTITIAL,
    
    /// "INTERNAL_REDIRECT_INTERSTITIAL"
    #[serde(rename="INTERNAL_REDIRECT_INTERSTITIAL")]
    INTERNALREDIRECTINTERSTITIAL,
    
    /// "JAVASCRIPT_INTERSTITIAL"
    #[serde(rename="JAVASCRIPT_INTERSTITIAL")]
    JAVASCRIPTINTERSTITIAL,
}

impl AsRef<str> for DirectorySiteInterstitialTagFormatsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DirectorySiteInterstitialTagFormatsEnum::IFRAMEJAVASCRIPTINTERSTITIAL => "IFRAME_JAVASCRIPT_INTERSTITIAL",
            DirectorySiteInterstitialTagFormatsEnum::INTERNALREDIRECTINTERSTITIAL => "INTERNAL_REDIRECT_INTERSTITIAL",
            DirectorySiteInterstitialTagFormatsEnum::JAVASCRIPTINTERSTITIAL => "JAVASCRIPT_INTERSTITIAL",
        }
    }
}

impl std::convert::TryFrom< &str> for DirectorySiteInterstitialTagFormatsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IFRAME_JAVASCRIPT_INTERSTITIAL" => Ok(DirectorySiteInterstitialTagFormatsEnum::IFRAMEJAVASCRIPTINTERSTITIAL),
           "INTERNAL_REDIRECT_INTERSTITIAL" => Ok(DirectorySiteInterstitialTagFormatsEnum::INTERNALREDIRECTINTERSTITIAL),
           "JAVASCRIPT_INTERSTITIAL" => Ok(DirectorySiteInterstitialTagFormatsEnum::JAVASCRIPTINTERSTITIAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DirectorySiteInterstitialTagFormatsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DynamicTargetingKeyObjectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the object of this dynamic targeting key. This is a required field.
pub enum DynamicTargetingKeyObjectTypeEnum {
    
    /// "OBJECT_ADVERTISER"
    #[serde(rename="OBJECT_ADVERTISER")]
    OBJECTADVERTISER,
    
    /// "OBJECT_AD"
    #[serde(rename="OBJECT_AD")]
    OBJECTAD,
    
    /// "OBJECT_CREATIVE"
    #[serde(rename="OBJECT_CREATIVE")]
    OBJECTCREATIVE,
    
    /// "OBJECT_PLACEMENT"
    #[serde(rename="OBJECT_PLACEMENT")]
    OBJECTPLACEMENT,
}

impl AsRef<str> for DynamicTargetingKeyObjectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DynamicTargetingKeyObjectTypeEnum::OBJECTADVERTISER => "OBJECT_ADVERTISER",
            DynamicTargetingKeyObjectTypeEnum::OBJECTAD => "OBJECT_AD",
            DynamicTargetingKeyObjectTypeEnum::OBJECTCREATIVE => "OBJECT_CREATIVE",
            DynamicTargetingKeyObjectTypeEnum::OBJECTPLACEMENT => "OBJECT_PLACEMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for DynamicTargetingKeyObjectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OBJECT_ADVERTISER" => Ok(DynamicTargetingKeyObjectTypeEnum::OBJECTADVERTISER),
           "OBJECT_AD" => Ok(DynamicTargetingKeyObjectTypeEnum::OBJECTAD),
           "OBJECT_CREATIVE" => Ok(DynamicTargetingKeyObjectTypeEnum::OBJECTCREATIVE),
           "OBJECT_PLACEMENT" => Ok(DynamicTargetingKeyObjectTypeEnum::OBJECTPLACEMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DynamicTargetingKeyObjectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EncryptionInfoEncryptionEntityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The encryption entity type. This should match the encryption configuration for ad serving or Data Transfer.
pub enum EncryptionInfoEncryptionEntityTypeEnum {
    
    /// "ENCRYPTION_ENTITY_TYPE_UNKNOWN"
    #[serde(rename="ENCRYPTION_ENTITY_TYPE_UNKNOWN")]
    ENCRYPTIONENTITYTYPEUNKNOWN,
    
    /// "DCM_ACCOUNT"
    #[serde(rename="DCM_ACCOUNT")]
    DCMACCOUNT,
    
    /// "DCM_ADVERTISER"
    #[serde(rename="DCM_ADVERTISER")]
    DCMADVERTISER,
    
    /// "DBM_PARTNER"
    #[serde(rename="DBM_PARTNER")]
    DBMPARTNER,
    
    /// "DBM_ADVERTISER"
    #[serde(rename="DBM_ADVERTISER")]
    DBMADVERTISER,
    
    /// "ADWORDS_CUSTOMER"
    #[serde(rename="ADWORDS_CUSTOMER")]
    ADWORDSCUSTOMER,
    
    /// "DFP_NETWORK_CODE"
    #[serde(rename="DFP_NETWORK_CODE")]
    DFPNETWORKCODE,
}

impl AsRef<str> for EncryptionInfoEncryptionEntityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EncryptionInfoEncryptionEntityTypeEnum::ENCRYPTIONENTITYTYPEUNKNOWN => "ENCRYPTION_ENTITY_TYPE_UNKNOWN",
            EncryptionInfoEncryptionEntityTypeEnum::DCMACCOUNT => "DCM_ACCOUNT",
            EncryptionInfoEncryptionEntityTypeEnum::DCMADVERTISER => "DCM_ADVERTISER",
            EncryptionInfoEncryptionEntityTypeEnum::DBMPARTNER => "DBM_PARTNER",
            EncryptionInfoEncryptionEntityTypeEnum::DBMADVERTISER => "DBM_ADVERTISER",
            EncryptionInfoEncryptionEntityTypeEnum::ADWORDSCUSTOMER => "ADWORDS_CUSTOMER",
            EncryptionInfoEncryptionEntityTypeEnum::DFPNETWORKCODE => "DFP_NETWORK_CODE",
        }
    }
}

impl std::convert::TryFrom< &str> for EncryptionInfoEncryptionEntityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_ENTITY_TYPE_UNKNOWN" => Ok(EncryptionInfoEncryptionEntityTypeEnum::ENCRYPTIONENTITYTYPEUNKNOWN),
           "DCM_ACCOUNT" => Ok(EncryptionInfoEncryptionEntityTypeEnum::DCMACCOUNT),
           "DCM_ADVERTISER" => Ok(EncryptionInfoEncryptionEntityTypeEnum::DCMADVERTISER),
           "DBM_PARTNER" => Ok(EncryptionInfoEncryptionEntityTypeEnum::DBMPARTNER),
           "DBM_ADVERTISER" => Ok(EncryptionInfoEncryptionEntityTypeEnum::DBMADVERTISER),
           "ADWORDS_CUSTOMER" => Ok(EncryptionInfoEncryptionEntityTypeEnum::ADWORDSCUSTOMER),
           "DFP_NETWORK_CODE" => Ok(EncryptionInfoEncryptionEntityTypeEnum::DFPNETWORKCODE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EncryptionInfoEncryptionEntityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EncryptionInfoEncryptionSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Describes whether the encrypted cookie was received from ad serving (the %m macro) or from Data Transfer.
pub enum EncryptionInfoEncryptionSourceEnum {
    
    /// "ENCRYPTION_SCOPE_UNKNOWN"
    #[serde(rename="ENCRYPTION_SCOPE_UNKNOWN")]
    ENCRYPTIONSCOPEUNKNOWN,
    
    /// "AD_SERVING"
    #[serde(rename="AD_SERVING")]
    ADSERVING,
    
    /// "DATA_TRANSFER"
    #[serde(rename="DATA_TRANSFER")]
    DATATRANSFER,
}

impl AsRef<str> for EncryptionInfoEncryptionSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EncryptionInfoEncryptionSourceEnum::ENCRYPTIONSCOPEUNKNOWN => "ENCRYPTION_SCOPE_UNKNOWN",
            EncryptionInfoEncryptionSourceEnum::ADSERVING => "AD_SERVING",
            EncryptionInfoEncryptionSourceEnum::DATATRANSFER => "DATA_TRANSFER",
        }
    }
}

impl std::convert::TryFrom< &str> for EncryptionInfoEncryptionSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_SCOPE_UNKNOWN" => Ok(EncryptionInfoEncryptionSourceEnum::ENCRYPTIONSCOPEUNKNOWN),
           "AD_SERVING" => Ok(EncryptionInfoEncryptionSourceEnum::ADSERVING),
           "DATA_TRANSFER" => Ok(EncryptionInfoEncryptionSourceEnum::DATATRANSFER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EncryptionInfoEncryptionSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTagSiteFilterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Site filter type for this event tag. If no type is specified then the event tag will be applied to all sites.
pub enum EventTagSiteFilterTypeEnum {
    
    /// "WHITELIST"
    #[serde(rename="WHITELIST")]
    WHITELIST,
    
    /// "BLACKLIST"
    #[serde(rename="BLACKLIST")]
    BLACKLIST,
}

impl AsRef<str> for EventTagSiteFilterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTagSiteFilterTypeEnum::WHITELIST => "WHITELIST",
            EventTagSiteFilterTypeEnum::BLACKLIST => "BLACKLIST",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTagSiteFilterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WHITELIST" => Ok(EventTagSiteFilterTypeEnum::WHITELIST),
           "BLACKLIST" => Ok(EventTagSiteFilterTypeEnum::BLACKLIST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTagSiteFilterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTagStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of this event tag. Must be ENABLED for this event tag to fire. This is a required field.
pub enum EventTagStatusEnum {
    
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for EventTagStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTagStatusEnum::ENABLED => "ENABLED",
            EventTagStatusEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTagStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENABLED" => Ok(EventTagStatusEnum::ENABLED),
           "DISABLED" => Ok(EventTagStatusEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTagStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTagTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Event tag type. Can be used to specify whether to use a third-party pixel, a third-party JavaScript URL, or a third-party click-through URL for either impression or click tracking. This is a required field.
pub enum EventTagTypeEnum {
    
    /// "IMPRESSION_IMAGE_EVENT_TAG"
    #[serde(rename="IMPRESSION_IMAGE_EVENT_TAG")]
    IMPRESSIONIMAGEEVENTTAG,
    
    /// "IMPRESSION_JAVASCRIPT_EVENT_TAG"
    #[serde(rename="IMPRESSION_JAVASCRIPT_EVENT_TAG")]
    IMPRESSIONJAVASCRIPTEVENTTAG,
    
    /// "CLICK_THROUGH_EVENT_TAG"
    #[serde(rename="CLICK_THROUGH_EVENT_TAG")]
    CLICKTHROUGHEVENTTAG,
}

impl AsRef<str> for EventTagTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTagTypeEnum::IMPRESSIONIMAGEEVENTTAG => "IMPRESSION_IMAGE_EVENT_TAG",
            EventTagTypeEnum::IMPRESSIONJAVASCRIPTEVENTTAG => "IMPRESSION_JAVASCRIPT_EVENT_TAG",
            EventTagTypeEnum::CLICKTHROUGHEVENTTAG => "CLICK_THROUGH_EVENT_TAG",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTagTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPRESSION_IMAGE_EVENT_TAG" => Ok(EventTagTypeEnum::IMPRESSIONIMAGEEVENTTAG),
           "IMPRESSION_JAVASCRIPT_EVENT_TAG" => Ok(EventTagTypeEnum::IMPRESSIONJAVASCRIPTEVENTTAG),
           "CLICK_THROUGH_EVENT_TAG" => Ok(EventTagTypeEnum::CLICKTHROUGHEVENTTAG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTagTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FileFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The output format of the report. Only available once the file is available.
pub enum FileFormatEnum {
    
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
    
    /// "EXCEL"
    #[serde(rename="EXCEL")]
    EXCEL,
}

impl AsRef<str> for FileFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileFormatEnum::CSV => "CSV",
            FileFormatEnum::EXCEL => "EXCEL",
        }
    }
}

impl std::convert::TryFrom< &str> for FileFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CSV" => Ok(FileFormatEnum::CSV),
           "EXCEL" => Ok(FileFormatEnum::EXCEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FileStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the report file.
pub enum FileStatusEnum {
    
    /// "PROCESSING"
    #[serde(rename="PROCESSING")]
    PROCESSING,
    
    /// "REPORT_AVAILABLE"
    #[serde(rename="REPORT_AVAILABLE")]
    REPORTAVAILABLE,
    
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for FileStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileStatusEnum::PROCESSING => "PROCESSING",
            FileStatusEnum::REPORTAVAILABLE => "REPORT_AVAILABLE",
            FileStatusEnum::FAILED => "FAILED",
            FileStatusEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for FileStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROCESSING" => Ok(FileStatusEnum::PROCESSING),
           "REPORT_AVAILABLE" => Ok(FileStatusEnum::REPORTAVAILABLE),
           "FAILED" => Ok(FileStatusEnum::FAILED),
           "CANCELLED" => Ok(FileStatusEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightActivityCacheBustingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Code type used for cache busting in the generated tag. Applicable only when floodlightActivityGroupType is COUNTER and countingMethod is STANDARD_COUNTING or UNIQUE_COUNTING.
pub enum FloodlightActivityCacheBustingTypeEnum {
    
    /// "JAVASCRIPT"
    #[serde(rename="JAVASCRIPT")]
    JAVASCRIPT,
    
    /// "ACTIVE_SERVER_PAGE"
    #[serde(rename="ACTIVE_SERVER_PAGE")]
    ACTIVESERVERPAGE,
    
    /// "JSP"
    #[serde(rename="JSP")]
    JSP,
    
    /// "PHP"
    #[serde(rename="PHP")]
    PHP,
    
    /// "COLD_FUSION"
    #[serde(rename="COLD_FUSION")]
    COLDFUSION,
}

impl AsRef<str> for FloodlightActivityCacheBustingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityCacheBustingTypeEnum::JAVASCRIPT => "JAVASCRIPT",
            FloodlightActivityCacheBustingTypeEnum::ACTIVESERVERPAGE => "ACTIVE_SERVER_PAGE",
            FloodlightActivityCacheBustingTypeEnum::JSP => "JSP",
            FloodlightActivityCacheBustingTypeEnum::PHP => "PHP",
            FloodlightActivityCacheBustingTypeEnum::COLDFUSION => "COLD_FUSION",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityCacheBustingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JAVASCRIPT" => Ok(FloodlightActivityCacheBustingTypeEnum::JAVASCRIPT),
           "ACTIVE_SERVER_PAGE" => Ok(FloodlightActivityCacheBustingTypeEnum::ACTIVESERVERPAGE),
           "JSP" => Ok(FloodlightActivityCacheBustingTypeEnum::JSP),
           "PHP" => Ok(FloodlightActivityCacheBustingTypeEnum::PHP),
           "COLD_FUSION" => Ok(FloodlightActivityCacheBustingTypeEnum::COLDFUSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityCacheBustingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightActivityCountingMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Counting method for conversions for this floodlight activity. This is a required field.
pub enum FloodlightActivityCountingMethodEnum {
    
    /// "STANDARD_COUNTING"
    #[serde(rename="STANDARD_COUNTING")]
    STANDARDCOUNTING,
    
    /// "UNIQUE_COUNTING"
    #[serde(rename="UNIQUE_COUNTING")]
    UNIQUECOUNTING,
    
    /// "SESSION_COUNTING"
    #[serde(rename="SESSION_COUNTING")]
    SESSIONCOUNTING,
    
    /// "TRANSACTIONS_COUNTING"
    #[serde(rename="TRANSACTIONS_COUNTING")]
    TRANSACTIONSCOUNTING,
    
    /// "ITEMS_SOLD_COUNTING"
    #[serde(rename="ITEMS_SOLD_COUNTING")]
    ITEMSSOLDCOUNTING,
}

impl AsRef<str> for FloodlightActivityCountingMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityCountingMethodEnum::STANDARDCOUNTING => "STANDARD_COUNTING",
            FloodlightActivityCountingMethodEnum::UNIQUECOUNTING => "UNIQUE_COUNTING",
            FloodlightActivityCountingMethodEnum::SESSIONCOUNTING => "SESSION_COUNTING",
            FloodlightActivityCountingMethodEnum::TRANSACTIONSCOUNTING => "TRANSACTIONS_COUNTING",
            FloodlightActivityCountingMethodEnum::ITEMSSOLDCOUNTING => "ITEMS_SOLD_COUNTING",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityCountingMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STANDARD_COUNTING" => Ok(FloodlightActivityCountingMethodEnum::STANDARDCOUNTING),
           "UNIQUE_COUNTING" => Ok(FloodlightActivityCountingMethodEnum::UNIQUECOUNTING),
           "SESSION_COUNTING" => Ok(FloodlightActivityCountingMethodEnum::SESSIONCOUNTING),
           "TRANSACTIONS_COUNTING" => Ok(FloodlightActivityCountingMethodEnum::TRANSACTIONSCOUNTING),
           "ITEMS_SOLD_COUNTING" => Ok(FloodlightActivityCountingMethodEnum::ITEMSSOLDCOUNTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityCountingMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightActivityFloodlightActivityGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the associated floodlight activity group. This is a read-only field.
pub enum FloodlightActivityFloodlightActivityGroupTypeEnum {
    
    /// "COUNTER"
    #[serde(rename="COUNTER")]
    COUNTER,
    
    /// "SALE"
    #[serde(rename="SALE")]
    SALE,
}

impl AsRef<str> for FloodlightActivityFloodlightActivityGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityFloodlightActivityGroupTypeEnum::COUNTER => "COUNTER",
            FloodlightActivityFloodlightActivityGroupTypeEnum::SALE => "SALE",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityFloodlightActivityGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COUNTER" => Ok(FloodlightActivityFloodlightActivityGroupTypeEnum::COUNTER),
           "SALE" => Ok(FloodlightActivityFloodlightActivityGroupTypeEnum::SALE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityFloodlightActivityGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightActivityFloodlightTagTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of Floodlight tag this activity will generate. This is a required field.
pub enum FloodlightActivityFloodlightTagTypeEnum {
    
    /// "IFRAME"
    #[serde(rename="IFRAME")]
    IFRAME,
    
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    
    /// "GLOBAL_SITE_TAG"
    #[serde(rename="GLOBAL_SITE_TAG")]
    GLOBALSITETAG,
}

impl AsRef<str> for FloodlightActivityFloodlightTagTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityFloodlightTagTypeEnum::IFRAME => "IFRAME",
            FloodlightActivityFloodlightTagTypeEnum::IMAGE => "IMAGE",
            FloodlightActivityFloodlightTagTypeEnum::GLOBALSITETAG => "GLOBAL_SITE_TAG",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityFloodlightTagTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IFRAME" => Ok(FloodlightActivityFloodlightTagTypeEnum::IFRAME),
           "IMAGE" => Ok(FloodlightActivityFloodlightTagTypeEnum::IMAGE),
           "GLOBAL_SITE_TAG" => Ok(FloodlightActivityFloodlightTagTypeEnum::GLOBALSITETAG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityFloodlightTagTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightActivityTagFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tag format type for the floodlight activity. If left blank, the tag format will default to HTML.
pub enum FloodlightActivityTagFormatEnum {
    
    /// "HTML"
    #[serde(rename="HTML")]
    HTML,
    
    /// "XHTML"
    #[serde(rename="XHTML")]
    XHTML,
}

impl AsRef<str> for FloodlightActivityTagFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityTagFormatEnum::HTML => "HTML",
            FloodlightActivityTagFormatEnum::XHTML => "XHTML",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityTagFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HTML" => Ok(FloodlightActivityTagFormatEnum::HTML),
           "XHTML" => Ok(FloodlightActivityTagFormatEnum::XHTML),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityTagFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightActivityUserDefinedVariableTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of the user-defined variables used by this conversion tag. These map to the "u[1-100]=" in the tags. Each of these can have a user defined type. Acceptable values are U1 to U100, inclusive. 
pub enum FloodlightActivityUserDefinedVariableTypesEnum {
    
    /// "U1"
    #[serde(rename="U1")]
    U1,
    
    /// "U2"
    #[serde(rename="U2")]
    U2,
    
    /// "U3"
    #[serde(rename="U3")]
    U3,
    
    /// "U4"
    #[serde(rename="U4")]
    U4,
    
    /// "U5"
    #[serde(rename="U5")]
    U5,
    
    /// "U6"
    #[serde(rename="U6")]
    U6,
    
    /// "U7"
    #[serde(rename="U7")]
    U7,
    
    /// "U8"
    #[serde(rename="U8")]
    U8,
    
    /// "U9"
    #[serde(rename="U9")]
    U9,
    
    /// "U10"
    #[serde(rename="U10")]
    U10,
    
    /// "U11"
    #[serde(rename="U11")]
    U11,
    
    /// "U12"
    #[serde(rename="U12")]
    U12,
    
    /// "U13"
    #[serde(rename="U13")]
    U13,
    
    /// "U14"
    #[serde(rename="U14")]
    U14,
    
    /// "U15"
    #[serde(rename="U15")]
    U15,
    
    /// "U16"
    #[serde(rename="U16")]
    U16,
    
    /// "U17"
    #[serde(rename="U17")]
    U17,
    
    /// "U18"
    #[serde(rename="U18")]
    U18,
    
    /// "U19"
    #[serde(rename="U19")]
    U19,
    
    /// "U20"
    #[serde(rename="U20")]
    U20,
    
    /// "U21"
    #[serde(rename="U21")]
    U21,
    
    /// "U22"
    #[serde(rename="U22")]
    U22,
    
    /// "U23"
    #[serde(rename="U23")]
    U23,
    
    /// "U24"
    #[serde(rename="U24")]
    U24,
    
    /// "U25"
    #[serde(rename="U25")]
    U25,
    
    /// "U26"
    #[serde(rename="U26")]
    U26,
    
    /// "U27"
    #[serde(rename="U27")]
    U27,
    
    /// "U28"
    #[serde(rename="U28")]
    U28,
    
    /// "U29"
    #[serde(rename="U29")]
    U29,
    
    /// "U30"
    #[serde(rename="U30")]
    U30,
    
    /// "U31"
    #[serde(rename="U31")]
    U31,
    
    /// "U32"
    #[serde(rename="U32")]
    U32,
    
    /// "U33"
    #[serde(rename="U33")]
    U33,
    
    /// "U34"
    #[serde(rename="U34")]
    U34,
    
    /// "U35"
    #[serde(rename="U35")]
    U35,
    
    /// "U36"
    #[serde(rename="U36")]
    U36,
    
    /// "U37"
    #[serde(rename="U37")]
    U37,
    
    /// "U38"
    #[serde(rename="U38")]
    U38,
    
    /// "U39"
    #[serde(rename="U39")]
    U39,
    
    /// "U40"
    #[serde(rename="U40")]
    U40,
    
    /// "U41"
    #[serde(rename="U41")]
    U41,
    
    /// "U42"
    #[serde(rename="U42")]
    U42,
    
    /// "U43"
    #[serde(rename="U43")]
    U43,
    
    /// "U44"
    #[serde(rename="U44")]
    U44,
    
    /// "U45"
    #[serde(rename="U45")]
    U45,
    
    /// "U46"
    #[serde(rename="U46")]
    U46,
    
    /// "U47"
    #[serde(rename="U47")]
    U47,
    
    /// "U48"
    #[serde(rename="U48")]
    U48,
    
    /// "U49"
    #[serde(rename="U49")]
    U49,
    
    /// "U50"
    #[serde(rename="U50")]
    U50,
    
    /// "U51"
    #[serde(rename="U51")]
    U51,
    
    /// "U52"
    #[serde(rename="U52")]
    U52,
    
    /// "U53"
    #[serde(rename="U53")]
    U53,
    
    /// "U54"
    #[serde(rename="U54")]
    U54,
    
    /// "U55"
    #[serde(rename="U55")]
    U55,
    
    /// "U56"
    #[serde(rename="U56")]
    U56,
    
    /// "U57"
    #[serde(rename="U57")]
    U57,
    
    /// "U58"
    #[serde(rename="U58")]
    U58,
    
    /// "U59"
    #[serde(rename="U59")]
    U59,
    
    /// "U60"
    #[serde(rename="U60")]
    U60,
    
    /// "U61"
    #[serde(rename="U61")]
    U61,
    
    /// "U62"
    #[serde(rename="U62")]
    U62,
    
    /// "U63"
    #[serde(rename="U63")]
    U63,
    
    /// "U64"
    #[serde(rename="U64")]
    U64,
    
    /// "U65"
    #[serde(rename="U65")]
    U65,
    
    /// "U66"
    #[serde(rename="U66")]
    U66,
    
    /// "U67"
    #[serde(rename="U67")]
    U67,
    
    /// "U68"
    #[serde(rename="U68")]
    U68,
    
    /// "U69"
    #[serde(rename="U69")]
    U69,
    
    /// "U70"
    #[serde(rename="U70")]
    U70,
    
    /// "U71"
    #[serde(rename="U71")]
    U71,
    
    /// "U72"
    #[serde(rename="U72")]
    U72,
    
    /// "U73"
    #[serde(rename="U73")]
    U73,
    
    /// "U74"
    #[serde(rename="U74")]
    U74,
    
    /// "U75"
    #[serde(rename="U75")]
    U75,
    
    /// "U76"
    #[serde(rename="U76")]
    U76,
    
    /// "U77"
    #[serde(rename="U77")]
    U77,
    
    /// "U78"
    #[serde(rename="U78")]
    U78,
    
    /// "U79"
    #[serde(rename="U79")]
    U79,
    
    /// "U80"
    #[serde(rename="U80")]
    U80,
    
    /// "U81"
    #[serde(rename="U81")]
    U81,
    
    /// "U82"
    #[serde(rename="U82")]
    U82,
    
    /// "U83"
    #[serde(rename="U83")]
    U83,
    
    /// "U84"
    #[serde(rename="U84")]
    U84,
    
    /// "U85"
    #[serde(rename="U85")]
    U85,
    
    /// "U86"
    #[serde(rename="U86")]
    U86,
    
    /// "U87"
    #[serde(rename="U87")]
    U87,
    
    /// "U88"
    #[serde(rename="U88")]
    U88,
    
    /// "U89"
    #[serde(rename="U89")]
    U89,
    
    /// "U90"
    #[serde(rename="U90")]
    U90,
    
    /// "U91"
    #[serde(rename="U91")]
    U91,
    
    /// "U92"
    #[serde(rename="U92")]
    U92,
    
    /// "U93"
    #[serde(rename="U93")]
    U93,
    
    /// "U94"
    #[serde(rename="U94")]
    U94,
    
    /// "U95"
    #[serde(rename="U95")]
    U95,
    
    /// "U96"
    #[serde(rename="U96")]
    U96,
    
    /// "U97"
    #[serde(rename="U97")]
    U97,
    
    /// "U98"
    #[serde(rename="U98")]
    U98,
    
    /// "U99"
    #[serde(rename="U99")]
    U99,
    
    /// "U100"
    #[serde(rename="U100")]
    U100,
}

impl AsRef<str> for FloodlightActivityUserDefinedVariableTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityUserDefinedVariableTypesEnum::U1 => "U1",
            FloodlightActivityUserDefinedVariableTypesEnum::U2 => "U2",
            FloodlightActivityUserDefinedVariableTypesEnum::U3 => "U3",
            FloodlightActivityUserDefinedVariableTypesEnum::U4 => "U4",
            FloodlightActivityUserDefinedVariableTypesEnum::U5 => "U5",
            FloodlightActivityUserDefinedVariableTypesEnum::U6 => "U6",
            FloodlightActivityUserDefinedVariableTypesEnum::U7 => "U7",
            FloodlightActivityUserDefinedVariableTypesEnum::U8 => "U8",
            FloodlightActivityUserDefinedVariableTypesEnum::U9 => "U9",
            FloodlightActivityUserDefinedVariableTypesEnum::U10 => "U10",
            FloodlightActivityUserDefinedVariableTypesEnum::U11 => "U11",
            FloodlightActivityUserDefinedVariableTypesEnum::U12 => "U12",
            FloodlightActivityUserDefinedVariableTypesEnum::U13 => "U13",
            FloodlightActivityUserDefinedVariableTypesEnum::U14 => "U14",
            FloodlightActivityUserDefinedVariableTypesEnum::U15 => "U15",
            FloodlightActivityUserDefinedVariableTypesEnum::U16 => "U16",
            FloodlightActivityUserDefinedVariableTypesEnum::U17 => "U17",
            FloodlightActivityUserDefinedVariableTypesEnum::U18 => "U18",
            FloodlightActivityUserDefinedVariableTypesEnum::U19 => "U19",
            FloodlightActivityUserDefinedVariableTypesEnum::U20 => "U20",
            FloodlightActivityUserDefinedVariableTypesEnum::U21 => "U21",
            FloodlightActivityUserDefinedVariableTypesEnum::U22 => "U22",
            FloodlightActivityUserDefinedVariableTypesEnum::U23 => "U23",
            FloodlightActivityUserDefinedVariableTypesEnum::U24 => "U24",
            FloodlightActivityUserDefinedVariableTypesEnum::U25 => "U25",
            FloodlightActivityUserDefinedVariableTypesEnum::U26 => "U26",
            FloodlightActivityUserDefinedVariableTypesEnum::U27 => "U27",
            FloodlightActivityUserDefinedVariableTypesEnum::U28 => "U28",
            FloodlightActivityUserDefinedVariableTypesEnum::U29 => "U29",
            FloodlightActivityUserDefinedVariableTypesEnum::U30 => "U30",
            FloodlightActivityUserDefinedVariableTypesEnum::U31 => "U31",
            FloodlightActivityUserDefinedVariableTypesEnum::U32 => "U32",
            FloodlightActivityUserDefinedVariableTypesEnum::U33 => "U33",
            FloodlightActivityUserDefinedVariableTypesEnum::U34 => "U34",
            FloodlightActivityUserDefinedVariableTypesEnum::U35 => "U35",
            FloodlightActivityUserDefinedVariableTypesEnum::U36 => "U36",
            FloodlightActivityUserDefinedVariableTypesEnum::U37 => "U37",
            FloodlightActivityUserDefinedVariableTypesEnum::U38 => "U38",
            FloodlightActivityUserDefinedVariableTypesEnum::U39 => "U39",
            FloodlightActivityUserDefinedVariableTypesEnum::U40 => "U40",
            FloodlightActivityUserDefinedVariableTypesEnum::U41 => "U41",
            FloodlightActivityUserDefinedVariableTypesEnum::U42 => "U42",
            FloodlightActivityUserDefinedVariableTypesEnum::U43 => "U43",
            FloodlightActivityUserDefinedVariableTypesEnum::U44 => "U44",
            FloodlightActivityUserDefinedVariableTypesEnum::U45 => "U45",
            FloodlightActivityUserDefinedVariableTypesEnum::U46 => "U46",
            FloodlightActivityUserDefinedVariableTypesEnum::U47 => "U47",
            FloodlightActivityUserDefinedVariableTypesEnum::U48 => "U48",
            FloodlightActivityUserDefinedVariableTypesEnum::U49 => "U49",
            FloodlightActivityUserDefinedVariableTypesEnum::U50 => "U50",
            FloodlightActivityUserDefinedVariableTypesEnum::U51 => "U51",
            FloodlightActivityUserDefinedVariableTypesEnum::U52 => "U52",
            FloodlightActivityUserDefinedVariableTypesEnum::U53 => "U53",
            FloodlightActivityUserDefinedVariableTypesEnum::U54 => "U54",
            FloodlightActivityUserDefinedVariableTypesEnum::U55 => "U55",
            FloodlightActivityUserDefinedVariableTypesEnum::U56 => "U56",
            FloodlightActivityUserDefinedVariableTypesEnum::U57 => "U57",
            FloodlightActivityUserDefinedVariableTypesEnum::U58 => "U58",
            FloodlightActivityUserDefinedVariableTypesEnum::U59 => "U59",
            FloodlightActivityUserDefinedVariableTypesEnum::U60 => "U60",
            FloodlightActivityUserDefinedVariableTypesEnum::U61 => "U61",
            FloodlightActivityUserDefinedVariableTypesEnum::U62 => "U62",
            FloodlightActivityUserDefinedVariableTypesEnum::U63 => "U63",
            FloodlightActivityUserDefinedVariableTypesEnum::U64 => "U64",
            FloodlightActivityUserDefinedVariableTypesEnum::U65 => "U65",
            FloodlightActivityUserDefinedVariableTypesEnum::U66 => "U66",
            FloodlightActivityUserDefinedVariableTypesEnum::U67 => "U67",
            FloodlightActivityUserDefinedVariableTypesEnum::U68 => "U68",
            FloodlightActivityUserDefinedVariableTypesEnum::U69 => "U69",
            FloodlightActivityUserDefinedVariableTypesEnum::U70 => "U70",
            FloodlightActivityUserDefinedVariableTypesEnum::U71 => "U71",
            FloodlightActivityUserDefinedVariableTypesEnum::U72 => "U72",
            FloodlightActivityUserDefinedVariableTypesEnum::U73 => "U73",
            FloodlightActivityUserDefinedVariableTypesEnum::U74 => "U74",
            FloodlightActivityUserDefinedVariableTypesEnum::U75 => "U75",
            FloodlightActivityUserDefinedVariableTypesEnum::U76 => "U76",
            FloodlightActivityUserDefinedVariableTypesEnum::U77 => "U77",
            FloodlightActivityUserDefinedVariableTypesEnum::U78 => "U78",
            FloodlightActivityUserDefinedVariableTypesEnum::U79 => "U79",
            FloodlightActivityUserDefinedVariableTypesEnum::U80 => "U80",
            FloodlightActivityUserDefinedVariableTypesEnum::U81 => "U81",
            FloodlightActivityUserDefinedVariableTypesEnum::U82 => "U82",
            FloodlightActivityUserDefinedVariableTypesEnum::U83 => "U83",
            FloodlightActivityUserDefinedVariableTypesEnum::U84 => "U84",
            FloodlightActivityUserDefinedVariableTypesEnum::U85 => "U85",
            FloodlightActivityUserDefinedVariableTypesEnum::U86 => "U86",
            FloodlightActivityUserDefinedVariableTypesEnum::U87 => "U87",
            FloodlightActivityUserDefinedVariableTypesEnum::U88 => "U88",
            FloodlightActivityUserDefinedVariableTypesEnum::U89 => "U89",
            FloodlightActivityUserDefinedVariableTypesEnum::U90 => "U90",
            FloodlightActivityUserDefinedVariableTypesEnum::U91 => "U91",
            FloodlightActivityUserDefinedVariableTypesEnum::U92 => "U92",
            FloodlightActivityUserDefinedVariableTypesEnum::U93 => "U93",
            FloodlightActivityUserDefinedVariableTypesEnum::U94 => "U94",
            FloodlightActivityUserDefinedVariableTypesEnum::U95 => "U95",
            FloodlightActivityUserDefinedVariableTypesEnum::U96 => "U96",
            FloodlightActivityUserDefinedVariableTypesEnum::U97 => "U97",
            FloodlightActivityUserDefinedVariableTypesEnum::U98 => "U98",
            FloodlightActivityUserDefinedVariableTypesEnum::U99 => "U99",
            FloodlightActivityUserDefinedVariableTypesEnum::U100 => "U100",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityUserDefinedVariableTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "U1" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U1),
           "U2" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U2),
           "U3" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U3),
           "U4" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U4),
           "U5" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U5),
           "U6" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U6),
           "U7" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U7),
           "U8" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U8),
           "U9" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U9),
           "U10" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U10),
           "U11" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U11),
           "U12" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U12),
           "U13" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U13),
           "U14" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U14),
           "U15" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U15),
           "U16" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U16),
           "U17" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U17),
           "U18" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U18),
           "U19" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U19),
           "U20" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U20),
           "U21" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U21),
           "U22" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U22),
           "U23" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U23),
           "U24" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U24),
           "U25" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U25),
           "U26" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U26),
           "U27" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U27),
           "U28" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U28),
           "U29" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U29),
           "U30" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U30),
           "U31" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U31),
           "U32" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U32),
           "U33" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U33),
           "U34" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U34),
           "U35" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U35),
           "U36" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U36),
           "U37" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U37),
           "U38" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U38),
           "U39" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U39),
           "U40" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U40),
           "U41" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U41),
           "U42" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U42),
           "U43" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U43),
           "U44" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U44),
           "U45" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U45),
           "U46" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U46),
           "U47" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U47),
           "U48" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U48),
           "U49" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U49),
           "U50" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U50),
           "U51" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U51),
           "U52" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U52),
           "U53" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U53),
           "U54" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U54),
           "U55" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U55),
           "U56" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U56),
           "U57" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U57),
           "U58" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U58),
           "U59" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U59),
           "U60" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U60),
           "U61" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U61),
           "U62" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U62),
           "U63" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U63),
           "U64" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U64),
           "U65" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U65),
           "U66" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U66),
           "U67" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U67),
           "U68" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U68),
           "U69" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U69),
           "U70" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U70),
           "U71" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U71),
           "U72" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U72),
           "U73" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U73),
           "U74" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U74),
           "U75" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U75),
           "U76" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U76),
           "U77" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U77),
           "U78" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U78),
           "U79" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U79),
           "U80" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U80),
           "U81" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U81),
           "U82" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U82),
           "U83" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U83),
           "U84" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U84),
           "U85" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U85),
           "U86" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U86),
           "U87" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U87),
           "U88" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U88),
           "U89" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U89),
           "U90" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U90),
           "U91" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U91),
           "U92" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U92),
           "U93" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U93),
           "U94" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U94),
           "U95" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U95),
           "U96" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U96),
           "U97" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U97),
           "U98" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U98),
           "U99" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U99),
           "U100" => Ok(FloodlightActivityUserDefinedVariableTypesEnum::U100),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityUserDefinedVariableTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightActivityGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the floodlight activity group. This is a required field that is read-only after insertion.
pub enum FloodlightActivityGroupTypeEnum {
    
    /// "COUNTER"
    #[serde(rename="COUNTER")]
    COUNTER,
    
    /// "SALE"
    #[serde(rename="SALE")]
    SALE,
}

impl AsRef<str> for FloodlightActivityGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityGroupTypeEnum::COUNTER => "COUNTER",
            FloodlightActivityGroupTypeEnum::SALE => "SALE",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COUNTER" => Ok(FloodlightActivityGroupTypeEnum::COUNTER),
           "SALE" => Ok(FloodlightActivityGroupTypeEnum::SALE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightConfigurationFirstDayOfWeekEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Day that will be counted as the first day of the week in reports. This is a required field.
pub enum FloodlightConfigurationFirstDayOfWeekEnum {
    
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for FloodlightConfigurationFirstDayOfWeekEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightConfigurationFirstDayOfWeekEnum::MONDAY => "MONDAY",
            FloodlightConfigurationFirstDayOfWeekEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightConfigurationFirstDayOfWeekEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MONDAY" => Ok(FloodlightConfigurationFirstDayOfWeekEnum::MONDAY),
           "SUNDAY" => Ok(FloodlightConfigurationFirstDayOfWeekEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightConfigurationFirstDayOfWeekEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Types of attribution options for natural search conversions.
pub enum FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum {
    
    /// "EXCLUDE_NATURAL_SEARCH_CONVERSION_ATTRIBUTION"
    #[serde(rename="EXCLUDE_NATURAL_SEARCH_CONVERSION_ATTRIBUTION")]
    EXCLUDENATURALSEARCHCONVERSIONATTRIBUTION,
    
    /// "INCLUDE_NATURAL_SEARCH_CONVERSION_ATTRIBUTION"
    #[serde(rename="INCLUDE_NATURAL_SEARCH_CONVERSION_ATTRIBUTION")]
    INCLUDENATURALSEARCHCONVERSIONATTRIBUTION,
    
    /// "INCLUDE_NATURAL_SEARCH_TIERED_CONVERSION_ATTRIBUTION"
    #[serde(rename="INCLUDE_NATURAL_SEARCH_TIERED_CONVERSION_ATTRIBUTION")]
    INCLUDENATURALSEARCHTIEREDCONVERSIONATTRIBUTION,
}

impl AsRef<str> for FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum::EXCLUDENATURALSEARCHCONVERSIONATTRIBUTION => "EXCLUDE_NATURAL_SEARCH_CONVERSION_ATTRIBUTION",
            FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum::INCLUDENATURALSEARCHCONVERSIONATTRIBUTION => "INCLUDE_NATURAL_SEARCH_CONVERSION_ATTRIBUTION",
            FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum::INCLUDENATURALSEARCHTIEREDCONVERSIONATTRIBUTION => "INCLUDE_NATURAL_SEARCH_TIERED_CONVERSION_ATTRIBUTION",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCLUDE_NATURAL_SEARCH_CONVERSION_ATTRIBUTION" => Ok(FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum::EXCLUDENATURALSEARCHCONVERSIONATTRIBUTION),
           "INCLUDE_NATURAL_SEARCH_CONVERSION_ATTRIBUTION" => Ok(FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum::INCLUDENATURALSEARCHCONVERSIONATTRIBUTION),
           "INCLUDE_NATURAL_SEARCH_TIERED_CONVERSION_ATTRIBUTION" => Ok(FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum::INCLUDENATURALSEARCHTIEREDCONVERSIONATTRIBUTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FsCommandPositionOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Position in the browser where the window will open.
pub enum FsCommandPositionOptionEnum {
    
    /// "CENTERED"
    #[serde(rename="CENTERED")]
    CENTERED,
    
    /// "DISTANCE_FROM_TOP_LEFT_CORNER"
    #[serde(rename="DISTANCE_FROM_TOP_LEFT_CORNER")]
    DISTANCEFROMTOPLEFTCORNER,
}

impl AsRef<str> for FsCommandPositionOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FsCommandPositionOptionEnum::CENTERED => "CENTERED",
            FsCommandPositionOptionEnum::DISTANCEFROMTOPLEFTCORNER => "DISTANCE_FROM_TOP_LEFT_CORNER",
        }
    }
}

impl std::convert::TryFrom< &str> for FsCommandPositionOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CENTERED" => Ok(FsCommandPositionOptionEnum::CENTERED),
           "DISTANCE_FROM_TOP_LEFT_CORNER" => Ok(FsCommandPositionOptionEnum::DISTANCEFROMTOPLEFTCORNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FsCommandPositionOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventoryItemTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of inventory item.
pub enum InventoryItemTypeEnum {
    
    /// "PLANNING_PLACEMENT_TYPE_REGULAR"
    #[serde(rename="PLANNING_PLACEMENT_TYPE_REGULAR")]
    PLANNINGPLACEMENTTYPEREGULAR,
    
    /// "PLANNING_PLACEMENT_TYPE_CREDIT"
    #[serde(rename="PLANNING_PLACEMENT_TYPE_CREDIT")]
    PLANNINGPLACEMENTTYPECREDIT,
}

impl AsRef<str> for InventoryItemTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventoryItemTypeEnum::PLANNINGPLACEMENTTYPEREGULAR => "PLANNING_PLACEMENT_TYPE_REGULAR",
            InventoryItemTypeEnum::PLANNINGPLACEMENTTYPECREDIT => "PLANNING_PLACEMENT_TYPE_CREDIT",
        }
    }
}

impl std::convert::TryFrom< &str> for InventoryItemTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLANNING_PLACEMENT_TYPE_REGULAR" => Ok(InventoryItemTypeEnum::PLANNINGPLACEMENTTYPEREGULAR),
           "PLANNING_PLACEMENT_TYPE_CREDIT" => Ok(InventoryItemTypeEnum::PLANNINGPLACEMENTTYPECREDIT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventoryItemTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ListPopulationTermOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Comparison operator of this term. This field is only relevant when type is left unset or set to CUSTOM_VARIABLE_TERM or REFERRER_TERM.
pub enum ListPopulationTermOperatorEnum {
    
    /// "NUM_EQUALS"
    #[serde(rename="NUM_EQUALS")]
    NUMEQUALS,
    
    /// "NUM_LESS_THAN"
    #[serde(rename="NUM_LESS_THAN")]
    NUMLESSTHAN,
    
    /// "NUM_LESS_THAN_EQUAL"
    #[serde(rename="NUM_LESS_THAN_EQUAL")]
    NUMLESSTHANEQUAL,
    
    /// "NUM_GREATER_THAN"
    #[serde(rename="NUM_GREATER_THAN")]
    NUMGREATERTHAN,
    
    /// "NUM_GREATER_THAN_EQUAL"
    #[serde(rename="NUM_GREATER_THAN_EQUAL")]
    NUMGREATERTHANEQUAL,
    
    /// "STRING_EQUALS"
    #[serde(rename="STRING_EQUALS")]
    STRINGEQUALS,
    
    /// "STRING_CONTAINS"
    #[serde(rename="STRING_CONTAINS")]
    STRINGCONTAINS,
}

impl AsRef<str> for ListPopulationTermOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ListPopulationTermOperatorEnum::NUMEQUALS => "NUM_EQUALS",
            ListPopulationTermOperatorEnum::NUMLESSTHAN => "NUM_LESS_THAN",
            ListPopulationTermOperatorEnum::NUMLESSTHANEQUAL => "NUM_LESS_THAN_EQUAL",
            ListPopulationTermOperatorEnum::NUMGREATERTHAN => "NUM_GREATER_THAN",
            ListPopulationTermOperatorEnum::NUMGREATERTHANEQUAL => "NUM_GREATER_THAN_EQUAL",
            ListPopulationTermOperatorEnum::STRINGEQUALS => "STRING_EQUALS",
            ListPopulationTermOperatorEnum::STRINGCONTAINS => "STRING_CONTAINS",
        }
    }
}

impl std::convert::TryFrom< &str> for ListPopulationTermOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NUM_EQUALS" => Ok(ListPopulationTermOperatorEnum::NUMEQUALS),
           "NUM_LESS_THAN" => Ok(ListPopulationTermOperatorEnum::NUMLESSTHAN),
           "NUM_LESS_THAN_EQUAL" => Ok(ListPopulationTermOperatorEnum::NUMLESSTHANEQUAL),
           "NUM_GREATER_THAN" => Ok(ListPopulationTermOperatorEnum::NUMGREATERTHAN),
           "NUM_GREATER_THAN_EQUAL" => Ok(ListPopulationTermOperatorEnum::NUMGREATERTHANEQUAL),
           "STRING_EQUALS" => Ok(ListPopulationTermOperatorEnum::STRINGEQUALS),
           "STRING_CONTAINS" => Ok(ListPopulationTermOperatorEnum::STRINGCONTAINS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ListPopulationTermOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ListPopulationTermTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List population term type determines the applicable fields in this object. If left unset or set to CUSTOM_VARIABLE_TERM, then variableName, variableFriendlyName, operator, value, and negation are applicable. If set to LIST_MEMBERSHIP_TERM then remarketingListId and contains are applicable. If set to REFERRER_TERM then operator, value, and negation are applicable.
pub enum ListPopulationTermTypeEnum {
    
    /// "CUSTOM_VARIABLE_TERM"
    #[serde(rename="CUSTOM_VARIABLE_TERM")]
    CUSTOMVARIABLETERM,
    
    /// "LIST_MEMBERSHIP_TERM"
    #[serde(rename="LIST_MEMBERSHIP_TERM")]
    LISTMEMBERSHIPTERM,
    
    /// "REFERRER_TERM"
    #[serde(rename="REFERRER_TERM")]
    REFERRERTERM,
}

impl AsRef<str> for ListPopulationTermTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ListPopulationTermTypeEnum::CUSTOMVARIABLETERM => "CUSTOM_VARIABLE_TERM",
            ListPopulationTermTypeEnum::LISTMEMBERSHIPTERM => "LIST_MEMBERSHIP_TERM",
            ListPopulationTermTypeEnum::REFERRERTERM => "REFERRER_TERM",
        }
    }
}

impl std::convert::TryFrom< &str> for ListPopulationTermTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CUSTOM_VARIABLE_TERM" => Ok(ListPopulationTermTypeEnum::CUSTOMVARIABLETERM),
           "LIST_MEMBERSHIP_TERM" => Ok(ListPopulationTermTypeEnum::LISTMEMBERSHIPTERM),
           "REFERRER_TERM" => Ok(ListPopulationTermTypeEnum::REFERRERTERM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ListPopulationTermTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MobileAppDirectoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobile app directory.
pub enum MobileAppDirectoryEnum {
    
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    
    /// "APPLE_APP_STORE"
    #[serde(rename="APPLE_APP_STORE")]
    APPLEAPPSTORE,
    
    /// "GOOGLE_PLAY_STORE"
    #[serde(rename="GOOGLE_PLAY_STORE")]
    GOOGLEPLAYSTORE,
}

impl AsRef<str> for MobileAppDirectoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MobileAppDirectoryEnum::UNKNOWN => "UNKNOWN",
            MobileAppDirectoryEnum::APPLEAPPSTORE => "APPLE_APP_STORE",
            MobileAppDirectoryEnum::GOOGLEPLAYSTORE => "GOOGLE_PLAY_STORE",
        }
    }
}

impl std::convert::TryFrom< &str> for MobileAppDirectoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(MobileAppDirectoryEnum::UNKNOWN),
           "APPLE_APP_STORE" => Ok(MobileAppDirectoryEnum::APPLEAPPSTORE),
           "GOOGLE_PLAY_STORE" => Ok(MobileAppDirectoryEnum::GOOGLEPLAYSTORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MobileAppDirectoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ObjectFilterStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the filter. NONE means the user has access to none of the objects. ALL means the user has access to all objects. ASSIGNED means the user has access to the objects with IDs in the objectIds list.
pub enum ObjectFilterStatusEnum {
    
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    
    /// "ASSIGNED"
    #[serde(rename="ASSIGNED")]
    ASSIGNED,
    
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
}

impl AsRef<str> for ObjectFilterStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ObjectFilterStatusEnum::NONE => "NONE",
            ObjectFilterStatusEnum::ASSIGNED => "ASSIGNED",
            ObjectFilterStatusEnum::ALL => "ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for ObjectFilterStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(ObjectFilterStatusEnum::NONE),
           "ASSIGNED" => Ok(ObjectFilterStatusEnum::ASSIGNED),
           "ALL" => Ok(ObjectFilterStatusEnum::ALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ObjectFilterStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderContactContactTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this contact.
pub enum OrderContactContactTypeEnum {
    
    /// "PLANNING_ORDER_CONTACT_BUYER_CONTACT"
    #[serde(rename="PLANNING_ORDER_CONTACT_BUYER_CONTACT")]
    PLANNINGORDERCONTACTBUYERCONTACT,
    
    /// "PLANNING_ORDER_CONTACT_BUYER_BILLING_CONTACT"
    #[serde(rename="PLANNING_ORDER_CONTACT_BUYER_BILLING_CONTACT")]
    PLANNINGORDERCONTACTBUYERBILLINGCONTACT,
    
    /// "PLANNING_ORDER_CONTACT_SELLER_CONTACT"
    #[serde(rename="PLANNING_ORDER_CONTACT_SELLER_CONTACT")]
    PLANNINGORDERCONTACTSELLERCONTACT,
}

impl AsRef<str> for OrderContactContactTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderContactContactTypeEnum::PLANNINGORDERCONTACTBUYERCONTACT => "PLANNING_ORDER_CONTACT_BUYER_CONTACT",
            OrderContactContactTypeEnum::PLANNINGORDERCONTACTBUYERBILLINGCONTACT => "PLANNING_ORDER_CONTACT_BUYER_BILLING_CONTACT",
            OrderContactContactTypeEnum::PLANNINGORDERCONTACTSELLERCONTACT => "PLANNING_ORDER_CONTACT_SELLER_CONTACT",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderContactContactTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLANNING_ORDER_CONTACT_BUYER_CONTACT" => Ok(OrderContactContactTypeEnum::PLANNINGORDERCONTACTBUYERCONTACT),
           "PLANNING_ORDER_CONTACT_BUYER_BILLING_CONTACT" => Ok(OrderContactContactTypeEnum::PLANNINGORDERCONTACTBUYERBILLINGCONTACT),
           "PLANNING_ORDER_CONTACT_SELLER_CONTACT" => Ok(OrderContactContactTypeEnum::PLANNINGORDERCONTACTSELLERCONTACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderContactContactTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderDocumentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this order document
pub enum OrderDocumentTypeEnum {
    
    /// "PLANNING_ORDER_TYPE_INSERTION_ORDER"
    #[serde(rename="PLANNING_ORDER_TYPE_INSERTION_ORDER")]
    PLANNINGORDERTYPEINSERTIONORDER,
    
    /// "PLANNING_ORDER_TYPE_CHANGE_ORDER"
    #[serde(rename="PLANNING_ORDER_TYPE_CHANGE_ORDER")]
    PLANNINGORDERTYPECHANGEORDER,
}

impl AsRef<str> for OrderDocumentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderDocumentTypeEnum::PLANNINGORDERTYPEINSERTIONORDER => "PLANNING_ORDER_TYPE_INSERTION_ORDER",
            OrderDocumentTypeEnum::PLANNINGORDERTYPECHANGEORDER => "PLANNING_ORDER_TYPE_CHANGE_ORDER",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderDocumentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLANNING_ORDER_TYPE_INSERTION_ORDER" => Ok(OrderDocumentTypeEnum::PLANNINGORDERTYPEINSERTIONORDER),
           "PLANNING_ORDER_TYPE_CHANGE_ORDER" => Ok(OrderDocumentTypeEnum::PLANNINGORDERTYPECHANGEORDER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderDocumentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementCompatibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Placement compatibility. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering on desktop, on mobile devices or in mobile apps for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are no longer allowed for new placement insertions. Instead, use DISPLAY or DISPLAY_INTERSTITIAL. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard. This field is required on insertion.
pub enum PlacementCompatibilityEnum {
    
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    
    /// "DISPLAY_INTERSTITIAL"
    #[serde(rename="DISPLAY_INTERSTITIAL")]
    DISPLAYINTERSTITIAL,
    
    /// "APP"
    #[serde(rename="APP")]
    APP,
    
    /// "APP_INTERSTITIAL"
    #[serde(rename="APP_INTERSTITIAL")]
    APPINTERSTITIAL,
    
    /// "IN_STREAM_VIDEO"
    #[serde(rename="IN_STREAM_VIDEO")]
    INSTREAMVIDEO,
    
    /// "IN_STREAM_AUDIO"
    #[serde(rename="IN_STREAM_AUDIO")]
    INSTREAMAUDIO,
}

impl AsRef<str> for PlacementCompatibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementCompatibilityEnum::DISPLAY => "DISPLAY",
            PlacementCompatibilityEnum::DISPLAYINTERSTITIAL => "DISPLAY_INTERSTITIAL",
            PlacementCompatibilityEnum::APP => "APP",
            PlacementCompatibilityEnum::APPINTERSTITIAL => "APP_INTERSTITIAL",
            PlacementCompatibilityEnum::INSTREAMVIDEO => "IN_STREAM_VIDEO",
            PlacementCompatibilityEnum::INSTREAMAUDIO => "IN_STREAM_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementCompatibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISPLAY" => Ok(PlacementCompatibilityEnum::DISPLAY),
           "DISPLAY_INTERSTITIAL" => Ok(PlacementCompatibilityEnum::DISPLAYINTERSTITIAL),
           "APP" => Ok(PlacementCompatibilityEnum::APP),
           "APP_INTERSTITIAL" => Ok(PlacementCompatibilityEnum::APPINTERSTITIAL),
           "IN_STREAM_VIDEO" => Ok(PlacementCompatibilityEnum::INSTREAMVIDEO),
           "IN_STREAM_AUDIO" => Ok(PlacementCompatibilityEnum::INSTREAMAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementCompatibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementPaymentSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Payment source for this placement. This is a required field that is read-only after insertion.
pub enum PlacementPaymentSourceEnum {
    
    /// "PLACEMENT_AGENCY_PAID"
    #[serde(rename="PLACEMENT_AGENCY_PAID")]
    PLACEMENTAGENCYPAID,
    
    /// "PLACEMENT_PUBLISHER_PAID"
    #[serde(rename="PLACEMENT_PUBLISHER_PAID")]
    PLACEMENTPUBLISHERPAID,
}

impl AsRef<str> for PlacementPaymentSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementPaymentSourceEnum::PLACEMENTAGENCYPAID => "PLACEMENT_AGENCY_PAID",
            PlacementPaymentSourceEnum::PLACEMENTPUBLISHERPAID => "PLACEMENT_PUBLISHER_PAID",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementPaymentSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLACEMENT_AGENCY_PAID" => Ok(PlacementPaymentSourceEnum::PLACEMENTAGENCYPAID),
           "PLACEMENT_PUBLISHER_PAID" => Ok(PlacementPaymentSourceEnum::PLACEMENTPUBLISHERPAID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementPaymentSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Third-party placement status.
pub enum PlacementStatusEnum {
    
    /// "PENDING_REVIEW"
    #[serde(rename="PENDING_REVIEW")]
    PENDINGREVIEW,
    
    /// "PAYMENT_ACCEPTED"
    #[serde(rename="PAYMENT_ACCEPTED")]
    PAYMENTACCEPTED,
    
    /// "PAYMENT_REJECTED"
    #[serde(rename="PAYMENT_REJECTED")]
    PAYMENTREJECTED,
    
    /// "ACKNOWLEDGE_REJECTION"
    #[serde(rename="ACKNOWLEDGE_REJECTION")]
    ACKNOWLEDGEREJECTION,
    
    /// "ACKNOWLEDGE_ACCEPTANCE"
    #[serde(rename="ACKNOWLEDGE_ACCEPTANCE")]
    ACKNOWLEDGEACCEPTANCE,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
}

impl AsRef<str> for PlacementStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementStatusEnum::PENDINGREVIEW => "PENDING_REVIEW",
            PlacementStatusEnum::PAYMENTACCEPTED => "PAYMENT_ACCEPTED",
            PlacementStatusEnum::PAYMENTREJECTED => "PAYMENT_REJECTED",
            PlacementStatusEnum::ACKNOWLEDGEREJECTION => "ACKNOWLEDGE_REJECTION",
            PlacementStatusEnum::ACKNOWLEDGEACCEPTANCE => "ACKNOWLEDGE_ACCEPTANCE",
            PlacementStatusEnum::DRAFT => "DRAFT",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PENDING_REVIEW" => Ok(PlacementStatusEnum::PENDINGREVIEW),
           "PAYMENT_ACCEPTED" => Ok(PlacementStatusEnum::PAYMENTACCEPTED),
           "PAYMENT_REJECTED" => Ok(PlacementStatusEnum::PAYMENTREJECTED),
           "ACKNOWLEDGE_REJECTION" => Ok(PlacementStatusEnum::ACKNOWLEDGEREJECTION),
           "ACKNOWLEDGE_ACCEPTANCE" => Ok(PlacementStatusEnum::ACKNOWLEDGEACCEPTANCE),
           "DRAFT" => Ok(PlacementStatusEnum::DRAFT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementTagFormatsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tag formats to generate for this placement. This field is required on insertion. Acceptable values are: - "PLACEMENT_TAG_STANDARD" - "PLACEMENT_TAG_IFRAME_JAVASCRIPT" - "PLACEMENT_TAG_IFRAME_ILAYER" - "PLACEMENT_TAG_INTERNAL_REDIRECT" - "PLACEMENT_TAG_JAVASCRIPT" - "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT" - "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT" - "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT" - "PLACEMENT_TAG_CLICK_COMMANDS" - "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH" - "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3" - "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4" - "PLACEMENT_TAG_TRACKING" - "PLACEMENT_TAG_TRACKING_IFRAME" - "PLACEMENT_TAG_TRACKING_JAVASCRIPT" 
pub enum PlacementTagFormatsEnum {
    
    /// "PLACEMENT_TAG_STANDARD"
    #[serde(rename="PLACEMENT_TAG_STANDARD")]
    PLACEMENTTAGSTANDARD,
    
    /// "PLACEMENT_TAG_IFRAME_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_IFRAME_JAVASCRIPT")]
    PLACEMENTTAGIFRAMEJAVASCRIPT,
    
    /// "PLACEMENT_TAG_IFRAME_ILAYER"
    #[serde(rename="PLACEMENT_TAG_IFRAME_ILAYER")]
    PLACEMENTTAGIFRAMEILAYER,
    
    /// "PLACEMENT_TAG_INTERNAL_REDIRECT"
    #[serde(rename="PLACEMENT_TAG_INTERNAL_REDIRECT")]
    PLACEMENTTAGINTERNALREDIRECT,
    
    /// "PLACEMENT_TAG_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_JAVASCRIPT")]
    PLACEMENTTAGJAVASCRIPT,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT")]
    PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPT,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT")]
    PLACEMENTTAGINTERSTITIALINTERNALREDIRECT,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT")]
    PLACEMENTTAGINTERSTITIALJAVASCRIPT,
    
    /// "PLACEMENT_TAG_CLICK_COMMANDS"
    #[serde(rename="PLACEMENT_TAG_CLICK_COMMANDS")]
    PLACEMENTTAGCLICKCOMMANDS,
    
    /// "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH"
    #[serde(rename="PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH")]
    PLACEMENTTAGINSTREAMVIDEOPREFETCH,
    
    /// "PLACEMENT_TAG_TRACKING"
    #[serde(rename="PLACEMENT_TAG_TRACKING")]
    PLACEMENTTAGTRACKING,
    
    /// "PLACEMENT_TAG_TRACKING_IFRAME"
    #[serde(rename="PLACEMENT_TAG_TRACKING_IFRAME")]
    PLACEMENTTAGTRACKINGIFRAME,
    
    /// "PLACEMENT_TAG_TRACKING_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_TRACKING_JAVASCRIPT")]
    PLACEMENTTAGTRACKINGJAVASCRIPT,
    
    /// "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3"
    #[serde(rename="PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3")]
    PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST3,
    
    /// "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGIFRAMEJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGINTERSTITIALJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4"
    #[serde(rename="PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4")]
    PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST4,
    
    /// "PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT"
    #[serde(rename="PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT")]
    PLACEMENTTAGTRACKINGTHIRDPARTYMEASUREMENT,
}

impl AsRef<str> for PlacementTagFormatsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementTagFormatsEnum::PLACEMENTTAGSTANDARD => "PLACEMENT_TAG_STANDARD",
            PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEJAVASCRIPT => "PLACEMENT_TAG_IFRAME_JAVASCRIPT",
            PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEILAYER => "PLACEMENT_TAG_IFRAME_ILAYER",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERNALREDIRECT => "PLACEMENT_TAG_INTERNAL_REDIRECT",
            PlacementTagFormatsEnum::PLACEMENTTAGJAVASCRIPT => "PLACEMENT_TAG_JAVASCRIPT",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPT => "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALINTERNALREDIRECT => "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPT => "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT",
            PlacementTagFormatsEnum::PLACEMENTTAGCLICKCOMMANDS => "PLACEMENT_TAG_CLICK_COMMANDS",
            PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCH => "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH",
            PlacementTagFormatsEnum::PLACEMENTTAGTRACKING => "PLACEMENT_TAG_TRACKING",
            PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGIFRAME => "PLACEMENT_TAG_TRACKING_IFRAME",
            PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGJAVASCRIPT => "PLACEMENT_TAG_TRACKING_JAVASCRIPT",
            PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST3 => "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3",
            PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEJAVASCRIPTLEGACY => "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY",
            PlacementTagFormatsEnum::PLACEMENTTAGJAVASCRIPTLEGACY => "PLACEMENT_TAG_JAVASCRIPT_LEGACY",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPTLEGACY => "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPTLEGACY => "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY",
            PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST4 => "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4",
            PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGTHIRDPARTYMEASUREMENT => "PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementTagFormatsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLACEMENT_TAG_STANDARD" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGSTANDARD),
           "PLACEMENT_TAG_IFRAME_JAVASCRIPT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEJAVASCRIPT),
           "PLACEMENT_TAG_IFRAME_ILAYER" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEILAYER),
           "PLACEMENT_TAG_INTERNAL_REDIRECT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERNALREDIRECT),
           "PLACEMENT_TAG_JAVASCRIPT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGJAVASCRIPT),
           "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPT),
           "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALINTERNALREDIRECT),
           "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPT),
           "PLACEMENT_TAG_CLICK_COMMANDS" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGCLICKCOMMANDS),
           "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCH),
           "PLACEMENT_TAG_TRACKING" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGTRACKING),
           "PLACEMENT_TAG_TRACKING_IFRAME" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGIFRAME),
           "PLACEMENT_TAG_TRACKING_JAVASCRIPT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGJAVASCRIPT),
           "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST3),
           "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_JAVASCRIPT_LEGACY" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST4),
           "PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGTHIRDPARTYMEASUREMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementTagFormatsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementVpaidAdapterChoiceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// VPAID adapter setting for this placement. Controls which VPAID format the measurement adapter will use for in-stream video creatives assigned to this placement. *Note:* Flash is no longer supported. This field now defaults to HTML5 when the following values are provided: FLASH, BOTH.
pub enum PlacementVpaidAdapterChoiceEnum {
    
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    
    /// "FLASH"
    #[serde(rename="FLASH")]
    FLASH,
    
    /// "HTML5"
    #[serde(rename="HTML5")]
    HTML5,
    
    /// "BOTH"
    #[serde(rename="BOTH")]
    BOTH,
}

impl AsRef<str> for PlacementVpaidAdapterChoiceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementVpaidAdapterChoiceEnum::DEFAULT => "DEFAULT",
            PlacementVpaidAdapterChoiceEnum::FLASH => "FLASH",
            PlacementVpaidAdapterChoiceEnum::HTML5 => "HTML5",
            PlacementVpaidAdapterChoiceEnum::BOTH => "BOTH",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementVpaidAdapterChoiceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(PlacementVpaidAdapterChoiceEnum::DEFAULT),
           "FLASH" => Ok(PlacementVpaidAdapterChoiceEnum::FLASH),
           "HTML5" => Ok(PlacementVpaidAdapterChoiceEnum::HTML5),
           "BOTH" => Ok(PlacementVpaidAdapterChoiceEnum::BOTH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementVpaidAdapterChoiceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementGroupPlacementGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this placement group. A package is a simple group of placements that acts as a single pricing point for a group of tags. A roadblock is a group of placements that not only acts as a single pricing point, but also assumes that all the tags in it will be served at the same time. A roadblock requires one of its assigned placements to be marked as primary for reporting. This field is required on insertion.
pub enum PlacementGroupPlacementGroupTypeEnum {
    
    /// "PLACEMENT_PACKAGE"
    #[serde(rename="PLACEMENT_PACKAGE")]
    PLACEMENTPACKAGE,
    
    /// "PLACEMENT_ROADBLOCK"
    #[serde(rename="PLACEMENT_ROADBLOCK")]
    PLACEMENTROADBLOCK,
}

impl AsRef<str> for PlacementGroupPlacementGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementGroupPlacementGroupTypeEnum::PLACEMENTPACKAGE => "PLACEMENT_PACKAGE",
            PlacementGroupPlacementGroupTypeEnum::PLACEMENTROADBLOCK => "PLACEMENT_ROADBLOCK",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementGroupPlacementGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLACEMENT_PACKAGE" => Ok(PlacementGroupPlacementGroupTypeEnum::PLACEMENTPACKAGE),
           "PLACEMENT_ROADBLOCK" => Ok(PlacementGroupPlacementGroupTypeEnum::PLACEMENTROADBLOCK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementGroupPlacementGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PopupWindowPropertyPositionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Popup window position either centered or at specific coordinate.
pub enum PopupWindowPropertyPositionTypeEnum {
    
    /// "CENTER"
    #[serde(rename="CENTER")]
    CENTER,
    
    /// "COORDINATES"
    #[serde(rename="COORDINATES")]
    COORDINATES,
}

impl AsRef<str> for PopupWindowPropertyPositionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PopupWindowPropertyPositionTypeEnum::CENTER => "CENTER",
            PopupWindowPropertyPositionTypeEnum::COORDINATES => "COORDINATES",
        }
    }
}

impl std::convert::TryFrom< &str> for PopupWindowPropertyPositionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CENTER" => Ok(PopupWindowPropertyPositionTypeEnum::CENTER),
           "COORDINATES" => Ok(PopupWindowPropertyPositionTypeEnum::COORDINATES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PopupWindowPropertyPositionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PricingCapCostTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Cap cost type of this inventory item.
pub enum PricingCapCostTypeEnum {
    
    /// "PLANNING_PLACEMENT_CAP_COST_TYPE_NONE"
    #[serde(rename="PLANNING_PLACEMENT_CAP_COST_TYPE_NONE")]
    PLANNINGPLACEMENTCAPCOSTTYPENONE,
    
    /// "PLANNING_PLACEMENT_CAP_COST_TYPE_MONTHLY"
    #[serde(rename="PLANNING_PLACEMENT_CAP_COST_TYPE_MONTHLY")]
    PLANNINGPLACEMENTCAPCOSTTYPEMONTHLY,
    
    /// "PLANNING_PLACEMENT_CAP_COST_TYPE_CUMULATIVE"
    #[serde(rename="PLANNING_PLACEMENT_CAP_COST_TYPE_CUMULATIVE")]
    PLANNINGPLACEMENTCAPCOSTTYPECUMULATIVE,
}

impl AsRef<str> for PricingCapCostTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PricingCapCostTypeEnum::PLANNINGPLACEMENTCAPCOSTTYPENONE => "PLANNING_PLACEMENT_CAP_COST_TYPE_NONE",
            PricingCapCostTypeEnum::PLANNINGPLACEMENTCAPCOSTTYPEMONTHLY => "PLANNING_PLACEMENT_CAP_COST_TYPE_MONTHLY",
            PricingCapCostTypeEnum::PLANNINGPLACEMENTCAPCOSTTYPECUMULATIVE => "PLANNING_PLACEMENT_CAP_COST_TYPE_CUMULATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for PricingCapCostTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLANNING_PLACEMENT_CAP_COST_TYPE_NONE" => Ok(PricingCapCostTypeEnum::PLANNINGPLACEMENTCAPCOSTTYPENONE),
           "PLANNING_PLACEMENT_CAP_COST_TYPE_MONTHLY" => Ok(PricingCapCostTypeEnum::PLANNINGPLACEMENTCAPCOSTTYPEMONTHLY),
           "PLANNING_PLACEMENT_CAP_COST_TYPE_CUMULATIVE" => Ok(PricingCapCostTypeEnum::PLANNINGPLACEMENTCAPCOSTTYPECUMULATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PricingCapCostTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PricingGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Group type of this inventory item if it represents a placement group. Is null otherwise. There are two type of placement groups: PLANNING_PLACEMENT_GROUP_TYPE_PACKAGE is a simple group of inventory items that acts as a single pricing point for a group of tags. PLANNING_PLACEMENT_GROUP_TYPE_ROADBLOCK is a group of inventory items that not only acts as a single pricing point, but also assumes that all the tags in it will be served at the same time. A roadblock requires one of its assigned inventory items to be marked as primary.
pub enum PricingGroupTypeEnum {
    
    /// "PLANNING_PLACEMENT_GROUP_TYPE_PACKAGE"
    #[serde(rename="PLANNING_PLACEMENT_GROUP_TYPE_PACKAGE")]
    PLANNINGPLACEMENTGROUPTYPEPACKAGE,
    
    /// "PLANNING_PLACEMENT_GROUP_TYPE_ROADBLOCK"
    #[serde(rename="PLANNING_PLACEMENT_GROUP_TYPE_ROADBLOCK")]
    PLANNINGPLACEMENTGROUPTYPEROADBLOCK,
}

impl AsRef<str> for PricingGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PricingGroupTypeEnum::PLANNINGPLACEMENTGROUPTYPEPACKAGE => "PLANNING_PLACEMENT_GROUP_TYPE_PACKAGE",
            PricingGroupTypeEnum::PLANNINGPLACEMENTGROUPTYPEROADBLOCK => "PLANNING_PLACEMENT_GROUP_TYPE_ROADBLOCK",
        }
    }
}

impl std::convert::TryFrom< &str> for PricingGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLANNING_PLACEMENT_GROUP_TYPE_PACKAGE" => Ok(PricingGroupTypeEnum::PLANNINGPLACEMENTGROUPTYPEPACKAGE),
           "PLANNING_PLACEMENT_GROUP_TYPE_ROADBLOCK" => Ok(PricingGroupTypeEnum::PLANNINGPLACEMENTGROUPTYPEROADBLOCK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PricingGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PricingPricingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Pricing type of this inventory item.
pub enum PricingPricingTypeEnum {
    
    /// "PLANNING_PLACEMENT_PRICING_TYPE_IMPRESSIONS"
    #[serde(rename="PLANNING_PLACEMENT_PRICING_TYPE_IMPRESSIONS")]
    PLANNINGPLACEMENTPRICINGTYPEIMPRESSIONS,
    
    /// "PLANNING_PLACEMENT_PRICING_TYPE_CPM"
    #[serde(rename="PLANNING_PLACEMENT_PRICING_TYPE_CPM")]
    PLANNINGPLACEMENTPRICINGTYPECPM,
    
    /// "PLANNING_PLACEMENT_PRICING_TYPE_CLICKS"
    #[serde(rename="PLANNING_PLACEMENT_PRICING_TYPE_CLICKS")]
    PLANNINGPLACEMENTPRICINGTYPECLICKS,
    
    /// "PLANNING_PLACEMENT_PRICING_TYPE_CPC"
    #[serde(rename="PLANNING_PLACEMENT_PRICING_TYPE_CPC")]
    PLANNINGPLACEMENTPRICINGTYPECPC,
    
    /// "PLANNING_PLACEMENT_PRICING_TYPE_CPA"
    #[serde(rename="PLANNING_PLACEMENT_PRICING_TYPE_CPA")]
    PLANNINGPLACEMENTPRICINGTYPECPA,
    
    /// "PLANNING_PLACEMENT_PRICING_TYPE_FLAT_RATE_IMPRESSIONS"
    #[serde(rename="PLANNING_PLACEMENT_PRICING_TYPE_FLAT_RATE_IMPRESSIONS")]
    PLANNINGPLACEMENTPRICINGTYPEFLATRATEIMPRESSIONS,
    
    /// "PLANNING_PLACEMENT_PRICING_TYPE_FLAT_RATE_CLICKS"
    #[serde(rename="PLANNING_PLACEMENT_PRICING_TYPE_FLAT_RATE_CLICKS")]
    PLANNINGPLACEMENTPRICINGTYPEFLATRATECLICKS,
    
    /// "PLANNING_PLACEMENT_PRICING_TYPE_CPM_ACTIVEVIEW"
    #[serde(rename="PLANNING_PLACEMENT_PRICING_TYPE_CPM_ACTIVEVIEW")]
    PLANNINGPLACEMENTPRICINGTYPECPMACTIVEVIEW,
}

impl AsRef<str> for PricingPricingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPEIMPRESSIONS => "PLANNING_PLACEMENT_PRICING_TYPE_IMPRESSIONS",
            PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPECPM => "PLANNING_PLACEMENT_PRICING_TYPE_CPM",
            PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPECLICKS => "PLANNING_PLACEMENT_PRICING_TYPE_CLICKS",
            PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPECPC => "PLANNING_PLACEMENT_PRICING_TYPE_CPC",
            PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPECPA => "PLANNING_PLACEMENT_PRICING_TYPE_CPA",
            PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPEFLATRATEIMPRESSIONS => "PLANNING_PLACEMENT_PRICING_TYPE_FLAT_RATE_IMPRESSIONS",
            PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPEFLATRATECLICKS => "PLANNING_PLACEMENT_PRICING_TYPE_FLAT_RATE_CLICKS",
            PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPECPMACTIVEVIEW => "PLANNING_PLACEMENT_PRICING_TYPE_CPM_ACTIVEVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for PricingPricingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLANNING_PLACEMENT_PRICING_TYPE_IMPRESSIONS" => Ok(PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPEIMPRESSIONS),
           "PLANNING_PLACEMENT_PRICING_TYPE_CPM" => Ok(PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPECPM),
           "PLANNING_PLACEMENT_PRICING_TYPE_CLICKS" => Ok(PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPECLICKS),
           "PLANNING_PLACEMENT_PRICING_TYPE_CPC" => Ok(PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPECPC),
           "PLANNING_PLACEMENT_PRICING_TYPE_CPA" => Ok(PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPECPA),
           "PLANNING_PLACEMENT_PRICING_TYPE_FLAT_RATE_IMPRESSIONS" => Ok(PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPEFLATRATEIMPRESSIONS),
           "PLANNING_PLACEMENT_PRICING_TYPE_FLAT_RATE_CLICKS" => Ok(PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPEFLATRATECLICKS),
           "PLANNING_PLACEMENT_PRICING_TYPE_CPM_ACTIVEVIEW" => Ok(PricingPricingTypeEnum::PLANNINGPLACEMENTPRICINGTYPECPMACTIVEVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PricingPricingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PricingScheduleCapCostOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Placement cap cost option.
pub enum PricingScheduleCapCostOptionEnum {
    
    /// "CAP_COST_NONE"
    #[serde(rename="CAP_COST_NONE")]
    CAPCOSTNONE,
    
    /// "CAP_COST_MONTHLY"
    #[serde(rename="CAP_COST_MONTHLY")]
    CAPCOSTMONTHLY,
    
    /// "CAP_COST_CUMULATIVE"
    #[serde(rename="CAP_COST_CUMULATIVE")]
    CAPCOSTCUMULATIVE,
}

impl AsRef<str> for PricingScheduleCapCostOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PricingScheduleCapCostOptionEnum::CAPCOSTNONE => "CAP_COST_NONE",
            PricingScheduleCapCostOptionEnum::CAPCOSTMONTHLY => "CAP_COST_MONTHLY",
            PricingScheduleCapCostOptionEnum::CAPCOSTCUMULATIVE => "CAP_COST_CUMULATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for PricingScheduleCapCostOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CAP_COST_NONE" => Ok(PricingScheduleCapCostOptionEnum::CAPCOSTNONE),
           "CAP_COST_MONTHLY" => Ok(PricingScheduleCapCostOptionEnum::CAPCOSTMONTHLY),
           "CAP_COST_CUMULATIVE" => Ok(PricingScheduleCapCostOptionEnum::CAPCOSTCUMULATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PricingScheduleCapCostOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PricingSchedulePricingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Placement pricing type. This field is required on insertion.
pub enum PricingSchedulePricingTypeEnum {
    
    /// "PRICING_TYPE_CPM"
    #[serde(rename="PRICING_TYPE_CPM")]
    PRICINGTYPECPM,
    
    /// "PRICING_TYPE_CPC"
    #[serde(rename="PRICING_TYPE_CPC")]
    PRICINGTYPECPC,
    
    /// "PRICING_TYPE_CPA"
    #[serde(rename="PRICING_TYPE_CPA")]
    PRICINGTYPECPA,
    
    /// "PRICING_TYPE_FLAT_RATE_IMPRESSIONS"
    #[serde(rename="PRICING_TYPE_FLAT_RATE_IMPRESSIONS")]
    PRICINGTYPEFLATRATEIMPRESSIONS,
    
    /// "PRICING_TYPE_FLAT_RATE_CLICKS"
    #[serde(rename="PRICING_TYPE_FLAT_RATE_CLICKS")]
    PRICINGTYPEFLATRATECLICKS,
    
    /// "PRICING_TYPE_CPM_ACTIVEVIEW"
    #[serde(rename="PRICING_TYPE_CPM_ACTIVEVIEW")]
    PRICINGTYPECPMACTIVEVIEW,
}

impl AsRef<str> for PricingSchedulePricingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PricingSchedulePricingTypeEnum::PRICINGTYPECPM => "PRICING_TYPE_CPM",
            PricingSchedulePricingTypeEnum::PRICINGTYPECPC => "PRICING_TYPE_CPC",
            PricingSchedulePricingTypeEnum::PRICINGTYPECPA => "PRICING_TYPE_CPA",
            PricingSchedulePricingTypeEnum::PRICINGTYPEFLATRATEIMPRESSIONS => "PRICING_TYPE_FLAT_RATE_IMPRESSIONS",
            PricingSchedulePricingTypeEnum::PRICINGTYPEFLATRATECLICKS => "PRICING_TYPE_FLAT_RATE_CLICKS",
            PricingSchedulePricingTypeEnum::PRICINGTYPECPMACTIVEVIEW => "PRICING_TYPE_CPM_ACTIVEVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for PricingSchedulePricingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRICING_TYPE_CPM" => Ok(PricingSchedulePricingTypeEnum::PRICINGTYPECPM),
           "PRICING_TYPE_CPC" => Ok(PricingSchedulePricingTypeEnum::PRICINGTYPECPC),
           "PRICING_TYPE_CPA" => Ok(PricingSchedulePricingTypeEnum::PRICINGTYPECPA),
           "PRICING_TYPE_FLAT_RATE_IMPRESSIONS" => Ok(PricingSchedulePricingTypeEnum::PRICINGTYPEFLATRATEIMPRESSIONS),
           "PRICING_TYPE_FLAT_RATE_CLICKS" => Ok(PricingSchedulePricingTypeEnum::PRICINGTYPEFLATRATECLICKS),
           "PRICING_TYPE_CPM_ACTIVEVIEW" => Ok(PricingSchedulePricingTypeEnum::PRICINGTYPECPMACTIVEVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PricingSchedulePricingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectAudienceAgeGroupEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Audience age group of this project.
pub enum ProjectAudienceAgeGroupEnum {
    
    /// "PLANNING_AUDIENCE_AGE_18_24"
    #[serde(rename="PLANNING_AUDIENCE_AGE_18_24")]
    PLANNINGAUDIENCEAGE1824,
    
    /// "PLANNING_AUDIENCE_AGE_25_34"
    #[serde(rename="PLANNING_AUDIENCE_AGE_25_34")]
    PLANNINGAUDIENCEAGE2534,
    
    /// "PLANNING_AUDIENCE_AGE_35_44"
    #[serde(rename="PLANNING_AUDIENCE_AGE_35_44")]
    PLANNINGAUDIENCEAGE3544,
    
    /// "PLANNING_AUDIENCE_AGE_45_54"
    #[serde(rename="PLANNING_AUDIENCE_AGE_45_54")]
    PLANNINGAUDIENCEAGE4554,
    
    /// "PLANNING_AUDIENCE_AGE_55_64"
    #[serde(rename="PLANNING_AUDIENCE_AGE_55_64")]
    PLANNINGAUDIENCEAGE5564,
    
    /// "PLANNING_AUDIENCE_AGE_65_OR_MORE"
    #[serde(rename="PLANNING_AUDIENCE_AGE_65_OR_MORE")]
    PLANNINGAUDIENCEAGE65ORMORE,
    
    /// "PLANNING_AUDIENCE_AGE_UNKNOWN"
    #[serde(rename="PLANNING_AUDIENCE_AGE_UNKNOWN")]
    PLANNINGAUDIENCEAGEUNKNOWN,
}

impl AsRef<str> for ProjectAudienceAgeGroupEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE1824 => "PLANNING_AUDIENCE_AGE_18_24",
            ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE2534 => "PLANNING_AUDIENCE_AGE_25_34",
            ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE3544 => "PLANNING_AUDIENCE_AGE_35_44",
            ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE4554 => "PLANNING_AUDIENCE_AGE_45_54",
            ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE5564 => "PLANNING_AUDIENCE_AGE_55_64",
            ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE65ORMORE => "PLANNING_AUDIENCE_AGE_65_OR_MORE",
            ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGEUNKNOWN => "PLANNING_AUDIENCE_AGE_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectAudienceAgeGroupEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLANNING_AUDIENCE_AGE_18_24" => Ok(ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE1824),
           "PLANNING_AUDIENCE_AGE_25_34" => Ok(ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE2534),
           "PLANNING_AUDIENCE_AGE_35_44" => Ok(ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE3544),
           "PLANNING_AUDIENCE_AGE_45_54" => Ok(ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE4554),
           "PLANNING_AUDIENCE_AGE_55_64" => Ok(ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE5564),
           "PLANNING_AUDIENCE_AGE_65_OR_MORE" => Ok(ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGE65ORMORE),
           "PLANNING_AUDIENCE_AGE_UNKNOWN" => Ok(ProjectAudienceAgeGroupEnum::PLANNINGAUDIENCEAGEUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectAudienceAgeGroupEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectAudienceGenderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Audience gender of this project.
pub enum ProjectAudienceGenderEnum {
    
    /// "PLANNING_AUDIENCE_GENDER_MALE"
    #[serde(rename="PLANNING_AUDIENCE_GENDER_MALE")]
    PLANNINGAUDIENCEGENDERMALE,
    
    /// "PLANNING_AUDIENCE_GENDER_FEMALE"
    #[serde(rename="PLANNING_AUDIENCE_GENDER_FEMALE")]
    PLANNINGAUDIENCEGENDERFEMALE,
}

impl AsRef<str> for ProjectAudienceGenderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectAudienceGenderEnum::PLANNINGAUDIENCEGENDERMALE => "PLANNING_AUDIENCE_GENDER_MALE",
            ProjectAudienceGenderEnum::PLANNINGAUDIENCEGENDERFEMALE => "PLANNING_AUDIENCE_GENDER_FEMALE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectAudienceGenderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLANNING_AUDIENCE_GENDER_MALE" => Ok(ProjectAudienceGenderEnum::PLANNINGAUDIENCEGENDERMALE),
           "PLANNING_AUDIENCE_GENDER_FEMALE" => Ok(ProjectAudienceGenderEnum::PLANNINGAUDIENCEGENDERFEMALE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectAudienceGenderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RecipientDeliveryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The delivery type for the recipient.
pub enum RecipientDeliveryTypeEnum {
    
    /// "LINK"
    #[serde(rename="LINK")]
    LINK,
    
    /// "ATTACHMENT"
    #[serde(rename="ATTACHMENT")]
    ATTACHMENT,
}

impl AsRef<str> for RecipientDeliveryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RecipientDeliveryTypeEnum::LINK => "LINK",
            RecipientDeliveryTypeEnum::ATTACHMENT => "ATTACHMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for RecipientDeliveryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINK" => Ok(RecipientDeliveryTypeEnum::LINK),
           "ATTACHMENT" => Ok(RecipientDeliveryTypeEnum::ATTACHMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RecipientDeliveryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RemarketingListListSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Product from which this remarketing list was originated.
pub enum RemarketingListListSourceEnum {
    
    /// "REMARKETING_LIST_SOURCE_OTHER"
    #[serde(rename="REMARKETING_LIST_SOURCE_OTHER")]
    REMARKETINGLISTSOURCEOTHER,
    
    /// "REMARKETING_LIST_SOURCE_ADX"
    #[serde(rename="REMARKETING_LIST_SOURCE_ADX")]
    REMARKETINGLISTSOURCEADX,
    
    /// "REMARKETING_LIST_SOURCE_DFP"
    #[serde(rename="REMARKETING_LIST_SOURCE_DFP")]
    REMARKETINGLISTSOURCEDFP,
    
    /// "REMARKETING_LIST_SOURCE_XFP"
    #[serde(rename="REMARKETING_LIST_SOURCE_XFP")]
    REMARKETINGLISTSOURCEXFP,
    
    /// "REMARKETING_LIST_SOURCE_DFA"
    #[serde(rename="REMARKETING_LIST_SOURCE_DFA")]
    REMARKETINGLISTSOURCEDFA,
    
    /// "REMARKETING_LIST_SOURCE_GA"
    #[serde(rename="REMARKETING_LIST_SOURCE_GA")]
    REMARKETINGLISTSOURCEGA,
    
    /// "REMARKETING_LIST_SOURCE_YOUTUBE"
    #[serde(rename="REMARKETING_LIST_SOURCE_YOUTUBE")]
    REMARKETINGLISTSOURCEYOUTUBE,
    
    /// "REMARKETING_LIST_SOURCE_DBM"
    #[serde(rename="REMARKETING_LIST_SOURCE_DBM")]
    REMARKETINGLISTSOURCEDBM,
    
    /// "REMARKETING_LIST_SOURCE_GPLUS"
    #[serde(rename="REMARKETING_LIST_SOURCE_GPLUS")]
    REMARKETINGLISTSOURCEGPLUS,
    
    /// "REMARKETING_LIST_SOURCE_DMP"
    #[serde(rename="REMARKETING_LIST_SOURCE_DMP")]
    REMARKETINGLISTSOURCEDMP,
    
    /// "REMARKETING_LIST_SOURCE_PLAY_STORE"
    #[serde(rename="REMARKETING_LIST_SOURCE_PLAY_STORE")]
    REMARKETINGLISTSOURCEPLAYSTORE,
}

impl AsRef<str> for RemarketingListListSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEOTHER => "REMARKETING_LIST_SOURCE_OTHER",
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEADX => "REMARKETING_LIST_SOURCE_ADX",
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEDFP => "REMARKETING_LIST_SOURCE_DFP",
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEXFP => "REMARKETING_LIST_SOURCE_XFP",
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEDFA => "REMARKETING_LIST_SOURCE_DFA",
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEGA => "REMARKETING_LIST_SOURCE_GA",
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEYOUTUBE => "REMARKETING_LIST_SOURCE_YOUTUBE",
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEDBM => "REMARKETING_LIST_SOURCE_DBM",
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEGPLUS => "REMARKETING_LIST_SOURCE_GPLUS",
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEDMP => "REMARKETING_LIST_SOURCE_DMP",
            RemarketingListListSourceEnum::REMARKETINGLISTSOURCEPLAYSTORE => "REMARKETING_LIST_SOURCE_PLAY_STORE",
        }
    }
}

impl std::convert::TryFrom< &str> for RemarketingListListSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REMARKETING_LIST_SOURCE_OTHER" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEOTHER),
           "REMARKETING_LIST_SOURCE_ADX" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEADX),
           "REMARKETING_LIST_SOURCE_DFP" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEDFP),
           "REMARKETING_LIST_SOURCE_XFP" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEXFP),
           "REMARKETING_LIST_SOURCE_DFA" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEDFA),
           "REMARKETING_LIST_SOURCE_GA" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEGA),
           "REMARKETING_LIST_SOURCE_YOUTUBE" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEYOUTUBE),
           "REMARKETING_LIST_SOURCE_DBM" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEDBM),
           "REMARKETING_LIST_SOURCE_GPLUS" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEGPLUS),
           "REMARKETING_LIST_SOURCE_DMP" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEDMP),
           "REMARKETING_LIST_SOURCE_PLAY_STORE" => Ok(RemarketingListListSourceEnum::REMARKETINGLISTSOURCEPLAYSTORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RemarketingListListSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The output format of the report. If not specified, default format is "CSV". Note that the actual format in the completed report file might differ if for instance the report's size exceeds the format's capabilities. "CSV" will then be the fallback format.
pub enum ReportFormatEnum {
    
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
    
    /// "EXCEL"
    #[serde(rename="EXCEL")]
    EXCEL,
}

impl AsRef<str> for ReportFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportFormatEnum::CSV => "CSV",
            ReportFormatEnum::EXCEL => "EXCEL",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CSV" => Ok(ReportFormatEnum::CSV),
           "EXCEL" => Ok(ReportFormatEnum::EXCEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the report.
pub enum ReportTypeEnum {
    
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    
    /// "REACH"
    #[serde(rename="REACH")]
    REACH,
    
    /// "PATH_TO_CONVERSION"
    #[serde(rename="PATH_TO_CONVERSION")]
    PATHTOCONVERSION,
    
    /// "CROSS_DIMENSION_REACH"
    #[serde(rename="CROSS_DIMENSION_REACH")]
    CROSSDIMENSIONREACH,
    
    /// "FLOODLIGHT"
    #[serde(rename="FLOODLIGHT")]
    FLOODLIGHT,
}

impl AsRef<str> for ReportTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportTypeEnum::STANDARD => "STANDARD",
            ReportTypeEnum::REACH => "REACH",
            ReportTypeEnum::PATHTOCONVERSION => "PATH_TO_CONVERSION",
            ReportTypeEnum::CROSSDIMENSIONREACH => "CROSS_DIMENSION_REACH",
            ReportTypeEnum::FLOODLIGHT => "FLOODLIGHT",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STANDARD" => Ok(ReportTypeEnum::STANDARD),
           "REACH" => Ok(ReportTypeEnum::REACH),
           "PATH_TO_CONVERSION" => Ok(ReportTypeEnum::PATHTOCONVERSION),
           "CROSS_DIMENSION_REACH" => Ok(ReportTypeEnum::CROSSDIMENSIONREACH),
           "FLOODLIGHT" => Ok(ReportTypeEnum::FLOODLIGHT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SiteContactContactTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Site contact type.
pub enum SiteContactContactTypeEnum {
    
    /// "SALES_PERSON"
    #[serde(rename="SALES_PERSON")]
    SALESPERSON,
    
    /// "TRAFFICKER"
    #[serde(rename="TRAFFICKER")]
    TRAFFICKER,
}

impl AsRef<str> for SiteContactContactTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SiteContactContactTypeEnum::SALESPERSON => "SALES_PERSON",
            SiteContactContactTypeEnum::TRAFFICKER => "TRAFFICKER",
        }
    }
}

impl std::convert::TryFrom< &str> for SiteContactContactTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SALES_PERSON" => Ok(SiteContactContactTypeEnum::SALESPERSON),
           "TRAFFICKER" => Ok(SiteContactContactTypeEnum::TRAFFICKER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SiteContactContactTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SiteSettingVpaidAdapterChoiceTemplateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Default VPAID adapter setting for new placements created under this site. This value will be used to populate the placements.vpaidAdapterChoice field, when no value is specified for the new placement. Controls which VPAID format the measurement adapter will use for in-stream video creatives assigned to the placement. The publisher's specifications will typically determine this setting. For VPAID creatives, the adapter format will match the VPAID format (HTML5 VPAID creatives use the HTML5 adapter). *Note:* Flash is no longer supported. This field now defaults to HTML5 when the following values are provided: FLASH, BOTH.
pub enum SiteSettingVpaidAdapterChoiceTemplateEnum {
    
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    
    /// "FLASH"
    #[serde(rename="FLASH")]
    FLASH,
    
    /// "HTML5"
    #[serde(rename="HTML5")]
    HTML5,
    
    /// "BOTH"
    #[serde(rename="BOTH")]
    BOTH,
}

impl AsRef<str> for SiteSettingVpaidAdapterChoiceTemplateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SiteSettingVpaidAdapterChoiceTemplateEnum::DEFAULT => "DEFAULT",
            SiteSettingVpaidAdapterChoiceTemplateEnum::FLASH => "FLASH",
            SiteSettingVpaidAdapterChoiceTemplateEnum::HTML5 => "HTML5",
            SiteSettingVpaidAdapterChoiceTemplateEnum::BOTH => "BOTH",
        }
    }
}

impl std::convert::TryFrom< &str> for SiteSettingVpaidAdapterChoiceTemplateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(SiteSettingVpaidAdapterChoiceTemplateEnum::DEFAULT),
           "FLASH" => Ok(SiteSettingVpaidAdapterChoiceTemplateEnum::FLASH),
           "HTML5" => Ok(SiteSettingVpaidAdapterChoiceTemplateEnum::HTML5),
           "BOTH" => Ok(SiteSettingVpaidAdapterChoiceTemplateEnum::BOTH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SiteSettingVpaidAdapterChoiceTemplateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SiteVideoSettingOrientationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Orientation of a site template used for video. This will act as default for new placements created under this site.
pub enum SiteVideoSettingOrientationEnum {
    
    /// "ANY"
    #[serde(rename="ANY")]
    ANY,
    
    /// "LANDSCAPE"
    #[serde(rename="LANDSCAPE")]
    LANDSCAPE,
    
    /// "PORTRAIT"
    #[serde(rename="PORTRAIT")]
    PORTRAIT,
}

impl AsRef<str> for SiteVideoSettingOrientationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SiteVideoSettingOrientationEnum::ANY => "ANY",
            SiteVideoSettingOrientationEnum::LANDSCAPE => "LANDSCAPE",
            SiteVideoSettingOrientationEnum::PORTRAIT => "PORTRAIT",
        }
    }
}

impl std::convert::TryFrom< &str> for SiteVideoSettingOrientationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANY" => Ok(SiteVideoSettingOrientationEnum::ANY),
           "LANDSCAPE" => Ok(SiteVideoSettingOrientationEnum::LANDSCAPE),
           "PORTRAIT" => Ok(SiteVideoSettingOrientationEnum::PORTRAIT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SiteVideoSettingOrientationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SortedDimensionSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An optional sort order for the dimension column.
pub enum SortedDimensionSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for SortedDimensionSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SortedDimensionSortOrderEnum::ASCENDING => "ASCENDING",
            SortedDimensionSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for SortedDimensionSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(SortedDimensionSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(SortedDimensionSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SortedDimensionSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TagDataFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// TagData tag format of this tag.
pub enum TagDataFormatEnum {
    
    /// "PLACEMENT_TAG_STANDARD"
    #[serde(rename="PLACEMENT_TAG_STANDARD")]
    PLACEMENTTAGSTANDARD,
    
    /// "PLACEMENT_TAG_IFRAME_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_IFRAME_JAVASCRIPT")]
    PLACEMENTTAGIFRAMEJAVASCRIPT,
    
    /// "PLACEMENT_TAG_IFRAME_ILAYER"
    #[serde(rename="PLACEMENT_TAG_IFRAME_ILAYER")]
    PLACEMENTTAGIFRAMEILAYER,
    
    /// "PLACEMENT_TAG_INTERNAL_REDIRECT"
    #[serde(rename="PLACEMENT_TAG_INTERNAL_REDIRECT")]
    PLACEMENTTAGINTERNALREDIRECT,
    
    /// "PLACEMENT_TAG_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_JAVASCRIPT")]
    PLACEMENTTAGJAVASCRIPT,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT")]
    PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPT,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT")]
    PLACEMENTTAGINTERSTITIALINTERNALREDIRECT,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT")]
    PLACEMENTTAGINTERSTITIALJAVASCRIPT,
    
    /// "PLACEMENT_TAG_CLICK_COMMANDS"
    #[serde(rename="PLACEMENT_TAG_CLICK_COMMANDS")]
    PLACEMENTTAGCLICKCOMMANDS,
    
    /// "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH"
    #[serde(rename="PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH")]
    PLACEMENTTAGINSTREAMVIDEOPREFETCH,
    
    /// "PLACEMENT_TAG_TRACKING"
    #[serde(rename="PLACEMENT_TAG_TRACKING")]
    PLACEMENTTAGTRACKING,
    
    /// "PLACEMENT_TAG_TRACKING_IFRAME"
    #[serde(rename="PLACEMENT_TAG_TRACKING_IFRAME")]
    PLACEMENTTAGTRACKINGIFRAME,
    
    /// "PLACEMENT_TAG_TRACKING_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_TRACKING_JAVASCRIPT")]
    PLACEMENTTAGTRACKINGJAVASCRIPT,
    
    /// "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3"
    #[serde(rename="PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3")]
    PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST3,
    
    /// "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGIFRAMEJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGINTERSTITIALJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4"
    #[serde(rename="PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4")]
    PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST4,
    
    /// "PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT"
    #[serde(rename="PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT")]
    PLACEMENTTAGTRACKINGTHIRDPARTYMEASUREMENT,
}

impl AsRef<str> for TagDataFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TagDataFormatEnum::PLACEMENTTAGSTANDARD => "PLACEMENT_TAG_STANDARD",
            TagDataFormatEnum::PLACEMENTTAGIFRAMEJAVASCRIPT => "PLACEMENT_TAG_IFRAME_JAVASCRIPT",
            TagDataFormatEnum::PLACEMENTTAGIFRAMEILAYER => "PLACEMENT_TAG_IFRAME_ILAYER",
            TagDataFormatEnum::PLACEMENTTAGINTERNALREDIRECT => "PLACEMENT_TAG_INTERNAL_REDIRECT",
            TagDataFormatEnum::PLACEMENTTAGJAVASCRIPT => "PLACEMENT_TAG_JAVASCRIPT",
            TagDataFormatEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPT => "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT",
            TagDataFormatEnum::PLACEMENTTAGINTERSTITIALINTERNALREDIRECT => "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT",
            TagDataFormatEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPT => "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT",
            TagDataFormatEnum::PLACEMENTTAGCLICKCOMMANDS => "PLACEMENT_TAG_CLICK_COMMANDS",
            TagDataFormatEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCH => "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH",
            TagDataFormatEnum::PLACEMENTTAGTRACKING => "PLACEMENT_TAG_TRACKING",
            TagDataFormatEnum::PLACEMENTTAGTRACKINGIFRAME => "PLACEMENT_TAG_TRACKING_IFRAME",
            TagDataFormatEnum::PLACEMENTTAGTRACKINGJAVASCRIPT => "PLACEMENT_TAG_TRACKING_JAVASCRIPT",
            TagDataFormatEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST3 => "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3",
            TagDataFormatEnum::PLACEMENTTAGIFRAMEJAVASCRIPTLEGACY => "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY",
            TagDataFormatEnum::PLACEMENTTAGJAVASCRIPTLEGACY => "PLACEMENT_TAG_JAVASCRIPT_LEGACY",
            TagDataFormatEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPTLEGACY => "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY",
            TagDataFormatEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPTLEGACY => "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY",
            TagDataFormatEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST4 => "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4",
            TagDataFormatEnum::PLACEMENTTAGTRACKINGTHIRDPARTYMEASUREMENT => "PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for TagDataFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLACEMENT_TAG_STANDARD" => Ok(TagDataFormatEnum::PLACEMENTTAGSTANDARD),
           "PLACEMENT_TAG_IFRAME_JAVASCRIPT" => Ok(TagDataFormatEnum::PLACEMENTTAGIFRAMEJAVASCRIPT),
           "PLACEMENT_TAG_IFRAME_ILAYER" => Ok(TagDataFormatEnum::PLACEMENTTAGIFRAMEILAYER),
           "PLACEMENT_TAG_INTERNAL_REDIRECT" => Ok(TagDataFormatEnum::PLACEMENTTAGINTERNALREDIRECT),
           "PLACEMENT_TAG_JAVASCRIPT" => Ok(TagDataFormatEnum::PLACEMENTTAGJAVASCRIPT),
           "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT" => Ok(TagDataFormatEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPT),
           "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT" => Ok(TagDataFormatEnum::PLACEMENTTAGINTERSTITIALINTERNALREDIRECT),
           "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT" => Ok(TagDataFormatEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPT),
           "PLACEMENT_TAG_CLICK_COMMANDS" => Ok(TagDataFormatEnum::PLACEMENTTAGCLICKCOMMANDS),
           "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH" => Ok(TagDataFormatEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCH),
           "PLACEMENT_TAG_TRACKING" => Ok(TagDataFormatEnum::PLACEMENTTAGTRACKING),
           "PLACEMENT_TAG_TRACKING_IFRAME" => Ok(TagDataFormatEnum::PLACEMENTTAGTRACKINGIFRAME),
           "PLACEMENT_TAG_TRACKING_JAVASCRIPT" => Ok(TagDataFormatEnum::PLACEMENTTAGTRACKINGJAVASCRIPT),
           "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3" => Ok(TagDataFormatEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST3),
           "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY" => Ok(TagDataFormatEnum::PLACEMENTTAGIFRAMEJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_JAVASCRIPT_LEGACY" => Ok(TagDataFormatEnum::PLACEMENTTAGJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY" => Ok(TagDataFormatEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY" => Ok(TagDataFormatEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4" => Ok(TagDataFormatEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST4),
           "PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT" => Ok(TagDataFormatEnum::PLACEMENTTAGTRACKINGTHIRDPARTYMEASUREMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TagDataFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TagSettingKeywordOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Option specifying how keywords are embedded in ad tags. This setting can be used to specify whether keyword placeholders are inserted in placement tags for this site. Publishers can then add keywords to those placeholders.
pub enum TagSettingKeywordOptionEnum {
    
    /// "PLACEHOLDER_WITH_LIST_OF_KEYWORDS"
    #[serde(rename="PLACEHOLDER_WITH_LIST_OF_KEYWORDS")]
    PLACEHOLDERWITHLISTOFKEYWORDS,
    
    /// "IGNORE"
    #[serde(rename="IGNORE")]
    IGNORE,
    
    /// "GENERATE_SEPARATE_TAG_FOR_EACH_KEYWORD"
    #[serde(rename="GENERATE_SEPARATE_TAG_FOR_EACH_KEYWORD")]
    GENERATESEPARATETAGFOREACHKEYWORD,
}

impl AsRef<str> for TagSettingKeywordOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TagSettingKeywordOptionEnum::PLACEHOLDERWITHLISTOFKEYWORDS => "PLACEHOLDER_WITH_LIST_OF_KEYWORDS",
            TagSettingKeywordOptionEnum::IGNORE => "IGNORE",
            TagSettingKeywordOptionEnum::GENERATESEPARATETAGFOREACHKEYWORD => "GENERATE_SEPARATE_TAG_FOR_EACH_KEYWORD",
        }
    }
}

impl std::convert::TryFrom< &str> for TagSettingKeywordOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLACEHOLDER_WITH_LIST_OF_KEYWORDS" => Ok(TagSettingKeywordOptionEnum::PLACEHOLDERWITHLISTOFKEYWORDS),
           "IGNORE" => Ok(TagSettingKeywordOptionEnum::IGNORE),
           "GENERATE_SEPARATE_TAG_FOR_EACH_KEYWORD" => Ok(TagSettingKeywordOptionEnum::GENERATESEPARATETAGFOREACHKEYWORD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TagSettingKeywordOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TargetWindowTargetWindowOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of browser window for which the backup image of the flash creative can be displayed.
pub enum TargetWindowTargetWindowOptionEnum {
    
    /// "NEW_WINDOW"
    #[serde(rename="NEW_WINDOW")]
    NEWWINDOW,
    
    /// "CURRENT_WINDOW"
    #[serde(rename="CURRENT_WINDOW")]
    CURRENTWINDOW,
    
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
}

impl AsRef<str> for TargetWindowTargetWindowOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetWindowTargetWindowOptionEnum::NEWWINDOW => "NEW_WINDOW",
            TargetWindowTargetWindowOptionEnum::CURRENTWINDOW => "CURRENT_WINDOW",
            TargetWindowTargetWindowOptionEnum::CUSTOM => "CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetWindowTargetWindowOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NEW_WINDOW" => Ok(TargetWindowTargetWindowOptionEnum::NEWWINDOW),
           "CURRENT_WINDOW" => Ok(TargetWindowTargetWindowOptionEnum::CURRENTWINDOW),
           "CUSTOM" => Ok(TargetWindowTargetWindowOptionEnum::CUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetWindowTargetWindowOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TargetableRemarketingListListSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Product from which this targetable remarketing list was originated.
pub enum TargetableRemarketingListListSourceEnum {
    
    /// "REMARKETING_LIST_SOURCE_OTHER"
    #[serde(rename="REMARKETING_LIST_SOURCE_OTHER")]
    REMARKETINGLISTSOURCEOTHER,
    
    /// "REMARKETING_LIST_SOURCE_ADX"
    #[serde(rename="REMARKETING_LIST_SOURCE_ADX")]
    REMARKETINGLISTSOURCEADX,
    
    /// "REMARKETING_LIST_SOURCE_DFP"
    #[serde(rename="REMARKETING_LIST_SOURCE_DFP")]
    REMARKETINGLISTSOURCEDFP,
    
    /// "REMARKETING_LIST_SOURCE_XFP"
    #[serde(rename="REMARKETING_LIST_SOURCE_XFP")]
    REMARKETINGLISTSOURCEXFP,
    
    /// "REMARKETING_LIST_SOURCE_DFA"
    #[serde(rename="REMARKETING_LIST_SOURCE_DFA")]
    REMARKETINGLISTSOURCEDFA,
    
    /// "REMARKETING_LIST_SOURCE_GA"
    #[serde(rename="REMARKETING_LIST_SOURCE_GA")]
    REMARKETINGLISTSOURCEGA,
    
    /// "REMARKETING_LIST_SOURCE_YOUTUBE"
    #[serde(rename="REMARKETING_LIST_SOURCE_YOUTUBE")]
    REMARKETINGLISTSOURCEYOUTUBE,
    
    /// "REMARKETING_LIST_SOURCE_DBM"
    #[serde(rename="REMARKETING_LIST_SOURCE_DBM")]
    REMARKETINGLISTSOURCEDBM,
    
    /// "REMARKETING_LIST_SOURCE_GPLUS"
    #[serde(rename="REMARKETING_LIST_SOURCE_GPLUS")]
    REMARKETINGLISTSOURCEGPLUS,
    
    /// "REMARKETING_LIST_SOURCE_DMP"
    #[serde(rename="REMARKETING_LIST_SOURCE_DMP")]
    REMARKETINGLISTSOURCEDMP,
    
    /// "REMARKETING_LIST_SOURCE_PLAY_STORE"
    #[serde(rename="REMARKETING_LIST_SOURCE_PLAY_STORE")]
    REMARKETINGLISTSOURCEPLAYSTORE,
}

impl AsRef<str> for TargetableRemarketingListListSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEOTHER => "REMARKETING_LIST_SOURCE_OTHER",
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEADX => "REMARKETING_LIST_SOURCE_ADX",
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEDFP => "REMARKETING_LIST_SOURCE_DFP",
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEXFP => "REMARKETING_LIST_SOURCE_XFP",
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEDFA => "REMARKETING_LIST_SOURCE_DFA",
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEGA => "REMARKETING_LIST_SOURCE_GA",
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEYOUTUBE => "REMARKETING_LIST_SOURCE_YOUTUBE",
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEDBM => "REMARKETING_LIST_SOURCE_DBM",
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEGPLUS => "REMARKETING_LIST_SOURCE_GPLUS",
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEDMP => "REMARKETING_LIST_SOURCE_DMP",
            TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEPLAYSTORE => "REMARKETING_LIST_SOURCE_PLAY_STORE",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetableRemarketingListListSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REMARKETING_LIST_SOURCE_OTHER" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEOTHER),
           "REMARKETING_LIST_SOURCE_ADX" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEADX),
           "REMARKETING_LIST_SOURCE_DFP" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEDFP),
           "REMARKETING_LIST_SOURCE_XFP" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEXFP),
           "REMARKETING_LIST_SOURCE_DFA" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEDFA),
           "REMARKETING_LIST_SOURCE_GA" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEGA),
           "REMARKETING_LIST_SOURCE_YOUTUBE" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEYOUTUBE),
           "REMARKETING_LIST_SOURCE_DBM" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEDBM),
           "REMARKETING_LIST_SOURCE_GPLUS" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEGPLUS),
           "REMARKETING_LIST_SOURCE_DMP" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEDMP),
           "REMARKETING_LIST_SOURCE_PLAY_STORE" => Ok(TargetableRemarketingListListSourceEnum::REMARKETINGLISTSOURCEPLAYSTORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetableRemarketingListListSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ThirdPartyTrackingUrlThirdPartyUrlTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Third-party URL type for in-stream video and in-stream audio creatives.
pub enum ThirdPartyTrackingUrlThirdPartyUrlTypeEnum {
    
    /// "IMPRESSION"
    #[serde(rename="IMPRESSION")]
    IMPRESSION,
    
    /// "CLICK_TRACKING"
    #[serde(rename="CLICK_TRACKING")]
    CLICKTRACKING,
    
    /// "VIDEO_START"
    #[serde(rename="VIDEO_START")]
    VIDEOSTART,
    
    /// "VIDEO_FIRST_QUARTILE"
    #[serde(rename="VIDEO_FIRST_QUARTILE")]
    VIDEOFIRSTQUARTILE,
    
    /// "VIDEO_MIDPOINT"
    #[serde(rename="VIDEO_MIDPOINT")]
    VIDEOMIDPOINT,
    
    /// "VIDEO_THIRD_QUARTILE"
    #[serde(rename="VIDEO_THIRD_QUARTILE")]
    VIDEOTHIRDQUARTILE,
    
    /// "VIDEO_COMPLETE"
    #[serde(rename="VIDEO_COMPLETE")]
    VIDEOCOMPLETE,
    
    /// "VIDEO_MUTE"
    #[serde(rename="VIDEO_MUTE")]
    VIDEOMUTE,
    
    /// "VIDEO_PAUSE"
    #[serde(rename="VIDEO_PAUSE")]
    VIDEOPAUSE,
    
    /// "VIDEO_REWIND"
    #[serde(rename="VIDEO_REWIND")]
    VIDEOREWIND,
    
    /// "VIDEO_FULLSCREEN"
    #[serde(rename="VIDEO_FULLSCREEN")]
    VIDEOFULLSCREEN,
    
    /// "VIDEO_STOP"
    #[serde(rename="VIDEO_STOP")]
    VIDEOSTOP,
    
    /// "VIDEO_CUSTOM"
    #[serde(rename="VIDEO_CUSTOM")]
    VIDEOCUSTOM,
    
    /// "SURVEY"
    #[serde(rename="SURVEY")]
    SURVEY,
    
    /// "RICH_MEDIA_IMPRESSION"
    #[serde(rename="RICH_MEDIA_IMPRESSION")]
    RICHMEDIAIMPRESSION,
    
    /// "RICH_MEDIA_RM_IMPRESSION"
    #[serde(rename="RICH_MEDIA_RM_IMPRESSION")]
    RICHMEDIARMIMPRESSION,
    
    /// "RICH_MEDIA_BACKUP_IMPRESSION"
    #[serde(rename="RICH_MEDIA_BACKUP_IMPRESSION")]
    RICHMEDIABACKUPIMPRESSION,
    
    /// "VIDEO_SKIP"
    #[serde(rename="VIDEO_SKIP")]
    VIDEOSKIP,
    
    /// "VIDEO_PROGRESS"
    #[serde(rename="VIDEO_PROGRESS")]
    VIDEOPROGRESS,
}

impl AsRef<str> for ThirdPartyTrackingUrlThirdPartyUrlTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::IMPRESSION => "IMPRESSION",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::CLICKTRACKING => "CLICK_TRACKING",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOSTART => "VIDEO_START",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOFIRSTQUARTILE => "VIDEO_FIRST_QUARTILE",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOMIDPOINT => "VIDEO_MIDPOINT",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOTHIRDQUARTILE => "VIDEO_THIRD_QUARTILE",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOCOMPLETE => "VIDEO_COMPLETE",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOMUTE => "VIDEO_MUTE",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOPAUSE => "VIDEO_PAUSE",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOREWIND => "VIDEO_REWIND",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOFULLSCREEN => "VIDEO_FULLSCREEN",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOSTOP => "VIDEO_STOP",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOCUSTOM => "VIDEO_CUSTOM",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::SURVEY => "SURVEY",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::RICHMEDIAIMPRESSION => "RICH_MEDIA_IMPRESSION",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::RICHMEDIARMIMPRESSION => "RICH_MEDIA_RM_IMPRESSION",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::RICHMEDIABACKUPIMPRESSION => "RICH_MEDIA_BACKUP_IMPRESSION",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOSKIP => "VIDEO_SKIP",
            ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOPROGRESS => "VIDEO_PROGRESS",
        }
    }
}

impl std::convert::TryFrom< &str> for ThirdPartyTrackingUrlThirdPartyUrlTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPRESSION" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::IMPRESSION),
           "CLICK_TRACKING" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::CLICKTRACKING),
           "VIDEO_START" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOSTART),
           "VIDEO_FIRST_QUARTILE" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOFIRSTQUARTILE),
           "VIDEO_MIDPOINT" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOMIDPOINT),
           "VIDEO_THIRD_QUARTILE" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOTHIRDQUARTILE),
           "VIDEO_COMPLETE" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOCOMPLETE),
           "VIDEO_MUTE" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOMUTE),
           "VIDEO_PAUSE" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOPAUSE),
           "VIDEO_REWIND" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOREWIND),
           "VIDEO_FULLSCREEN" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOFULLSCREEN),
           "VIDEO_STOP" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOSTOP),
           "VIDEO_CUSTOM" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOCUSTOM),
           "SURVEY" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::SURVEY),
           "RICH_MEDIA_IMPRESSION" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::RICHMEDIAIMPRESSION),
           "RICH_MEDIA_RM_IMPRESSION" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::RICHMEDIARMIMPRESSION),
           "RICH_MEDIA_BACKUP_IMPRESSION" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::RICHMEDIABACKUPIMPRESSION),
           "VIDEO_SKIP" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOSKIP),
           "VIDEO_PROGRESS" => Ok(ThirdPartyTrackingUrlThirdPartyUrlTypeEnum::VIDEOPROGRESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThirdPartyTrackingUrlThirdPartyUrlTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UniversalAdIdRegistryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Registry used for the Ad ID value.
pub enum UniversalAdIdRegistryEnum {
    
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
    
    /// "AD_ID.ORG"
    #[serde(rename="AD_ID.ORG")]
    ADID_ORG,
    
    /// "CLEARCAST"
    #[serde(rename="CLEARCAST")]
    CLEARCAST,
    
    /// "DCM"
    #[serde(rename="DCM")]
    DCM,
}

impl AsRef<str> for UniversalAdIdRegistryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UniversalAdIdRegistryEnum::OTHER => "OTHER",
            UniversalAdIdRegistryEnum::ADID_ORG => "AD_ID.ORG",
            UniversalAdIdRegistryEnum::CLEARCAST => "CLEARCAST",
            UniversalAdIdRegistryEnum::DCM => "DCM",
        }
    }
}

impl std::convert::TryFrom< &str> for UniversalAdIdRegistryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OTHER" => Ok(UniversalAdIdRegistryEnum::OTHER),
           "AD_ID.ORG" => Ok(UniversalAdIdRegistryEnum::ADID_ORG),
           "CLEARCAST" => Ok(UniversalAdIdRegistryEnum::CLEARCAST),
           "DCM" => Ok(UniversalAdIdRegistryEnum::DCM),
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


// region UserDefinedVariableConfigurationDataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Data type for the variable. This is a required field.
pub enum UserDefinedVariableConfigurationDataTypeEnum {
    
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    
    /// "NUMBER"
    #[serde(rename="NUMBER")]
    NUMBER,
}

impl AsRef<str> for UserDefinedVariableConfigurationDataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserDefinedVariableConfigurationDataTypeEnum::STRING => "STRING",
            UserDefinedVariableConfigurationDataTypeEnum::NUMBER => "NUMBER",
        }
    }
}

impl std::convert::TryFrom< &str> for UserDefinedVariableConfigurationDataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STRING" => Ok(UserDefinedVariableConfigurationDataTypeEnum::STRING),
           "NUMBER" => Ok(UserDefinedVariableConfigurationDataTypeEnum::NUMBER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserDefinedVariableConfigurationDataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserDefinedVariableConfigurationVariableTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Variable name in the tag. This is a required field.
pub enum UserDefinedVariableConfigurationVariableTypeEnum {
    
    /// "U1"
    #[serde(rename="U1")]
    U1,
    
    /// "U2"
    #[serde(rename="U2")]
    U2,
    
    /// "U3"
    #[serde(rename="U3")]
    U3,
    
    /// "U4"
    #[serde(rename="U4")]
    U4,
    
    /// "U5"
    #[serde(rename="U5")]
    U5,
    
    /// "U6"
    #[serde(rename="U6")]
    U6,
    
    /// "U7"
    #[serde(rename="U7")]
    U7,
    
    /// "U8"
    #[serde(rename="U8")]
    U8,
    
    /// "U9"
    #[serde(rename="U9")]
    U9,
    
    /// "U10"
    #[serde(rename="U10")]
    U10,
    
    /// "U11"
    #[serde(rename="U11")]
    U11,
    
    /// "U12"
    #[serde(rename="U12")]
    U12,
    
    /// "U13"
    #[serde(rename="U13")]
    U13,
    
    /// "U14"
    #[serde(rename="U14")]
    U14,
    
    /// "U15"
    #[serde(rename="U15")]
    U15,
    
    /// "U16"
    #[serde(rename="U16")]
    U16,
    
    /// "U17"
    #[serde(rename="U17")]
    U17,
    
    /// "U18"
    #[serde(rename="U18")]
    U18,
    
    /// "U19"
    #[serde(rename="U19")]
    U19,
    
    /// "U20"
    #[serde(rename="U20")]
    U20,
    
    /// "U21"
    #[serde(rename="U21")]
    U21,
    
    /// "U22"
    #[serde(rename="U22")]
    U22,
    
    /// "U23"
    #[serde(rename="U23")]
    U23,
    
    /// "U24"
    #[serde(rename="U24")]
    U24,
    
    /// "U25"
    #[serde(rename="U25")]
    U25,
    
    /// "U26"
    #[serde(rename="U26")]
    U26,
    
    /// "U27"
    #[serde(rename="U27")]
    U27,
    
    /// "U28"
    #[serde(rename="U28")]
    U28,
    
    /// "U29"
    #[serde(rename="U29")]
    U29,
    
    /// "U30"
    #[serde(rename="U30")]
    U30,
    
    /// "U31"
    #[serde(rename="U31")]
    U31,
    
    /// "U32"
    #[serde(rename="U32")]
    U32,
    
    /// "U33"
    #[serde(rename="U33")]
    U33,
    
    /// "U34"
    #[serde(rename="U34")]
    U34,
    
    /// "U35"
    #[serde(rename="U35")]
    U35,
    
    /// "U36"
    #[serde(rename="U36")]
    U36,
    
    /// "U37"
    #[serde(rename="U37")]
    U37,
    
    /// "U38"
    #[serde(rename="U38")]
    U38,
    
    /// "U39"
    #[serde(rename="U39")]
    U39,
    
    /// "U40"
    #[serde(rename="U40")]
    U40,
    
    /// "U41"
    #[serde(rename="U41")]
    U41,
    
    /// "U42"
    #[serde(rename="U42")]
    U42,
    
    /// "U43"
    #[serde(rename="U43")]
    U43,
    
    /// "U44"
    #[serde(rename="U44")]
    U44,
    
    /// "U45"
    #[serde(rename="U45")]
    U45,
    
    /// "U46"
    #[serde(rename="U46")]
    U46,
    
    /// "U47"
    #[serde(rename="U47")]
    U47,
    
    /// "U48"
    #[serde(rename="U48")]
    U48,
    
    /// "U49"
    #[serde(rename="U49")]
    U49,
    
    /// "U50"
    #[serde(rename="U50")]
    U50,
    
    /// "U51"
    #[serde(rename="U51")]
    U51,
    
    /// "U52"
    #[serde(rename="U52")]
    U52,
    
    /// "U53"
    #[serde(rename="U53")]
    U53,
    
    /// "U54"
    #[serde(rename="U54")]
    U54,
    
    /// "U55"
    #[serde(rename="U55")]
    U55,
    
    /// "U56"
    #[serde(rename="U56")]
    U56,
    
    /// "U57"
    #[serde(rename="U57")]
    U57,
    
    /// "U58"
    #[serde(rename="U58")]
    U58,
    
    /// "U59"
    #[serde(rename="U59")]
    U59,
    
    /// "U60"
    #[serde(rename="U60")]
    U60,
    
    /// "U61"
    #[serde(rename="U61")]
    U61,
    
    /// "U62"
    #[serde(rename="U62")]
    U62,
    
    /// "U63"
    #[serde(rename="U63")]
    U63,
    
    /// "U64"
    #[serde(rename="U64")]
    U64,
    
    /// "U65"
    #[serde(rename="U65")]
    U65,
    
    /// "U66"
    #[serde(rename="U66")]
    U66,
    
    /// "U67"
    #[serde(rename="U67")]
    U67,
    
    /// "U68"
    #[serde(rename="U68")]
    U68,
    
    /// "U69"
    #[serde(rename="U69")]
    U69,
    
    /// "U70"
    #[serde(rename="U70")]
    U70,
    
    /// "U71"
    #[serde(rename="U71")]
    U71,
    
    /// "U72"
    #[serde(rename="U72")]
    U72,
    
    /// "U73"
    #[serde(rename="U73")]
    U73,
    
    /// "U74"
    #[serde(rename="U74")]
    U74,
    
    /// "U75"
    #[serde(rename="U75")]
    U75,
    
    /// "U76"
    #[serde(rename="U76")]
    U76,
    
    /// "U77"
    #[serde(rename="U77")]
    U77,
    
    /// "U78"
    #[serde(rename="U78")]
    U78,
    
    /// "U79"
    #[serde(rename="U79")]
    U79,
    
    /// "U80"
    #[serde(rename="U80")]
    U80,
    
    /// "U81"
    #[serde(rename="U81")]
    U81,
    
    /// "U82"
    #[serde(rename="U82")]
    U82,
    
    /// "U83"
    #[serde(rename="U83")]
    U83,
    
    /// "U84"
    #[serde(rename="U84")]
    U84,
    
    /// "U85"
    #[serde(rename="U85")]
    U85,
    
    /// "U86"
    #[serde(rename="U86")]
    U86,
    
    /// "U87"
    #[serde(rename="U87")]
    U87,
    
    /// "U88"
    #[serde(rename="U88")]
    U88,
    
    /// "U89"
    #[serde(rename="U89")]
    U89,
    
    /// "U90"
    #[serde(rename="U90")]
    U90,
    
    /// "U91"
    #[serde(rename="U91")]
    U91,
    
    /// "U92"
    #[serde(rename="U92")]
    U92,
    
    /// "U93"
    #[serde(rename="U93")]
    U93,
    
    /// "U94"
    #[serde(rename="U94")]
    U94,
    
    /// "U95"
    #[serde(rename="U95")]
    U95,
    
    /// "U96"
    #[serde(rename="U96")]
    U96,
    
    /// "U97"
    #[serde(rename="U97")]
    U97,
    
    /// "U98"
    #[serde(rename="U98")]
    U98,
    
    /// "U99"
    #[serde(rename="U99")]
    U99,
    
    /// "U100"
    #[serde(rename="U100")]
    U100,
}

impl AsRef<str> for UserDefinedVariableConfigurationVariableTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserDefinedVariableConfigurationVariableTypeEnum::U1 => "U1",
            UserDefinedVariableConfigurationVariableTypeEnum::U2 => "U2",
            UserDefinedVariableConfigurationVariableTypeEnum::U3 => "U3",
            UserDefinedVariableConfigurationVariableTypeEnum::U4 => "U4",
            UserDefinedVariableConfigurationVariableTypeEnum::U5 => "U5",
            UserDefinedVariableConfigurationVariableTypeEnum::U6 => "U6",
            UserDefinedVariableConfigurationVariableTypeEnum::U7 => "U7",
            UserDefinedVariableConfigurationVariableTypeEnum::U8 => "U8",
            UserDefinedVariableConfigurationVariableTypeEnum::U9 => "U9",
            UserDefinedVariableConfigurationVariableTypeEnum::U10 => "U10",
            UserDefinedVariableConfigurationVariableTypeEnum::U11 => "U11",
            UserDefinedVariableConfigurationVariableTypeEnum::U12 => "U12",
            UserDefinedVariableConfigurationVariableTypeEnum::U13 => "U13",
            UserDefinedVariableConfigurationVariableTypeEnum::U14 => "U14",
            UserDefinedVariableConfigurationVariableTypeEnum::U15 => "U15",
            UserDefinedVariableConfigurationVariableTypeEnum::U16 => "U16",
            UserDefinedVariableConfigurationVariableTypeEnum::U17 => "U17",
            UserDefinedVariableConfigurationVariableTypeEnum::U18 => "U18",
            UserDefinedVariableConfigurationVariableTypeEnum::U19 => "U19",
            UserDefinedVariableConfigurationVariableTypeEnum::U20 => "U20",
            UserDefinedVariableConfigurationVariableTypeEnum::U21 => "U21",
            UserDefinedVariableConfigurationVariableTypeEnum::U22 => "U22",
            UserDefinedVariableConfigurationVariableTypeEnum::U23 => "U23",
            UserDefinedVariableConfigurationVariableTypeEnum::U24 => "U24",
            UserDefinedVariableConfigurationVariableTypeEnum::U25 => "U25",
            UserDefinedVariableConfigurationVariableTypeEnum::U26 => "U26",
            UserDefinedVariableConfigurationVariableTypeEnum::U27 => "U27",
            UserDefinedVariableConfigurationVariableTypeEnum::U28 => "U28",
            UserDefinedVariableConfigurationVariableTypeEnum::U29 => "U29",
            UserDefinedVariableConfigurationVariableTypeEnum::U30 => "U30",
            UserDefinedVariableConfigurationVariableTypeEnum::U31 => "U31",
            UserDefinedVariableConfigurationVariableTypeEnum::U32 => "U32",
            UserDefinedVariableConfigurationVariableTypeEnum::U33 => "U33",
            UserDefinedVariableConfigurationVariableTypeEnum::U34 => "U34",
            UserDefinedVariableConfigurationVariableTypeEnum::U35 => "U35",
            UserDefinedVariableConfigurationVariableTypeEnum::U36 => "U36",
            UserDefinedVariableConfigurationVariableTypeEnum::U37 => "U37",
            UserDefinedVariableConfigurationVariableTypeEnum::U38 => "U38",
            UserDefinedVariableConfigurationVariableTypeEnum::U39 => "U39",
            UserDefinedVariableConfigurationVariableTypeEnum::U40 => "U40",
            UserDefinedVariableConfigurationVariableTypeEnum::U41 => "U41",
            UserDefinedVariableConfigurationVariableTypeEnum::U42 => "U42",
            UserDefinedVariableConfigurationVariableTypeEnum::U43 => "U43",
            UserDefinedVariableConfigurationVariableTypeEnum::U44 => "U44",
            UserDefinedVariableConfigurationVariableTypeEnum::U45 => "U45",
            UserDefinedVariableConfigurationVariableTypeEnum::U46 => "U46",
            UserDefinedVariableConfigurationVariableTypeEnum::U47 => "U47",
            UserDefinedVariableConfigurationVariableTypeEnum::U48 => "U48",
            UserDefinedVariableConfigurationVariableTypeEnum::U49 => "U49",
            UserDefinedVariableConfigurationVariableTypeEnum::U50 => "U50",
            UserDefinedVariableConfigurationVariableTypeEnum::U51 => "U51",
            UserDefinedVariableConfigurationVariableTypeEnum::U52 => "U52",
            UserDefinedVariableConfigurationVariableTypeEnum::U53 => "U53",
            UserDefinedVariableConfigurationVariableTypeEnum::U54 => "U54",
            UserDefinedVariableConfigurationVariableTypeEnum::U55 => "U55",
            UserDefinedVariableConfigurationVariableTypeEnum::U56 => "U56",
            UserDefinedVariableConfigurationVariableTypeEnum::U57 => "U57",
            UserDefinedVariableConfigurationVariableTypeEnum::U58 => "U58",
            UserDefinedVariableConfigurationVariableTypeEnum::U59 => "U59",
            UserDefinedVariableConfigurationVariableTypeEnum::U60 => "U60",
            UserDefinedVariableConfigurationVariableTypeEnum::U61 => "U61",
            UserDefinedVariableConfigurationVariableTypeEnum::U62 => "U62",
            UserDefinedVariableConfigurationVariableTypeEnum::U63 => "U63",
            UserDefinedVariableConfigurationVariableTypeEnum::U64 => "U64",
            UserDefinedVariableConfigurationVariableTypeEnum::U65 => "U65",
            UserDefinedVariableConfigurationVariableTypeEnum::U66 => "U66",
            UserDefinedVariableConfigurationVariableTypeEnum::U67 => "U67",
            UserDefinedVariableConfigurationVariableTypeEnum::U68 => "U68",
            UserDefinedVariableConfigurationVariableTypeEnum::U69 => "U69",
            UserDefinedVariableConfigurationVariableTypeEnum::U70 => "U70",
            UserDefinedVariableConfigurationVariableTypeEnum::U71 => "U71",
            UserDefinedVariableConfigurationVariableTypeEnum::U72 => "U72",
            UserDefinedVariableConfigurationVariableTypeEnum::U73 => "U73",
            UserDefinedVariableConfigurationVariableTypeEnum::U74 => "U74",
            UserDefinedVariableConfigurationVariableTypeEnum::U75 => "U75",
            UserDefinedVariableConfigurationVariableTypeEnum::U76 => "U76",
            UserDefinedVariableConfigurationVariableTypeEnum::U77 => "U77",
            UserDefinedVariableConfigurationVariableTypeEnum::U78 => "U78",
            UserDefinedVariableConfigurationVariableTypeEnum::U79 => "U79",
            UserDefinedVariableConfigurationVariableTypeEnum::U80 => "U80",
            UserDefinedVariableConfigurationVariableTypeEnum::U81 => "U81",
            UserDefinedVariableConfigurationVariableTypeEnum::U82 => "U82",
            UserDefinedVariableConfigurationVariableTypeEnum::U83 => "U83",
            UserDefinedVariableConfigurationVariableTypeEnum::U84 => "U84",
            UserDefinedVariableConfigurationVariableTypeEnum::U85 => "U85",
            UserDefinedVariableConfigurationVariableTypeEnum::U86 => "U86",
            UserDefinedVariableConfigurationVariableTypeEnum::U87 => "U87",
            UserDefinedVariableConfigurationVariableTypeEnum::U88 => "U88",
            UserDefinedVariableConfigurationVariableTypeEnum::U89 => "U89",
            UserDefinedVariableConfigurationVariableTypeEnum::U90 => "U90",
            UserDefinedVariableConfigurationVariableTypeEnum::U91 => "U91",
            UserDefinedVariableConfigurationVariableTypeEnum::U92 => "U92",
            UserDefinedVariableConfigurationVariableTypeEnum::U93 => "U93",
            UserDefinedVariableConfigurationVariableTypeEnum::U94 => "U94",
            UserDefinedVariableConfigurationVariableTypeEnum::U95 => "U95",
            UserDefinedVariableConfigurationVariableTypeEnum::U96 => "U96",
            UserDefinedVariableConfigurationVariableTypeEnum::U97 => "U97",
            UserDefinedVariableConfigurationVariableTypeEnum::U98 => "U98",
            UserDefinedVariableConfigurationVariableTypeEnum::U99 => "U99",
            UserDefinedVariableConfigurationVariableTypeEnum::U100 => "U100",
        }
    }
}

impl std::convert::TryFrom< &str> for UserDefinedVariableConfigurationVariableTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "U1" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U1),
           "U2" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U2),
           "U3" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U3),
           "U4" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U4),
           "U5" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U5),
           "U6" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U6),
           "U7" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U7),
           "U8" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U8),
           "U9" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U9),
           "U10" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U10),
           "U11" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U11),
           "U12" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U12),
           "U13" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U13),
           "U14" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U14),
           "U15" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U15),
           "U16" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U16),
           "U17" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U17),
           "U18" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U18),
           "U19" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U19),
           "U20" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U20),
           "U21" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U21),
           "U22" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U22),
           "U23" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U23),
           "U24" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U24),
           "U25" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U25),
           "U26" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U26),
           "U27" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U27),
           "U28" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U28),
           "U29" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U29),
           "U30" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U30),
           "U31" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U31),
           "U32" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U32),
           "U33" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U33),
           "U34" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U34),
           "U35" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U35),
           "U36" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U36),
           "U37" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U37),
           "U38" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U38),
           "U39" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U39),
           "U40" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U40),
           "U41" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U41),
           "U42" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U42),
           "U43" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U43),
           "U44" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U44),
           "U45" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U45),
           "U46" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U46),
           "U47" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U47),
           "U48" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U48),
           "U49" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U49),
           "U50" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U50),
           "U51" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U51),
           "U52" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U52),
           "U53" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U53),
           "U54" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U54),
           "U55" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U55),
           "U56" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U56),
           "U57" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U57),
           "U58" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U58),
           "U59" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U59),
           "U60" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U60),
           "U61" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U61),
           "U62" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U62),
           "U63" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U63),
           "U64" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U64),
           "U65" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U65),
           "U66" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U66),
           "U67" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U67),
           "U68" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U68),
           "U69" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U69),
           "U70" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U70),
           "U71" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U71),
           "U72" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U72),
           "U73" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U73),
           "U74" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U74),
           "U75" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U75),
           "U76" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U76),
           "U77" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U77),
           "U78" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U78),
           "U79" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U79),
           "U80" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U80),
           "U81" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U81),
           "U82" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U82),
           "U83" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U83),
           "U84" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U84),
           "U85" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U85),
           "U86" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U86),
           "U87" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U87),
           "U88" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U88),
           "U89" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U89),
           "U90" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U90),
           "U91" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U91),
           "U92" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U92),
           "U93" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U93),
           "U94" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U94),
           "U95" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U95),
           "U96" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U96),
           "U97" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U97),
           "U98" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U98),
           "U99" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U99),
           "U100" => Ok(UserDefinedVariableConfigurationVariableTypeEnum::U100),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserDefinedVariableConfigurationVariableTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserRolePermissionAvailabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Levels of availability for a user role permission.
pub enum UserRolePermissionAvailabilityEnum {
    
    /// "NOT_AVAILABLE_BY_DEFAULT"
    #[serde(rename="NOT_AVAILABLE_BY_DEFAULT")]
    NOTAVAILABLEBYDEFAULT,
    
    /// "ACCOUNT_BY_DEFAULT"
    #[serde(rename="ACCOUNT_BY_DEFAULT")]
    ACCOUNTBYDEFAULT,
    
    /// "SUBACCOUNT_AND_ACCOUNT_BY_DEFAULT"
    #[serde(rename="SUBACCOUNT_AND_ACCOUNT_BY_DEFAULT")]
    SUBACCOUNTANDACCOUNTBYDEFAULT,
    
    /// "ACCOUNT_ALWAYS"
    #[serde(rename="ACCOUNT_ALWAYS")]
    ACCOUNTALWAYS,
    
    /// "SUBACCOUNT_AND_ACCOUNT_ALWAYS"
    #[serde(rename="SUBACCOUNT_AND_ACCOUNT_ALWAYS")]
    SUBACCOUNTANDACCOUNTALWAYS,
    
    /// "USER_PROFILE_ONLY"
    #[serde(rename="USER_PROFILE_ONLY")]
    USERPROFILEONLY,
}

impl AsRef<str> for UserRolePermissionAvailabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserRolePermissionAvailabilityEnum::NOTAVAILABLEBYDEFAULT => "NOT_AVAILABLE_BY_DEFAULT",
            UserRolePermissionAvailabilityEnum::ACCOUNTBYDEFAULT => "ACCOUNT_BY_DEFAULT",
            UserRolePermissionAvailabilityEnum::SUBACCOUNTANDACCOUNTBYDEFAULT => "SUBACCOUNT_AND_ACCOUNT_BY_DEFAULT",
            UserRolePermissionAvailabilityEnum::ACCOUNTALWAYS => "ACCOUNT_ALWAYS",
            UserRolePermissionAvailabilityEnum::SUBACCOUNTANDACCOUNTALWAYS => "SUBACCOUNT_AND_ACCOUNT_ALWAYS",
            UserRolePermissionAvailabilityEnum::USERPROFILEONLY => "USER_PROFILE_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for UserRolePermissionAvailabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOT_AVAILABLE_BY_DEFAULT" => Ok(UserRolePermissionAvailabilityEnum::NOTAVAILABLEBYDEFAULT),
           "ACCOUNT_BY_DEFAULT" => Ok(UserRolePermissionAvailabilityEnum::ACCOUNTBYDEFAULT),
           "SUBACCOUNT_AND_ACCOUNT_BY_DEFAULT" => Ok(UserRolePermissionAvailabilityEnum::SUBACCOUNTANDACCOUNTBYDEFAULT),
           "ACCOUNT_ALWAYS" => Ok(UserRolePermissionAvailabilityEnum::ACCOUNTALWAYS),
           "SUBACCOUNT_AND_ACCOUNT_ALWAYS" => Ok(UserRolePermissionAvailabilityEnum::SUBACCOUNTANDACCOUNTALWAYS),
           "USER_PROFILE_ONLY" => Ok(UserRolePermissionAvailabilityEnum::USERPROFILEONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserRolePermissionAvailabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoFormatFileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// File type of the video format.
pub enum VideoFormatFileTypeEnum {
    
    /// "FLV"
    #[serde(rename="FLV")]
    FLV,
    
    /// "THREEGPP"
    #[serde(rename="THREEGPP")]
    THREEGPP,
    
    /// "MP4"
    #[serde(rename="MP4")]
    MP4,
    
    /// "WEBM"
    #[serde(rename="WEBM")]
    WEBM,
    
    /// "M3U8"
    #[serde(rename="M3U8")]
    M3U8,
}

impl AsRef<str> for VideoFormatFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoFormatFileTypeEnum::FLV => "FLV",
            VideoFormatFileTypeEnum::THREEGPP => "THREEGPP",
            VideoFormatFileTypeEnum::MP4 => "MP4",
            VideoFormatFileTypeEnum::WEBM => "WEBM",
            VideoFormatFileTypeEnum::M3U8 => "M3U8",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoFormatFileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FLV" => Ok(VideoFormatFileTypeEnum::FLV),
           "THREEGPP" => Ok(VideoFormatFileTypeEnum::THREEGPP),
           "MP4" => Ok(VideoFormatFileTypeEnum::MP4),
           "WEBM" => Ok(VideoFormatFileTypeEnum::WEBM),
           "M3U8" => Ok(VideoFormatFileTypeEnum::M3U8),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoFormatFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoSettingOrientationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Orientation of a video placement. If this value is set, placement will return assets matching the specified orientation.
pub enum VideoSettingOrientationEnum {
    
    /// "ANY"
    #[serde(rename="ANY")]
    ANY,
    
    /// "LANDSCAPE"
    #[serde(rename="LANDSCAPE")]
    LANDSCAPE,
    
    /// "PORTRAIT"
    #[serde(rename="PORTRAIT")]
    PORTRAIT,
}

impl AsRef<str> for VideoSettingOrientationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSettingOrientationEnum::ANY => "ANY",
            VideoSettingOrientationEnum::LANDSCAPE => "LANDSCAPE",
            VideoSettingOrientationEnum::PORTRAIT => "PORTRAIT",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoSettingOrientationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANY" => Ok(VideoSettingOrientationEnum::ANY),
           "LANDSCAPE" => Ok(VideoSettingOrientationEnum::LANDSCAPE),
           "PORTRAIT" => Ok(VideoSettingOrientationEnum::PORTRAIT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSettingOrientationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportCrossDimensionReachCriterionDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The dimension option.
pub enum ReportCrossDimensionReachCriterionDimensionEnum {
    
    /// "ADVERTISER"
    #[serde(rename="ADVERTISER")]
    ADVERTISER,
    
    /// "CAMPAIGN"
    #[serde(rename="CAMPAIGN")]
    CAMPAIGN,
    
    /// "SITE_BY_ADVERTISER"
    #[serde(rename="SITE_BY_ADVERTISER")]
    SITEBYADVERTISER,
    
    /// "SITE_BY_CAMPAIGN"
    #[serde(rename="SITE_BY_CAMPAIGN")]
    SITEBYCAMPAIGN,
}

impl AsRef<str> for ReportCrossDimensionReachCriterionDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportCrossDimensionReachCriterionDimensionEnum::ADVERTISER => "ADVERTISER",
            ReportCrossDimensionReachCriterionDimensionEnum::CAMPAIGN => "CAMPAIGN",
            ReportCrossDimensionReachCriterionDimensionEnum::SITEBYADVERTISER => "SITE_BY_ADVERTISER",
            ReportCrossDimensionReachCriterionDimensionEnum::SITEBYCAMPAIGN => "SITE_BY_CAMPAIGN",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportCrossDimensionReachCriterionDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADVERTISER" => Ok(ReportCrossDimensionReachCriterionDimensionEnum::ADVERTISER),
           "CAMPAIGN" => Ok(ReportCrossDimensionReachCriterionDimensionEnum::CAMPAIGN),
           "SITE_BY_ADVERTISER" => Ok(ReportCrossDimensionReachCriterionDimensionEnum::SITEBYADVERTISER),
           "SITE_BY_CAMPAIGN" => Ok(ReportCrossDimensionReachCriterionDimensionEnum::SITEBYCAMPAIGN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportCrossDimensionReachCriterionDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportDeliveryEmailOwnerDeliveryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of delivery for the owner to receive, if enabled.
pub enum ReportDeliveryEmailOwnerDeliveryTypeEnum {
    
    /// "LINK"
    #[serde(rename="LINK")]
    LINK,
    
    /// "ATTACHMENT"
    #[serde(rename="ATTACHMENT")]
    ATTACHMENT,
}

impl AsRef<str> for ReportDeliveryEmailOwnerDeliveryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportDeliveryEmailOwnerDeliveryTypeEnum::LINK => "LINK",
            ReportDeliveryEmailOwnerDeliveryTypeEnum::ATTACHMENT => "ATTACHMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportDeliveryEmailOwnerDeliveryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINK" => Ok(ReportDeliveryEmailOwnerDeliveryTypeEnum::LINK),
           "ATTACHMENT" => Ok(ReportDeliveryEmailOwnerDeliveryTypeEnum::ATTACHMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportDeliveryEmailOwnerDeliveryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportScheduleRepeatsOnWeekDaysEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of week days "WEEKLY" on which scheduled reports should run.
pub enum ReportScheduleRepeatsOnWeekDaysEnum {
    
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
    
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
}

impl AsRef<str> for ReportScheduleRepeatsOnWeekDaysEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportScheduleRepeatsOnWeekDaysEnum::SUNDAY => "SUNDAY",
            ReportScheduleRepeatsOnWeekDaysEnum::MONDAY => "MONDAY",
            ReportScheduleRepeatsOnWeekDaysEnum::TUESDAY => "TUESDAY",
            ReportScheduleRepeatsOnWeekDaysEnum::WEDNESDAY => "WEDNESDAY",
            ReportScheduleRepeatsOnWeekDaysEnum::THURSDAY => "THURSDAY",
            ReportScheduleRepeatsOnWeekDaysEnum::FRIDAY => "FRIDAY",
            ReportScheduleRepeatsOnWeekDaysEnum::SATURDAY => "SATURDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportScheduleRepeatsOnWeekDaysEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUNDAY" => Ok(ReportScheduleRepeatsOnWeekDaysEnum::SUNDAY),
           "MONDAY" => Ok(ReportScheduleRepeatsOnWeekDaysEnum::MONDAY),
           "TUESDAY" => Ok(ReportScheduleRepeatsOnWeekDaysEnum::TUESDAY),
           "WEDNESDAY" => Ok(ReportScheduleRepeatsOnWeekDaysEnum::WEDNESDAY),
           "THURSDAY" => Ok(ReportScheduleRepeatsOnWeekDaysEnum::THURSDAY),
           "FRIDAY" => Ok(ReportScheduleRepeatsOnWeekDaysEnum::FRIDAY),
           "SATURDAY" => Ok(ReportScheduleRepeatsOnWeekDaysEnum::SATURDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportScheduleRepeatsOnWeekDaysEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportScheduleRunsOnDayOfMonthEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Enum to define for "MONTHLY" scheduled reports whether reports should be repeated on the same day of the month as "startDate" or the same day of the week of the month. Example: If 'startDate' is Monday, April 2nd 2012 (2012-04-02), "DAY_OF_MONTH" would run subsequent reports on the 2nd of every Month, and "WEEK_OF_MONTH" would run subsequent reports on the first Monday of the month.
pub enum ReportScheduleRunsOnDayOfMonthEnum {
    
    /// "DAY_OF_MONTH"
    #[serde(rename="DAY_OF_MONTH")]
    DAYOFMONTH,
    
    /// "WEEK_OF_MONTH"
    #[serde(rename="WEEK_OF_MONTH")]
    WEEKOFMONTH,
}

impl AsRef<str> for ReportScheduleRunsOnDayOfMonthEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportScheduleRunsOnDayOfMonthEnum::DAYOFMONTH => "DAY_OF_MONTH",
            ReportScheduleRunsOnDayOfMonthEnum::WEEKOFMONTH => "WEEK_OF_MONTH",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportScheduleRunsOnDayOfMonthEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_MONTH" => Ok(ReportScheduleRunsOnDayOfMonthEnum::DAYOFMONTH),
           "WEEK_OF_MONTH" => Ok(ReportScheduleRunsOnDayOfMonthEnum::WEEKOFMONTH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportScheduleRunsOnDayOfMonthEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountUserProfileSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum AccountUserProfileSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for AccountUserProfileSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountUserProfileSortFieldEnum::ID => "ID",
            AccountUserProfileSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountUserProfileSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(AccountUserProfileSortFieldEnum::ID),
           "NAME" => Ok(AccountUserProfileSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountUserProfileSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AccountUserProfileSortFieldEnum {
    fn default() -> AccountUserProfileSortFieldEnum {
        AccountUserProfileSortFieldEnum::ID
    }
}

// endregion


// region AccountUserProfileSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum AccountUserProfileSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for AccountUserProfileSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountUserProfileSortOrderEnum::ASCENDING => "ASCENDING",
            AccountUserProfileSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountUserProfileSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(AccountUserProfileSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(AccountUserProfileSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountUserProfileSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AccountUserProfileSortOrderEnum {
    fn default() -> AccountUserProfileSortOrderEnum {
        AccountUserProfileSortOrderEnum::ASCENDING
    }
}

// endregion


// region AccountSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum AccountSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for AccountSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountSortFieldEnum::ID => "ID",
            AccountSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(AccountSortFieldEnum::ID),
           "NAME" => Ok(AccountSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AccountSortFieldEnum {
    fn default() -> AccountSortFieldEnum {
        AccountSortFieldEnum::ID
    }
}

// endregion


// region AccountSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum AccountSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for AccountSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountSortOrderEnum::ASCENDING => "ASCENDING",
            AccountSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(AccountSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(AccountSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AccountSortOrderEnum {
    fn default() -> AccountSortOrderEnum {
        AccountSortOrderEnum::ASCENDING
    }
}

// endregion


// region AdCompatibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select default ads with the specified compatibility. Applicable when type is AD_SERVING_DEFAULT_AD. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop or on mobile devices for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. IN_STREAM_VIDEO refers to rendering an in-stream video ads developed with the VAST standard.
pub enum AdCompatibilityEnum {
    
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    
    /// "DISPLAY_INTERSTITIAL"
    #[serde(rename="DISPLAY_INTERSTITIAL")]
    DISPLAYINTERSTITIAL,
    
    /// "APP"
    #[serde(rename="APP")]
    APP,
    
    /// "APP_INTERSTITIAL"
    #[serde(rename="APP_INTERSTITIAL")]
    APPINTERSTITIAL,
    
    /// "IN_STREAM_VIDEO"
    #[serde(rename="IN_STREAM_VIDEO")]
    INSTREAMVIDEO,
    
    /// "IN_STREAM_AUDIO"
    #[serde(rename="IN_STREAM_AUDIO")]
    INSTREAMAUDIO,
}

impl AsRef<str> for AdCompatibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdCompatibilityEnum::DISPLAY => "DISPLAY",
            AdCompatibilityEnum::DISPLAYINTERSTITIAL => "DISPLAY_INTERSTITIAL",
            AdCompatibilityEnum::APP => "APP",
            AdCompatibilityEnum::APPINTERSTITIAL => "APP_INTERSTITIAL",
            AdCompatibilityEnum::INSTREAMVIDEO => "IN_STREAM_VIDEO",
            AdCompatibilityEnum::INSTREAMAUDIO => "IN_STREAM_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for AdCompatibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISPLAY" => Ok(AdCompatibilityEnum::DISPLAY),
           "DISPLAY_INTERSTITIAL" => Ok(AdCompatibilityEnum::DISPLAYINTERSTITIAL),
           "APP" => Ok(AdCompatibilityEnum::APP),
           "APP_INTERSTITIAL" => Ok(AdCompatibilityEnum::APPINTERSTITIAL),
           "IN_STREAM_VIDEO" => Ok(AdCompatibilityEnum::INSTREAMVIDEO),
           "IN_STREAM_AUDIO" => Ok(AdCompatibilityEnum::INSTREAMAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdCompatibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum AdSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for AdSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdSortFieldEnum::ID => "ID",
            AdSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for AdSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(AdSortFieldEnum::ID),
           "NAME" => Ok(AdSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AdSortFieldEnum {
    fn default() -> AdSortFieldEnum {
        AdSortFieldEnum::ID
    }
}

// endregion


// region AdSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum AdSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for AdSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdSortOrderEnum::ASCENDING => "ASCENDING",
            AdSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for AdSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(AdSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(AdSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AdSortOrderEnum {
    fn default() -> AdSortOrderEnum {
        AdSortOrderEnum::ASCENDING
    }
}

// endregion


// region AdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only ads with these types.
pub enum AdTypeEnum {
    
    /// "AD_SERVING_STANDARD_AD"
    #[serde(rename="AD_SERVING_STANDARD_AD")]
    ADSERVINGSTANDARDAD,
    
    /// "AD_SERVING_DEFAULT_AD"
    #[serde(rename="AD_SERVING_DEFAULT_AD")]
    ADSERVINGDEFAULTAD,
    
    /// "AD_SERVING_CLICK_TRACKER"
    #[serde(rename="AD_SERVING_CLICK_TRACKER")]
    ADSERVINGCLICKTRACKER,
    
    /// "AD_SERVING_TRACKING"
    #[serde(rename="AD_SERVING_TRACKING")]
    ADSERVINGTRACKING,
    
    /// "AD_SERVING_BRAND_SAFE_AD"
    #[serde(rename="AD_SERVING_BRAND_SAFE_AD")]
    ADSERVINGBRANDSAFEAD,
}

impl AsRef<str> for AdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdTypeEnum::ADSERVINGSTANDARDAD => "AD_SERVING_STANDARD_AD",
            AdTypeEnum::ADSERVINGDEFAULTAD => "AD_SERVING_DEFAULT_AD",
            AdTypeEnum::ADSERVINGCLICKTRACKER => "AD_SERVING_CLICK_TRACKER",
            AdTypeEnum::ADSERVINGTRACKING => "AD_SERVING_TRACKING",
            AdTypeEnum::ADSERVINGBRANDSAFEAD => "AD_SERVING_BRAND_SAFE_AD",
        }
    }
}

impl std::convert::TryFrom< &str> for AdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AD_SERVING_STANDARD_AD" => Ok(AdTypeEnum::ADSERVINGSTANDARDAD),
           "AD_SERVING_DEFAULT_AD" => Ok(AdTypeEnum::ADSERVINGDEFAULTAD),
           "AD_SERVING_CLICK_TRACKER" => Ok(AdTypeEnum::ADSERVINGCLICKTRACKER),
           "AD_SERVING_TRACKING" => Ok(AdTypeEnum::ADSERVINGTRACKING),
           "AD_SERVING_BRAND_SAFE_AD" => Ok(AdTypeEnum::ADSERVINGBRANDSAFEAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdvertiserGroupSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum AdvertiserGroupSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for AdvertiserGroupSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserGroupSortFieldEnum::ID => "ID",
            AdvertiserGroupSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserGroupSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(AdvertiserGroupSortFieldEnum::ID),
           "NAME" => Ok(AdvertiserGroupSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserGroupSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AdvertiserGroupSortFieldEnum {
    fn default() -> AdvertiserGroupSortFieldEnum {
        AdvertiserGroupSortFieldEnum::ID
    }
}

// endregion


// region AdvertiserGroupSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum AdvertiserGroupSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for AdvertiserGroupSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserGroupSortOrderEnum::ASCENDING => "ASCENDING",
            AdvertiserGroupSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserGroupSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(AdvertiserGroupSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(AdvertiserGroupSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserGroupSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AdvertiserGroupSortOrderEnum {
    fn default() -> AdvertiserGroupSortOrderEnum {
        AdvertiserGroupSortOrderEnum::ASCENDING
    }
}

// endregion


// region AdvertiserLandingPageSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum AdvertiserLandingPageSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for AdvertiserLandingPageSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserLandingPageSortFieldEnum::ID => "ID",
            AdvertiserLandingPageSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserLandingPageSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(AdvertiserLandingPageSortFieldEnum::ID),
           "NAME" => Ok(AdvertiserLandingPageSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserLandingPageSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AdvertiserLandingPageSortFieldEnum {
    fn default() -> AdvertiserLandingPageSortFieldEnum {
        AdvertiserLandingPageSortFieldEnum::ID
    }
}

// endregion


// region AdvertiserLandingPageSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum AdvertiserLandingPageSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for AdvertiserLandingPageSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserLandingPageSortOrderEnum::ASCENDING => "ASCENDING",
            AdvertiserLandingPageSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserLandingPageSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(AdvertiserLandingPageSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(AdvertiserLandingPageSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserLandingPageSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AdvertiserLandingPageSortOrderEnum {
    fn default() -> AdvertiserLandingPageSortOrderEnum {
        AdvertiserLandingPageSortOrderEnum::ASCENDING
    }
}

// endregion


// region AdvertiserSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum AdvertiserSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for AdvertiserSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserSortFieldEnum::ID => "ID",
            AdvertiserSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(AdvertiserSortFieldEnum::ID),
           "NAME" => Ok(AdvertiserSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AdvertiserSortFieldEnum {
    fn default() -> AdvertiserSortFieldEnum {
        AdvertiserSortFieldEnum::ID
    }
}

// endregion


// region AdvertiserSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum AdvertiserSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for AdvertiserSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserSortOrderEnum::ASCENDING => "ASCENDING",
            AdvertiserSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(AdvertiserSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(AdvertiserSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for AdvertiserSortOrderEnum {
    fn default() -> AdvertiserSortOrderEnum {
        AdvertiserSortOrderEnum::ASCENDING
    }
}

// endregion


// region AdvertiserStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only advertisers with the specified status.
pub enum AdvertiserStatusEnum {
    
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    
    /// "ON_HOLD"
    #[serde(rename="ON_HOLD")]
    ONHOLD,
}

impl AsRef<str> for AdvertiserStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserStatusEnum::APPROVED => "APPROVED",
            AdvertiserStatusEnum::ONHOLD => "ON_HOLD",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APPROVED" => Ok(AdvertiserStatusEnum::APPROVED),
           "ON_HOLD" => Ok(AdvertiserStatusEnum::ONHOLD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CampaignCreativeAssociationSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum CampaignCreativeAssociationSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for CampaignCreativeAssociationSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CampaignCreativeAssociationSortOrderEnum::ASCENDING => "ASCENDING",
            CampaignCreativeAssociationSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for CampaignCreativeAssociationSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(CampaignCreativeAssociationSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(CampaignCreativeAssociationSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CampaignCreativeAssociationSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CampaignCreativeAssociationSortOrderEnum {
    fn default() -> CampaignCreativeAssociationSortOrderEnum {
        CampaignCreativeAssociationSortOrderEnum::ASCENDING
    }
}

// endregion


// region CampaignSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum CampaignSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for CampaignSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CampaignSortFieldEnum::ID => "ID",
            CampaignSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for CampaignSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(CampaignSortFieldEnum::ID),
           "NAME" => Ok(CampaignSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CampaignSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CampaignSortFieldEnum {
    fn default() -> CampaignSortFieldEnum {
        CampaignSortFieldEnum::ID
    }
}

// endregion


// region CampaignSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum CampaignSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for CampaignSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CampaignSortOrderEnum::ASCENDING => "ASCENDING",
            CampaignSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for CampaignSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(CampaignSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(CampaignSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CampaignSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CampaignSortOrderEnum {
    fn default() -> CampaignSortOrderEnum {
        CampaignSortOrderEnum::ASCENDING
    }
}

// endregion


// region ChangeLogActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only change logs with the specified action.
pub enum ChangeLogActionEnum {
    
    /// "ACTION_CREATE"
    #[serde(rename="ACTION_CREATE")]
    ACTIONCREATE,
    
    /// "ACTION_UPDATE"
    #[serde(rename="ACTION_UPDATE")]
    ACTIONUPDATE,
    
    /// "ACTION_DELETE"
    #[serde(rename="ACTION_DELETE")]
    ACTIONDELETE,
    
    /// "ACTION_ENABLE"
    #[serde(rename="ACTION_ENABLE")]
    ACTIONENABLE,
    
    /// "ACTION_DISABLE"
    #[serde(rename="ACTION_DISABLE")]
    ACTIONDISABLE,
    
    /// "ACTION_ADD"
    #[serde(rename="ACTION_ADD")]
    ACTIONADD,
    
    /// "ACTION_REMOVE"
    #[serde(rename="ACTION_REMOVE")]
    ACTIONREMOVE,
    
    /// "ACTION_MARK_AS_DEFAULT"
    #[serde(rename="ACTION_MARK_AS_DEFAULT")]
    ACTIONMARKASDEFAULT,
    
    /// "ACTION_ASSOCIATE"
    #[serde(rename="ACTION_ASSOCIATE")]
    ACTIONASSOCIATE,
    
    /// "ACTION_ASSIGN"
    #[serde(rename="ACTION_ASSIGN")]
    ACTIONASSIGN,
    
    /// "ACTION_UNASSIGN"
    #[serde(rename="ACTION_UNASSIGN")]
    ACTIONUNASSIGN,
    
    /// "ACTION_SEND"
    #[serde(rename="ACTION_SEND")]
    ACTIONSEND,
    
    /// "ACTION_LINK"
    #[serde(rename="ACTION_LINK")]
    ACTIONLINK,
    
    /// "ACTION_UNLINK"
    #[serde(rename="ACTION_UNLINK")]
    ACTIONUNLINK,
    
    /// "ACTION_PUSH"
    #[serde(rename="ACTION_PUSH")]
    ACTIONPUSH,
    
    /// "ACTION_EMAIL_TAGS"
    #[serde(rename="ACTION_EMAIL_TAGS")]
    ACTIONEMAILTAGS,
    
    /// "ACTION_SHARE"
    #[serde(rename="ACTION_SHARE")]
    ACTIONSHARE,
}

impl AsRef<str> for ChangeLogActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChangeLogActionEnum::ACTIONCREATE => "ACTION_CREATE",
            ChangeLogActionEnum::ACTIONUPDATE => "ACTION_UPDATE",
            ChangeLogActionEnum::ACTIONDELETE => "ACTION_DELETE",
            ChangeLogActionEnum::ACTIONENABLE => "ACTION_ENABLE",
            ChangeLogActionEnum::ACTIONDISABLE => "ACTION_DISABLE",
            ChangeLogActionEnum::ACTIONADD => "ACTION_ADD",
            ChangeLogActionEnum::ACTIONREMOVE => "ACTION_REMOVE",
            ChangeLogActionEnum::ACTIONMARKASDEFAULT => "ACTION_MARK_AS_DEFAULT",
            ChangeLogActionEnum::ACTIONASSOCIATE => "ACTION_ASSOCIATE",
            ChangeLogActionEnum::ACTIONASSIGN => "ACTION_ASSIGN",
            ChangeLogActionEnum::ACTIONUNASSIGN => "ACTION_UNASSIGN",
            ChangeLogActionEnum::ACTIONSEND => "ACTION_SEND",
            ChangeLogActionEnum::ACTIONLINK => "ACTION_LINK",
            ChangeLogActionEnum::ACTIONUNLINK => "ACTION_UNLINK",
            ChangeLogActionEnum::ACTIONPUSH => "ACTION_PUSH",
            ChangeLogActionEnum::ACTIONEMAILTAGS => "ACTION_EMAIL_TAGS",
            ChangeLogActionEnum::ACTIONSHARE => "ACTION_SHARE",
        }
    }
}

impl std::convert::TryFrom< &str> for ChangeLogActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_CREATE" => Ok(ChangeLogActionEnum::ACTIONCREATE),
           "ACTION_UPDATE" => Ok(ChangeLogActionEnum::ACTIONUPDATE),
           "ACTION_DELETE" => Ok(ChangeLogActionEnum::ACTIONDELETE),
           "ACTION_ENABLE" => Ok(ChangeLogActionEnum::ACTIONENABLE),
           "ACTION_DISABLE" => Ok(ChangeLogActionEnum::ACTIONDISABLE),
           "ACTION_ADD" => Ok(ChangeLogActionEnum::ACTIONADD),
           "ACTION_REMOVE" => Ok(ChangeLogActionEnum::ACTIONREMOVE),
           "ACTION_MARK_AS_DEFAULT" => Ok(ChangeLogActionEnum::ACTIONMARKASDEFAULT),
           "ACTION_ASSOCIATE" => Ok(ChangeLogActionEnum::ACTIONASSOCIATE),
           "ACTION_ASSIGN" => Ok(ChangeLogActionEnum::ACTIONASSIGN),
           "ACTION_UNASSIGN" => Ok(ChangeLogActionEnum::ACTIONUNASSIGN),
           "ACTION_SEND" => Ok(ChangeLogActionEnum::ACTIONSEND),
           "ACTION_LINK" => Ok(ChangeLogActionEnum::ACTIONLINK),
           "ACTION_UNLINK" => Ok(ChangeLogActionEnum::ACTIONUNLINK),
           "ACTION_PUSH" => Ok(ChangeLogActionEnum::ACTIONPUSH),
           "ACTION_EMAIL_TAGS" => Ok(ChangeLogActionEnum::ACTIONEMAILTAGS),
           "ACTION_SHARE" => Ok(ChangeLogActionEnum::ACTIONSHARE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChangeLogActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChangeLogObjectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only change logs with the specified object type.
pub enum ChangeLogObjectTypeEnum {
    
    /// "OBJECT_ADVERTISER"
    #[serde(rename="OBJECT_ADVERTISER")]
    OBJECTADVERTISER,
    
    /// "OBJECT_FLOODLIGHT_CONFIGURATION"
    #[serde(rename="OBJECT_FLOODLIGHT_CONFIGURATION")]
    OBJECTFLOODLIGHTCONFIGURATION,
    
    /// "OBJECT_AD"
    #[serde(rename="OBJECT_AD")]
    OBJECTAD,
    
    /// "OBJECT_FLOODLIGHT_ACTVITY"
    #[serde(rename="OBJECT_FLOODLIGHT_ACTVITY")]
    OBJECTFLOODLIGHTACTVITY,
    
    /// "OBJECT_CAMPAIGN"
    #[serde(rename="OBJECT_CAMPAIGN")]
    OBJECTCAMPAIGN,
    
    /// "OBJECT_FLOODLIGHT_ACTIVITY_GROUP"
    #[serde(rename="OBJECT_FLOODLIGHT_ACTIVITY_GROUP")]
    OBJECTFLOODLIGHTACTIVITYGROUP,
    
    /// "OBJECT_CREATIVE"
    #[serde(rename="OBJECT_CREATIVE")]
    OBJECTCREATIVE,
    
    /// "OBJECT_PLACEMENT"
    #[serde(rename="OBJECT_PLACEMENT")]
    OBJECTPLACEMENT,
    
    /// "OBJECT_DFA_SITE"
    #[serde(rename="OBJECT_DFA_SITE")]
    OBJECTDFASITE,
    
    /// "OBJECT_USER_ROLE"
    #[serde(rename="OBJECT_USER_ROLE")]
    OBJECTUSERROLE,
    
    /// "OBJECT_USER_PROFILE"
    #[serde(rename="OBJECT_USER_PROFILE")]
    OBJECTUSERPROFILE,
    
    /// "OBJECT_ADVERTISER_GROUP"
    #[serde(rename="OBJECT_ADVERTISER_GROUP")]
    OBJECTADVERTISERGROUP,
    
    /// "OBJECT_ACCOUNT"
    #[serde(rename="OBJECT_ACCOUNT")]
    OBJECTACCOUNT,
    
    /// "OBJECT_SUBACCOUNT"
    #[serde(rename="OBJECT_SUBACCOUNT")]
    OBJECTSUBACCOUNT,
    
    /// "OBJECT_RICHMEDIA_CREATIVE"
    #[serde(rename="OBJECT_RICHMEDIA_CREATIVE")]
    OBJECTRICHMEDIACREATIVE,
    
    /// "OBJECT_INSTREAM_CREATIVE"
    #[serde(rename="OBJECT_INSTREAM_CREATIVE")]
    OBJECTINSTREAMCREATIVE,
    
    /// "OBJECT_MEDIA_ORDER"
    #[serde(rename="OBJECT_MEDIA_ORDER")]
    OBJECTMEDIAORDER,
    
    /// "OBJECT_CONTENT_CATEGORY"
    #[serde(rename="OBJECT_CONTENT_CATEGORY")]
    OBJECTCONTENTCATEGORY,
    
    /// "OBJECT_PLACEMENT_STRATEGY"
    #[serde(rename="OBJECT_PLACEMENT_STRATEGY")]
    OBJECTPLACEMENTSTRATEGY,
    
    /// "OBJECT_SD_SITE"
    #[serde(rename="OBJECT_SD_SITE")]
    OBJECTSDSITE,
    
    /// "OBJECT_SIZE"
    #[serde(rename="OBJECT_SIZE")]
    OBJECTSIZE,
    
    /// "OBJECT_CREATIVE_GROUP"
    #[serde(rename="OBJECT_CREATIVE_GROUP")]
    OBJECTCREATIVEGROUP,
    
    /// "OBJECT_CREATIVE_ASSET"
    #[serde(rename="OBJECT_CREATIVE_ASSET")]
    OBJECTCREATIVEASSET,
    
    /// "OBJECT_USER_PROFILE_FILTER"
    #[serde(rename="OBJECT_USER_PROFILE_FILTER")]
    OBJECTUSERPROFILEFILTER,
    
    /// "OBJECT_LANDING_PAGE"
    #[serde(rename="OBJECT_LANDING_PAGE")]
    OBJECTLANDINGPAGE,
    
    /// "OBJECT_CREATIVE_FIELD"
    #[serde(rename="OBJECT_CREATIVE_FIELD")]
    OBJECTCREATIVEFIELD,
    
    /// "OBJECT_REMARKETING_LIST"
    #[serde(rename="OBJECT_REMARKETING_LIST")]
    OBJECTREMARKETINGLIST,
    
    /// "OBJECT_PROVIDED_LIST_CLIENT"
    #[serde(rename="OBJECT_PROVIDED_LIST_CLIENT")]
    OBJECTPROVIDEDLISTCLIENT,
    
    /// "OBJECT_EVENT_TAG"
    #[serde(rename="OBJECT_EVENT_TAG")]
    OBJECTEVENTTAG,
    
    /// "OBJECT_CREATIVE_BUNDLE"
    #[serde(rename="OBJECT_CREATIVE_BUNDLE")]
    OBJECTCREATIVEBUNDLE,
    
    /// "OBJECT_BILLING_ACCOUNT_GROUP"
    #[serde(rename="OBJECT_BILLING_ACCOUNT_GROUP")]
    OBJECTBILLINGACCOUNTGROUP,
    
    /// "OBJECT_BILLING_FEATURE"
    #[serde(rename="OBJECT_BILLING_FEATURE")]
    OBJECTBILLINGFEATURE,
    
    /// "OBJECT_RATE_CARD"
    #[serde(rename="OBJECT_RATE_CARD")]
    OBJECTRATECARD,
    
    /// "OBJECT_ACCOUNT_BILLING_FEATURE"
    #[serde(rename="OBJECT_ACCOUNT_BILLING_FEATURE")]
    OBJECTACCOUNTBILLINGFEATURE,
    
    /// "OBJECT_BILLING_MINIMUM_FEE"
    #[serde(rename="OBJECT_BILLING_MINIMUM_FEE")]
    OBJECTBILLINGMINIMUMFEE,
    
    /// "OBJECT_BILLING_PROFILE"
    #[serde(rename="OBJECT_BILLING_PROFILE")]
    OBJECTBILLINGPROFILE,
    
    /// "OBJECT_PLAYSTORE_LINK"
    #[serde(rename="OBJECT_PLAYSTORE_LINK")]
    OBJECTPLAYSTORELINK,
    
    /// "OBJECT_TARGETING_TEMPLATE"
    #[serde(rename="OBJECT_TARGETING_TEMPLATE")]
    OBJECTTARGETINGTEMPLATE,
    
    /// "OBJECT_SEARCH_LIFT_STUDY"
    #[serde(rename="OBJECT_SEARCH_LIFT_STUDY")]
    OBJECTSEARCHLIFTSTUDY,
    
    /// "OBJECT_FLOODLIGHT_DV360_LINK"
    #[serde(rename="OBJECT_FLOODLIGHT_DV360_LINK")]
    OBJECTFLOODLIGHTDV360LINK,
}

impl AsRef<str> for ChangeLogObjectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChangeLogObjectTypeEnum::OBJECTADVERTISER => "OBJECT_ADVERTISER",
            ChangeLogObjectTypeEnum::OBJECTFLOODLIGHTCONFIGURATION => "OBJECT_FLOODLIGHT_CONFIGURATION",
            ChangeLogObjectTypeEnum::OBJECTAD => "OBJECT_AD",
            ChangeLogObjectTypeEnum::OBJECTFLOODLIGHTACTVITY => "OBJECT_FLOODLIGHT_ACTVITY",
            ChangeLogObjectTypeEnum::OBJECTCAMPAIGN => "OBJECT_CAMPAIGN",
            ChangeLogObjectTypeEnum::OBJECTFLOODLIGHTACTIVITYGROUP => "OBJECT_FLOODLIGHT_ACTIVITY_GROUP",
            ChangeLogObjectTypeEnum::OBJECTCREATIVE => "OBJECT_CREATIVE",
            ChangeLogObjectTypeEnum::OBJECTPLACEMENT => "OBJECT_PLACEMENT",
            ChangeLogObjectTypeEnum::OBJECTDFASITE => "OBJECT_DFA_SITE",
            ChangeLogObjectTypeEnum::OBJECTUSERROLE => "OBJECT_USER_ROLE",
            ChangeLogObjectTypeEnum::OBJECTUSERPROFILE => "OBJECT_USER_PROFILE",
            ChangeLogObjectTypeEnum::OBJECTADVERTISERGROUP => "OBJECT_ADVERTISER_GROUP",
            ChangeLogObjectTypeEnum::OBJECTACCOUNT => "OBJECT_ACCOUNT",
            ChangeLogObjectTypeEnum::OBJECTSUBACCOUNT => "OBJECT_SUBACCOUNT",
            ChangeLogObjectTypeEnum::OBJECTRICHMEDIACREATIVE => "OBJECT_RICHMEDIA_CREATIVE",
            ChangeLogObjectTypeEnum::OBJECTINSTREAMCREATIVE => "OBJECT_INSTREAM_CREATIVE",
            ChangeLogObjectTypeEnum::OBJECTMEDIAORDER => "OBJECT_MEDIA_ORDER",
            ChangeLogObjectTypeEnum::OBJECTCONTENTCATEGORY => "OBJECT_CONTENT_CATEGORY",
            ChangeLogObjectTypeEnum::OBJECTPLACEMENTSTRATEGY => "OBJECT_PLACEMENT_STRATEGY",
            ChangeLogObjectTypeEnum::OBJECTSDSITE => "OBJECT_SD_SITE",
            ChangeLogObjectTypeEnum::OBJECTSIZE => "OBJECT_SIZE",
            ChangeLogObjectTypeEnum::OBJECTCREATIVEGROUP => "OBJECT_CREATIVE_GROUP",
            ChangeLogObjectTypeEnum::OBJECTCREATIVEASSET => "OBJECT_CREATIVE_ASSET",
            ChangeLogObjectTypeEnum::OBJECTUSERPROFILEFILTER => "OBJECT_USER_PROFILE_FILTER",
            ChangeLogObjectTypeEnum::OBJECTLANDINGPAGE => "OBJECT_LANDING_PAGE",
            ChangeLogObjectTypeEnum::OBJECTCREATIVEFIELD => "OBJECT_CREATIVE_FIELD",
            ChangeLogObjectTypeEnum::OBJECTREMARKETINGLIST => "OBJECT_REMARKETING_LIST",
            ChangeLogObjectTypeEnum::OBJECTPROVIDEDLISTCLIENT => "OBJECT_PROVIDED_LIST_CLIENT",
            ChangeLogObjectTypeEnum::OBJECTEVENTTAG => "OBJECT_EVENT_TAG",
            ChangeLogObjectTypeEnum::OBJECTCREATIVEBUNDLE => "OBJECT_CREATIVE_BUNDLE",
            ChangeLogObjectTypeEnum::OBJECTBILLINGACCOUNTGROUP => "OBJECT_BILLING_ACCOUNT_GROUP",
            ChangeLogObjectTypeEnum::OBJECTBILLINGFEATURE => "OBJECT_BILLING_FEATURE",
            ChangeLogObjectTypeEnum::OBJECTRATECARD => "OBJECT_RATE_CARD",
            ChangeLogObjectTypeEnum::OBJECTACCOUNTBILLINGFEATURE => "OBJECT_ACCOUNT_BILLING_FEATURE",
            ChangeLogObjectTypeEnum::OBJECTBILLINGMINIMUMFEE => "OBJECT_BILLING_MINIMUM_FEE",
            ChangeLogObjectTypeEnum::OBJECTBILLINGPROFILE => "OBJECT_BILLING_PROFILE",
            ChangeLogObjectTypeEnum::OBJECTPLAYSTORELINK => "OBJECT_PLAYSTORE_LINK",
            ChangeLogObjectTypeEnum::OBJECTTARGETINGTEMPLATE => "OBJECT_TARGETING_TEMPLATE",
            ChangeLogObjectTypeEnum::OBJECTSEARCHLIFTSTUDY => "OBJECT_SEARCH_LIFT_STUDY",
            ChangeLogObjectTypeEnum::OBJECTFLOODLIGHTDV360LINK => "OBJECT_FLOODLIGHT_DV360_LINK",
        }
    }
}

impl std::convert::TryFrom< &str> for ChangeLogObjectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OBJECT_ADVERTISER" => Ok(ChangeLogObjectTypeEnum::OBJECTADVERTISER),
           "OBJECT_FLOODLIGHT_CONFIGURATION" => Ok(ChangeLogObjectTypeEnum::OBJECTFLOODLIGHTCONFIGURATION),
           "OBJECT_AD" => Ok(ChangeLogObjectTypeEnum::OBJECTAD),
           "OBJECT_FLOODLIGHT_ACTVITY" => Ok(ChangeLogObjectTypeEnum::OBJECTFLOODLIGHTACTVITY),
           "OBJECT_CAMPAIGN" => Ok(ChangeLogObjectTypeEnum::OBJECTCAMPAIGN),
           "OBJECT_FLOODLIGHT_ACTIVITY_GROUP" => Ok(ChangeLogObjectTypeEnum::OBJECTFLOODLIGHTACTIVITYGROUP),
           "OBJECT_CREATIVE" => Ok(ChangeLogObjectTypeEnum::OBJECTCREATIVE),
           "OBJECT_PLACEMENT" => Ok(ChangeLogObjectTypeEnum::OBJECTPLACEMENT),
           "OBJECT_DFA_SITE" => Ok(ChangeLogObjectTypeEnum::OBJECTDFASITE),
           "OBJECT_USER_ROLE" => Ok(ChangeLogObjectTypeEnum::OBJECTUSERROLE),
           "OBJECT_USER_PROFILE" => Ok(ChangeLogObjectTypeEnum::OBJECTUSERPROFILE),
           "OBJECT_ADVERTISER_GROUP" => Ok(ChangeLogObjectTypeEnum::OBJECTADVERTISERGROUP),
           "OBJECT_ACCOUNT" => Ok(ChangeLogObjectTypeEnum::OBJECTACCOUNT),
           "OBJECT_SUBACCOUNT" => Ok(ChangeLogObjectTypeEnum::OBJECTSUBACCOUNT),
           "OBJECT_RICHMEDIA_CREATIVE" => Ok(ChangeLogObjectTypeEnum::OBJECTRICHMEDIACREATIVE),
           "OBJECT_INSTREAM_CREATIVE" => Ok(ChangeLogObjectTypeEnum::OBJECTINSTREAMCREATIVE),
           "OBJECT_MEDIA_ORDER" => Ok(ChangeLogObjectTypeEnum::OBJECTMEDIAORDER),
           "OBJECT_CONTENT_CATEGORY" => Ok(ChangeLogObjectTypeEnum::OBJECTCONTENTCATEGORY),
           "OBJECT_PLACEMENT_STRATEGY" => Ok(ChangeLogObjectTypeEnum::OBJECTPLACEMENTSTRATEGY),
           "OBJECT_SD_SITE" => Ok(ChangeLogObjectTypeEnum::OBJECTSDSITE),
           "OBJECT_SIZE" => Ok(ChangeLogObjectTypeEnum::OBJECTSIZE),
           "OBJECT_CREATIVE_GROUP" => Ok(ChangeLogObjectTypeEnum::OBJECTCREATIVEGROUP),
           "OBJECT_CREATIVE_ASSET" => Ok(ChangeLogObjectTypeEnum::OBJECTCREATIVEASSET),
           "OBJECT_USER_PROFILE_FILTER" => Ok(ChangeLogObjectTypeEnum::OBJECTUSERPROFILEFILTER),
           "OBJECT_LANDING_PAGE" => Ok(ChangeLogObjectTypeEnum::OBJECTLANDINGPAGE),
           "OBJECT_CREATIVE_FIELD" => Ok(ChangeLogObjectTypeEnum::OBJECTCREATIVEFIELD),
           "OBJECT_REMARKETING_LIST" => Ok(ChangeLogObjectTypeEnum::OBJECTREMARKETINGLIST),
           "OBJECT_PROVIDED_LIST_CLIENT" => Ok(ChangeLogObjectTypeEnum::OBJECTPROVIDEDLISTCLIENT),
           "OBJECT_EVENT_TAG" => Ok(ChangeLogObjectTypeEnum::OBJECTEVENTTAG),
           "OBJECT_CREATIVE_BUNDLE" => Ok(ChangeLogObjectTypeEnum::OBJECTCREATIVEBUNDLE),
           "OBJECT_BILLING_ACCOUNT_GROUP" => Ok(ChangeLogObjectTypeEnum::OBJECTBILLINGACCOUNTGROUP),
           "OBJECT_BILLING_FEATURE" => Ok(ChangeLogObjectTypeEnum::OBJECTBILLINGFEATURE),
           "OBJECT_RATE_CARD" => Ok(ChangeLogObjectTypeEnum::OBJECTRATECARD),
           "OBJECT_ACCOUNT_BILLING_FEATURE" => Ok(ChangeLogObjectTypeEnum::OBJECTACCOUNTBILLINGFEATURE),
           "OBJECT_BILLING_MINIMUM_FEE" => Ok(ChangeLogObjectTypeEnum::OBJECTBILLINGMINIMUMFEE),
           "OBJECT_BILLING_PROFILE" => Ok(ChangeLogObjectTypeEnum::OBJECTBILLINGPROFILE),
           "OBJECT_PLAYSTORE_LINK" => Ok(ChangeLogObjectTypeEnum::OBJECTPLAYSTORELINK),
           "OBJECT_TARGETING_TEMPLATE" => Ok(ChangeLogObjectTypeEnum::OBJECTTARGETINGTEMPLATE),
           "OBJECT_SEARCH_LIFT_STUDY" => Ok(ChangeLogObjectTypeEnum::OBJECTSEARCHLIFTSTUDY),
           "OBJECT_FLOODLIGHT_DV360_LINK" => Ok(ChangeLogObjectTypeEnum::OBJECTFLOODLIGHTDV360LINK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChangeLogObjectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentCategorySortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum ContentCategorySortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for ContentCategorySortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentCategorySortFieldEnum::ID => "ID",
            ContentCategorySortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentCategorySortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(ContentCategorySortFieldEnum::ID),
           "NAME" => Ok(ContentCategorySortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentCategorySortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ContentCategorySortFieldEnum {
    fn default() -> ContentCategorySortFieldEnum {
        ContentCategorySortFieldEnum::ID
    }
}

// endregion


// region ContentCategorySortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum ContentCategorySortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for ContentCategorySortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentCategorySortOrderEnum::ASCENDING => "ASCENDING",
            ContentCategorySortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentCategorySortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(ContentCategorySortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(ContentCategorySortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentCategorySortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ContentCategorySortOrderEnum {
    fn default() -> ContentCategorySortOrderEnum {
        ContentCategorySortOrderEnum::ASCENDING
    }
}

// endregion


// region CreativeFieldValueSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum CreativeFieldValueSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "VALUE"
    #[serde(rename="VALUE")]
    VALUE,
}

impl AsRef<str> for CreativeFieldValueSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeFieldValueSortFieldEnum::ID => "ID",
            CreativeFieldValueSortFieldEnum::VALUE => "VALUE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeFieldValueSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(CreativeFieldValueSortFieldEnum::ID),
           "VALUE" => Ok(CreativeFieldValueSortFieldEnum::VALUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeFieldValueSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CreativeFieldValueSortFieldEnum {
    fn default() -> CreativeFieldValueSortFieldEnum {
        CreativeFieldValueSortFieldEnum::ID
    }
}

// endregion


// region CreativeFieldValueSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum CreativeFieldValueSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for CreativeFieldValueSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeFieldValueSortOrderEnum::ASCENDING => "ASCENDING",
            CreativeFieldValueSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeFieldValueSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(CreativeFieldValueSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(CreativeFieldValueSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeFieldValueSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CreativeFieldValueSortOrderEnum {
    fn default() -> CreativeFieldValueSortOrderEnum {
        CreativeFieldValueSortOrderEnum::ASCENDING
    }
}

// endregion


// region CreativeFieldSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum CreativeFieldSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for CreativeFieldSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeFieldSortFieldEnum::ID => "ID",
            CreativeFieldSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeFieldSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(CreativeFieldSortFieldEnum::ID),
           "NAME" => Ok(CreativeFieldSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeFieldSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CreativeFieldSortFieldEnum {
    fn default() -> CreativeFieldSortFieldEnum {
        CreativeFieldSortFieldEnum::ID
    }
}

// endregion


// region CreativeFieldSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum CreativeFieldSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for CreativeFieldSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeFieldSortOrderEnum::ASCENDING => "ASCENDING",
            CreativeFieldSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeFieldSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(CreativeFieldSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(CreativeFieldSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeFieldSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CreativeFieldSortOrderEnum {
    fn default() -> CreativeFieldSortOrderEnum {
        CreativeFieldSortOrderEnum::ASCENDING
    }
}

// endregion


// region CreativeGroupSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum CreativeGroupSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for CreativeGroupSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeGroupSortFieldEnum::ID => "ID",
            CreativeGroupSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeGroupSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(CreativeGroupSortFieldEnum::ID),
           "NAME" => Ok(CreativeGroupSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeGroupSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CreativeGroupSortFieldEnum {
    fn default() -> CreativeGroupSortFieldEnum {
        CreativeGroupSortFieldEnum::ID
    }
}

// endregion


// region CreativeGroupSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum CreativeGroupSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for CreativeGroupSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeGroupSortOrderEnum::ASCENDING => "ASCENDING",
            CreativeGroupSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeGroupSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(CreativeGroupSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(CreativeGroupSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeGroupSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CreativeGroupSortOrderEnum {
    fn default() -> CreativeGroupSortOrderEnum {
        CreativeGroupSortOrderEnum::ASCENDING
    }
}

// endregion


// region CreativeSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum CreativeSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for CreativeSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeSortFieldEnum::ID => "ID",
            CreativeSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(CreativeSortFieldEnum::ID),
           "NAME" => Ok(CreativeSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CreativeSortFieldEnum {
    fn default() -> CreativeSortFieldEnum {
        CreativeSortFieldEnum::ID
    }
}

// endregion


// region CreativeSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum CreativeSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for CreativeSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeSortOrderEnum::ASCENDING => "ASCENDING",
            CreativeSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(CreativeSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(CreativeSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CreativeSortOrderEnum {
    fn default() -> CreativeSortOrderEnum {
        CreativeSortOrderEnum::ASCENDING
    }
}

// endregion


// region CreativeTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only creatives with these creative types.
pub enum CreativeTypesEnum {
    
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    
    /// "DISPLAY_REDIRECT"
    #[serde(rename="DISPLAY_REDIRECT")]
    DISPLAYREDIRECT,
    
    /// "CUSTOM_DISPLAY"
    #[serde(rename="CUSTOM_DISPLAY")]
    CUSTOMDISPLAY,
    
    /// "INTERNAL_REDIRECT"
    #[serde(rename="INTERNAL_REDIRECT")]
    INTERNALREDIRECT,
    
    /// "CUSTOM_DISPLAY_INTERSTITIAL"
    #[serde(rename="CUSTOM_DISPLAY_INTERSTITIAL")]
    CUSTOMDISPLAYINTERSTITIAL,
    
    /// "INTERSTITIAL_INTERNAL_REDIRECT"
    #[serde(rename="INTERSTITIAL_INTERNAL_REDIRECT")]
    INTERSTITIALINTERNALREDIRECT,
    
    /// "TRACKING_TEXT"
    #[serde(rename="TRACKING_TEXT")]
    TRACKINGTEXT,
    
    /// "RICH_MEDIA_DISPLAY_BANNER"
    #[serde(rename="RICH_MEDIA_DISPLAY_BANNER")]
    RICHMEDIADISPLAYBANNER,
    
    /// "RICH_MEDIA_INPAGE_FLOATING"
    #[serde(rename="RICH_MEDIA_INPAGE_FLOATING")]
    RICHMEDIAINPAGEFLOATING,
    
    /// "RICH_MEDIA_IM_EXPAND"
    #[serde(rename="RICH_MEDIA_IM_EXPAND")]
    RICHMEDIAIMEXPAND,
    
    /// "RICH_MEDIA_DISPLAY_EXPANDING"
    #[serde(rename="RICH_MEDIA_DISPLAY_EXPANDING")]
    RICHMEDIADISPLAYEXPANDING,
    
    /// "RICH_MEDIA_DISPLAY_INTERSTITIAL"
    #[serde(rename="RICH_MEDIA_DISPLAY_INTERSTITIAL")]
    RICHMEDIADISPLAYINTERSTITIAL,
    
    /// "RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL"
    #[serde(rename="RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL")]
    RICHMEDIADISPLAYMULTIFLOATINGINTERSTITIAL,
    
    /// "RICH_MEDIA_MOBILE_IN_APP"
    #[serde(rename="RICH_MEDIA_MOBILE_IN_APP")]
    RICHMEDIAMOBILEINAPP,
    
    /// "FLASH_INPAGE"
    #[serde(rename="FLASH_INPAGE")]
    FLASHINPAGE,
    
    /// "INSTREAM_VIDEO"
    #[serde(rename="INSTREAM_VIDEO")]
    INSTREAMVIDEO,
    
    /// "VPAID_LINEAR_VIDEO"
    #[serde(rename="VPAID_LINEAR_VIDEO")]
    VPAIDLINEARVIDEO,
    
    /// "VPAID_NON_LINEAR_VIDEO"
    #[serde(rename="VPAID_NON_LINEAR_VIDEO")]
    VPAIDNONLINEARVIDEO,
    
    /// "INSTREAM_VIDEO_REDIRECT"
    #[serde(rename="INSTREAM_VIDEO_REDIRECT")]
    INSTREAMVIDEOREDIRECT,
    
    /// "RICH_MEDIA_PEEL_DOWN"
    #[serde(rename="RICH_MEDIA_PEEL_DOWN")]
    RICHMEDIAPEELDOWN,
    
    /// "HTML5_BANNER"
    #[serde(rename="HTML5_BANNER")]
    HTML5BANNER,
    
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    
    /// "DISPLAY_IMAGE_GALLERY"
    #[serde(rename="DISPLAY_IMAGE_GALLERY")]
    DISPLAYIMAGEGALLERY,
    
    /// "BRAND_SAFE_DEFAULT_INSTREAM_VIDEO"
    #[serde(rename="BRAND_SAFE_DEFAULT_INSTREAM_VIDEO")]
    BRANDSAFEDEFAULTINSTREAMVIDEO,
    
    /// "INSTREAM_AUDIO"
    #[serde(rename="INSTREAM_AUDIO")]
    INSTREAMAUDIO,
}

impl AsRef<str> for CreativeTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeTypesEnum::IMAGE => "IMAGE",
            CreativeTypesEnum::DISPLAYREDIRECT => "DISPLAY_REDIRECT",
            CreativeTypesEnum::CUSTOMDISPLAY => "CUSTOM_DISPLAY",
            CreativeTypesEnum::INTERNALREDIRECT => "INTERNAL_REDIRECT",
            CreativeTypesEnum::CUSTOMDISPLAYINTERSTITIAL => "CUSTOM_DISPLAY_INTERSTITIAL",
            CreativeTypesEnum::INTERSTITIALINTERNALREDIRECT => "INTERSTITIAL_INTERNAL_REDIRECT",
            CreativeTypesEnum::TRACKINGTEXT => "TRACKING_TEXT",
            CreativeTypesEnum::RICHMEDIADISPLAYBANNER => "RICH_MEDIA_DISPLAY_BANNER",
            CreativeTypesEnum::RICHMEDIAINPAGEFLOATING => "RICH_MEDIA_INPAGE_FLOATING",
            CreativeTypesEnum::RICHMEDIAIMEXPAND => "RICH_MEDIA_IM_EXPAND",
            CreativeTypesEnum::RICHMEDIADISPLAYEXPANDING => "RICH_MEDIA_DISPLAY_EXPANDING",
            CreativeTypesEnum::RICHMEDIADISPLAYINTERSTITIAL => "RICH_MEDIA_DISPLAY_INTERSTITIAL",
            CreativeTypesEnum::RICHMEDIADISPLAYMULTIFLOATINGINTERSTITIAL => "RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL",
            CreativeTypesEnum::RICHMEDIAMOBILEINAPP => "RICH_MEDIA_MOBILE_IN_APP",
            CreativeTypesEnum::FLASHINPAGE => "FLASH_INPAGE",
            CreativeTypesEnum::INSTREAMVIDEO => "INSTREAM_VIDEO",
            CreativeTypesEnum::VPAIDLINEARVIDEO => "VPAID_LINEAR_VIDEO",
            CreativeTypesEnum::VPAIDNONLINEARVIDEO => "VPAID_NON_LINEAR_VIDEO",
            CreativeTypesEnum::INSTREAMVIDEOREDIRECT => "INSTREAM_VIDEO_REDIRECT",
            CreativeTypesEnum::RICHMEDIAPEELDOWN => "RICH_MEDIA_PEEL_DOWN",
            CreativeTypesEnum::HTML5BANNER => "HTML5_BANNER",
            CreativeTypesEnum::DISPLAY => "DISPLAY",
            CreativeTypesEnum::DISPLAYIMAGEGALLERY => "DISPLAY_IMAGE_GALLERY",
            CreativeTypesEnum::BRANDSAFEDEFAULTINSTREAMVIDEO => "BRAND_SAFE_DEFAULT_INSTREAM_VIDEO",
            CreativeTypesEnum::INSTREAMAUDIO => "INSTREAM_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMAGE" => Ok(CreativeTypesEnum::IMAGE),
           "DISPLAY_REDIRECT" => Ok(CreativeTypesEnum::DISPLAYREDIRECT),
           "CUSTOM_DISPLAY" => Ok(CreativeTypesEnum::CUSTOMDISPLAY),
           "INTERNAL_REDIRECT" => Ok(CreativeTypesEnum::INTERNALREDIRECT),
           "CUSTOM_DISPLAY_INTERSTITIAL" => Ok(CreativeTypesEnum::CUSTOMDISPLAYINTERSTITIAL),
           "INTERSTITIAL_INTERNAL_REDIRECT" => Ok(CreativeTypesEnum::INTERSTITIALINTERNALREDIRECT),
           "TRACKING_TEXT" => Ok(CreativeTypesEnum::TRACKINGTEXT),
           "RICH_MEDIA_DISPLAY_BANNER" => Ok(CreativeTypesEnum::RICHMEDIADISPLAYBANNER),
           "RICH_MEDIA_INPAGE_FLOATING" => Ok(CreativeTypesEnum::RICHMEDIAINPAGEFLOATING),
           "RICH_MEDIA_IM_EXPAND" => Ok(CreativeTypesEnum::RICHMEDIAIMEXPAND),
           "RICH_MEDIA_DISPLAY_EXPANDING" => Ok(CreativeTypesEnum::RICHMEDIADISPLAYEXPANDING),
           "RICH_MEDIA_DISPLAY_INTERSTITIAL" => Ok(CreativeTypesEnum::RICHMEDIADISPLAYINTERSTITIAL),
           "RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL" => Ok(CreativeTypesEnum::RICHMEDIADISPLAYMULTIFLOATINGINTERSTITIAL),
           "RICH_MEDIA_MOBILE_IN_APP" => Ok(CreativeTypesEnum::RICHMEDIAMOBILEINAPP),
           "FLASH_INPAGE" => Ok(CreativeTypesEnum::FLASHINPAGE),
           "INSTREAM_VIDEO" => Ok(CreativeTypesEnum::INSTREAMVIDEO),
           "VPAID_LINEAR_VIDEO" => Ok(CreativeTypesEnum::VPAIDLINEARVIDEO),
           "VPAID_NON_LINEAR_VIDEO" => Ok(CreativeTypesEnum::VPAIDNONLINEARVIDEO),
           "INSTREAM_VIDEO_REDIRECT" => Ok(CreativeTypesEnum::INSTREAMVIDEOREDIRECT),
           "RICH_MEDIA_PEEL_DOWN" => Ok(CreativeTypesEnum::RICHMEDIAPEELDOWN),
           "HTML5_BANNER" => Ok(CreativeTypesEnum::HTML5BANNER),
           "DISPLAY" => Ok(CreativeTypesEnum::DISPLAY),
           "DISPLAY_IMAGE_GALLERY" => Ok(CreativeTypesEnum::DISPLAYIMAGEGALLERY),
           "BRAND_SAFE_DEFAULT_INSTREAM_VIDEO" => Ok(CreativeTypesEnum::BRANDSAFEDEFAULTINSTREAMVIDEO),
           "INSTREAM_AUDIO" => Ok(CreativeTypesEnum::INSTREAMAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DirectorySiteSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum DirectorySiteSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for DirectorySiteSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DirectorySiteSortFieldEnum::ID => "ID",
            DirectorySiteSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for DirectorySiteSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(DirectorySiteSortFieldEnum::ID),
           "NAME" => Ok(DirectorySiteSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DirectorySiteSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for DirectorySiteSortFieldEnum {
    fn default() -> DirectorySiteSortFieldEnum {
        DirectorySiteSortFieldEnum::ID
    }
}

// endregion


// region DirectorySiteSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum DirectorySiteSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for DirectorySiteSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DirectorySiteSortOrderEnum::ASCENDING => "ASCENDING",
            DirectorySiteSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for DirectorySiteSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(DirectorySiteSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(DirectorySiteSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DirectorySiteSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for DirectorySiteSortOrderEnum {
    fn default() -> DirectorySiteSortOrderEnum {
        DirectorySiteSortOrderEnum::ASCENDING
    }
}

// endregion


// region DynamicTargetingKeyObjectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only dynamic targeting keys with this object type.
pub enum DynamicTargetingKeyObjectTypeEnum {
    
    /// "OBJECT_ADVERTISER"
    #[serde(rename="OBJECT_ADVERTISER")]
    OBJECTADVERTISER,
    
    /// "OBJECT_AD"
    #[serde(rename="OBJECT_AD")]
    OBJECTAD,
    
    /// "OBJECT_CREATIVE"
    #[serde(rename="OBJECT_CREATIVE")]
    OBJECTCREATIVE,
    
    /// "OBJECT_PLACEMENT"
    #[serde(rename="OBJECT_PLACEMENT")]
    OBJECTPLACEMENT,
}

impl AsRef<str> for DynamicTargetingKeyObjectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DynamicTargetingKeyObjectTypeEnum::OBJECTADVERTISER => "OBJECT_ADVERTISER",
            DynamicTargetingKeyObjectTypeEnum::OBJECTAD => "OBJECT_AD",
            DynamicTargetingKeyObjectTypeEnum::OBJECTCREATIVE => "OBJECT_CREATIVE",
            DynamicTargetingKeyObjectTypeEnum::OBJECTPLACEMENT => "OBJECT_PLACEMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for DynamicTargetingKeyObjectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OBJECT_ADVERTISER" => Ok(DynamicTargetingKeyObjectTypeEnum::OBJECTADVERTISER),
           "OBJECT_AD" => Ok(DynamicTargetingKeyObjectTypeEnum::OBJECTAD),
           "OBJECT_CREATIVE" => Ok(DynamicTargetingKeyObjectTypeEnum::OBJECTCREATIVE),
           "OBJECT_PLACEMENT" => Ok(DynamicTargetingKeyObjectTypeEnum::OBJECTPLACEMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DynamicTargetingKeyObjectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTagEventTagTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only event tags with the specified event tag types. Event tag types can be used to specify whether to use a third-party pixel, a third-party JavaScript URL, or a third-party click-through URL for either impression or click tracking.
pub enum EventTagEventTagTypesEnum {
    
    /// "IMPRESSION_IMAGE_EVENT_TAG"
    #[serde(rename="IMPRESSION_IMAGE_EVENT_TAG")]
    IMPRESSIONIMAGEEVENTTAG,
    
    /// "IMPRESSION_JAVASCRIPT_EVENT_TAG"
    #[serde(rename="IMPRESSION_JAVASCRIPT_EVENT_TAG")]
    IMPRESSIONJAVASCRIPTEVENTTAG,
    
    /// "CLICK_THROUGH_EVENT_TAG"
    #[serde(rename="CLICK_THROUGH_EVENT_TAG")]
    CLICKTHROUGHEVENTTAG,
}

impl AsRef<str> for EventTagEventTagTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTagEventTagTypesEnum::IMPRESSIONIMAGEEVENTTAG => "IMPRESSION_IMAGE_EVENT_TAG",
            EventTagEventTagTypesEnum::IMPRESSIONJAVASCRIPTEVENTTAG => "IMPRESSION_JAVASCRIPT_EVENT_TAG",
            EventTagEventTagTypesEnum::CLICKTHROUGHEVENTTAG => "CLICK_THROUGH_EVENT_TAG",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTagEventTagTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPRESSION_IMAGE_EVENT_TAG" => Ok(EventTagEventTagTypesEnum::IMPRESSIONIMAGEEVENTTAG),
           "IMPRESSION_JAVASCRIPT_EVENT_TAG" => Ok(EventTagEventTagTypesEnum::IMPRESSIONJAVASCRIPTEVENTTAG),
           "CLICK_THROUGH_EVENT_TAG" => Ok(EventTagEventTagTypesEnum::CLICKTHROUGHEVENTTAG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTagEventTagTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTagSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum EventTagSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for EventTagSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTagSortFieldEnum::ID => "ID",
            EventTagSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTagSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(EventTagSortFieldEnum::ID),
           "NAME" => Ok(EventTagSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTagSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for EventTagSortFieldEnum {
    fn default() -> EventTagSortFieldEnum {
        EventTagSortFieldEnum::ID
    }
}

// endregion


// region EventTagSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum EventTagSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for EventTagSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTagSortOrderEnum::ASCENDING => "ASCENDING",
            EventTagSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTagSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(EventTagSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(EventTagSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTagSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for EventTagSortOrderEnum {
    fn default() -> EventTagSortOrderEnum {
        EventTagSortOrderEnum::ASCENDING
    }
}

// endregion


// region FileScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The scope that defines which results are returned.
pub enum FileScopeEnum {
    

    /// All files in account.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// My files.
    ///
    /// "MINE"
    #[serde(rename="MINE")]
    MINE,
    

    /// Files shared with me.
    ///
    /// "SHARED_WITH_ME"
    #[serde(rename="SHARED_WITH_ME")]
    SHAREDWITHME,
}

impl AsRef<str> for FileScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileScopeEnum::ALL => "ALL",
            FileScopeEnum::MINE => "MINE",
            FileScopeEnum::SHAREDWITHME => "SHARED_WITH_ME",
        }
    }
}

impl std::convert::TryFrom< &str> for FileScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL" => Ok(FileScopeEnum::ALL),
           "MINE" => Ok(FileScopeEnum::MINE),
           "SHARED_WITH_ME" => Ok(FileScopeEnum::SHAREDWITHME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for FileScopeEnum {
    fn default() -> FileScopeEnum {
        FileScopeEnum::MINE
    }
}

// endregion


// region FileSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The field by which to sort the list.
pub enum FileSortFieldEnum {
    

    /// Sort by file ID.
    ///
    /// "ID"
    #[serde(rename="ID")]
    ID,
    

    /// Sort by 'lastmodifiedAt' field.
    ///
    /// "LAST_MODIFIED_TIME"
    #[serde(rename="LAST_MODIFIED_TIME")]
    LASTMODIFIEDTIME,
}

impl AsRef<str> for FileSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileSortFieldEnum::ID => "ID",
            FileSortFieldEnum::LASTMODIFIEDTIME => "LAST_MODIFIED_TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for FileSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(FileSortFieldEnum::ID),
           "LAST_MODIFIED_TIME" => Ok(FileSortFieldEnum::LASTMODIFIEDTIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for FileSortFieldEnum {
    fn default() -> FileSortFieldEnum {
        FileSortFieldEnum::LASTMODIFIEDTIME
    }
}

// endregion


// region FileSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum FileSortOrderEnum {
    

    /// Ascending order.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// Descending order.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for FileSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileSortOrderEnum::ASCENDING => "ASCENDING",
            FileSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for FileSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(FileSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(FileSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for FileSortOrderEnum {
    fn default() -> FileSortOrderEnum {
        FileSortOrderEnum::DESCENDING
    }
}

// endregion


// region FloodlightActivityFloodlightActivityGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only floodlight activities with the specified floodlight activity group type.
pub enum FloodlightActivityFloodlightActivityGroupTypeEnum {
    
    /// "COUNTER"
    #[serde(rename="COUNTER")]
    COUNTER,
    
    /// "SALE"
    #[serde(rename="SALE")]
    SALE,
}

impl AsRef<str> for FloodlightActivityFloodlightActivityGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityFloodlightActivityGroupTypeEnum::COUNTER => "COUNTER",
            FloodlightActivityFloodlightActivityGroupTypeEnum::SALE => "SALE",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityFloodlightActivityGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COUNTER" => Ok(FloodlightActivityFloodlightActivityGroupTypeEnum::COUNTER),
           "SALE" => Ok(FloodlightActivityFloodlightActivityGroupTypeEnum::SALE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityFloodlightActivityGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FloodlightActivitySortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum FloodlightActivitySortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for FloodlightActivitySortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivitySortFieldEnum::ID => "ID",
            FloodlightActivitySortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivitySortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(FloodlightActivitySortFieldEnum::ID),
           "NAME" => Ok(FloodlightActivitySortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivitySortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for FloodlightActivitySortFieldEnum {
    fn default() -> FloodlightActivitySortFieldEnum {
        FloodlightActivitySortFieldEnum::ID
    }
}

// endregion


// region FloodlightActivitySortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum FloodlightActivitySortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for FloodlightActivitySortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivitySortOrderEnum::ASCENDING => "ASCENDING",
            FloodlightActivitySortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivitySortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(FloodlightActivitySortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(FloodlightActivitySortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivitySortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for FloodlightActivitySortOrderEnum {
    fn default() -> FloodlightActivitySortOrderEnum {
        FloodlightActivitySortOrderEnum::ASCENDING
    }
}

// endregion


// region FloodlightActivityGroupSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum FloodlightActivityGroupSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for FloodlightActivityGroupSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityGroupSortFieldEnum::ID => "ID",
            FloodlightActivityGroupSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityGroupSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(FloodlightActivityGroupSortFieldEnum::ID),
           "NAME" => Ok(FloodlightActivityGroupSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityGroupSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for FloodlightActivityGroupSortFieldEnum {
    fn default() -> FloodlightActivityGroupSortFieldEnum {
        FloodlightActivityGroupSortFieldEnum::ID
    }
}

// endregion


// region FloodlightActivityGroupSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum FloodlightActivityGroupSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for FloodlightActivityGroupSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityGroupSortOrderEnum::ASCENDING => "ASCENDING",
            FloodlightActivityGroupSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityGroupSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(FloodlightActivityGroupSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(FloodlightActivityGroupSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityGroupSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for FloodlightActivityGroupSortOrderEnum {
    fn default() -> FloodlightActivityGroupSortOrderEnum {
        FloodlightActivityGroupSortOrderEnum::ASCENDING
    }
}

// endregion


// region FloodlightActivityGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only floodlight activity groups with the specified floodlight activity group type.
pub enum FloodlightActivityGroupTypeEnum {
    
    /// "COUNTER"
    #[serde(rename="COUNTER")]
    COUNTER,
    
    /// "SALE"
    #[serde(rename="SALE")]
    SALE,
}

impl AsRef<str> for FloodlightActivityGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FloodlightActivityGroupTypeEnum::COUNTER => "COUNTER",
            FloodlightActivityGroupTypeEnum::SALE => "SALE",
        }
    }
}

impl std::convert::TryFrom< &str> for FloodlightActivityGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COUNTER" => Ok(FloodlightActivityGroupTypeEnum::COUNTER),
           "SALE" => Ok(FloodlightActivityGroupTypeEnum::SALE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FloodlightActivityGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventoryItemSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum InventoryItemSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for InventoryItemSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventoryItemSortFieldEnum::ID => "ID",
            InventoryItemSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for InventoryItemSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(InventoryItemSortFieldEnum::ID),
           "NAME" => Ok(InventoryItemSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventoryItemSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for InventoryItemSortFieldEnum {
    fn default() -> InventoryItemSortFieldEnum {
        InventoryItemSortFieldEnum::ID
    }
}

// endregion


// region InventoryItemSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum InventoryItemSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for InventoryItemSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventoryItemSortOrderEnum::ASCENDING => "ASCENDING",
            InventoryItemSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for InventoryItemSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(InventoryItemSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(InventoryItemSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventoryItemSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for InventoryItemSortOrderEnum {
    fn default() -> InventoryItemSortOrderEnum {
        InventoryItemSortOrderEnum::ASCENDING
    }
}

// endregion


// region InventoryItemTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only inventory items with this type.
pub enum InventoryItemTypeEnum {
    
    /// "PLANNING_PLACEMENT_TYPE_REGULAR"
    #[serde(rename="PLANNING_PLACEMENT_TYPE_REGULAR")]
    PLANNINGPLACEMENTTYPEREGULAR,
    
    /// "PLANNING_PLACEMENT_TYPE_CREDIT"
    #[serde(rename="PLANNING_PLACEMENT_TYPE_CREDIT")]
    PLANNINGPLACEMENTTYPECREDIT,
}

impl AsRef<str> for InventoryItemTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventoryItemTypeEnum::PLANNINGPLACEMENTTYPEREGULAR => "PLANNING_PLACEMENT_TYPE_REGULAR",
            InventoryItemTypeEnum::PLANNINGPLACEMENTTYPECREDIT => "PLANNING_PLACEMENT_TYPE_CREDIT",
        }
    }
}

impl std::convert::TryFrom< &str> for InventoryItemTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLANNING_PLACEMENT_TYPE_REGULAR" => Ok(InventoryItemTypeEnum::PLANNINGPLACEMENTTYPEREGULAR),
           "PLANNING_PLACEMENT_TYPE_CREDIT" => Ok(InventoryItemTypeEnum::PLANNINGPLACEMENTTYPECREDIT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventoryItemTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MobileAppDirectoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only apps from these directories.
pub enum MobileAppDirectoriesEnum {
    
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    
    /// "APPLE_APP_STORE"
    #[serde(rename="APPLE_APP_STORE")]
    APPLEAPPSTORE,
    
    /// "GOOGLE_PLAY_STORE"
    #[serde(rename="GOOGLE_PLAY_STORE")]
    GOOGLEPLAYSTORE,
}

impl AsRef<str> for MobileAppDirectoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MobileAppDirectoriesEnum::UNKNOWN => "UNKNOWN",
            MobileAppDirectoriesEnum::APPLEAPPSTORE => "APPLE_APP_STORE",
            MobileAppDirectoriesEnum::GOOGLEPLAYSTORE => "GOOGLE_PLAY_STORE",
        }
    }
}

impl std::convert::TryFrom< &str> for MobileAppDirectoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(MobileAppDirectoriesEnum::UNKNOWN),
           "APPLE_APP_STORE" => Ok(MobileAppDirectoriesEnum::APPLEAPPSTORE),
           "GOOGLE_PLAY_STORE" => Ok(MobileAppDirectoriesEnum::GOOGLEPLAYSTORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MobileAppDirectoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderDocumentSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum OrderDocumentSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for OrderDocumentSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderDocumentSortFieldEnum::ID => "ID",
            OrderDocumentSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderDocumentSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(OrderDocumentSortFieldEnum::ID),
           "NAME" => Ok(OrderDocumentSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderDocumentSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for OrderDocumentSortFieldEnum {
    fn default() -> OrderDocumentSortFieldEnum {
        OrderDocumentSortFieldEnum::ID
    }
}

// endregion


// region OrderDocumentSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum OrderDocumentSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for OrderDocumentSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderDocumentSortOrderEnum::ASCENDING => "ASCENDING",
            OrderDocumentSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderDocumentSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(OrderDocumentSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(OrderDocumentSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderDocumentSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for OrderDocumentSortOrderEnum {
    fn default() -> OrderDocumentSortOrderEnum {
        OrderDocumentSortOrderEnum::ASCENDING
    }
}

// endregion


// region OrderSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum OrderSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for OrderSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderSortFieldEnum::ID => "ID",
            OrderSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(OrderSortFieldEnum::ID),
           "NAME" => Ok(OrderSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for OrderSortFieldEnum {
    fn default() -> OrderSortFieldEnum {
        OrderSortFieldEnum::ID
    }
}

// endregion


// region OrderSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum OrderSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for OrderSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderSortOrderEnum::ASCENDING => "ASCENDING",
            OrderSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(OrderSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(OrderSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for OrderSortOrderEnum {
    fn default() -> OrderSortOrderEnum {
        OrderSortOrderEnum::ASCENDING
    }
}

// endregion


// region PlacementGroupPlacementGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only placement groups belonging with this group type. A package is a simple group of placements that acts as a single pricing point for a group of tags. A roadblock is a group of placements that not only acts as a single pricing point but also assumes that all the tags in it will be served at the same time. A roadblock requires one of its assigned placements to be marked as primary for reporting.
pub enum PlacementGroupPlacementGroupTypeEnum {
    
    /// "PLACEMENT_PACKAGE"
    #[serde(rename="PLACEMENT_PACKAGE")]
    PLACEMENTPACKAGE,
    
    /// "PLACEMENT_ROADBLOCK"
    #[serde(rename="PLACEMENT_ROADBLOCK")]
    PLACEMENTROADBLOCK,
}

impl AsRef<str> for PlacementGroupPlacementGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementGroupPlacementGroupTypeEnum::PLACEMENTPACKAGE => "PLACEMENT_PACKAGE",
            PlacementGroupPlacementGroupTypeEnum::PLACEMENTROADBLOCK => "PLACEMENT_ROADBLOCK",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementGroupPlacementGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLACEMENT_PACKAGE" => Ok(PlacementGroupPlacementGroupTypeEnum::PLACEMENTPACKAGE),
           "PLACEMENT_ROADBLOCK" => Ok(PlacementGroupPlacementGroupTypeEnum::PLACEMENTROADBLOCK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementGroupPlacementGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementGroupPricingTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only placement groups with these pricing types.
pub enum PlacementGroupPricingTypesEnum {
    
    /// "PRICING_TYPE_CPM"
    #[serde(rename="PRICING_TYPE_CPM")]
    PRICINGTYPECPM,
    
    /// "PRICING_TYPE_CPC"
    #[serde(rename="PRICING_TYPE_CPC")]
    PRICINGTYPECPC,
    
    /// "PRICING_TYPE_CPA"
    #[serde(rename="PRICING_TYPE_CPA")]
    PRICINGTYPECPA,
    
    /// "PRICING_TYPE_FLAT_RATE_IMPRESSIONS"
    #[serde(rename="PRICING_TYPE_FLAT_RATE_IMPRESSIONS")]
    PRICINGTYPEFLATRATEIMPRESSIONS,
    
    /// "PRICING_TYPE_FLAT_RATE_CLICKS"
    #[serde(rename="PRICING_TYPE_FLAT_RATE_CLICKS")]
    PRICINGTYPEFLATRATECLICKS,
    
    /// "PRICING_TYPE_CPM_ACTIVEVIEW"
    #[serde(rename="PRICING_TYPE_CPM_ACTIVEVIEW")]
    PRICINGTYPECPMACTIVEVIEW,
}

impl AsRef<str> for PlacementGroupPricingTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementGroupPricingTypesEnum::PRICINGTYPECPM => "PRICING_TYPE_CPM",
            PlacementGroupPricingTypesEnum::PRICINGTYPECPC => "PRICING_TYPE_CPC",
            PlacementGroupPricingTypesEnum::PRICINGTYPECPA => "PRICING_TYPE_CPA",
            PlacementGroupPricingTypesEnum::PRICINGTYPEFLATRATEIMPRESSIONS => "PRICING_TYPE_FLAT_RATE_IMPRESSIONS",
            PlacementGroupPricingTypesEnum::PRICINGTYPEFLATRATECLICKS => "PRICING_TYPE_FLAT_RATE_CLICKS",
            PlacementGroupPricingTypesEnum::PRICINGTYPECPMACTIVEVIEW => "PRICING_TYPE_CPM_ACTIVEVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementGroupPricingTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRICING_TYPE_CPM" => Ok(PlacementGroupPricingTypesEnum::PRICINGTYPECPM),
           "PRICING_TYPE_CPC" => Ok(PlacementGroupPricingTypesEnum::PRICINGTYPECPC),
           "PRICING_TYPE_CPA" => Ok(PlacementGroupPricingTypesEnum::PRICINGTYPECPA),
           "PRICING_TYPE_FLAT_RATE_IMPRESSIONS" => Ok(PlacementGroupPricingTypesEnum::PRICINGTYPEFLATRATEIMPRESSIONS),
           "PRICING_TYPE_FLAT_RATE_CLICKS" => Ok(PlacementGroupPricingTypesEnum::PRICINGTYPEFLATRATECLICKS),
           "PRICING_TYPE_CPM_ACTIVEVIEW" => Ok(PlacementGroupPricingTypesEnum::PRICINGTYPECPMACTIVEVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementGroupPricingTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementGroupSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum PlacementGroupSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for PlacementGroupSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementGroupSortFieldEnum::ID => "ID",
            PlacementGroupSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementGroupSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(PlacementGroupSortFieldEnum::ID),
           "NAME" => Ok(PlacementGroupSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementGroupSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for PlacementGroupSortFieldEnum {
    fn default() -> PlacementGroupSortFieldEnum {
        PlacementGroupSortFieldEnum::ID
    }
}

// endregion


// region PlacementGroupSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum PlacementGroupSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for PlacementGroupSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementGroupSortOrderEnum::ASCENDING => "ASCENDING",
            PlacementGroupSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementGroupSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(PlacementGroupSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(PlacementGroupSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementGroupSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for PlacementGroupSortOrderEnum {
    fn default() -> PlacementGroupSortOrderEnum {
        PlacementGroupSortOrderEnum::ASCENDING
    }
}

// endregion


// region PlacementStrategySortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum PlacementStrategySortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for PlacementStrategySortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementStrategySortFieldEnum::ID => "ID",
            PlacementStrategySortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementStrategySortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(PlacementStrategySortFieldEnum::ID),
           "NAME" => Ok(PlacementStrategySortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementStrategySortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for PlacementStrategySortFieldEnum {
    fn default() -> PlacementStrategySortFieldEnum {
        PlacementStrategySortFieldEnum::ID
    }
}

// endregion


// region PlacementStrategySortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum PlacementStrategySortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for PlacementStrategySortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementStrategySortOrderEnum::ASCENDING => "ASCENDING",
            PlacementStrategySortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementStrategySortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(PlacementStrategySortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(PlacementStrategySortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementStrategySortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for PlacementStrategySortOrderEnum {
    fn default() -> PlacementStrategySortOrderEnum {
        PlacementStrategySortOrderEnum::ASCENDING
    }
}

// endregion


// region PlacementTagFormatsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tag formats to generate for these placements. *Note:* PLACEMENT_TAG_STANDARD can only be generated for 1x1 placements.
pub enum PlacementTagFormatsEnum {
    
    /// "PLACEMENT_TAG_STANDARD"
    #[serde(rename="PLACEMENT_TAG_STANDARD")]
    PLACEMENTTAGSTANDARD,
    
    /// "PLACEMENT_TAG_IFRAME_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_IFRAME_JAVASCRIPT")]
    PLACEMENTTAGIFRAMEJAVASCRIPT,
    
    /// "PLACEMENT_TAG_IFRAME_ILAYER"
    #[serde(rename="PLACEMENT_TAG_IFRAME_ILAYER")]
    PLACEMENTTAGIFRAMEILAYER,
    
    /// "PLACEMENT_TAG_INTERNAL_REDIRECT"
    #[serde(rename="PLACEMENT_TAG_INTERNAL_REDIRECT")]
    PLACEMENTTAGINTERNALREDIRECT,
    
    /// "PLACEMENT_TAG_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_JAVASCRIPT")]
    PLACEMENTTAGJAVASCRIPT,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT")]
    PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPT,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT")]
    PLACEMENTTAGINTERSTITIALINTERNALREDIRECT,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT")]
    PLACEMENTTAGINTERSTITIALJAVASCRIPT,
    
    /// "PLACEMENT_TAG_CLICK_COMMANDS"
    #[serde(rename="PLACEMENT_TAG_CLICK_COMMANDS")]
    PLACEMENTTAGCLICKCOMMANDS,
    
    /// "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH"
    #[serde(rename="PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH")]
    PLACEMENTTAGINSTREAMVIDEOPREFETCH,
    
    /// "PLACEMENT_TAG_TRACKING"
    #[serde(rename="PLACEMENT_TAG_TRACKING")]
    PLACEMENTTAGTRACKING,
    
    /// "PLACEMENT_TAG_TRACKING_IFRAME"
    #[serde(rename="PLACEMENT_TAG_TRACKING_IFRAME")]
    PLACEMENTTAGTRACKINGIFRAME,
    
    /// "PLACEMENT_TAG_TRACKING_JAVASCRIPT"
    #[serde(rename="PLACEMENT_TAG_TRACKING_JAVASCRIPT")]
    PLACEMENTTAGTRACKINGJAVASCRIPT,
    
    /// "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3"
    #[serde(rename="PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3")]
    PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST3,
    
    /// "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGIFRAMEJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY"
    #[serde(rename="PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY")]
    PLACEMENTTAGINTERSTITIALJAVASCRIPTLEGACY,
    
    /// "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4"
    #[serde(rename="PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4")]
    PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST4,
    
    /// "PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT"
    #[serde(rename="PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT")]
    PLACEMENTTAGTRACKINGTHIRDPARTYMEASUREMENT,
}

impl AsRef<str> for PlacementTagFormatsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementTagFormatsEnum::PLACEMENTTAGSTANDARD => "PLACEMENT_TAG_STANDARD",
            PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEJAVASCRIPT => "PLACEMENT_TAG_IFRAME_JAVASCRIPT",
            PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEILAYER => "PLACEMENT_TAG_IFRAME_ILAYER",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERNALREDIRECT => "PLACEMENT_TAG_INTERNAL_REDIRECT",
            PlacementTagFormatsEnum::PLACEMENTTAGJAVASCRIPT => "PLACEMENT_TAG_JAVASCRIPT",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPT => "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALINTERNALREDIRECT => "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPT => "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT",
            PlacementTagFormatsEnum::PLACEMENTTAGCLICKCOMMANDS => "PLACEMENT_TAG_CLICK_COMMANDS",
            PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCH => "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH",
            PlacementTagFormatsEnum::PLACEMENTTAGTRACKING => "PLACEMENT_TAG_TRACKING",
            PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGIFRAME => "PLACEMENT_TAG_TRACKING_IFRAME",
            PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGJAVASCRIPT => "PLACEMENT_TAG_TRACKING_JAVASCRIPT",
            PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST3 => "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3",
            PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEJAVASCRIPTLEGACY => "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY",
            PlacementTagFormatsEnum::PLACEMENTTAGJAVASCRIPTLEGACY => "PLACEMENT_TAG_JAVASCRIPT_LEGACY",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPTLEGACY => "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY",
            PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPTLEGACY => "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY",
            PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST4 => "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4",
            PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGTHIRDPARTYMEASUREMENT => "PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementTagFormatsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLACEMENT_TAG_STANDARD" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGSTANDARD),
           "PLACEMENT_TAG_IFRAME_JAVASCRIPT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEJAVASCRIPT),
           "PLACEMENT_TAG_IFRAME_ILAYER" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEILAYER),
           "PLACEMENT_TAG_INTERNAL_REDIRECT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERNALREDIRECT),
           "PLACEMENT_TAG_JAVASCRIPT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGJAVASCRIPT),
           "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPT),
           "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALINTERNALREDIRECT),
           "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPT),
           "PLACEMENT_TAG_CLICK_COMMANDS" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGCLICKCOMMANDS),
           "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCH),
           "PLACEMENT_TAG_TRACKING" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGTRACKING),
           "PLACEMENT_TAG_TRACKING_IFRAME" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGIFRAME),
           "PLACEMENT_TAG_TRACKING_JAVASCRIPT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGJAVASCRIPT),
           "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST3),
           "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGIFRAMEJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_JAVASCRIPT_LEGACY" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALIFRAMEJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINTERSTITIALJAVASCRIPTLEGACY),
           "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGINSTREAMVIDEOPREFETCHVAST4),
           "PLACEMENT_TAG_TRACKING_THIRD_PARTY_MEASUREMENT" => Ok(PlacementTagFormatsEnum::PLACEMENTTAGTRACKINGTHIRDPARTYMEASUREMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementTagFormatsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementCompatibilitiesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only placements that are associated with these compatibilities. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop or on mobile devices for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard.
pub enum PlacementCompatibilitiesEnum {
    
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    
    /// "DISPLAY_INTERSTITIAL"
    #[serde(rename="DISPLAY_INTERSTITIAL")]
    DISPLAYINTERSTITIAL,
    
    /// "APP"
    #[serde(rename="APP")]
    APP,
    
    /// "APP_INTERSTITIAL"
    #[serde(rename="APP_INTERSTITIAL")]
    APPINTERSTITIAL,
    
    /// "IN_STREAM_VIDEO"
    #[serde(rename="IN_STREAM_VIDEO")]
    INSTREAMVIDEO,
    
    /// "IN_STREAM_AUDIO"
    #[serde(rename="IN_STREAM_AUDIO")]
    INSTREAMAUDIO,
}

impl AsRef<str> for PlacementCompatibilitiesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementCompatibilitiesEnum::DISPLAY => "DISPLAY",
            PlacementCompatibilitiesEnum::DISPLAYINTERSTITIAL => "DISPLAY_INTERSTITIAL",
            PlacementCompatibilitiesEnum::APP => "APP",
            PlacementCompatibilitiesEnum::APPINTERSTITIAL => "APP_INTERSTITIAL",
            PlacementCompatibilitiesEnum::INSTREAMVIDEO => "IN_STREAM_VIDEO",
            PlacementCompatibilitiesEnum::INSTREAMAUDIO => "IN_STREAM_AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementCompatibilitiesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISPLAY" => Ok(PlacementCompatibilitiesEnum::DISPLAY),
           "DISPLAY_INTERSTITIAL" => Ok(PlacementCompatibilitiesEnum::DISPLAYINTERSTITIAL),
           "APP" => Ok(PlacementCompatibilitiesEnum::APP),
           "APP_INTERSTITIAL" => Ok(PlacementCompatibilitiesEnum::APPINTERSTITIAL),
           "IN_STREAM_VIDEO" => Ok(PlacementCompatibilitiesEnum::INSTREAMVIDEO),
           "IN_STREAM_AUDIO" => Ok(PlacementCompatibilitiesEnum::INSTREAMAUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementCompatibilitiesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementPaymentSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only placements with this payment source.
pub enum PlacementPaymentSourceEnum {
    
    /// "PLACEMENT_AGENCY_PAID"
    #[serde(rename="PLACEMENT_AGENCY_PAID")]
    PLACEMENTAGENCYPAID,
    
    /// "PLACEMENT_PUBLISHER_PAID"
    #[serde(rename="PLACEMENT_PUBLISHER_PAID")]
    PLACEMENTPUBLISHERPAID,
}

impl AsRef<str> for PlacementPaymentSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementPaymentSourceEnum::PLACEMENTAGENCYPAID => "PLACEMENT_AGENCY_PAID",
            PlacementPaymentSourceEnum::PLACEMENTPUBLISHERPAID => "PLACEMENT_PUBLISHER_PAID",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementPaymentSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLACEMENT_AGENCY_PAID" => Ok(PlacementPaymentSourceEnum::PLACEMENTAGENCYPAID),
           "PLACEMENT_PUBLISHER_PAID" => Ok(PlacementPaymentSourceEnum::PLACEMENTPUBLISHERPAID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementPaymentSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementPricingTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select only placements with these pricing types.
pub enum PlacementPricingTypesEnum {
    
    /// "PRICING_TYPE_CPM"
    #[serde(rename="PRICING_TYPE_CPM")]
    PRICINGTYPECPM,
    
    /// "PRICING_TYPE_CPC"
    #[serde(rename="PRICING_TYPE_CPC")]
    PRICINGTYPECPC,
    
    /// "PRICING_TYPE_CPA"
    #[serde(rename="PRICING_TYPE_CPA")]
    PRICINGTYPECPA,
    
    /// "PRICING_TYPE_FLAT_RATE_IMPRESSIONS"
    #[serde(rename="PRICING_TYPE_FLAT_RATE_IMPRESSIONS")]
    PRICINGTYPEFLATRATEIMPRESSIONS,
    
    /// "PRICING_TYPE_FLAT_RATE_CLICKS"
    #[serde(rename="PRICING_TYPE_FLAT_RATE_CLICKS")]
    PRICINGTYPEFLATRATECLICKS,
    
    /// "PRICING_TYPE_CPM_ACTIVEVIEW"
    #[serde(rename="PRICING_TYPE_CPM_ACTIVEVIEW")]
    PRICINGTYPECPMACTIVEVIEW,
}

impl AsRef<str> for PlacementPricingTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementPricingTypesEnum::PRICINGTYPECPM => "PRICING_TYPE_CPM",
            PlacementPricingTypesEnum::PRICINGTYPECPC => "PRICING_TYPE_CPC",
            PlacementPricingTypesEnum::PRICINGTYPECPA => "PRICING_TYPE_CPA",
            PlacementPricingTypesEnum::PRICINGTYPEFLATRATEIMPRESSIONS => "PRICING_TYPE_FLAT_RATE_IMPRESSIONS",
            PlacementPricingTypesEnum::PRICINGTYPEFLATRATECLICKS => "PRICING_TYPE_FLAT_RATE_CLICKS",
            PlacementPricingTypesEnum::PRICINGTYPECPMACTIVEVIEW => "PRICING_TYPE_CPM_ACTIVEVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementPricingTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRICING_TYPE_CPM" => Ok(PlacementPricingTypesEnum::PRICINGTYPECPM),
           "PRICING_TYPE_CPC" => Ok(PlacementPricingTypesEnum::PRICINGTYPECPC),
           "PRICING_TYPE_CPA" => Ok(PlacementPricingTypesEnum::PRICINGTYPECPA),
           "PRICING_TYPE_FLAT_RATE_IMPRESSIONS" => Ok(PlacementPricingTypesEnum::PRICINGTYPEFLATRATEIMPRESSIONS),
           "PRICING_TYPE_FLAT_RATE_CLICKS" => Ok(PlacementPricingTypesEnum::PRICINGTYPEFLATRATECLICKS),
           "PRICING_TYPE_CPM_ACTIVEVIEW" => Ok(PlacementPricingTypesEnum::PRICINGTYPECPMACTIVEVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementPricingTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum PlacementSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for PlacementSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementSortFieldEnum::ID => "ID",
            PlacementSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(PlacementSortFieldEnum::ID),
           "NAME" => Ok(PlacementSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for PlacementSortFieldEnum {
    fn default() -> PlacementSortFieldEnum {
        PlacementSortFieldEnum::ID
    }
}

// endregion


// region PlacementSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum PlacementSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for PlacementSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementSortOrderEnum::ASCENDING => "ASCENDING",
            PlacementSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(PlacementSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(PlacementSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for PlacementSortOrderEnum {
    fn default() -> PlacementSortOrderEnum {
        PlacementSortOrderEnum::ASCENDING
    }
}

// endregion


// region ProjectSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum ProjectSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for ProjectSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectSortFieldEnum::ID => "ID",
            ProjectSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(ProjectSortFieldEnum::ID),
           "NAME" => Ok(ProjectSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ProjectSortFieldEnum {
    fn default() -> ProjectSortFieldEnum {
        ProjectSortFieldEnum::ID
    }
}

// endregion


// region ProjectSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum ProjectSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for ProjectSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectSortOrderEnum::ASCENDING => "ASCENDING",
            ProjectSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(ProjectSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(ProjectSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ProjectSortOrderEnum {
    fn default() -> ProjectSortOrderEnum {
        ProjectSortOrderEnum::ASCENDING
    }
}

// endregion


// region RemarketingListSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum RemarketingListSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for RemarketingListSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RemarketingListSortFieldEnum::ID => "ID",
            RemarketingListSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for RemarketingListSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(RemarketingListSortFieldEnum::ID),
           "NAME" => Ok(RemarketingListSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RemarketingListSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for RemarketingListSortFieldEnum {
    fn default() -> RemarketingListSortFieldEnum {
        RemarketingListSortFieldEnum::ID
    }
}

// endregion


// region RemarketingListSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum RemarketingListSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for RemarketingListSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RemarketingListSortOrderEnum::ASCENDING => "ASCENDING",
            RemarketingListSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for RemarketingListSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(RemarketingListSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(RemarketingListSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RemarketingListSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for RemarketingListSortOrderEnum {
    fn default() -> RemarketingListSortOrderEnum {
        RemarketingListSortOrderEnum::ASCENDING
    }
}

// endregion


// region ReportSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The field by which to sort the list.
pub enum ReportSortFieldEnum {
    

    /// Sort by report ID.
    ///
    /// "ID"
    #[serde(rename="ID")]
    ID,
    

    /// Sort by 'lastModifiedTime' field.
    ///
    /// "LAST_MODIFIED_TIME"
    #[serde(rename="LAST_MODIFIED_TIME")]
    LASTMODIFIEDTIME,
    

    /// Sort by name of reports.
    ///
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for ReportSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportSortFieldEnum::ID => "ID",
            ReportSortFieldEnum::LASTMODIFIEDTIME => "LAST_MODIFIED_TIME",
            ReportSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(ReportSortFieldEnum::ID),
           "LAST_MODIFIED_TIME" => Ok(ReportSortFieldEnum::LASTMODIFIEDTIME),
           "NAME" => Ok(ReportSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ReportSortFieldEnum {
    fn default() -> ReportSortFieldEnum {
        ReportSortFieldEnum::LASTMODIFIEDTIME
    }
}

// endregion


// region ReportSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum ReportSortOrderEnum {
    

    /// Ascending order.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// Descending order.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for ReportSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportSortOrderEnum::ASCENDING => "ASCENDING",
            ReportSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(ReportSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(ReportSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ReportSortOrderEnum {
    fn default() -> ReportSortOrderEnum {
        ReportSortOrderEnum::DESCENDING
    }
}

// endregion


// region ReportScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The scope that defines which results are returned.
pub enum ReportScopeEnum {
    

    /// All reports in account.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// My reports.
    ///
    /// "MINE"
    #[serde(rename="MINE")]
    MINE,
}

impl AsRef<str> for ReportScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportScopeEnum::ALL => "ALL",
            ReportScopeEnum::MINE => "MINE",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL" => Ok(ReportScopeEnum::ALL),
           "MINE" => Ok(ReportScopeEnum::MINE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ReportScopeEnum {
    fn default() -> ReportScopeEnum {
        ReportScopeEnum::MINE
    }
}

// endregion


// region SiteSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum SiteSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for SiteSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SiteSortFieldEnum::ID => "ID",
            SiteSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for SiteSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(SiteSortFieldEnum::ID),
           "NAME" => Ok(SiteSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SiteSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for SiteSortFieldEnum {
    fn default() -> SiteSortFieldEnum {
        SiteSortFieldEnum::ID
    }
}

// endregion


// region SiteSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum SiteSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for SiteSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SiteSortOrderEnum::ASCENDING => "ASCENDING",
            SiteSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for SiteSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(SiteSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(SiteSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SiteSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for SiteSortOrderEnum {
    fn default() -> SiteSortOrderEnum {
        SiteSortOrderEnum::ASCENDING
    }
}

// endregion


// region SubaccountSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum SubaccountSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for SubaccountSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubaccountSortFieldEnum::ID => "ID",
            SubaccountSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for SubaccountSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(SubaccountSortFieldEnum::ID),
           "NAME" => Ok(SubaccountSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubaccountSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for SubaccountSortFieldEnum {
    fn default() -> SubaccountSortFieldEnum {
        SubaccountSortFieldEnum::ID
    }
}

// endregion


// region SubaccountSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum SubaccountSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for SubaccountSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubaccountSortOrderEnum::ASCENDING => "ASCENDING",
            SubaccountSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for SubaccountSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(SubaccountSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(SubaccountSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubaccountSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for SubaccountSortOrderEnum {
    fn default() -> SubaccountSortOrderEnum {
        SubaccountSortOrderEnum::ASCENDING
    }
}

// endregion


// region TargetableRemarketingListSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum TargetableRemarketingListSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for TargetableRemarketingListSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetableRemarketingListSortFieldEnum::ID => "ID",
            TargetableRemarketingListSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetableRemarketingListSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(TargetableRemarketingListSortFieldEnum::ID),
           "NAME" => Ok(TargetableRemarketingListSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetableRemarketingListSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for TargetableRemarketingListSortFieldEnum {
    fn default() -> TargetableRemarketingListSortFieldEnum {
        TargetableRemarketingListSortFieldEnum::ID
    }
}

// endregion


// region TargetableRemarketingListSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum TargetableRemarketingListSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for TargetableRemarketingListSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetableRemarketingListSortOrderEnum::ASCENDING => "ASCENDING",
            TargetableRemarketingListSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetableRemarketingListSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(TargetableRemarketingListSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(TargetableRemarketingListSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetableRemarketingListSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for TargetableRemarketingListSortOrderEnum {
    fn default() -> TargetableRemarketingListSortOrderEnum {
        TargetableRemarketingListSortOrderEnum::ASCENDING
    }
}

// endregion


// region TargetingTemplateSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum TargetingTemplateSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for TargetingTemplateSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetingTemplateSortFieldEnum::ID => "ID",
            TargetingTemplateSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetingTemplateSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(TargetingTemplateSortFieldEnum::ID),
           "NAME" => Ok(TargetingTemplateSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetingTemplateSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for TargetingTemplateSortFieldEnum {
    fn default() -> TargetingTemplateSortFieldEnum {
        TargetingTemplateSortFieldEnum::ID
    }
}

// endregion


// region TargetingTemplateSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum TargetingTemplateSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for TargetingTemplateSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetingTemplateSortOrderEnum::ASCENDING => "ASCENDING",
            TargetingTemplateSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetingTemplateSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(TargetingTemplateSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(TargetingTemplateSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetingTemplateSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for TargetingTemplateSortOrderEnum {
    fn default() -> TargetingTemplateSortOrderEnum {
        TargetingTemplateSortOrderEnum::ASCENDING
    }
}

// endregion


// region UserRoleSortFieldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Field by which to sort the list.
pub enum UserRoleSortFieldEnum {
    
    /// "ID"
    #[serde(rename="ID")]
    ID,
    
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
}

impl AsRef<str> for UserRoleSortFieldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserRoleSortFieldEnum::ID => "ID",
            UserRoleSortFieldEnum::NAME => "NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for UserRoleSortFieldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ID" => Ok(UserRoleSortFieldEnum::ID),
           "NAME" => Ok(UserRoleSortFieldEnum::NAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserRoleSortFieldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for UserRoleSortFieldEnum {
    fn default() -> UserRoleSortFieldEnum {
        UserRoleSortFieldEnum::ID
    }
}

// endregion


// region UserRoleSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Order of sorted results.
pub enum UserRoleSortOrderEnum {
    
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for UserRoleSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserRoleSortOrderEnum::ASCENDING => "ASCENDING",
            UserRoleSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for UserRoleSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASCENDING" => Ok(UserRoleSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(UserRoleSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserRoleSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for UserRoleSortOrderEnum {
    fn default() -> UserRoleSortOrderEnum {
        UserRoleSortOrderEnum::ASCENDING
    }
}

// endregion


