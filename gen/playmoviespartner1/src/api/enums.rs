use super::*;



// region OrderStatusDetailEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Detailed status of the order
pub enum OrderStatusDetailEnum {
    

    /// Value could not be determined, please contact technical support if
it should.
    ///
    /// "ORDER_STATUS_UNSPECIFIED"
    #[serde(rename="ORDER_STATUS_UNSPECIFIED")]
    ORDERSTATUSUNSPECIFIED,
    

    /// Approved by Google's Quality Control team.
    ///
    /// "ORDER_STATUS_QC_APPROVED"
    #[serde(rename="ORDER_STATUS_QC_APPROVED")]
    ORDERSTATUSQCAPPROVED,
    

    /// Rejected by Google's Quality Control team, pending partner redelivery.
    ///
    /// "ORDER_STATUS_QC_REJECTION"
    #[serde(rename="ORDER_STATUS_QC_REJECTION")]
    ORDERSTATUSQCREJECTION,
    

    /// Internal error while processing the Order.
    ///
    /// "ORDER_STATUS_INTERNAL_FIX"
    #[serde(rename="ORDER_STATUS_INTERNAL_FIX")]
    ORDERSTATUSINTERNALFIX,
    

    /// Waiting for initial delivery from partner.
    ///
    /// "ORDER_STATUS_OPEN_ORDER"
    #[serde(rename="ORDER_STATUS_OPEN_ORDER")]
    ORDERSTATUSOPENORDER,
    

    /// Used on Orders that do not have Status, like TV Seasons.
    ///
    /// "ORDER_STATUS_NOT_AVAILABLE"
    #[serde(rename="ORDER_STATUS_NOT_AVAILABLE")]
    ORDERSTATUSNOTAVAILABLE,
    

    /// Waiting for re-delivery from partner.
    ///
    /// "ORDER_STATUS_AWAITING_REDELIVERY"
    #[serde(rename="ORDER_STATUS_AWAITING_REDELIVERY")]
    ORDERSTATUSAWAITINGREDELIVERY,
    

    /// Asset was delivered by partner, but is being reviewed by Google's
Quality Control team.
    ///
    /// "ORDER_STATUS_READY_FOR_QC"
    #[serde(rename="ORDER_STATUS_READY_FOR_QC")]
    ORDERSTATUSREADYFORQC,
    

    /// Waiting for Google to process the asset.
    ///
    /// "ORDER_STATUS_FILE_PROCESSING"
    #[serde(rename="ORDER_STATUS_FILE_PROCESSING")]
    ORDERSTATUSFILEPROCESSING,
}

