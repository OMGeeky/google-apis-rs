use super::*;



// region MyconfigLicenseTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of access license to request. If not specified, the default is BOTH.
pub enum MyconfigLicenseTypesEnum {
    
    /// "LICENSE_TYPES_UNDEFINED"
    #[serde(rename="LICENSE_TYPES_UNDEFINED")]
    LICENSETYPESUNDEFINED,
    

    /// Both concurrent and download licenses.
    ///
    /// "BOTH"
    #[serde(rename="BOTH")]
    BOTH,
    

    /// Concurrent access license.
    ///
    /// "CONCURRENT"
    #[serde(rename="CONCURRENT")]
    CONCURRENT,
    

    /// Offline download access license.
    ///
    /// "DOWNLOAD"
    #[serde(rename="DOWNLOAD")]
    DOWNLOAD,
}

impl AsRef<str> for MyconfigLicenseTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MyconfigLicenseTypesEnum::LICENSETYPESUNDEFINED => "LICENSE_TYPES_UNDEFINED",
            MyconfigLicenseTypesEnum::BOTH => "BOTH",
            MyconfigLicenseTypesEnum::CONCURRENT => "CONCURRENT",
            MyconfigLicenseTypesEnum::DOWNLOAD => "DOWNLOAD",
        }
    }
}

impl std::convert::TryFrom< &str> for MyconfigLicenseTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LICENSE_TYPES_UNDEFINED" => Ok(MyconfigLicenseTypesEnum::LICENSETYPESUNDEFINED),
           "BOTH" => Ok(MyconfigLicenseTypesEnum::BOTH),
           "CONCURRENT" => Ok(MyconfigLicenseTypesEnum::CONCURRENT),
           "DOWNLOAD" => Ok(MyconfigLicenseTypesEnum::DOWNLOAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MyconfigLicenseTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MyconfigFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of features supported by the client, i.e., 'RENTALS'
pub enum MyconfigFeaturesEnum {
    
    /// "FEATURES_UNDEFINED"
    #[serde(rename="FEATURES_UNDEFINED")]
    FEATURESUNDEFINED,
    

    /// Client supports rentals.
    ///
    /// "RENTALS"
    #[serde(rename="RENTALS")]
    RENTALS,
}

impl AsRef<str> for MyconfigFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MyconfigFeaturesEnum::FEATURESUNDEFINED => "FEATURES_UNDEFINED",
            MyconfigFeaturesEnum::RENTALS => "RENTALS",
        }
    }
}

impl std::convert::TryFrom< &str> for MyconfigFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEATURES_UNDEFINED" => Ok(MyconfigFeaturesEnum::FEATURESUNDEFINED),
           "RENTALS" => Ok(MyconfigFeaturesEnum::RENTALS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MyconfigFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MylibraryProjectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restrict information returned to a set of selected fields.
pub enum MylibraryProjectionEnum {
    
    /// "PROJECTION_UNDEFINED"
    #[serde(rename="PROJECTION_UNDEFINED")]
    PROJECTIONUNDEFINED,
    

    /// Includes all volume data.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Includes a subset of fields in volumeInfo and accessInfo.
    ///
    /// "LITE"
    #[serde(rename="LITE")]
    LITE,
}

impl AsRef<str> for MylibraryProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MylibraryProjectionEnum::PROJECTIONUNDEFINED => "PROJECTION_UNDEFINED",
            MylibraryProjectionEnum::FULL => "FULL",
            MylibraryProjectionEnum::LITE => "LITE",
        }
    }
}

