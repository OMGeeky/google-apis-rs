use super::*;



// region GoogleChromeManagementV1AppDetailTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. App type.
pub enum GoogleChromeManagementV1AppDetailTypeEnum {
    

    /// App type unspecified.
    ///
    /// "APP_ITEM_TYPE_UNSPECIFIED"
    #[serde(rename="APP_ITEM_TYPE_UNSPECIFIED")]
    APPITEMTYPEUNSPECIFIED,
    

    /// Chrome app.
    ///
    /// "CHROME"
    #[serde(rename="CHROME")]
    CHROME,
    

    /// ARC++ app.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Web app.
    ///
    /// "WEB"
    #[serde(rename="WEB")]
    WEB,
}

impl AsRef<str> for GoogleChromeManagementV1AppDetailTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1AppDetailTypeEnum::APPITEMTYPEUNSPECIFIED => "APP_ITEM_TYPE_UNSPECIFIED",
            GoogleChromeManagementV1AppDetailTypeEnum::CHROME => "CHROME",
            GoogleChromeManagementV1AppDetailTypeEnum::ANDROID => "ANDROID",
            GoogleChromeManagementV1AppDetailTypeEnum::WEB => "WEB",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1AppDetailTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_ITEM_TYPE_UNSPECIFIED" => Ok(GoogleChromeManagementV1AppDetailTypeEnum::APPITEMTYPEUNSPECIFIED),
           "CHROME" => Ok(GoogleChromeManagementV1AppDetailTypeEnum::CHROME),
           "ANDROID" => Ok(GoogleChromeManagementV1AppDetailTypeEnum::ANDROID),
           "WEB" => Ok(GoogleChromeManagementV1AppDetailTypeEnum::WEB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1AppDetailTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Battery health.
pub enum GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum {
    

    /// Health unknown.
    ///
    /// "BATTERY_HEALTH_UNSPECIFIED"
    #[serde(rename="BATTERY_HEALTH_UNSPECIFIED")]
    BATTERYHEALTHUNSPECIFIED,
    

    /// Battery is healthy, full charge capacity / design capacity > 80%
    ///
    /// "BATTERY_HEALTH_NORMAL"
    #[serde(rename="BATTERY_HEALTH_NORMAL")]
    BATTERYHEALTHNORMAL,
    

    /// Battery is moderately unhealthy and suggested to be replaced soon, full charge capacity / design capacity 75% - 80%
    ///
    /// "BATTERY_REPLACE_SOON"
    #[serde(rename="BATTERY_REPLACE_SOON")]
    BATTERYREPLACESOON,
    

    /// Battery is unhealthy and suggested to be replaced, full charge capacity / design capacity < 75%
    ///
    /// "BATTERY_REPLACE_NOW"
    #[serde(rename="BATTERY_REPLACE_NOW")]
    BATTERYREPLACENOW,
}

impl AsRef<str> for GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum::BATTERYHEALTHUNSPECIFIED => "BATTERY_HEALTH_UNSPECIFIED",
            GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum::BATTERYHEALTHNORMAL => "BATTERY_HEALTH_NORMAL",
            GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum::BATTERYREPLACESOON => "BATTERY_REPLACE_SOON",
            GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum::BATTERYREPLACENOW => "BATTERY_REPLACE_NOW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BATTERY_HEALTH_UNSPECIFIED" => Ok(GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum::BATTERYHEALTHUNSPECIFIED),
           "BATTERY_HEALTH_NORMAL" => Ok(GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum::BATTERYHEALTHNORMAL),
           "BATTERY_REPLACE_SOON" => Ok(GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum::BATTERYREPLACESOON),
           "BATTERY_REPLACE_NOW" => Ok(GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum::BATTERYREPLACENOW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1BatteryStatusReportBatteryHealthEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The shutdown reason.
pub enum GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum {
    

    /// Shutdown reason is not specified.
    ///
    /// "SHUTDOWN_REASON_UNSPECIFIED"
    #[serde(rename="SHUTDOWN_REASON_UNSPECIFIED")]
    SHUTDOWNREASONUNSPECIFIED,
    

    /// User initiated.
    ///
    /// "USER_REQUEST"
    #[serde(rename="USER_REQUEST")]
    USERREQUEST,
    

    /// System update initiated.
    ///
    /// "SYSTEM_UPDATE"
    #[serde(rename="SYSTEM_UPDATE")]
    SYSTEMUPDATE,
    

    /// Shutdown due to low battery.
    ///
    /// "LOW_BATTERY"
    #[serde(rename="LOW_BATTERY")]
    LOWBATTERY,
    

    /// Shutdown due to other reasons.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum::SHUTDOWNREASONUNSPECIFIED => "SHUTDOWN_REASON_UNSPECIFIED",
            GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum::USERREQUEST => "USER_REQUEST",
            GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum::SYSTEMUPDATE => "SYSTEM_UPDATE",
            GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum::LOWBATTERY => "LOW_BATTERY",
            GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SHUTDOWN_REASON_UNSPECIFIED" => Ok(GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum::SHUTDOWNREASONUNSPECIFIED),
           "USER_REQUEST" => Ok(GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum::USERREQUEST),
           "SYSTEM_UPDATE" => Ok(GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum::SYSTEMUPDATE),
           "LOW_BATTERY" => Ok(GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum::LOWBATTERY),
           "OTHER" => Ok(GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1BootPerformanceReportShutdownReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1BrowserVersionChannelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The release channel of the installed browser.
pub enum GoogleChromeManagementV1BrowserVersionChannelEnum {
    

    /// No release channel specified.
    ///
    /// "RELEASE_CHANNEL_UNSPECIFIED"
    #[serde(rename="RELEASE_CHANNEL_UNSPECIFIED")]
    RELEASECHANNELUNSPECIFIED,
    

    /// Canary release channel.
    ///
    /// "CANARY"
    #[serde(rename="CANARY")]
    CANARY,
    

    /// Dev release channel.
    ///
    /// "DEV"
    #[serde(rename="DEV")]
    DEV,
    

    /// Beta release channel.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// Stable release channel.
    ///
    /// "STABLE"
    #[serde(rename="STABLE")]
    STABLE,
}

impl AsRef<str> for GoogleChromeManagementV1BrowserVersionChannelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1BrowserVersionChannelEnum::RELEASECHANNELUNSPECIFIED => "RELEASE_CHANNEL_UNSPECIFIED",
            GoogleChromeManagementV1BrowserVersionChannelEnum::CANARY => "CANARY",
            GoogleChromeManagementV1BrowserVersionChannelEnum::DEV => "DEV",
            GoogleChromeManagementV1BrowserVersionChannelEnum::BETA => "BETA",
            GoogleChromeManagementV1BrowserVersionChannelEnum::STABLE => "STABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1BrowserVersionChannelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELEASE_CHANNEL_UNSPECIFIED" => Ok(GoogleChromeManagementV1BrowserVersionChannelEnum::RELEASECHANNELUNSPECIFIED),
           "CANARY" => Ok(GoogleChromeManagementV1BrowserVersionChannelEnum::CANARY),
           "DEV" => Ok(GoogleChromeManagementV1BrowserVersionChannelEnum::DEV),
           "BETA" => Ok(GoogleChromeManagementV1BrowserVersionChannelEnum::BETA),
           "STABLE" => Ok(GoogleChromeManagementV1BrowserVersionChannelEnum::STABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1BrowserVersionChannelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1BrowserVersionSystemEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The device operating system.
pub enum GoogleChromeManagementV1BrowserVersionSystemEnum {
    

    /// No operating system specified.
    ///
    /// "DEVICE_SYSTEM_UNSPECIFIED"
    #[serde(rename="DEVICE_SYSTEM_UNSPECIFIED")]
    DEVICESYSTEMUNSPECIFIED,
    

    /// Other operating system.
    ///
    /// "SYSTEM_OTHER"
    #[serde(rename="SYSTEM_OTHER")]
    SYSTEMOTHER,
    

    /// Android operating system.
    ///
    /// "SYSTEM_ANDROID"
    #[serde(rename="SYSTEM_ANDROID")]
    SYSTEMANDROID,
    

    /// Apple iOS operating system.
    ///
    /// "SYSTEM_IOS"
    #[serde(rename="SYSTEM_IOS")]
    SYSTEMIOS,
    

    /// ChromeOS operating system.
    ///
    /// "SYSTEM_CROS"
    #[serde(rename="SYSTEM_CROS")]
    SYSTEMCROS,
    

    /// Microsoft Windows operating system.
    ///
    /// "SYSTEM_WINDOWS"
    #[serde(rename="SYSTEM_WINDOWS")]
    SYSTEMWINDOWS,
    

    /// Apple macOS operating system.
    ///
    /// "SYSTEM_MAC"
    #[serde(rename="SYSTEM_MAC")]
    SYSTEMMAC,
    

    /// Linux operating system.
    ///
    /// "SYSTEM_LINUX"
    #[serde(rename="SYSTEM_LINUX")]
    SYSTEMLINUX,
}

impl AsRef<str> for GoogleChromeManagementV1BrowserVersionSystemEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1BrowserVersionSystemEnum::DEVICESYSTEMUNSPECIFIED => "DEVICE_SYSTEM_UNSPECIFIED",
            GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMOTHER => "SYSTEM_OTHER",
            GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMANDROID => "SYSTEM_ANDROID",
            GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMIOS => "SYSTEM_IOS",
            GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMCROS => "SYSTEM_CROS",
            GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMWINDOWS => "SYSTEM_WINDOWS",
            GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMMAC => "SYSTEM_MAC",
            GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMLINUX => "SYSTEM_LINUX",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1BrowserVersionSystemEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_SYSTEM_UNSPECIFIED" => Ok(GoogleChromeManagementV1BrowserVersionSystemEnum::DEVICESYSTEMUNSPECIFIED),
           "SYSTEM_OTHER" => Ok(GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMOTHER),
           "SYSTEM_ANDROID" => Ok(GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMANDROID),
           "SYSTEM_IOS" => Ok(GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMIOS),
           "SYSTEM_CROS" => Ok(GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMCROS),
           "SYSTEM_WINDOWS" => Ok(GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMWINDOWS),
           "SYSTEM_MAC" => Ok(GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMMAC),
           "SYSTEM_LINUX" => Ok(GoogleChromeManagementV1BrowserVersionSystemEnum::SYSTEMLINUX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1BrowserVersionSystemEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1ChromeAppInfoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Types of an item in the Chrome Web Store
pub enum GoogleChromeManagementV1ChromeAppInfoTypeEnum {
    

    /// Unspecified ItemType.
    ///
    /// "ITEM_TYPE_UNSPECIFIED"
    #[serde(rename="ITEM_TYPE_UNSPECIFIED")]
    ITEMTYPEUNSPECIFIED,
    

    /// Chrome Extensions.
    ///
    /// "EXTENSION"
    #[serde(rename="EXTENSION")]
    EXTENSION,
    

    /// Any other type than extension.
    ///
    /// "OTHERS"
    #[serde(rename="OTHERS")]
    OTHERS,
}

impl AsRef<str> for GoogleChromeManagementV1ChromeAppInfoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1ChromeAppInfoTypeEnum::ITEMTYPEUNSPECIFIED => "ITEM_TYPE_UNSPECIFIED",
            GoogleChromeManagementV1ChromeAppInfoTypeEnum::EXTENSION => "EXTENSION",
            GoogleChromeManagementV1ChromeAppInfoTypeEnum::OTHERS => "OTHERS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1ChromeAppInfoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ITEM_TYPE_UNSPECIFIED" => Ok(GoogleChromeManagementV1ChromeAppInfoTypeEnum::ITEMTYPEUNSPECIFIED),
           "EXTENSION" => Ok(GoogleChromeManagementV1ChromeAppInfoTypeEnum::EXTENSION),
           "OTHERS" => Ok(GoogleChromeManagementV1ChromeAppInfoTypeEnum::OTHERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1ChromeAppInfoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1CpuInfoArchitectureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Architecture type for the CPU. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceCpuInfo](https://chromeenterprise.google/policies/#ReportDeviceCpuInfo) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
pub enum GoogleChromeManagementV1CpuInfoArchitectureEnum {
    

    /// Architecture unknown.
    ///
    /// "ARCHITECTURE_UNSPECIFIED"
    #[serde(rename="ARCHITECTURE_UNSPECIFIED")]
    ARCHITECTUREUNSPECIFIED,
    

    /// x64 architecture
    ///
    /// "X64"
    #[serde(rename="X64")]
    X64,
}

impl AsRef<str> for GoogleChromeManagementV1CpuInfoArchitectureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1CpuInfoArchitectureEnum::ARCHITECTUREUNSPECIFIED => "ARCHITECTURE_UNSPECIFIED",
            GoogleChromeManagementV1CpuInfoArchitectureEnum::X64 => "X64",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1CpuInfoArchitectureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ARCHITECTURE_UNSPECIFIED" => Ok(GoogleChromeManagementV1CpuInfoArchitectureEnum::ARCHITECTUREUNSPECIFIED),
           "X64" => Ok(GoogleChromeManagementV1CpuInfoArchitectureEnum::X64),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1CpuInfoArchitectureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Device activity state.
pub enum GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum {
    

    /// Device activity state is unspecified.
    ///
    /// "DEVICE_ACTIVITY_STATE_UNSPECIFIED"
    #[serde(rename="DEVICE_ACTIVITY_STATE_UNSPECIFIED")]
    DEVICEACTIVITYSTATEUNSPECIFIED,
    

    /// Device is currently being used.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Device is currently idle.
    ///
    /// "IDLE"
    #[serde(rename="IDLE")]
    IDLE,
    

    /// Device is currently locked.
    ///
    /// "LOCKED"
    #[serde(rename="LOCKED")]
    LOCKED,
}

impl AsRef<str> for GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum::DEVICEACTIVITYSTATEUNSPECIFIED => "DEVICE_ACTIVITY_STATE_UNSPECIFIED",
            GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum::ACTIVE => "ACTIVE",
            GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum::IDLE => "IDLE",
            GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum::LOCKED => "LOCKED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_ACTIVITY_STATE_UNSPECIFIED" => Ok(GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum::DEVICEACTIVITYSTATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum::ACTIVE),
           "IDLE" => Ok(GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum::IDLE),
           "LOCKED" => Ok(GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum::LOCKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1DeviceActivityReportDeviceActivityStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Enum value of month corresponding to the auto update expiration date in UTC time zone. If the device is already expired, this field is empty.
pub enum GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum {
    

    /// The unspecified month.
    ///
    /// "MONTH_UNSPECIFIED"
    #[serde(rename="MONTH_UNSPECIFIED")]
    MONTHUNSPECIFIED,
    

    /// The month of January.
    ///
    /// "JANUARY"
    #[serde(rename="JANUARY")]
    JANUARY,
    

    /// The month of February.
    ///
    /// "FEBRUARY"
    #[serde(rename="FEBRUARY")]
    FEBRUARY,
    

    /// The month of March.
    ///
    /// "MARCH"
    #[serde(rename="MARCH")]
    MARCH,
    

    /// The month of April.
    ///
    /// "APRIL"
    #[serde(rename="APRIL")]
    APRIL,
    

    /// The month of May.
    ///
    /// "MAY"
    #[serde(rename="MAY")]
    MAY,
    

    /// The month of June.
    ///
    /// "JUNE"
    #[serde(rename="JUNE")]
    JUNE,
    

    /// The month of July.
    ///
    /// "JULY"
    #[serde(rename="JULY")]
    JULY,
    

    /// The month of August.
    ///
    /// "AUGUST"
    #[serde(rename="AUGUST")]
    AUGUST,
    

    /// The month of September.
    ///
    /// "SEPTEMBER"
    #[serde(rename="SEPTEMBER")]
    SEPTEMBER,
    

    /// The month of October.
    ///
    /// "OCTOBER"
    #[serde(rename="OCTOBER")]
    OCTOBER,
    

    /// The month of November.
    ///
    /// "NOVEMBER"
    #[serde(rename="NOVEMBER")]
    NOVEMBER,
    

    /// The month of December.
    ///
    /// "DECEMBER"
    #[serde(rename="DECEMBER")]
    DECEMBER,
}

impl AsRef<str> for GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::MONTHUNSPECIFIED => "MONTH_UNSPECIFIED",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::JANUARY => "JANUARY",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::FEBRUARY => "FEBRUARY",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::MARCH => "MARCH",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::APRIL => "APRIL",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::MAY => "MAY",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::JUNE => "JUNE",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::JULY => "JULY",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::AUGUST => "AUGUST",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::SEPTEMBER => "SEPTEMBER",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::OCTOBER => "OCTOBER",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::NOVEMBER => "NOVEMBER",
            GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::DECEMBER => "DECEMBER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MONTH_UNSPECIFIED" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::MONTHUNSPECIFIED),
           "JANUARY" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::JANUARY),
           "FEBRUARY" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::FEBRUARY),
           "MARCH" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::MARCH),
           "APRIL" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::APRIL),
           "MAY" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::MAY),
           "JUNE" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::JUNE),
           "JULY" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::JULY),
           "AUGUST" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::AUGUST),
           "SEPTEMBER" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::SEPTEMBER),
           "OCTOBER" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::OCTOBER),
           "NOVEMBER" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::NOVEMBER),
           "DECEMBER" => Ok(GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum::DECEMBER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1DeviceAueCountReportAueMonthEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1HeartbeatStatusReportStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State the device changed to
pub enum GoogleChromeManagementV1HeartbeatStatusReportStateEnum {
    

    /// State not specified
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Device is not eligible for heartbeat monitoring
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Device is online
    ///
    /// "ONLINE"
    #[serde(rename="ONLINE")]
    ONLINE,
    

    /// Device is offline
    ///
    /// "OFFLINE"
    #[serde(rename="OFFLINE")]
    OFFLINE,
}

impl AsRef<str> for GoogleChromeManagementV1HeartbeatStatusReportStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1HeartbeatStatusReportStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleChromeManagementV1HeartbeatStatusReportStateEnum::UNKNOWN => "UNKNOWN",
            GoogleChromeManagementV1HeartbeatStatusReportStateEnum::ONLINE => "ONLINE",
            GoogleChromeManagementV1HeartbeatStatusReportStateEnum::OFFLINE => "OFFLINE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1HeartbeatStatusReportStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleChromeManagementV1HeartbeatStatusReportStateEnum::STATEUNSPECIFIED),
           "UNKNOWN" => Ok(GoogleChromeManagementV1HeartbeatStatusReportStateEnum::UNKNOWN),
           "ONLINE" => Ok(GoogleChromeManagementV1HeartbeatStatusReportStateEnum::ONLINE),
           "OFFLINE" => Ok(GoogleChromeManagementV1HeartbeatStatusReportStateEnum::OFFLINE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1HeartbeatStatusReportStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. HTTPS latency routine problem if a problem occurred.
pub enum GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum {
    

    /// HTTPS latency problem not specified.
    ///
    /// "HTTPS_LATENCY_PROBLEM_UNSPECIFIED"
    #[serde(rename="HTTPS_LATENCY_PROBLEM_UNSPECIFIED")]
    HTTPSLATENCYPROBLEMUNSPECIFIED,
    

    /// One or more DNS resolutions resulted in a failure.
    ///
    /// "FAILED_DNS_RESOLUTIONS"
    #[serde(rename="FAILED_DNS_RESOLUTIONS")]
    FAILEDDNSRESOLUTIONS,
    

    /// One or more HTTPS requests resulted in a failure.
    ///
    /// "FAILED_HTTPS_REQUESTS"
    #[serde(rename="FAILED_HTTPS_REQUESTS")]
    FAILEDHTTPSREQUESTS,
    

    /// Average HTTPS request latency time between 500ms and 1000ms is high.
    ///
    /// "HIGH_LATENCY"
    #[serde(rename="HIGH_LATENCY")]
    HIGHLATENCY,
    

    /// Average HTTPS request latency time greater than 1000ms is very high.
    ///
    /// "VERY_HIGH_LATENCY"
    #[serde(rename="VERY_HIGH_LATENCY")]
    VERYHIGHLATENCY,
}

impl AsRef<str> for GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum::HTTPSLATENCYPROBLEMUNSPECIFIED => "HTTPS_LATENCY_PROBLEM_UNSPECIFIED",
            GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum::FAILEDDNSRESOLUTIONS => "FAILED_DNS_RESOLUTIONS",
            GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum::FAILEDHTTPSREQUESTS => "FAILED_HTTPS_REQUESTS",
            GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum::HIGHLATENCY => "HIGH_LATENCY",
            GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum::VERYHIGHLATENCY => "VERY_HIGH_LATENCY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HTTPS_LATENCY_PROBLEM_UNSPECIFIED" => Ok(GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum::HTTPSLATENCYPROBLEMUNSPECIFIED),
           "FAILED_DNS_RESOLUTIONS" => Ok(GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum::FAILEDDNSRESOLUTIONS),
           "FAILED_HTTPS_REQUESTS" => Ok(GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum::FAILEDHTTPSREQUESTS),
           "HIGH_LATENCY" => Ok(GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum::HIGHLATENCY),
           "VERY_HIGH_LATENCY" => Ok(GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum::VERYHIGHLATENCY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1HttpsLatencyRoutineDataProblemEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1InstalledAppAppInstallTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. How the app was installed.
pub enum GoogleChromeManagementV1InstalledAppAppInstallTypeEnum {
    

    /// Application install type not specified.
    ///
    /// "APP_INSTALL_TYPE_UNSPECIFIED"
    #[serde(rename="APP_INSTALL_TYPE_UNSPECIFIED")]
    APPINSTALLTYPEUNSPECIFIED,
    

    /// Multiple app install types.
    ///
    /// "MULTIPLE"
    #[serde(rename="MULTIPLE")]
    MULTIPLE,
    

    /// Normal app install type.
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// Administrator app install type.
    ///
    /// "ADMIN"
    #[serde(rename="ADMIN")]
    ADMIN,
    

    /// Development app install type.
    ///
    /// "DEVELOPMENT"
    #[serde(rename="DEVELOPMENT")]
    DEVELOPMENT,
    

    /// Sideloaded app install type.
    ///
    /// "SIDELOAD"
    #[serde(rename="SIDELOAD")]
    SIDELOAD,
    

    /// Other app install type.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for GoogleChromeManagementV1InstalledAppAppInstallTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::APPINSTALLTYPEUNSPECIFIED => "APP_INSTALL_TYPE_UNSPECIFIED",
            GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::MULTIPLE => "MULTIPLE",
            GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::NORMAL => "NORMAL",
            GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::ADMIN => "ADMIN",
            GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::DEVELOPMENT => "DEVELOPMENT",
            GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::SIDELOAD => "SIDELOAD",
            GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1InstalledAppAppInstallTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_INSTALL_TYPE_UNSPECIFIED" => Ok(GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::APPINSTALLTYPEUNSPECIFIED),
           "MULTIPLE" => Ok(GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::MULTIPLE),
           "NORMAL" => Ok(GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::NORMAL),
           "ADMIN" => Ok(GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::ADMIN),
           "DEVELOPMENT" => Ok(GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::DEVELOPMENT),
           "SIDELOAD" => Ok(GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::SIDELOAD),
           "OTHER" => Ok(GoogleChromeManagementV1InstalledAppAppInstallTypeEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1InstalledAppAppInstallTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1InstalledAppAppSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Source of the installed app.
pub enum GoogleChromeManagementV1InstalledAppAppSourceEnum {
    

    /// Application source not specified.
    ///
    /// "APP_SOURCE_UNSPECIFIED"
    #[serde(rename="APP_SOURCE_UNSPECIFIED")]
    APPSOURCEUNSPECIFIED,
    

    /// Generally for extensions and Chrome apps.
    ///
    /// "CHROME_WEBSTORE"
    #[serde(rename="CHROME_WEBSTORE")]
    CHROMEWEBSTORE,
    

    /// Play Store app.
    ///
    /// "PLAY_STORE"
    #[serde(rename="PLAY_STORE")]
    PLAYSTORE,
}

impl AsRef<str> for GoogleChromeManagementV1InstalledAppAppSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1InstalledAppAppSourceEnum::APPSOURCEUNSPECIFIED => "APP_SOURCE_UNSPECIFIED",
            GoogleChromeManagementV1InstalledAppAppSourceEnum::CHROMEWEBSTORE => "CHROME_WEBSTORE",
            GoogleChromeManagementV1InstalledAppAppSourceEnum::PLAYSTORE => "PLAY_STORE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1InstalledAppAppSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_SOURCE_UNSPECIFIED" => Ok(GoogleChromeManagementV1InstalledAppAppSourceEnum::APPSOURCEUNSPECIFIED),
           "CHROME_WEBSTORE" => Ok(GoogleChromeManagementV1InstalledAppAppSourceEnum::CHROMEWEBSTORE),
           "PLAY_STORE" => Ok(GoogleChromeManagementV1InstalledAppAppSourceEnum::PLAYSTORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1InstalledAppAppSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1InstalledAppAppTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Type of the app.
pub enum GoogleChromeManagementV1InstalledAppAppTypeEnum {
    

    /// App type not specified.
    ///
    /// "APP_TYPE_UNSPECIFIED"
    #[serde(rename="APP_TYPE_UNSPECIFIED")]
    APPTYPEUNSPECIFIED,
    

    /// Chrome extension.
    ///
    /// "EXTENSION"
    #[serde(rename="EXTENSION")]
    EXTENSION,
    

    /// Chrome app.
    ///
    /// "APP"
    #[serde(rename="APP")]
    APP,
    

    /// Chrome theme.
    ///
    /// "THEME"
    #[serde(rename="THEME")]
    THEME,
    

    /// Chrome hosted app.
    ///
    /// "HOSTED_APP"
    #[serde(rename="HOSTED_APP")]
    HOSTEDAPP,
    

    /// ARC++ app.
    ///
    /// "ANDROID_APP"
    #[serde(rename="ANDROID_APP")]
    ANDROIDAPP,
}

impl AsRef<str> for GoogleChromeManagementV1InstalledAppAppTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1InstalledAppAppTypeEnum::APPTYPEUNSPECIFIED => "APP_TYPE_UNSPECIFIED",
            GoogleChromeManagementV1InstalledAppAppTypeEnum::EXTENSION => "EXTENSION",
            GoogleChromeManagementV1InstalledAppAppTypeEnum::APP => "APP",
            GoogleChromeManagementV1InstalledAppAppTypeEnum::THEME => "THEME",
            GoogleChromeManagementV1InstalledAppAppTypeEnum::HOSTEDAPP => "HOSTED_APP",
            GoogleChromeManagementV1InstalledAppAppTypeEnum::ANDROIDAPP => "ANDROID_APP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1InstalledAppAppTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_TYPE_UNSPECIFIED" => Ok(GoogleChromeManagementV1InstalledAppAppTypeEnum::APPTYPEUNSPECIFIED),
           "EXTENSION" => Ok(GoogleChromeManagementV1InstalledAppAppTypeEnum::EXTENSION),
           "APP" => Ok(GoogleChromeManagementV1InstalledAppAppTypeEnum::APP),
           "THEME" => Ok(GoogleChromeManagementV1InstalledAppAppTypeEnum::THEME),
           "HOSTED_APP" => Ok(GoogleChromeManagementV1InstalledAppAppTypeEnum::HOSTEDAPP),
           "ANDROID_APP" => Ok(GoogleChromeManagementV1InstalledAppAppTypeEnum::ANDROIDAPP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1InstalledAppAppTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1NetworkDeviceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Network device type.
pub enum GoogleChromeManagementV1NetworkDeviceTypeEnum {
    

    /// Network device type not specified.
    ///
    /// "NETWORK_DEVICE_TYPE_UNSPECIFIED"
    #[serde(rename="NETWORK_DEVICE_TYPE_UNSPECIFIED")]
    NETWORKDEVICETYPEUNSPECIFIED,
    

    /// Cellular device.
    ///
    /// "CELLULAR_DEVICE"
    #[serde(rename="CELLULAR_DEVICE")]
    CELLULARDEVICE,
    

    /// Ethernet device.
    ///
    /// "ETHERNET_DEVICE"
    #[serde(rename="ETHERNET_DEVICE")]
    ETHERNETDEVICE,
    

    /// Wifi device.
    ///
    /// "WIFI_DEVICE"
    #[serde(rename="WIFI_DEVICE")]
    WIFIDEVICE,
}

impl AsRef<str> for GoogleChromeManagementV1NetworkDeviceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1NetworkDeviceTypeEnum::NETWORKDEVICETYPEUNSPECIFIED => "NETWORK_DEVICE_TYPE_UNSPECIFIED",
            GoogleChromeManagementV1NetworkDeviceTypeEnum::CELLULARDEVICE => "CELLULAR_DEVICE",
            GoogleChromeManagementV1NetworkDeviceTypeEnum::ETHERNETDEVICE => "ETHERNET_DEVICE",
            GoogleChromeManagementV1NetworkDeviceTypeEnum::WIFIDEVICE => "WIFI_DEVICE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1NetworkDeviceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NETWORK_DEVICE_TYPE_UNSPECIFIED" => Ok(GoogleChromeManagementV1NetworkDeviceTypeEnum::NETWORKDEVICETYPEUNSPECIFIED),
           "CELLULAR_DEVICE" => Ok(GoogleChromeManagementV1NetworkDeviceTypeEnum::CELLULARDEVICE),
           "ETHERNET_DEVICE" => Ok(GoogleChromeManagementV1NetworkDeviceTypeEnum::ETHERNETDEVICE),
           "WIFI_DEVICE" => Ok(GoogleChromeManagementV1NetworkDeviceTypeEnum::WIFIDEVICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1NetworkDeviceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current connection state of the network.
pub enum GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum {
    

    /// Network connection state unspecified.
    ///
    /// "NETWORK_CONNECTION_STATE_UNSPECIFIED"
    #[serde(rename="NETWORK_CONNECTION_STATE_UNSPECIFIED")]
    NETWORKCONNECTIONSTATEUNSPECIFIED,
    

    /// The network is connected and internet connectivity is available.
    ///
    /// "ONLINE"
    #[serde(rename="ONLINE")]
    ONLINE,
    

    /// The network is connected and not in a detected portal state, but internet connectivity may not be available.
    ///
    /// "CONNECTED"
    #[serde(rename="CONNECTED")]
    CONNECTED,
    

    /// The network is connected but a portal state was detected. Internet connectivity may be limited.
    ///
    /// "PORTAL"
    #[serde(rename="PORTAL")]
    PORTAL,
    

    /// The network is in the process of connecting.
    ///
    /// "CONNECTING"
    #[serde(rename="CONNECTING")]
    CONNECTING,
    

    /// The network is not connected.
    ///
    /// "NOT_CONNECTED"
    #[serde(rename="NOT_CONNECTED")]
    NOTCONNECTED,
}

impl AsRef<str> for GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::NETWORKCONNECTIONSTATEUNSPECIFIED => "NETWORK_CONNECTION_STATE_UNSPECIFIED",
            GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::ONLINE => "ONLINE",
            GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::CONNECTED => "CONNECTED",
            GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::PORTAL => "PORTAL",
            GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::CONNECTING => "CONNECTING",
            GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::NOTCONNECTED => "NOT_CONNECTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NETWORK_CONNECTION_STATE_UNSPECIFIED" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::NETWORKCONNECTIONSTATEUNSPECIFIED),
           "ONLINE" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::ONLINE),
           "CONNECTED" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::CONNECTED),
           "PORTAL" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::PORTAL),
           "CONNECTING" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::CONNECTING),
           "NOT_CONNECTED" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum::NOTCONNECTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1NetworkStatusReportConnectionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Network connection type.
pub enum GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum {
    

    /// Network connection type unspecified
    ///
    /// "NETWORK_TYPE_UNSPECIFIED"
    #[serde(rename="NETWORK_TYPE_UNSPECIFIED")]
    NETWORKTYPEUNSPECIFIED,
    

    /// Cellular network connection.
    ///
    /// "CELLULAR"
    #[serde(rename="CELLULAR")]
    CELLULAR,
    

    /// Ethernet network connection.
    ///
    /// "ETHERNET"
    #[serde(rename="ETHERNET")]
    ETHERNET,
    

    /// Tether network connection.
    ///
    /// "TETHER"
    #[serde(rename="TETHER")]
    TETHER,
    

    /// VPN network connection.
    ///
    /// "VPN"
    #[serde(rename="VPN")]
    VPN,
    

    /// Wifi network connection.
    ///
    /// "WIFI"
    #[serde(rename="WIFI")]
    WIFI,
}

impl AsRef<str> for GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::NETWORKTYPEUNSPECIFIED => "NETWORK_TYPE_UNSPECIFIED",
            GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::CELLULAR => "CELLULAR",
            GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::ETHERNET => "ETHERNET",
            GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::TETHER => "TETHER",
            GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::VPN => "VPN",
            GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::WIFI => "WIFI",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NETWORK_TYPE_UNSPECIFIED" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::NETWORKTYPEUNSPECIFIED),
           "CELLULAR" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::CELLULAR),
           "ETHERNET" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::ETHERNET),
           "TETHER" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::TETHER),
           "VPN" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::VPN),
           "WIFI" => Ok(GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum::WIFI),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1NetworkStatusReportConnectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the os update.