impl AsRef<str> for OrderStatusDetailEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderStatusDetailEnum::ORDERSTATUSUNSPECIFIED => "ORDER_STATUS_UNSPECIFIED",
            OrderStatusDetailEnum::ORDERSTATUSQCAPPROVED => "ORDER_STATUS_QC_APPROVED",
            OrderStatusDetailEnum::ORDERSTATUSQCREJECTION => "ORDER_STATUS_QC_REJECTION",
            OrderStatusDetailEnum::ORDERSTATUSINTERNALFIX => "ORDER_STATUS_INTERNAL_FIX",
            OrderStatusDetailEnum::ORDERSTATUSOPENORDER => "ORDER_STATUS_OPEN_ORDER",
            OrderStatusDetailEnum::ORDERSTATUSNOTAVAILABLE => "ORDER_STATUS_NOT_AVAILABLE",
            OrderStatusDetailEnum::ORDERSTATUSAWAITINGREDELIVERY => "ORDER_STATUS_AWAITING_REDELIVERY",
            OrderStatusDetailEnum::ORDERSTATUSREADYFORQC => "ORDER_STATUS_READY_FOR_QC",
            OrderStatusDetailEnum::ORDERSTATUSFILEPROCESSING => "ORDER_STATUS_FILE_PROCESSING",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderStatusDetailEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORDER_STATUS_UNSPECIFIED" => Ok(OrderStatusDetailEnum::ORDERSTATUSUNSPECIFIED),
           "ORDER_STATUS_QC_APPROVED" => Ok(OrderStatusDetailEnum::ORDERSTATUSQCAPPROVED),
           "ORDER_STATUS_QC_REJECTION" => Ok(OrderStatusDetailEnum::ORDERSTATUSQCREJECTION),
           "ORDER_STATUS_INTERNAL_FIX" => Ok(OrderStatusDetailEnum::ORDERSTATUSINTERNALFIX),
           "ORDER_STATUS_OPEN_ORDER" => Ok(OrderStatusDetailEnum::ORDERSTATUSOPENORDER),
           "ORDER_STATUS_NOT_AVAILABLE" => Ok(OrderStatusDetailEnum::ORDERSTATUSNOTAVAILABLE),
           "ORDER_STATUS_AWAITING_REDELIVERY" => Ok(OrderStatusDetailEnum::ORDERSTATUSAWAITINGREDELIVERY),
           "ORDER_STATUS_READY_FOR_QC" => Ok(OrderStatusDetailEnum::ORDERSTATUSREADYFORQC),
           "ORDER_STATUS_FILE_PROCESSING" => Ok(OrderStatusDetailEnum::ORDERSTATUSFILEPROCESSING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderStatusDetailEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// High-level status of the order.
pub enum OrderStatusEnum {
    

    /// Value could not be determined, please contact technical support if
it should.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// Approved by Google.
    ///
    /// "STATUS_APPROVED"
    #[serde(rename="STATUS_APPROVED")]
    STATUSAPPROVED,
    

    /// Waiting for partner to re-deliver the asset after a rejection by Google.
    ///
    /// "STATUS_FAILED"
    #[serde(rename="STATUS_FAILED")]
    STATUSFAILED,
    

    /// Waiting for Google to process the asset.
    ///
    /// "STATUS_PROCESSING"
    #[serde(rename="STATUS_PROCESSING")]
    STATUSPROCESSING,
    

    /// Waiting for partner to deliver the asset.
    ///
    /// "STATUS_UNFULFILLED"
    #[serde(rename="STATUS_UNFULFILLED")]
    STATUSUNFULFILLED,
    

    /// Used when Status is not available (i.e: Orders for TV Seasons).
    ///
    /// "STATUS_NOT_AVAILABLE"
    #[serde(rename="STATUS_NOT_AVAILABLE")]
    STATUSNOTAVAILABLE,
}

impl AsRef<str> for OrderStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            OrderStatusEnum::STATUSAPPROVED => "STATUS_APPROVED",
            OrderStatusEnum::STATUSFAILED => "STATUS_FAILED",
            OrderStatusEnum::STATUSPROCESSING => "STATUS_PROCESSING",
            OrderStatusEnum::STATUSUNFULFILLED => "STATUS_UNFULFILLED",
            OrderStatusEnum::STATUSNOTAVAILABLE => "STATUS_NOT_AVAILABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(OrderStatusEnum::STATUSUNSPECIFIED),
           "STATUS_APPROVED" => Ok(OrderStatusEnum::STATUSAPPROVED),
           "STATUS_FAILED" => Ok(OrderStatusEnum::STATUSFAILED),
           "STATUS_PROCESSING" => Ok(OrderStatusEnum::STATUSPROCESSING),
           "STATUS_UNFULFILLED" => Ok(OrderStatusEnum::STATUSUNFULFILLED),
           "STATUS_NOT_AVAILABLE" => Ok(OrderStatusEnum::STATUSNOTAVAILABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderNormalizedPriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A simpler representation of the priority.
pub enum OrderNormalizedPriorityEnum {
    

    /// Value could not be determined, please contact technical support if
it should.
    ///
    /// "NORMALIZED_PRIORITY_UNSPECIFIED"
    #[serde(rename="NORMALIZED_PRIORITY_UNSPECIFIED")]
    NORMALIZEDPRIORITYUNSPECIFIED,
    

    /// A low-priority asset, typically from a library movie.
    ///
    /// "LOW_PRIORITY"
    #[serde(rename="LOW_PRIORITY")]
    LOWPRIORITY,
    

    /// A high-priority asset, typically from a new release or box office hit.
    ///
    /// "HIGH_PRIORITY"
    #[serde(rename="HIGH_PRIORITY")]
    HIGHPRIORITY,
}

impl AsRef<str> for OrderNormalizedPriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderNormalizedPriorityEnum::NORMALIZEDPRIORITYUNSPECIFIED => "NORMALIZED_PRIORITY_UNSPECIFIED",
            OrderNormalizedPriorityEnum::LOWPRIORITY => "LOW_PRIORITY",
            OrderNormalizedPriorityEnum::HIGHPRIORITY => "HIGH_PRIORITY",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderNormalizedPriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NORMALIZED_PRIORITY_UNSPECIFIED" => Ok(OrderNormalizedPriorityEnum::NORMALIZEDPRIORITYUNSPECIFIED),
           "LOW_PRIORITY" => Ok(OrderNormalizedPriorityEnum::LOWPRIORITY),
           "HIGH_PRIORITY" => Ok(OrderNormalizedPriorityEnum::HIGHPRIORITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderNormalizedPriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the Edit linked to the Order.
pub enum OrderTypeEnum {
    

    /// Value could not be determined, please contact technical support if
it should.
    ///
    /// "TITLE_TYPE_UNSPECIFIED"
    #[serde(rename="TITLE_TYPE_UNSPECIFIED")]
    TITLETYPEUNSPECIFIED,
    

    /// A movie picture.
    ///
    /// "MOVIE"
    #[serde(rename="MOVIE")]
    MOVIE,
    

    /// A season of a TV show.
    ///
    /// "SEASON"
    #[serde(rename="SEASON")]
    SEASON,
    

    /// An episode of a TV show.
    ///
    /// "EPISODE"
    #[serde(rename="EPISODE")]
    EPISODE,
    

    /// A collection of movies, i.e. "Googlers 1 and Googlers, the return"
    ///
    /// "BUNDLE"
    #[serde(rename="BUNDLE")]
    BUNDLE,
}

impl AsRef<str> for OrderTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderTypeEnum::TITLETYPEUNSPECIFIED => "TITLE_TYPE_UNSPECIFIED",
            OrderTypeEnum::MOVIE => "MOVIE",
            OrderTypeEnum::SEASON => "SEASON",
            OrderTypeEnum::EPISODE => "EPISODE",
            OrderTypeEnum::BUNDLE => "BUNDLE",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TITLE_TYPE_UNSPECIFIED" => Ok(OrderTypeEnum::TITLETYPEUNSPECIFIED),
           "MOVIE" => Ok(OrderTypeEnum::MOVIE),
           "SEASON" => Ok(OrderTypeEnum::SEASON),
           "EPISODE" => Ok(OrderTypeEnum::EPISODE),
           "BUNDLE" => Ok(OrderTypeEnum::BUNDLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StoreInfoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Edit type, like Movie, Episode or Season.
pub enum StoreInfoTypeEnum {
    

    /// Value could not be determined, please contact technical support if
it should.
    ///
    /// "TITLE_TYPE_UNSPECIFIED"
    #[serde(rename="TITLE_TYPE_UNSPECIFIED")]
    TITLETYPEUNSPECIFIED,
    

    /// A movie picture.
    ///
    /// "MOVIE"
    #[serde(rename="MOVIE")]
    MOVIE,
    

    /// A season of a TV show.
    ///
    /// "SEASON"
    #[serde(rename="SEASON")]
    SEASON,
    

    /// An episode of a TV show.
    ///
    /// "EPISODE"
    #[serde(rename="EPISODE")]
    EPISODE,
    

    /// A collection of movies, i.e. "Googlers 1 and Googlers, the return"
    ///
    /// "BUNDLE"
    #[serde(rename="BUNDLE")]
    BUNDLE,
}

impl AsRef<str> for StoreInfoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StoreInfoTypeEnum::TITLETYPEUNSPECIFIED => "TITLE_TYPE_UNSPECIFIED",
            StoreInfoTypeEnum::MOVIE => "MOVIE",
            StoreInfoTypeEnum::SEASON => "SEASON",
            StoreInfoTypeEnum::EPISODE => "EPISODE",
            StoreInfoTypeEnum::BUNDLE => "BUNDLE",
        }
    }
}

impl std::convert::TryFrom< &str> for StoreInfoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TITLE_TYPE_UNSPECIFIED" => Ok(StoreInfoTypeEnum::TITLETYPEUNSPECIFIED),
           "MOVIE" => Ok(StoreInfoTypeEnum::MOVIE),
           "SEASON" => Ok(StoreInfoTypeEnum::SEASON),
           "EPISODE" => Ok(StoreInfoTypeEnum::EPISODE),
           "BUNDLE" => Ok(StoreInfoTypeEnum::BUNDLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StoreInfoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AvailFormatProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the format profile covered by the transaction.
pub enum AvailFormatProfileEnum {
    

    /// Value could not be determined, please contact technical support if
it should.
    ///
    /// "FORMAT_PROFILE_UNSPECIFIED"
    #[serde(rename="FORMAT_PROFILE_UNSPECIFIED")]
    FORMATPROFILEUNSPECIFIED,
    

    /// Standard-definition format.
    ///
    /// "SD"
    #[serde(rename="SD")]
    SD,
    

    /// High-definition format.
    ///
    /// "HD"
    #[serde(rename="HD")]
    HD,
    

    /// 4K UHD.
    ///
    /// "UHD"
    #[serde(rename="UHD")]
    UHD,
}

impl AsRef<str> for AvailFormatProfileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AvailFormatProfileEnum::FORMATPROFILEUNSPECIFIED => "FORMAT_PROFILE_UNSPECIFIED",
            AvailFormatProfileEnum::SD => "SD",
            AvailFormatProfileEnum::HD => "HD",
            AvailFormatProfileEnum::UHD => "UHD",
        }
    }
}