impl std::convert::TryFrom< &str> for MylibraryProjectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROJECTION_UNDEFINED" => Ok(MylibraryProjectionEnum::PROJECTIONUNDEFINED),
           "FULL" => Ok(MylibraryProjectionEnum::FULL),
           "LITE" => Ok(MylibraryProjectionEnum::LITE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MylibraryProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MylibraryReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason for which the book is removed from the library.
pub enum MylibraryReasonEnum {
    
    /// "REASON_UNDEFINED"
    #[serde(rename="REASON_UNDEFINED")]
    REASONUNDEFINED,
    

    /// Samples removed from the Onboarding flow.
    ///
    /// "ONBOARDING"
    #[serde(rename="ONBOARDING")]
    ONBOARDING,
}

impl AsRef<str> for MylibraryReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MylibraryReasonEnum::REASONUNDEFINED => "REASON_UNDEFINED",
            MylibraryReasonEnum::ONBOARDING => "ONBOARDING",
        }
    }
}

impl std::convert::TryFrom< &str> for MylibraryReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REASON_UNDEFINED" => Ok(MylibraryReasonEnum::REASONUNDEFINED),
           "ONBOARDING" => Ok(MylibraryReasonEnum::ONBOARDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MylibraryReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MylibraryActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Action that caused this reading position to be set.
pub enum MylibraryActionEnum {
    
    /// "ACTION_UNDEFINED"
    #[serde(rename="ACTION_UNDEFINED")]
    ACTIONUNDEFINED,
    

    /// User chose bookmark within volume.
    ///
    /// "bookmark"
    #[serde(rename="bookmark")]
    Bookmark,
    

    /// User selected chapter from list.
    ///
    /// "chapter"
    #[serde(rename="chapter")]
    Chapter,
    

    /// Next page event.
    ///
    /// "next-page"
    #[serde(rename="next-page")]
    NextPage,
    

    /// Previous page event.
    ///
    /// "prev-page"
    #[serde(rename="prev-page")]
    PrevPage,
    

    /// User navigated to page.
    ///
    /// "scroll"
    #[serde(rename="scroll")]
    Scroll,
    

    /// User chose search results within volume.
    ///
    /// "search"
    #[serde(rename="search")]
    Search,
}

impl AsRef<str> for MylibraryActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MylibraryActionEnum::ACTIONUNDEFINED => "ACTION_UNDEFINED",
            MylibraryActionEnum::Bookmark => "bookmark",
            MylibraryActionEnum::Chapter => "chapter",
            MylibraryActionEnum::NextPage => "next-page",
            MylibraryActionEnum::PrevPage => "prev-page",
            MylibraryActionEnum::Scroll => "scroll",
            MylibraryActionEnum::Search => "search",
        }
    }
}

impl std::convert::TryFrom< &str> for MylibraryActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_UNDEFINED" => Ok(MylibraryActionEnum::ACTIONUNDEFINED),
           "bookmark" => Ok(MylibraryActionEnum::Bookmark),
           "chapter" => Ok(MylibraryActionEnum::Chapter),
           "next-page" => Ok(MylibraryActionEnum::NextPage),
           "prev-page" => Ok(MylibraryActionEnum::PrevPage),
           "scroll" => Ok(MylibraryActionEnum::Scroll),
           "search" => Ok(MylibraryActionEnum::Search),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MylibraryActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OnboardingMaxAllowedMaturityRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The maximum allowed maturity rating of returned volumes. Books with a higher maturity rating are filtered out.
pub enum OnboardingMaxAllowedMaturityRatingEnum {
    
    /// "MAX_ALLOWED_MATURITY_RATING_UNDEFINED"
    #[serde(rename="MAX_ALLOWED_MATURITY_RATING_UNDEFINED")]
    MAXALLOWEDMATURITYRATINGUNDEFINED,
    

    /// Show books which are rated mature or lower.
    ///
    /// "MATURE"
    #[serde(rename="MATURE")]
    MATURE,
    

    /// Show books which are rated not mature.
    ///
    /// "not-mature"
    #[serde(rename="not-mature")]
    NotMature,
}

