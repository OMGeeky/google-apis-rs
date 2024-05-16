use super::*;



// region ChannelChannelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of channel.
pub enum ChannelChannelTypeEnum {
    
    /// "CHANNEL_TYPE_UNSPECIFIED"
    #[serde(rename="CHANNEL_TYPE_UNSPECIFIED")]
    CHANNELTYPEUNSPECIFIED,
    

    /// The Stable channel.
    ///
    /// "STABLE"
    #[serde(rename="STABLE")]
    STABLE,
    

    /// The Beta channel.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// The Dev channel.
    ///
    /// "DEV"
    #[serde(rename="DEV")]
    DEV,
    

    /// The Canary channel.
    ///
    /// "CANARY"
    #[serde(rename="CANARY")]
    CANARY,
    

    /// The Canary channel for Chrome, with DCHECK/ASAN enabled.
    ///
    /// "CANARY_ASAN"
    #[serde(rename="CANARY_ASAN")]
    CANARYASAN,
    
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// The Extended Stable channel for Chrome.
    ///
    /// "EXTENDED"
    #[serde(rename="EXTENDED")]
    EXTENDED,
    

    /// The Long-term support channel for ChromeOS.
    ///
    /// "LTS"
    #[serde(rename="LTS")]
    LTS,
    

    /// The Long-term support candidate channel for ChromeOS.
    ///
    /// "LTC"
    #[serde(rename="LTC")]
    LTC,
}

impl AsRef<str> for ChannelChannelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelChannelTypeEnum::CHANNELTYPEUNSPECIFIED => "CHANNEL_TYPE_UNSPECIFIED",
            ChannelChannelTypeEnum::STABLE => "STABLE",
            ChannelChannelTypeEnum::BETA => "BETA",
            ChannelChannelTypeEnum::DEV => "DEV",
            ChannelChannelTypeEnum::CANARY => "CANARY",
            ChannelChannelTypeEnum::CANARYASAN => "CANARY_ASAN",
            ChannelChannelTypeEnum::ALL => "ALL",
            ChannelChannelTypeEnum::EXTENDED => "EXTENDED",
            ChannelChannelTypeEnum::LTS => "LTS",
            ChannelChannelTypeEnum::LTC => "LTC",
        }
    }
}