impl std::convert::TryFrom< &str> for AvailFormatProfileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMAT_PROFILE_UNSPECIFIED" => Ok(AvailFormatProfileEnum::FORMATPROFILEUNSPECIFIED),
           "SD" => Ok(AvailFormatProfileEnum::SD),
           "HD" => Ok(AvailFormatProfileEnum::HD),
           "UHD" => Ok(AvailFormatProfileEnum::UHD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AvailFormatProfileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AvailWorkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Work type as enumerated in EMA.
pub enum AvailWorkTypeEnum {
    

    /// Value could not be determined, please contact technical support if
it should.
    ///
    /// "TITLE_TYPE_UNSPECIFIED"
    #[serde(rename="TITLE_TYPE_UNSPECIFIED")]
    TITLETYPEUNSPECIFIED,
    

    /// A movie picture.
    ///
    /// "MOVIE"
    #[serde(rename="MOVIE")]
    MOVIE,
    

    /// A season of a TV show.
    ///
    /// "SEASON"
    #[serde(rename="SEASON")]
    SEASON,
    

    /// An episode of a TV show.
    ///
    /// "EPISODE"
    #[serde(rename="EPISODE")]
    EPISODE,
    

    /// A collection of movies, i.e. "Googlers 1 and Googlers, the return"
    ///
    /// "BUNDLE"
    #[serde(rename="BUNDLE")]
    BUNDLE,
}

impl AsRef<str> for AvailWorkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AvailWorkTypeEnum::TITLETYPEUNSPECIFIED => "TITLE_TYPE_UNSPECIFIED",
            AvailWorkTypeEnum::MOVIE => "MOVIE",
            AvailWorkTypeEnum::SEASON => "SEASON",
            AvailWorkTypeEnum::EPISODE => "EPISODE",
            AvailWorkTypeEnum::BUNDLE => "BUNDLE",
        }
    }
}