impl AsRef<str> for OnboardingMaxAllowedMaturityRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OnboardingMaxAllowedMaturityRatingEnum::MAXALLOWEDMATURITYRATINGUNDEFINED => "MAX_ALLOWED_MATURITY_RATING_UNDEFINED",
            OnboardingMaxAllowedMaturityRatingEnum::MATURE => "MATURE",
            OnboardingMaxAllowedMaturityRatingEnum::NotMature => "not-mature",
        }
    }
}

impl std::convert::TryFrom< &str> for OnboardingMaxAllowedMaturityRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MAX_ALLOWED_MATURITY_RATING_UNDEFINED" => Ok(OnboardingMaxAllowedMaturityRatingEnum::MAXALLOWEDMATURITYRATINGUNDEFINED),
           "MATURE" => Ok(OnboardingMaxAllowedMaturityRatingEnum::MATURE),
           "not-mature" => Ok(OnboardingMaxAllowedMaturityRatingEnum::NotMature),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OnboardingMaxAllowedMaturityRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonalizedstreamMaxAllowedMaturityRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.
pub enum PersonalizedstreamMaxAllowedMaturityRatingEnum {
    
    /// "MAX_ALLOWED_MATURITY_RATING_UNDEFINED"
    #[serde(rename="MAX_ALLOWED_MATURITY_RATING_UNDEFINED")]
    MAXALLOWEDMATURITYRATINGUNDEFINED,
    

    /// Show books which are rated mature or lower.
    ///
    /// "MATURE"
    #[serde(rename="MATURE")]
    MATURE,
    

    /// Show books which are rated not mature.
    ///
    /// "not-mature"
    #[serde(rename="not-mature")]
    NotMature,
}

impl AsRef<str> for PersonalizedstreamMaxAllowedMaturityRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonalizedstreamMaxAllowedMaturityRatingEnum::MAXALLOWEDMATURITYRATINGUNDEFINED => "MAX_ALLOWED_MATURITY_RATING_UNDEFINED",
            PersonalizedstreamMaxAllowedMaturityRatingEnum::MATURE => "MATURE",
            PersonalizedstreamMaxAllowedMaturityRatingEnum::NotMature => "not-mature",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonalizedstreamMaxAllowedMaturityRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MAX_ALLOWED_MATURITY_RATING_UNDEFINED" => Ok(PersonalizedstreamMaxAllowedMaturityRatingEnum::MAXALLOWEDMATURITYRATINGUNDEFINED),
           "MATURE" => Ok(PersonalizedstreamMaxAllowedMaturityRatingEnum::MATURE),
           "not-mature" => Ok(PersonalizedstreamMaxAllowedMaturityRatingEnum::NotMature),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonalizedstreamMaxAllowedMaturityRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeAssociationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Association type.
pub enum VolumeAssociationEnum {
    
    /// "ASSOCIATION_UNDEFINED"
    #[serde(rename="ASSOCIATION_UNDEFINED")]
    ASSOCIATIONUNDEFINED,
    

    /// Recommendations for display end-of-sample.
    ///
    /// "end-of-sample"
    #[serde(rename="end-of-sample")]
    EndOfSample,
    

    /// Recommendations for display end-of-volume.
    ///
    /// "end-of-volume"
    #[serde(rename="end-of-volume")]
    EndOfVolume,
    

    /// Related volumes for Play Store.
    ///
    /// "related-for-play"
    #[serde(rename="related-for-play")]
    RelatedForPlay,
}

impl AsRef<str> for VolumeAssociationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeAssociationEnum::ASSOCIATIONUNDEFINED => "ASSOCIATION_UNDEFINED",
            VolumeAssociationEnum::EndOfSample => "end-of-sample",
            VolumeAssociationEnum::EndOfVolume => "end-of-volume",
            VolumeAssociationEnum::RelatedForPlay => "related-for-play",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeAssociationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSOCIATION_UNDEFINED" => Ok(VolumeAssociationEnum::ASSOCIATIONUNDEFINED),
           "end-of-sample" => Ok(VolumeAssociationEnum::EndOfSample),
           "end-of-volume" => Ok(VolumeAssociationEnum::EndOfVolume),
           "related-for-play" => Ok(VolumeAssociationEnum::RelatedForPlay),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeAssociationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeMaxAllowedMaturityRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.
