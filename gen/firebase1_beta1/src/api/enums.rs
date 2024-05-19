use super::*;



// region AndroidAppStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The lifecycle state of the App.
pub enum AndroidAppStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The App is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The App has been soft-deleted. After an App has been in the `DELETED` state for more than 30 days, it is considered expired and will be permanently deleted. Up until this time, you can restore the App by calling `Undelete` ([Android](projects.androidApps/undelete) | [iOS](projects.iosApps/undelete) | [web](projects.webApps/undelete)).
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for AndroidAppStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AndroidAppStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AndroidAppStateEnum::ACTIVE => "ACTIVE",
            AndroidAppStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for AndroidAppStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AndroidAppStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(AndroidAppStateEnum::ACTIVE),
           "DELETED" => Ok(AndroidAppStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AndroidAppStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirebaseAppInfoPlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The platform of the Firebase App.
pub enum FirebaseAppInfoPlatformEnum {
    

    /// Unknown state. This is only used for distinguishing unset values.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// The Firebase App is associated with iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// The Firebase App is associated with Android.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// The Firebase App is associated with web.
    ///
    /// "WEB"
    #[serde(rename="WEB")]
    WEB,
}

impl AsRef<str> for FirebaseAppInfoPlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirebaseAppInfoPlatformEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            FirebaseAppInfoPlatformEnum::IOS => "IOS",
            FirebaseAppInfoPlatformEnum::ANDROID => "ANDROID",
            FirebaseAppInfoPlatformEnum::WEB => "WEB",
        }
    }
}