impl std::convert::TryFrom< &str> for AvailWorkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TITLE_TYPE_UNSPECIFIED" => Ok(AvailWorkTypeEnum::TITLETYPEUNSPECIFIED),
           "MOVIE" => Ok(AvailWorkTypeEnum::MOVIE),
           "SEASON" => Ok(AvailWorkTypeEnum::SEASON),
           "EPISODE" => Ok(AvailWorkTypeEnum::EPISODE),
           "BUNDLE" => Ok(AvailWorkTypeEnum::BUNDLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AvailWorkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AvailLicenseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of transaction.
pub enum AvailLicenseTypeEnum {
    

    /// Value could not be determined, please contact technical support if
it should.
    ///
    /// "LICENSE_TYPE_UNSPECIFIED"
    #[serde(rename="LICENSE_TYPE_UNSPECIFIED")]
    LICENSETYPEUNSPECIFIED,
    

    /// Electronic Sell Through - purchase policy for unlimited viewing.
    ///
    /// "EST"
    #[serde(rename="EST")]
    EST,
    

    /// Video On Demand - rental policy for temporary viewing.
    ///
    /// "VOD"
    #[serde(rename="VOD")]
    VOD,
    

    /// Subscription Video On Demand - used for subscription platforms.
Not supported on Google Play.
    ///
    /// "SVOD"
    #[serde(rename="SVOD")]
    SVOD,
    

    /// Pre-order Electronic Sell Through - pre-order purchase only window.
    ///
    /// "POEST"
    #[serde(rename="POEST")]
    POEST,
}

impl AsRef<str> for AvailLicenseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AvailLicenseTypeEnum::LICENSETYPEUNSPECIFIED => "LICENSE_TYPE_UNSPECIFIED",
            AvailLicenseTypeEnum::EST => "EST",
            AvailLicenseTypeEnum::VOD => "VOD",
            AvailLicenseTypeEnum::SVOD => "SVOD",
            AvailLicenseTypeEnum::POEST => "POEST",
        }
    }
}