impl std::convert::TryFrom< &str> for ChannelChannelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHANNEL_TYPE_UNSPECIFIED" => Ok(ChannelChannelTypeEnum::CHANNELTYPEUNSPECIFIED),
           "STABLE" => Ok(ChannelChannelTypeEnum::STABLE),
           "BETA" => Ok(ChannelChannelTypeEnum::BETA),
           "DEV" => Ok(ChannelChannelTypeEnum::DEV),
           "CANARY" => Ok(ChannelChannelTypeEnum::CANARY),
           "CANARY_ASAN" => Ok(ChannelChannelTypeEnum::CANARYASAN),
           "ALL" => Ok(ChannelChannelTypeEnum::ALL),
           "EXTENDED" => Ok(ChannelChannelTypeEnum::EXTENDED),
           "LTS" => Ok(ChannelChannelTypeEnum::LTS),
           "LTC" => Ok(ChannelChannelTypeEnum::LTC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelChannelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlatformPlatformTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of platform.
pub enum PlatformPlatformTypeEnum {
    
    /// "PLATFORM_TYPE_UNSPECIFIED"
    #[serde(rename="PLATFORM_TYPE_UNSPECIFIED")]
    PLATFORMTYPEUNSPECIFIED,
    

    /// Chrome Desktop for Windows (32-bit).
    ///
    /// "WIN"
    #[serde(rename="WIN")]
    WIN,
    

    /// Chrome Desktop for Windows (x86_64).
    ///
    /// "WIN64"
    #[serde(rename="WIN64")]
    WIN64,
    

    /// Chrome Desktop for macOS (x86_64).
    ///
    /// "MAC"
    #[serde(rename="MAC")]
    MAC,
    

    /// Chrome Desktop for Linux.
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Chrome for Android.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// WebView for Android.
    ///
    /// "WEBVIEW"
    #[serde(rename="WEBVIEW")]
    WEBVIEW,
    

    /// Chrome for iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Chrome for macOS (ARM64).
    ///
    /// "MAC_ARM64"
    #[serde(rename="MAC_ARM64")]
    MACARM64,
    

    /// ChromeOS Lacros (x86_64).
    ///
    /// "LACROS"
    #[serde(rename="LACROS")]
    LACROS,
    

    /// ChromeOS Lacros (ARM).
    ///
    /// "LACROS_ARM32"
    #[serde(rename="LACROS_ARM32")]
    LACROSARM32,
    

    /// ChromeOS.
    ///
    /// "CHROMEOS"
    #[serde(rename="CHROMEOS")]
    CHROMEOS,
    

    /// ChromeOS Lacros (ARM64).
    ///
    /// "LACROS_ARM64"
    #[serde(rename="LACROS_ARM64")]
    LACROSARM64,
    

    /// Chrome for Fuchsia.
    ///
    /// "FUCHSIA"
    #[serde(rename="FUCHSIA")]
    FUCHSIA,
    

    /// Chrome Desktop for Windows (ARM64).
    ///
    /// "WIN_ARM64"
    #[serde(rename="WIN_ARM64")]
    WINARM64,
}

impl AsRef<str> for PlatformPlatformTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlatformPlatformTypeEnum::PLATFORMTYPEUNSPECIFIED => "PLATFORM_TYPE_UNSPECIFIED",
            PlatformPlatformTypeEnum::WIN => "WIN",
            PlatformPlatformTypeEnum::WIN64 => "WIN64",
            PlatformPlatformTypeEnum::MAC => "MAC",
            PlatformPlatformTypeEnum::LINUX => "LINUX",
            PlatformPlatformTypeEnum::ANDROID => "ANDROID",
            PlatformPlatformTypeEnum::WEBVIEW => "WEBVIEW",
            PlatformPlatformTypeEnum::IOS => "IOS",
            PlatformPlatformTypeEnum::ALL => "ALL",
            PlatformPlatformTypeEnum::MACARM64 => "MAC_ARM64",
            PlatformPlatformTypeEnum::LACROS => "LACROS",
            PlatformPlatformTypeEnum::LACROSARM32 => "LACROS_ARM32",
            PlatformPlatformTypeEnum::CHROMEOS => "CHROMEOS",
            PlatformPlatformTypeEnum::LACROSARM64 => "LACROS_ARM64",
            PlatformPlatformTypeEnum::FUCHSIA => "FUCHSIA",
            PlatformPlatformTypeEnum::WINARM64 => "WIN_ARM64",
        }
    }
}

impl std::convert::TryFrom< &str> for PlatformPlatformTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_TYPE_UNSPECIFIED" => Ok(PlatformPlatformTypeEnum::PLATFORMTYPEUNSPECIFIED),
           "WIN" => Ok(PlatformPlatformTypeEnum::WIN),
           "WIN64" => Ok(PlatformPlatformTypeEnum::WIN64),
           "MAC" => Ok(PlatformPlatformTypeEnum::MAC),
           "LINUX" => Ok(PlatformPlatformTypeEnum::LINUX),
           "ANDROID" => Ok(PlatformPlatformTypeEnum::ANDROID),
           "WEBVIEW" => Ok(PlatformPlatformTypeEnum::WEBVIEW),
           "IOS" => Ok(PlatformPlatformTypeEnum::IOS),
           "ALL" => Ok(PlatformPlatformTypeEnum::ALL),
           "MAC_ARM64" => Ok(PlatformPlatformTypeEnum::MACARM64),
           "LACROS" => Ok(PlatformPlatformTypeEnum::LACROS),
           "LACROS_ARM32" => Ok(PlatformPlatformTypeEnum::LACROSARM32),
           "CHROMEOS" => Ok(PlatformPlatformTypeEnum::CHROMEOS),
           "LACROS_ARM64" => Ok(PlatformPlatformTypeEnum::LACROSARM64),
           "FUCHSIA" => Ok(PlatformPlatformTypeEnum::FUCHSIA),
           "WIN_ARM64" => Ok(PlatformPlatformTypeEnum::WINARM64),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlatformPlatformTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