pub enum VolumeMaxAllowedMaturityRatingEnum {
    
    /// "MAX_ALLOWED_MATURITY_RATING_UNDEFINED"
    #[serde(rename="MAX_ALLOWED_MATURITY_RATING_UNDEFINED")]
    MAXALLOWEDMATURITYRATINGUNDEFINED,
    

    /// Show books which are rated mature or lower.
    ///
    /// "MATURE"
    #[serde(rename="MATURE")]
    MATURE,
    

    /// Show books which are rated not mature.
    ///
    /// "not-mature"
    #[serde(rename="not-mature")]
    NotMature,
}

impl AsRef<str> for VolumeMaxAllowedMaturityRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeMaxAllowedMaturityRatingEnum::MAXALLOWEDMATURITYRATINGUNDEFINED => "MAX_ALLOWED_MATURITY_RATING_UNDEFINED",
            VolumeMaxAllowedMaturityRatingEnum::MATURE => "MATURE",
            VolumeMaxAllowedMaturityRatingEnum::NotMature => "not-mature",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeMaxAllowedMaturityRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MAX_ALLOWED_MATURITY_RATING_UNDEFINED" => Ok(VolumeMaxAllowedMaturityRatingEnum::MAXALLOWEDMATURITYRATINGUNDEFINED),
           "MATURE" => Ok(VolumeMaxAllowedMaturityRatingEnum::MATURE),
           "not-mature" => Ok(VolumeMaxAllowedMaturityRatingEnum::NotMature),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeMaxAllowedMaturityRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeAcquireMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the book was acquired
pub enum VolumeAcquireMethodEnum {
    
    /// "ACQUIRE_METHOD_UNDEFINED"
    #[serde(rename="ACQUIRE_METHOD_UNDEFINED")]
    ACQUIREMETHODUNDEFINED,
    

    /// Books acquired via Family Sharing
    ///
    /// "FAMILY_SHARED"
    #[serde(rename="FAMILY_SHARED")]
    FAMILYSHARED,
    

    /// Preordered books (not yet available)
    ///
    /// "PREORDERED"
    #[serde(rename="PREORDERED")]
    PREORDERED,
    

    /// User-rented books past their expiration time
    ///
    /// "PREVIOUSLY_RENTED"
    #[serde(rename="PREVIOUSLY_RENTED")]
    PREVIOUSLYRENTED,
    

    /// Public domain books
    ///
    /// "PUBLIC_DOMAIN"
    #[serde(rename="PUBLIC_DOMAIN")]
    PUBLICDOMAIN,
    

    /// Purchased books
    ///
    /// "PURCHASED"
    #[serde(rename="PURCHASED")]
    PURCHASED,
    

    /// User-rented books
    ///
    /// "RENTED"
    #[serde(rename="RENTED")]
    RENTED,
    

    /// Sample books
    ///
    /// "SAMPLE"
    #[serde(rename="SAMPLE")]
    SAMPLE,
    

    /// User uploaded books
    ///
    /// "UPLOADED"
    #[serde(rename="UPLOADED")]
    UPLOADED,
}