impl std::convert::TryFrom< &str> for FirebaseAppInfoPlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(FirebaseAppInfoPlatformEnum::PLATFORMUNSPECIFIED),
           "IOS" => Ok(FirebaseAppInfoPlatformEnum::IOS),
           "ANDROID" => Ok(FirebaseAppInfoPlatformEnum::ANDROID),
           "WEB" => Ok(FirebaseAppInfoPlatformEnum::WEB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirebaseAppInfoPlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirebaseAppInfoStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The lifecycle state of the App.
pub enum FirebaseAppInfoStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The App is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The App has been soft-deleted. After an App has been in the `DELETED` state for more than 30 days, it is considered expired and will be permanently deleted. Up until this time, you can restore the App by calling `Undelete` ([Android](projects.androidApps/undelete) | [iOS](projects.iosApps/undelete) | [web](projects.webApps/undelete)).
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for FirebaseAppInfoStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirebaseAppInfoStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            FirebaseAppInfoStateEnum::ACTIVE => "ACTIVE",
            FirebaseAppInfoStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for FirebaseAppInfoStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(FirebaseAppInfoStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(FirebaseAppInfoStateEnum::ACTIVE),
           "DELETED" => Ok(FirebaseAppInfoStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirebaseAppInfoStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirebaseProjectStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The lifecycle state of the Project.
pub enum FirebaseProjectStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The Project is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The Project has been soft-deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for FirebaseProjectStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirebaseProjectStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            FirebaseProjectStateEnum::ACTIVE => "ACTIVE",
            FirebaseProjectStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for FirebaseProjectStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(FirebaseProjectStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(FirebaseProjectStateEnum::ACTIVE),
           "DELETED" => Ok(FirebaseProjectStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirebaseProjectStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IosAppStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The lifecycle state of the App.
pub enum IosAppStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The App is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The App has been soft-deleted. After an App has been in the `DELETED` state for more than 30 days, it is considered expired and will be permanently deleted. Up until this time, you can restore the App by calling `Undelete` ([Android](projects.androidApps/undelete) | [iOS](projects.iosApps/undelete) | [web](projects.webApps/undelete)).
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for IosAppStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IosAppStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            IosAppStateEnum::ACTIVE => "ACTIVE",
            IosAppStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for IosAppStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(IosAppStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(IosAppStateEnum::ACTIVE),
           "DELETED" => Ok(IosAppStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IosAppStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LocationFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Products and services that are available in the GCP resource location.
pub enum LocationFeaturesEnum {
    

    /// Used internally for distinguishing unset values and is not intended for external use.
    ///
    /// "LOCATION_FEATURE_UNSPECIFIED"
    #[serde(rename="LOCATION_FEATURE_UNSPECIFIED")]
    LOCATIONFEATUREUNSPECIFIED,
    

    /// This location supports Cloud Firestore database instances. App Engine is available in this location, so it can be a Project's [default GCP resource location](//firebase.google.com/docs/projects/locations#default-cloud-location).
    ///
    /// "FIRESTORE"
    #[serde(rename="FIRESTORE")]
    FIRESTORE,
    

    /// This location supports default Cloud Storage buckets. App Engine is available in this location, so it can be a Project's [default GCP resource location](//firebase.google.com/docs/projects/locations#default-cloud-location).
    ///
    /// "DEFAULT_STORAGE"
    #[serde(rename="DEFAULT_STORAGE")]
    DEFAULTSTORAGE,
    

    /// Cloud Functions for Firebase is available in this location.
    ///
    /// "FUNCTIONS"
    #[serde(rename="FUNCTIONS")]
    FUNCTIONS,
}

impl AsRef<str> for LocationFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LocationFeaturesEnum::LOCATIONFEATUREUNSPECIFIED => "LOCATION_FEATURE_UNSPECIFIED",
            LocationFeaturesEnum::FIRESTORE => "FIRESTORE",
            LocationFeaturesEnum::DEFAULTSTORAGE => "DEFAULT_STORAGE",
            LocationFeaturesEnum::FUNCTIONS => "FUNCTIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for LocationFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOCATION_FEATURE_UNSPECIFIED" => Ok(LocationFeaturesEnum::LOCATIONFEATUREUNSPECIFIED),
           "FIRESTORE" => Ok(LocationFeaturesEnum::FIRESTORE),
           "DEFAULT_STORAGE" => Ok(LocationFeaturesEnum::DEFAULTSTORAGE),
           "FUNCTIONS" => Ok(LocationFeaturesEnum::FUNCTIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LocationFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LocationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the GCP resource location is a [regional or multi-regional location](https://firebase.google.com/docs/projects/locations#types) for data replication.
pub enum LocationTypeEnum {
    

    /// Used internally for distinguishing unset values and is not intended for external use.
    ///
    /// "LOCATION_TYPE_UNSPECIFIED"
    #[serde(rename="LOCATION_TYPE_UNSPECIFIED")]
    LOCATIONTYPEUNSPECIFIED,
    

    /// The location is a regional location. Data in a regional location is replicated in multiple zones within a region.
    ///
    /// "REGIONAL"
    #[serde(rename="REGIONAL")]
    REGIONAL,
    

    /// The location is a multi-regional location. Data in a multi-region location is replicated in multiple regions. Within each region, data is replicated in multiple zones.
    ///
    /// "MULTI_REGIONAL"
    #[serde(rename="MULTI_REGIONAL")]
    MULTIREGIONAL,
}

impl AsRef<str> for LocationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LocationTypeEnum::LOCATIONTYPEUNSPECIFIED => "LOCATION_TYPE_UNSPECIFIED",
            LocationTypeEnum::REGIONAL => "REGIONAL",
            LocationTypeEnum::MULTIREGIONAL => "MULTI_REGIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for LocationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOCATION_TYPE_UNSPECIFIED" => Ok(LocationTypeEnum::LOCATIONTYPEUNSPECIFIED),
           "REGIONAL" => Ok(LocationTypeEnum::REGIONAL),
           "MULTI_REGIONAL" => Ok(LocationTypeEnum::MULTIREGIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LocationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ShaCertificateCertTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of SHA certificate encoded in the hash.
pub enum ShaCertificateCertTypeEnum {
    

    /// Unknown state. This is only used for distinguishing unset values.
    ///
    /// "SHA_CERTIFICATE_TYPE_UNSPECIFIED"
    #[serde(rename="SHA_CERTIFICATE_TYPE_UNSPECIFIED")]
    SHACERTIFICATETYPEUNSPECIFIED,
    

    /// Certificate is a SHA-1 type certificate.
    ///
    /// "SHA_1"
    #[serde(rename="SHA_1")]
    SHA1,
    

    /// Certificate is a SHA-256 type certificate.
    ///
    /// "SHA_256"
    #[serde(rename="SHA_256")]
    SHA256,
}

impl AsRef<str> for ShaCertificateCertTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ShaCertificateCertTypeEnum::SHACERTIFICATETYPEUNSPECIFIED => "SHA_CERTIFICATE_TYPE_UNSPECIFIED",
            ShaCertificateCertTypeEnum::SHA1 => "SHA_1",
            ShaCertificateCertTypeEnum::SHA256 => "SHA_256",
        }
    }
}

impl std::convert::TryFrom< &str> for ShaCertificateCertTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SHA_CERTIFICATE_TYPE_UNSPECIFIED" => Ok(ShaCertificateCertTypeEnum::SHACERTIFICATETYPEUNSPECIFIED),
           "SHA_1" => Ok(ShaCertificateCertTypeEnum::SHA1),
           "SHA_256" => Ok(ShaCertificateCertTypeEnum::SHA256),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ShaCertificateCertTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WebAppStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The lifecycle state of the App.
pub enum WebAppStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The App is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The App has been soft-deleted. After an App has been in the `DELETED` state for more than 30 days, it is considered expired and will be permanently deleted. Up until this time, you can restore the App by calling `Undelete` ([Android](projects.androidApps/undelete) | [iOS](projects.iosApps/undelete) | [web](projects.webApps/undelete)).
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for WebAppStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WebAppStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            WebAppStateEnum::ACTIVE => "ACTIVE",
            WebAppStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for WebAppStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(WebAppStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(WebAppStateEnum::ACTIVE),
           "DELETED" => Ok(WebAppStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WebAppStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