pub enum GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum {
    

    /// State unspecified.
    ///
    /// "UPDATE_STATE_UNSPECIFIED"
    #[serde(rename="UPDATE_STATE_UNSPECIFIED")]
    UPDATESTATEUNSPECIFIED,
    

    /// OS has not started downloading.
    ///
    /// "OS_IMAGE_DOWNLOAD_NOT_STARTED"
    #[serde(rename="OS_IMAGE_DOWNLOAD_NOT_STARTED")]
    OSIMAGEDOWNLOADNOTSTARTED,
    

    /// OS has started download on device.
    ///
    /// "OS_IMAGE_DOWNLOAD_IN_PROGRESS"
    #[serde(rename="OS_IMAGE_DOWNLOAD_IN_PROGRESS")]
    OSIMAGEDOWNLOADINPROGRESS,
    

    /// Device needs reboot to finish upload.
    ///
    /// "OS_UPDATE_NEED_REBOOT"
    #[serde(rename="OS_UPDATE_NEED_REBOOT")]
    OSUPDATENEEDREBOOT,
}

impl AsRef<str> for GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum::UPDATESTATEUNSPECIFIED => "UPDATE_STATE_UNSPECIFIED",
            GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum::OSIMAGEDOWNLOADNOTSTARTED => "OS_IMAGE_DOWNLOAD_NOT_STARTED",
            GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum::OSIMAGEDOWNLOADINPROGRESS => "OS_IMAGE_DOWNLOAD_IN_PROGRESS",
            GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum::OSUPDATENEEDREBOOT => "OS_UPDATE_NEED_REBOOT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UPDATE_STATE_UNSPECIFIED" => Ok(GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum::UPDATESTATEUNSPECIFIED),
           "OS_IMAGE_DOWNLOAD_NOT_STARTED" => Ok(GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum::OSIMAGEDOWNLOADNOTSTARTED),
           "OS_IMAGE_DOWNLOAD_IN_PROGRESS" => Ok(GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum::OSIMAGEDOWNLOADINPROGRESS),
           "OS_UPDATE_NEED_REBOOT" => Ok(GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum::OSUPDATENEEDREBOOT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1OsUpdateStatusUpdateStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1PrintJobColorModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Color mode.
pub enum GoogleChromeManagementV1PrintJobColorModeEnum {
    

    /// Unspecified.
    ///
    /// "COLOR_MODE_UNSPECIFIED"
    #[serde(rename="COLOR_MODE_UNSPECIFIED")]
    COLORMODEUNSPECIFIED,
    

    /// Black and white.
    ///
    /// "BLACK_AND_WHITE"
    #[serde(rename="BLACK_AND_WHITE")]
    BLACKANDWHITE,
    

    /// Color.
    ///
    /// "COLOR"
    #[serde(rename="COLOR")]
    COLOR,
}

impl AsRef<str> for GoogleChromeManagementV1PrintJobColorModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1PrintJobColorModeEnum::COLORMODEUNSPECIFIED => "COLOR_MODE_UNSPECIFIED",
            GoogleChromeManagementV1PrintJobColorModeEnum::BLACKANDWHITE => "BLACK_AND_WHITE",
            GoogleChromeManagementV1PrintJobColorModeEnum::COLOR => "COLOR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1PrintJobColorModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COLOR_MODE_UNSPECIFIED" => Ok(GoogleChromeManagementV1PrintJobColorModeEnum::COLORMODEUNSPECIFIED),
           "BLACK_AND_WHITE" => Ok(GoogleChromeManagementV1PrintJobColorModeEnum::BLACKANDWHITE),
           "COLOR" => Ok(GoogleChromeManagementV1PrintJobColorModeEnum::COLOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1PrintJobColorModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1PrintJobDuplexModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Duplex mode.
pub enum GoogleChromeManagementV1PrintJobDuplexModeEnum {
    

    /// Unspecified.
    ///
    /// "DUPLEX_MODE_UNSPECIFIED"
    #[serde(rename="DUPLEX_MODE_UNSPECIFIED")]
    DUPLEXMODEUNSPECIFIED,
    

    /// One-sided.
    ///
    /// "ONE_SIDED"
    #[serde(rename="ONE_SIDED")]
    ONESIDED,
    

    /// Two-sided flipping over long edge.
    ///
    /// "TWO_SIDED_LONG_EDGE"
    #[serde(rename="TWO_SIDED_LONG_EDGE")]
    TWOSIDEDLONGEDGE,
    

    /// Two-sided flipping over short edge.
    ///
    /// "TWO_SIDED_SHORT_EDGE"
    #[serde(rename="TWO_SIDED_SHORT_EDGE")]
    TWOSIDEDSHORTEDGE,
}

impl AsRef<str> for GoogleChromeManagementV1PrintJobDuplexModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1PrintJobDuplexModeEnum::DUPLEXMODEUNSPECIFIED => "DUPLEX_MODE_UNSPECIFIED",
            GoogleChromeManagementV1PrintJobDuplexModeEnum::ONESIDED => "ONE_SIDED",
            GoogleChromeManagementV1PrintJobDuplexModeEnum::TWOSIDEDLONGEDGE => "TWO_SIDED_LONG_EDGE",
            GoogleChromeManagementV1PrintJobDuplexModeEnum::TWOSIDEDSHORTEDGE => "TWO_SIDED_SHORT_EDGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1PrintJobDuplexModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DUPLEX_MODE_UNSPECIFIED" => Ok(GoogleChromeManagementV1PrintJobDuplexModeEnum::DUPLEXMODEUNSPECIFIED),
           "ONE_SIDED" => Ok(GoogleChromeManagementV1PrintJobDuplexModeEnum::ONESIDED),
           "TWO_SIDED_LONG_EDGE" => Ok(GoogleChromeManagementV1PrintJobDuplexModeEnum::TWOSIDEDLONGEDGE),
           "TWO_SIDED_SHORT_EDGE" => Ok(GoogleChromeManagementV1PrintJobDuplexModeEnum::TWOSIDEDSHORTEDGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1PrintJobDuplexModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1PrintJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The final state of the job.
pub enum GoogleChromeManagementV1PrintJobStateEnum {
    

    /// Print job is in an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The document was successfully printed.
    ///
    /// "PRINTED"
    #[serde(rename="PRINTED")]
    PRINTED,
    

    /// Print job was cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// Print job failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for GoogleChromeManagementV1PrintJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1PrintJobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleChromeManagementV1PrintJobStateEnum::PRINTED => "PRINTED",
            GoogleChromeManagementV1PrintJobStateEnum::CANCELLED => "CANCELLED",
            GoogleChromeManagementV1PrintJobStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1PrintJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleChromeManagementV1PrintJobStateEnum::STATEUNSPECIFIED),
           "PRINTED" => Ok(GoogleChromeManagementV1PrintJobStateEnum::PRINTED),
           "CANCELLED" => Ok(GoogleChromeManagementV1PrintJobStateEnum::CANCELLED),
           "FAILED" => Ok(GoogleChromeManagementV1PrintJobStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1PrintJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1TelemetryEventEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The event type of the current event.
pub enum GoogleChromeManagementV1TelemetryEventEventTypeEnum {
    

    /// Event type unknown.
    ///
    /// "EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="EVENT_TYPE_UNSPECIFIED")]
    EVENTTYPEUNSPECIFIED,
    

    /// Triggered when a audio devices run out of buffer data for more than 5 seconds.
    ///
    /// "AUDIO_SEVERE_UNDERRUN"
    #[serde(rename="AUDIO_SEVERE_UNDERRUN")]
    AUDIOSEVEREUNDERRUN,
    

    /// Triggered immediately on any changes to a network connection.
    ///
    /// "NETWORK_STATE_CHANGE"
    #[serde(rename="NETWORK_STATE_CHANGE")]
    NETWORKSTATECHANGE,
    

    /// Triggered when USB devices are added.
    ///
    /// "USB_ADDED"
    #[serde(rename="USB_ADDED")]
    USBADDED,
    

    /// Triggered when USB devices are removed.
    ///
    /// "USB_REMOVED"
    #[serde(rename="USB_REMOVED")]
    USBREMOVED,
    

    /// Triggered when a new HTTPS latency problem was detected or the device has recovered form an existing HTTPS latency problem.
    ///
    /// "NETWORK_HTTPS_LATENCY_CHANGE"
    #[serde(rename="NETWORK_HTTPS_LATENCY_CHANGE")]
    NETWORKHTTPSLATENCYCHANGE,
    

    /// Triggered when connected WiFi network signal strength drops below -70dBm.
    ///
    /// "WIFI_SIGNAL_STRENGTH_LOW"
    #[serde(rename="WIFI_SIGNAL_STRENGTH_LOW")]
    WIFISIGNALSTRENGTHLOW,
    

    /// Triggered when connected WiFi network signal strength is recovered from a signal drop.
    ///
    /// "WIFI_SIGNAL_STRENGTH_RECOVERED"
    #[serde(rename="WIFI_SIGNAL_STRENGTH_RECOVERED")]
    WIFISIGNALSTRENGTHRECOVERED,
    

    /// Triggered on changes to VPN connections.
    ///
    /// "VPN_CONNECTION_STATE_CHANGE"
    #[serde(rename="VPN_CONNECTION_STATE_CHANGE")]
    VPNCONNECTIONSTATECHANGE,
}

impl AsRef<str> for GoogleChromeManagementV1TelemetryEventEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1TelemetryEventEventTypeEnum::EVENTTYPEUNSPECIFIED => "EVENT_TYPE_UNSPECIFIED",
            GoogleChromeManagementV1TelemetryEventEventTypeEnum::AUDIOSEVEREUNDERRUN => "AUDIO_SEVERE_UNDERRUN",
            GoogleChromeManagementV1TelemetryEventEventTypeEnum::NETWORKSTATECHANGE => "NETWORK_STATE_CHANGE",
            GoogleChromeManagementV1TelemetryEventEventTypeEnum::USBADDED => "USB_ADDED",
            GoogleChromeManagementV1TelemetryEventEventTypeEnum::USBREMOVED => "USB_REMOVED",
            GoogleChromeManagementV1TelemetryEventEventTypeEnum::NETWORKHTTPSLATENCYCHANGE => "NETWORK_HTTPS_LATENCY_CHANGE",
            GoogleChromeManagementV1TelemetryEventEventTypeEnum::WIFISIGNALSTRENGTHLOW => "WIFI_SIGNAL_STRENGTH_LOW",
            GoogleChromeManagementV1TelemetryEventEventTypeEnum::WIFISIGNALSTRENGTHRECOVERED => "WIFI_SIGNAL_STRENGTH_RECOVERED",
            GoogleChromeManagementV1TelemetryEventEventTypeEnum::VPNCONNECTIONSTATECHANGE => "VPN_CONNECTION_STATE_CHANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1TelemetryEventEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_TYPE_UNSPECIFIED" => Ok(GoogleChromeManagementV1TelemetryEventEventTypeEnum::EVENTTYPEUNSPECIFIED),
           "AUDIO_SEVERE_UNDERRUN" => Ok(GoogleChromeManagementV1TelemetryEventEventTypeEnum::AUDIOSEVEREUNDERRUN),
           "NETWORK_STATE_CHANGE" => Ok(GoogleChromeManagementV1TelemetryEventEventTypeEnum::NETWORKSTATECHANGE),
           "USB_ADDED" => Ok(GoogleChromeManagementV1TelemetryEventEventTypeEnum::USBADDED),
           "USB_REMOVED" => Ok(GoogleChromeManagementV1TelemetryEventEventTypeEnum::USBREMOVED),
           "NETWORK_HTTPS_LATENCY_CHANGE" => Ok(GoogleChromeManagementV1TelemetryEventEventTypeEnum::NETWORKHTTPSLATENCYCHANGE),
           "WIFI_SIGNAL_STRENGTH_LOW" => Ok(GoogleChromeManagementV1TelemetryEventEventTypeEnum::WIFISIGNALSTRENGTHLOW),
           "WIFI_SIGNAL_STRENGTH_RECOVERED" => Ok(GoogleChromeManagementV1TelemetryEventEventTypeEnum::WIFISIGNALSTRENGTHRECOVERED),
           "VPN_CONNECTION_STATE_CHANGE" => Ok(GoogleChromeManagementV1TelemetryEventEventTypeEnum::VPNCONNECTIONSTATECHANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1TelemetryEventEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Only sends the notifications for events of these types. Must not be empty.
pub enum GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum {
    

    /// Event type unknown.
    ///
    /// "EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="EVENT_TYPE_UNSPECIFIED")]
    EVENTTYPEUNSPECIFIED,
    

    /// Triggered when a audio devices run out of buffer data for more than 5 seconds.
    ///
    /// "AUDIO_SEVERE_UNDERRUN"
    #[serde(rename="AUDIO_SEVERE_UNDERRUN")]
    AUDIOSEVEREUNDERRUN,
    

    /// Triggered immediately on any changes to a network connection.
    ///
    /// "NETWORK_STATE_CHANGE"
    #[serde(rename="NETWORK_STATE_CHANGE")]
    NETWORKSTATECHANGE,
    

    /// Triggered when USB devices are added.
    ///
    /// "USB_ADDED"
    #[serde(rename="USB_ADDED")]
    USBADDED,
    

    /// Triggered when USB devices are removed.
    ///
    /// "USB_REMOVED"
    #[serde(rename="USB_REMOVED")]
    USBREMOVED,
    

    /// Triggered when a new HTTPS latency problem was detected or the device has recovered form an existing HTTPS latency problem.
    ///
    /// "NETWORK_HTTPS_LATENCY_CHANGE"
    #[serde(rename="NETWORK_HTTPS_LATENCY_CHANGE")]
    NETWORKHTTPSLATENCYCHANGE,
    

    /// Triggered when connected WiFi network signal strength drops below -70dBm.
    ///
    /// "WIFI_SIGNAL_STRENGTH_LOW"
    #[serde(rename="WIFI_SIGNAL_STRENGTH_LOW")]
    WIFISIGNALSTRENGTHLOW,
    

    /// Triggered when connected WiFi network signal strength is recovered from a signal drop.
    ///
    /// "WIFI_SIGNAL_STRENGTH_RECOVERED"
    #[serde(rename="WIFI_SIGNAL_STRENGTH_RECOVERED")]
    WIFISIGNALSTRENGTHRECOVERED,
    

    /// Triggered on changes to VPN connections.
    ///
    /// "VPN_CONNECTION_STATE_CHANGE"
    #[serde(rename="VPN_CONNECTION_STATE_CHANGE")]
    VPNCONNECTIONSTATECHANGE,
}

impl AsRef<str> for GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::EVENTTYPEUNSPECIFIED => "EVENT_TYPE_UNSPECIFIED",
            GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::AUDIOSEVEREUNDERRUN => "AUDIO_SEVERE_UNDERRUN",
            GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::NETWORKSTATECHANGE => "NETWORK_STATE_CHANGE",
            GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::USBADDED => "USB_ADDED",
            GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::USBREMOVED => "USB_REMOVED",
            GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::NETWORKHTTPSLATENCYCHANGE => "NETWORK_HTTPS_LATENCY_CHANGE",
            GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::WIFISIGNALSTRENGTHLOW => "WIFI_SIGNAL_STRENGTH_LOW",
            GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::WIFISIGNALSTRENGTHRECOVERED => "WIFI_SIGNAL_STRENGTH_RECOVERED",
            GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::VPNCONNECTIONSTATECHANGE => "VPN_CONNECTION_STATE_CHANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_TYPE_UNSPECIFIED" => Ok(GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::EVENTTYPEUNSPECIFIED),
           "AUDIO_SEVERE_UNDERRUN" => Ok(GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::AUDIOSEVEREUNDERRUN),
           "NETWORK_STATE_CHANGE" => Ok(GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::NETWORKSTATECHANGE),
           "USB_ADDED" => Ok(GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::USBADDED),
           "USB_REMOVED" => Ok(GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::USBREMOVED),
           "NETWORK_HTTPS_LATENCY_CHANGE" => Ok(GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::NETWORKHTTPSLATENCYCHANGE),
           "WIFI_SIGNAL_STRENGTH_LOW" => Ok(GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::WIFISIGNALSTRENGTHLOW),
           "WIFI_SIGNAL_STRENGTH_RECOVERED" => Ok(GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::WIFISIGNALSTRENGTHRECOVERED),
           "VPN_CONNECTION_STATE_CHANGE" => Ok(GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum::VPNCONNECTIONSTATECHANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1TelemetryEventNotificationFilterEventTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current HTTPS latency state.
pub enum GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum {
    

    /// HTTPS latency state is unspecified.
    ///
    /// "HTTPS_LATENCY_STATE_UNSPECIFIED"
    #[serde(rename="HTTPS_LATENCY_STATE_UNSPECIFIED")]
    HTTPSLATENCYSTATEUNSPECIFIED,
    

    /// HTTPS latency recovered from a problem.
    ///
    /// "RECOVERY"
    #[serde(rename="RECOVERY")]
    RECOVERY,
    

    /// HTTPS latency problem.
    ///
    /// "PROBLEM"
    #[serde(rename="PROBLEM")]
    PROBLEM,
}

impl AsRef<str> for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum::HTTPSLATENCYSTATEUNSPECIFIED => "HTTPS_LATENCY_STATE_UNSPECIFIED",
            GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum::RECOVERY => "RECOVERY",
            GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum::PROBLEM => "PROBLEM",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HTTPS_LATENCY_STATE_UNSPECIFIED" => Ok(GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum::HTTPSLATENCYSTATEUNSPECIFIED),
           "RECOVERY" => Ok(GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum::RECOVERY),
           "PROBLEM" => Ok(GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum::PROBLEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1TelemetryHttpsLatencyChangeEventHttpsLatencyStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current connection state of the network.
pub enum GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum {
    

    /// Network connection state unspecified.
    ///
    /// "NETWORK_CONNECTION_STATE_UNSPECIFIED"
    #[serde(rename="NETWORK_CONNECTION_STATE_UNSPECIFIED")]
    NETWORKCONNECTIONSTATEUNSPECIFIED,
    

    /// The network is connected and internet connectivity is available.
    ///
    /// "ONLINE"
    #[serde(rename="ONLINE")]
    ONLINE,
    

    /// The network is connected and not in a detected portal state, but internet connectivity may not be available.
    ///
    /// "CONNECTED"
    #[serde(rename="CONNECTED")]
    CONNECTED,
    

    /// The network is connected but a portal state was detected. Internet connectivity may be limited.
    ///
    /// "PORTAL"
    #[serde(rename="PORTAL")]
    PORTAL,
    

    /// The network is in the process of connecting.
    ///
    /// "CONNECTING"
    #[serde(rename="CONNECTING")]
    CONNECTING,
    

    /// The network is not connected.
    ///
    /// "NOT_CONNECTED"
    #[serde(rename="NOT_CONNECTED")]
    NOTCONNECTED,
}

impl AsRef<str> for GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::NETWORKCONNECTIONSTATEUNSPECIFIED => "NETWORK_CONNECTION_STATE_UNSPECIFIED",
            GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::ONLINE => "ONLINE",
            GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::CONNECTED => "CONNECTED",
            GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::PORTAL => "PORTAL",
            GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::CONNECTING => "CONNECTING",
            GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::NOTCONNECTED => "NOT_CONNECTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NETWORK_CONNECTION_STATE_UNSPECIFIED" => Ok(GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::NETWORKCONNECTIONSTATEUNSPECIFIED),
           "ONLINE" => Ok(GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::ONLINE),
           "CONNECTED" => Ok(GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::CONNECTED),
           "PORTAL" => Ok(GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::PORTAL),
           "CONNECTING" => Ok(GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::CONNECTING),
           "NOT_CONNECTED" => Ok(GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum::NOTCONNECTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1TelemetryNetworkConnectionStateChangeEventConnectionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Security level of the Thunderbolt bus.
pub enum GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum {
    

    /// Thunderbolt security level is not set.
    ///
    /// "THUNDERBOLT_SECURITY_LEVEL_UNSPECIFIED"
    #[serde(rename="THUNDERBOLT_SECURITY_LEVEL_UNSPECIFIED")]
    THUNDERBOLTSECURITYLEVELUNSPECIFIED,
    

    /// All devices are automatically connected by the firmware. No user approval is needed.
    ///
    /// "THUNDERBOLT_SECURITY_NONE_LEVEL"
    #[serde(rename="THUNDERBOLT_SECURITY_NONE_LEVEL")]
    THUNDERBOLTSECURITYNONELEVEL,
    

    /// User is asked whether the device is allowed to be connected.
    ///
    /// "THUNDERBOLT_SECURITY_USER_LEVEL"
    #[serde(rename="THUNDERBOLT_SECURITY_USER_LEVEL")]
    THUNDERBOLTSECURITYUSERLEVEL,
    

    /// User is asked whether the device is allowed to be connected. In addition the device is sent a challenge that should match the expected one based on a random key written to the key sysfs attribute
    ///
    /// "THUNDERBOLT_SECURITY_SECURE_LEVEL"
    #[serde(rename="THUNDERBOLT_SECURITY_SECURE_LEVEL")]
    THUNDERBOLTSECURITYSECURELEVEL,
    

    /// The firmware automatically creates tunnels for Thunderbolt.
    ///
    /// "THUNDERBOLT_SECURITY_DP_ONLY_LEVEL"
    #[serde(rename="THUNDERBOLT_SECURITY_DP_ONLY_LEVEL")]
    THUNDERBOLTSECURITYDPONLYLEVEL,
    

    /// The firmware automatically creates tunnels for the USB controller and Display Port in a dock. All PCIe links downstream of the dock are removed.
    ///
    /// "THUNDERBOLT_SECURITY_USB_ONLY_LEVEL"
    #[serde(rename="THUNDERBOLT_SECURITY_USB_ONLY_LEVEL")]
    THUNDERBOLTSECURITYUSBONLYLEVEL,
    

    /// PCIE tunneling is disabled.
    ///
    /// "THUNDERBOLT_SECURITY_NO_PCIE_LEVEL"
    #[serde(rename="THUNDERBOLT_SECURITY_NO_PCIE_LEVEL")]
    THUNDERBOLTSECURITYNOPCIELEVEL,
}

impl AsRef<str> for GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYLEVELUNSPECIFIED => "THUNDERBOLT_SECURITY_LEVEL_UNSPECIFIED",
            GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYNONELEVEL => "THUNDERBOLT_SECURITY_NONE_LEVEL",
            GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYUSERLEVEL => "THUNDERBOLT_SECURITY_USER_LEVEL",
            GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYSECURELEVEL => "THUNDERBOLT_SECURITY_SECURE_LEVEL",
            GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYDPONLYLEVEL => "THUNDERBOLT_SECURITY_DP_ONLY_LEVEL",
            GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYUSBONLYLEVEL => "THUNDERBOLT_SECURITY_USB_ONLY_LEVEL",
            GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYNOPCIELEVEL => "THUNDERBOLT_SECURITY_NO_PCIE_LEVEL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THUNDERBOLT_SECURITY_LEVEL_UNSPECIFIED" => Ok(GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYLEVELUNSPECIFIED),
           "THUNDERBOLT_SECURITY_NONE_LEVEL" => Ok(GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYNONELEVEL),
           "THUNDERBOLT_SECURITY_USER_LEVEL" => Ok(GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYUSERLEVEL),
           "THUNDERBOLT_SECURITY_SECURE_LEVEL" => Ok(GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYSECURELEVEL),
           "THUNDERBOLT_SECURITY_DP_ONLY_LEVEL" => Ok(GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYDPONLYLEVEL),
           "THUNDERBOLT_SECURITY_USB_ONLY_LEVEL" => Ok(GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYUSBONLYLEVEL),
           "THUNDERBOLT_SECURITY_NO_PCIE_LEVEL" => Ok(GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum::THUNDERBOLTSECURITYNOPCIELEVEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1ThunderboltInfoSecurityLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Memory encryption algorithm.
pub enum GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum {
    

    /// Memory encryption algorithm is not set.
    ///
    /// "MEMORY_ENCRYPTION_ALGORITHM_UNSPECIFIED"
    #[serde(rename="MEMORY_ENCRYPTION_ALGORITHM_UNSPECIFIED")]
    MEMORYENCRYPTIONALGORITHMUNSPECIFIED,
    

    /// The memory encryption algorithm being used is unknown.
    ///
    /// "MEMORY_ENCRYPTION_ALGORITHM_UNKNOWN"
    #[serde(rename="MEMORY_ENCRYPTION_ALGORITHM_UNKNOWN")]
    MEMORYENCRYPTIONALGORITHMUNKNOWN,
    

    /// The memory encryption algorithm is using the AES_XTS encryption algorithm with a 128 bit block cypher.
    ///
    /// "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_128"
    #[serde(rename="MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_128")]
    MEMORYENCRYPTIONALGORITHMAESXTS128,
    

    /// The memory encryption algorithm is using the AES_XTS encryption algorithm with a 256 bit block cypher.
    ///
    /// "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_256"
    #[serde(rename="MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_256")]
    MEMORYENCRYPTIONALGORITHMAESXTS256,
}

impl AsRef<str> for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum::MEMORYENCRYPTIONALGORITHMUNSPECIFIED => "MEMORY_ENCRYPTION_ALGORITHM_UNSPECIFIED",
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum::MEMORYENCRYPTIONALGORITHMUNKNOWN => "MEMORY_ENCRYPTION_ALGORITHM_UNKNOWN",
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum::MEMORYENCRYPTIONALGORITHMAESXTS128 => "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_128",
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum::MEMORYENCRYPTIONALGORITHMAESXTS256 => "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_256",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEMORY_ENCRYPTION_ALGORITHM_UNSPECIFIED" => Ok(GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum::MEMORYENCRYPTIONALGORITHMUNSPECIFIED),
           "MEMORY_ENCRYPTION_ALGORITHM_UNKNOWN" => Ok(GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum::MEMORYENCRYPTIONALGORITHMUNKNOWN),
           "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_128" => Ok(GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum::MEMORYENCRYPTIONALGORITHMAESXTS128),
           "MEMORY_ENCRYPTION_ALGORITHM_AES_XTS_256" => Ok(GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum::MEMORYENCRYPTIONALGORITHMAESXTS256),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of memory encryption on the device.
pub enum GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum {
    

    /// Memory encryption state is not set.
    ///
    /// "MEMORY_ENCRYPTION_STATE_UNSPECIFIED"
    #[serde(rename="MEMORY_ENCRYPTION_STATE_UNSPECIFIED")]
    MEMORYENCRYPTIONSTATEUNSPECIFIED,
    

    /// The memory encryption state is unknown.
    ///
    /// "MEMORY_ENCRYPTION_STATE_UNKNOWN"
    #[serde(rename="MEMORY_ENCRYPTION_STATE_UNKNOWN")]
    MEMORYENCRYPTIONSTATEUNKNOWN,
    

    /// Memory encrpytion on the device is disabled.
    ///
    /// "MEMORY_ENCRYPTION_STATE_DISABLED"
    #[serde(rename="MEMORY_ENCRYPTION_STATE_DISABLED")]
    MEMORYENCRYPTIONSTATEDISABLED,
    

    /// Memory encryption on the device uses total memory encryption.
    ///
    /// "MEMORY_ENCRYPTION_STATE_TME"
    #[serde(rename="MEMORY_ENCRYPTION_STATE_TME")]
    MEMORYENCRYPTIONSTATETME,
    

    /// Memory encryption on the device uses multi-key total memory encryption.
    ///
    /// "MEMORY_ENCRYPTION_STATE_MKTME"
    #[serde(rename="MEMORY_ENCRYPTION_STATE_MKTME")]
    MEMORYENCRYPTIONSTATEMKTME,
}

impl AsRef<str> for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum::MEMORYENCRYPTIONSTATEUNSPECIFIED => "MEMORY_ENCRYPTION_STATE_UNSPECIFIED",
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum::MEMORYENCRYPTIONSTATEUNKNOWN => "MEMORY_ENCRYPTION_STATE_UNKNOWN",
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum::MEMORYENCRYPTIONSTATEDISABLED => "MEMORY_ENCRYPTION_STATE_DISABLED",
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum::MEMORYENCRYPTIONSTATETME => "MEMORY_ENCRYPTION_STATE_TME",
            GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum::MEMORYENCRYPTIONSTATEMKTME => "MEMORY_ENCRYPTION_STATE_MKTME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEMORY_ENCRYPTION_STATE_UNSPECIFIED" => Ok(GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum::MEMORYENCRYPTIONSTATEUNSPECIFIED),
           "MEMORY_ENCRYPTION_STATE_UNKNOWN" => Ok(GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum::MEMORYENCRYPTIONSTATEUNKNOWN),
           "MEMORY_ENCRYPTION_STATE_DISABLED" => Ok(GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum::MEMORYENCRYPTIONSTATEDISABLED),
           "MEMORY_ENCRYPTION_STATE_TME" => Ok(GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum::MEMORYENCRYPTIONSTATETME),
           "MEMORY_ENCRYPTION_STATE_MKTME" => Ok(GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum::MEMORYENCRYPTIONSTATEMKTME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleChromeManagementV1TotalMemoryEncryptionInfoEncryptionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomerAppTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the app.
pub enum CustomerAppTypeEnum {
    

    /// App type not specified.
    ///
    /// "APP_TYPE_UNSPECIFIED"
    #[serde(rename="APP_TYPE_UNSPECIFIED")]
    APPTYPEUNSPECIFIED,
    

    /// Chrome extension.
    ///
    /// "EXTENSION"
    #[serde(rename="EXTENSION")]
    EXTENSION,
    

    /// Chrome app.
    ///
    /// "APP"
    #[serde(rename="APP")]
    APP,
    

    /// Chrome theme.
    ///
    /// "THEME"
    #[serde(rename="THEME")]
    THEME,
    

    /// Chrome hosted app.
    ///
    /// "HOSTED_APP"
    #[serde(rename="HOSTED_APP")]
    HOSTEDAPP,
    

    /// ARC++ app.
    ///
    /// "ANDROID_APP"
    #[serde(rename="ANDROID_APP")]
    ANDROIDAPP,
}

impl AsRef<str> for CustomerAppTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomerAppTypeEnum::APPTYPEUNSPECIFIED => "APP_TYPE_UNSPECIFIED",
            CustomerAppTypeEnum::EXTENSION => "EXTENSION",
            CustomerAppTypeEnum::APP => "APP",
            CustomerAppTypeEnum::THEME => "THEME",
            CustomerAppTypeEnum::HOSTEDAPP => "HOSTED_APP",
            CustomerAppTypeEnum::ANDROIDAPP => "ANDROID_APP",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomerAppTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_TYPE_UNSPECIFIED" => Ok(CustomerAppTypeEnum::APPTYPEUNSPECIFIED),
           "EXTENSION" => Ok(CustomerAppTypeEnum::EXTENSION),
           "APP" => Ok(CustomerAppTypeEnum::APP),
           "THEME" => Ok(CustomerAppTypeEnum::THEME),
           "HOSTED_APP" => Ok(CustomerAppTypeEnum::HOSTEDAPP),
           "ANDROID_APP" => Ok(CustomerAppTypeEnum::ANDROIDAPP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomerAppTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