impl AsRef<str> for VolumeAcquireMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeAcquireMethodEnum::ACQUIREMETHODUNDEFINED => "ACQUIRE_METHOD_UNDEFINED",
            VolumeAcquireMethodEnum::FAMILYSHARED => "FAMILY_SHARED",
            VolumeAcquireMethodEnum::PREORDERED => "PREORDERED",
            VolumeAcquireMethodEnum::PREVIOUSLYRENTED => "PREVIOUSLY_RENTED",
            VolumeAcquireMethodEnum::PUBLICDOMAIN => "PUBLIC_DOMAIN",
            VolumeAcquireMethodEnum::PURCHASED => "PURCHASED",
            VolumeAcquireMethodEnum::RENTED => "RENTED",
            VolumeAcquireMethodEnum::SAMPLE => "SAMPLE",
            VolumeAcquireMethodEnum::UPLOADED => "UPLOADED",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeAcquireMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACQUIRE_METHOD_UNDEFINED" => Ok(VolumeAcquireMethodEnum::ACQUIREMETHODUNDEFINED),
           "FAMILY_SHARED" => Ok(VolumeAcquireMethodEnum::FAMILYSHARED),
           "PREORDERED" => Ok(VolumeAcquireMethodEnum::PREORDERED),
           "PREVIOUSLY_RENTED" => Ok(VolumeAcquireMethodEnum::PREVIOUSLYRENTED),
           "PUBLIC_DOMAIN" => Ok(VolumeAcquireMethodEnum::PUBLICDOMAIN),
           "PURCHASED" => Ok(VolumeAcquireMethodEnum::PURCHASED),
           "RENTED" => Ok(VolumeAcquireMethodEnum::RENTED),
           "SAMPLE" => Ok(VolumeAcquireMethodEnum::SAMPLE),
           "UPLOADED" => Ok(VolumeAcquireMethodEnum::UPLOADED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeAcquireMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeProcessingStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The processing state of the user uploaded volumes to be returned.
pub enum VolumeProcessingStateEnum {
    
    /// "PROCESSING_STATE_UNDEFINED"
    #[serde(rename="PROCESSING_STATE_UNDEFINED")]
    PROCESSINGSTATEUNDEFINED,
    

    /// The volume processing hase failed.
    ///
    /// "COMPLETED_FAILED"
    #[serde(rename="COMPLETED_FAILED")]
    COMPLETEDFAILED,
    

    /// The volume processing was completed.
    ///
    /// "COMPLETED_SUCCESS"
    #[serde(rename="COMPLETED_SUCCESS")]
    COMPLETEDSUCCESS,
    

    /// The volume processing is not completed.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
}

impl AsRef<str> for VolumeProcessingStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeProcessingStateEnum::PROCESSINGSTATEUNDEFINED => "PROCESSING_STATE_UNDEFINED",
            VolumeProcessingStateEnum::COMPLETEDFAILED => "COMPLETED_FAILED",
            VolumeProcessingStateEnum::COMPLETEDSUCCESS => "COMPLETED_SUCCESS",
            VolumeProcessingStateEnum::RUNNING => "RUNNING",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeProcessingStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROCESSING_STATE_UNDEFINED" => Ok(VolumeProcessingStateEnum::PROCESSINGSTATEUNDEFINED),
           "COMPLETED_FAILED" => Ok(VolumeProcessingStateEnum::COMPLETEDFAILED),
           "COMPLETED_SUCCESS" => Ok(VolumeProcessingStateEnum::COMPLETEDSUCCESS),
           "RUNNING" => Ok(VolumeProcessingStateEnum::RUNNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeProcessingStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rating to be given to the volume.
pub enum VolumeRatingEnum {
    
    /// "RATING_UNDEFINED"
    #[serde(rename="RATING_UNDEFINED")]
    RATINGUNDEFINED,
    

    /// Rating indicating a dismissal due to ownership.
    ///
    /// "HAVE_IT"
    #[serde(rename="HAVE_IT")]
    HAVEIT,
    

    /// Rating indicating a negative dismissal of a volume.
    ///
    /// "NOT_INTERESTED"
    #[serde(rename="NOT_INTERESTED")]
    NOTINTERESTED,
}

impl AsRef<str> for VolumeRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeRatingEnum::RATINGUNDEFINED => "RATING_UNDEFINED",
            VolumeRatingEnum::HAVEIT => "HAVE_IT",
            VolumeRatingEnum::NOTINTERESTED => "NOT_INTERESTED",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RATING_UNDEFINED" => Ok(VolumeRatingEnum::RATINGUNDEFINED),
           "HAVE_IT" => Ok(VolumeRatingEnum::HAVEIT),
           "NOT_INTERESTED" => Ok(VolumeRatingEnum::NOTINTERESTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeProjectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restrict information returned to a set of selected fields.
pub enum VolumeProjectionEnum {
    
    /// "PROJECTION_UNDEFINED"
    #[serde(rename="PROJECTION_UNDEFINED")]
    PROJECTIONUNDEFINED,
    

    /// Includes all volume data.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Includes a subset of fields in volumeInfo and accessInfo.
    ///
    /// "LITE"
    #[serde(rename="LITE")]
    LITE,
}

impl AsRef<str> for VolumeProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeProjectionEnum::PROJECTIONUNDEFINED => "PROJECTION_UNDEFINED",
            VolumeProjectionEnum::FULL => "FULL",
            VolumeProjectionEnum::LITE => "LITE",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeProjectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROJECTION_UNDEFINED" => Ok(VolumeProjectionEnum::PROJECTIONUNDEFINED),
           "FULL" => Ok(VolumeProjectionEnum::FULL),
           "LITE" => Ok(VolumeProjectionEnum::LITE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeDownloadEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restrict to volumes by download availability.
pub enum VolumeDownloadEnum {
    
    /// "DOWNLOAD_UNDEFINED"
    #[serde(rename="DOWNLOAD_UNDEFINED")]
    DOWNLOADUNDEFINED,
    

    /// All volumes with epub.
    ///
    /// "EPUB"
    #[serde(rename="EPUB")]
    EPUB,
}

impl AsRef<str> for VolumeDownloadEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeDownloadEnum::DOWNLOADUNDEFINED => "DOWNLOAD_UNDEFINED",
            VolumeDownloadEnum::EPUB => "EPUB",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeDownloadEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DOWNLOAD_UNDEFINED" => Ok(VolumeDownloadEnum::DOWNLOADUNDEFINED),
           "EPUB" => Ok(VolumeDownloadEnum::EPUB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeDownloadEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeFilterEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter search results.
pub enum VolumeFilterEnum {
    
    /// "FILTER_UNDEFINED"
    #[serde(rename="FILTER_UNDEFINED")]
    FILTERUNDEFINED,
    

    /// All Google eBooks.
    ///
    /// "ebooks"
    #[serde(rename="ebooks")]
    Ebooks,
    

    /// Google eBook with full volume text viewability.
    ///
    /// "free-ebooks"
    #[serde(rename="free-ebooks")]
    FreeEbooks,
    

    /// Public can view entire volume text.
    ///
    /// "full"
    #[serde(rename="full")]
    Full,
    

    /// Google eBook with a price.
    ///
    /// "paid-ebooks"
    #[serde(rename="paid-ebooks")]
    PaidEbooks,
    

    /// Public able to see parts of text.
    ///
    /// "partial"
    #[serde(rename="partial")]
    Partial,
}

impl AsRef<str> for VolumeFilterEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeFilterEnum::FILTERUNDEFINED => "FILTER_UNDEFINED",
            VolumeFilterEnum::Ebooks => "ebooks",
            VolumeFilterEnum::FreeEbooks => "free-ebooks",
            VolumeFilterEnum::Full => "full",
            VolumeFilterEnum::PaidEbooks => "paid-ebooks",
            VolumeFilterEnum::Partial => "partial",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeFilterEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILTER_UNDEFINED" => Ok(VolumeFilterEnum::FILTERUNDEFINED),
           "ebooks" => Ok(VolumeFilterEnum::Ebooks),
           "free-ebooks" => Ok(VolumeFilterEnum::FreeEbooks),
           "full" => Ok(VolumeFilterEnum::Full),
           "paid-ebooks" => Ok(VolumeFilterEnum::PaidEbooks),
           "partial" => Ok(VolumeFilterEnum::Partial),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeFilterEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeLibraryRestrictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restrict search to this user's library.
pub enum VolumeLibraryRestrictEnum {
    
    /// "LIBRARY_RESTRICT_UNDEFINED"
    #[serde(rename="LIBRARY_RESTRICT_UNDEFINED")]
    LIBRARYRESTRICTUNDEFINED,
    

    /// Restrict to the user's library, any shelf.
    ///
    /// "my-library"
    #[serde(rename="my-library")]
    MyLibrary,
    

    /// Do not restrict based on user's library.
    ///
    /// "no-restrict"
    #[serde(rename="no-restrict")]
    NoRestrict,
}

impl AsRef<str> for VolumeLibraryRestrictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeLibraryRestrictEnum::LIBRARYRESTRICTUNDEFINED => "LIBRARY_RESTRICT_UNDEFINED",
            VolumeLibraryRestrictEnum::MyLibrary => "my-library",
            VolumeLibraryRestrictEnum::NoRestrict => "no-restrict",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeLibraryRestrictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIBRARY_RESTRICT_UNDEFINED" => Ok(VolumeLibraryRestrictEnum::LIBRARYRESTRICTUNDEFINED),
           "my-library" => Ok(VolumeLibraryRestrictEnum::MyLibrary),
           "no-restrict" => Ok(VolumeLibraryRestrictEnum::NoRestrict),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeLibraryRestrictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeOrderByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sort search results.
pub enum VolumeOrderByEnum {
    
    /// "ORDER_BY_UNDEFINED"
    #[serde(rename="ORDER_BY_UNDEFINED")]
    ORDERBYUNDEFINED,
    

    /// Most recently published.
    ///
    /// "newest"
    #[serde(rename="newest")]
    Newest,
    

    /// Relevance to search terms.
    ///
    /// "relevance"
    #[serde(rename="relevance")]
    Relevance,
}

impl AsRef<str> for VolumeOrderByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeOrderByEnum::ORDERBYUNDEFINED => "ORDER_BY_UNDEFINED",
            VolumeOrderByEnum::Newest => "newest",
            VolumeOrderByEnum::Relevance => "relevance",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeOrderByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORDER_BY_UNDEFINED" => Ok(VolumeOrderByEnum::ORDERBYUNDEFINED),
           "newest" => Ok(VolumeOrderByEnum::Newest),
           "relevance" => Ok(VolumeOrderByEnum::Relevance),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeOrderByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumePrintTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restrict to books or magazines.
pub enum VolumePrintTypeEnum {
    
    /// "PRINT_TYPE_UNDEFINED"
    #[serde(rename="PRINT_TYPE_UNDEFINED")]
    PRINTTYPEUNDEFINED,
    

    /// All volume content types.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Just books.
    ///
    /// "BOOKS"
    #[serde(rename="BOOKS")]
    BOOKS,
    

    /// Just magazines.
    ///
    /// "MAGAZINES"
    #[serde(rename="MAGAZINES")]
    MAGAZINES,
}

impl AsRef<str> for VolumePrintTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumePrintTypeEnum::PRINTTYPEUNDEFINED => "PRINT_TYPE_UNDEFINED",
            VolumePrintTypeEnum::ALL => "ALL",
            VolumePrintTypeEnum::BOOKS => "BOOKS",
            VolumePrintTypeEnum::MAGAZINES => "MAGAZINES",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumePrintTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRINT_TYPE_UNDEFINED" => Ok(VolumePrintTypeEnum::PRINTTYPEUNDEFINED),
           "ALL" => Ok(VolumePrintTypeEnum::ALL),
           "BOOKS" => Ok(VolumePrintTypeEnum::BOOKS),
           "MAGAZINES" => Ok(VolumePrintTypeEnum::MAGAZINES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumePrintTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