impl std::convert::TryFrom< &str> for AvailLicenseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LICENSE_TYPE_UNSPECIFIED" => Ok(AvailLicenseTypeEnum::LICENSETYPEUNSPECIFIED),
           "EST" => Ok(AvailLicenseTypeEnum::EST),
           "VOD" => Ok(AvailLicenseTypeEnum::VOD),
           "SVOD" => Ok(AvailLicenseTypeEnum::SVOD),
           "POEST" => Ok(AvailLicenseTypeEnum::POEST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AvailLicenseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter Orders that match one of the given status.
pub enum AccountStatusEnum {
    

    /// no description found
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// no description found
    ///
    /// "STATUS_APPROVED"
    #[serde(rename="STATUS_APPROVED")]
    STATUSAPPROVED,
    

    /// no description found
    ///
    /// "STATUS_FAILED"
    #[serde(rename="STATUS_FAILED")]
    STATUSFAILED,
    

    /// no description found
    ///
    /// "STATUS_PROCESSING"
    #[serde(rename="STATUS_PROCESSING")]
    STATUSPROCESSING,
    

    /// no description found
    ///
    /// "STATUS_UNFULFILLED"
    #[serde(rename="STATUS_UNFULFILLED")]
    STATUSUNFULFILLED,
    

    /// no description found
    ///
    /// "STATUS_NOT_AVAILABLE"
    #[serde(rename="STATUS_NOT_AVAILABLE")]
    STATUSNOTAVAILABLE,
}

impl AsRef<str> for AccountStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            AccountStatusEnum::STATUSAPPROVED => "STATUS_APPROVED",
            AccountStatusEnum::STATUSFAILED => "STATUS_FAILED",
            AccountStatusEnum::STATUSPROCESSING => "STATUS_PROCESSING",
            AccountStatusEnum::STATUSUNFULFILLED => "STATUS_UNFULFILLED",
            AccountStatusEnum::STATUSNOTAVAILABLE => "STATUS_NOT_AVAILABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(AccountStatusEnum::STATUSUNSPECIFIED),
           "STATUS_APPROVED" => Ok(AccountStatusEnum::STATUSAPPROVED),
           "STATUS_FAILED" => Ok(AccountStatusEnum::STATUSFAILED),
           "STATUS_PROCESSING" => Ok(AccountStatusEnum::STATUSPROCESSING),
           "STATUS_UNFULFILLED" => Ok(AccountStatusEnum::STATUSUNFULFILLED),
           "STATUS_NOT_AVAILABLE" => Ok(AccountStatusEnum::STATUSNOTAVAILABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


