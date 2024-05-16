use super::*;



// region DestinationStatusStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the destination.
pub enum DestinationStatusStatusEnum {
    

    /// Unspecified status, never used.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The product is used for this destination.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The decision is still pending.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The product is disapproved. Please look at the issues.
    ///
    /// "DISAPPROVED"
    #[serde(rename="DISAPPROVED")]
    DISAPPROVED,
}

impl AsRef<str> for DestinationStatusStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DestinationStatusStatusEnum::UNKNOWN => "UNKNOWN",
            DestinationStatusStatusEnum::ACTIVE => "ACTIVE",
            DestinationStatusStatusEnum::PENDING => "PENDING",
            DestinationStatusStatusEnum::DISAPPROVED => "DISAPPROVED",
        }
    }
}

impl std::convert::TryFrom< &str> for DestinationStatusStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(DestinationStatusStatusEnum::UNKNOWN),
           "ACTIVE" => Ok(DestinationStatusStatusEnum::ACTIVE),
           "PENDING" => Ok(DestinationStatusStatusEnum::PENDING),
           "DISAPPROVED" => Ok(DestinationStatusStatusEnum::DISAPPROVED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DestinationStatusStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImageStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the image. @OutputOnly
pub enum ImageStatusEnum {
    

    /// The image status is unspecified. Should not be used.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The image was uploaded and is being processed.
    ///
    /// "PENDING_PROCESSING"
    #[serde(rename="PENDING_PROCESSING")]
    PENDINGPROCESSING,
    

    /// The image crawl is still pending.
    ///
    /// "PENDING_CRAWL"
    #[serde(rename="PENDING_CRAWL")]
    PENDINGCRAWL,
    

    /// The image was processed and it meets the requirements.
    ///
    /// "OK"
    #[serde(rename="OK")]
    OK,
    

    /// The image URL is protected by robots.txt file and cannot be crawled.
    ///
    /// "ROBOTED"
    #[serde(rename="ROBOTED")]
    ROBOTED,
    

    /// The image URL is protected by X-Robots-Tag and cannot be crawled.
    ///
    /// "XROBOTED"
    #[serde(rename="XROBOTED")]
    XROBOTED,
    

    /// There was an error while crawling the image.
    ///
    /// "CRAWL_ERROR"
    #[serde(rename="CRAWL_ERROR")]
    CRAWLERROR,
    

    /// The image cannot be processed.
    ///
    /// "PROCESSING_ERROR"
    #[serde(rename="PROCESSING_ERROR")]
    PROCESSINGERROR,
    

    /// The image cannot be decoded.
    ///
    /// "DECODING_ERROR"
    #[serde(rename="DECODING_ERROR")]
    DECODINGERROR,
    

    /// The image is too big.
    ///
    /// "TOO_BIG"
    #[serde(rename="TOO_BIG")]
    TOOBIG,
    

    /// The image was manually overridden and will not be crawled.
    ///
    /// "CRAWL_SKIPPED"
    #[serde(rename="CRAWL_SKIPPED")]
    CRAWLSKIPPED,
    

    /// The image crawl was postponed to avoid overloading the host.
    ///
    /// "HOSTLOADED"
    #[serde(rename="HOSTLOADED")]
    HOSTLOADED,
    

    /// The image URL returned a "404 Not Found" error.
    ///
    /// "HTTP_404"
    #[serde(rename="HTTP_404")]
    HTTP404,
}

impl AsRef<str> for ImageStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImageStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            ImageStatusEnum::PENDINGPROCESSING => "PENDING_PROCESSING",
            ImageStatusEnum::PENDINGCRAWL => "PENDING_CRAWL",
            ImageStatusEnum::OK => "OK",
            ImageStatusEnum::ROBOTED => "ROBOTED",
            ImageStatusEnum::XROBOTED => "XROBOTED",
            ImageStatusEnum::CRAWLERROR => "CRAWL_ERROR",
            ImageStatusEnum::PROCESSINGERROR => "PROCESSING_ERROR",
            ImageStatusEnum::DECODINGERROR => "DECODING_ERROR",
            ImageStatusEnum::TOOBIG => "TOO_BIG",
            ImageStatusEnum::CRAWLSKIPPED => "CRAWL_SKIPPED",
            ImageStatusEnum::HOSTLOADED => "HOSTLOADED",
            ImageStatusEnum::HTTP404 => "HTTP_404",
        }
    }
}

impl std::convert::TryFrom< &str> for ImageStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(ImageStatusEnum::STATUSUNSPECIFIED),
           "PENDING_PROCESSING" => Ok(ImageStatusEnum::PENDINGPROCESSING),
           "PENDING_CRAWL" => Ok(ImageStatusEnum::PENDINGCRAWL),
           "OK" => Ok(ImageStatusEnum::OK),
           "ROBOTED" => Ok(ImageStatusEnum::ROBOTED),
           "XROBOTED" => Ok(ImageStatusEnum::XROBOTED),
           "CRAWL_ERROR" => Ok(ImageStatusEnum::CRAWLERROR),
           "PROCESSING_ERROR" => Ok(ImageStatusEnum::PROCESSINGERROR),
           "DECODING_ERROR" => Ok(ImageStatusEnum::DECODINGERROR),
           "TOO_BIG" => Ok(ImageStatusEnum::TOOBIG),
           "CRAWL_SKIPPED" => Ok(ImageStatusEnum::CRAWLSKIPPED),
           "HOSTLOADED" => Ok(ImageStatusEnum::HOSTLOADED),
           "HTTP_404" => Ok(ImageStatusEnum::HTTP404),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImageStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImageTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the image, i.e., crawled or uploaded. @OutputOnly
pub enum ImageTypeEnum {
    

    /// Type is unspecified. Should not be used.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The image was crawled from a provided URL.
    ///
    /// "CRAWLED"
    #[serde(rename="CRAWLED")]
    CRAWLED,
    

    /// The image was uploaded.
    ///
    /// "UPLOADED"
    #[serde(rename="UPLOADED")]
    UPLOADED,
}

impl AsRef<str> for ImageTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImageTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ImageTypeEnum::CRAWLED => "CRAWLED",
            ImageTypeEnum::UPLOADED => "UPLOADED",
        }
    }
}

impl std::convert::TryFrom< &str> for ImageTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ImageTypeEnum::TYPEUNSPECIFIED),
           "CRAWLED" => Ok(ImageTypeEnum::CRAWLED),
           "UPLOADED" => Ok(ImageTypeEnum::UPLOADED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImageTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IssueResolutionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// What needs to happen to resolve the issue.
pub enum IssueResolutionEnum {
    

    /// Unspecified resolution, never used.
    ///
    /// "RESOLUTION_UNSPECIFIED"
    #[serde(rename="RESOLUTION_UNSPECIFIED")]
    RESOLUTIONUNSPECIFIED,
    

    /// The user who provided the data must act in order to resolve the issue (for example by correcting some data).
    ///
    /// "USER_ACTION"
    #[serde(rename="USER_ACTION")]
    USERACTION,
    

    /// The issue will be resolved automatically (for example image crawl or Google review). No action is required now. Resolution might lead to another issue (for example if crawl fails).
    ///
    /// "PENDING_PROCESSING"
    #[serde(rename="PENDING_PROCESSING")]
    PENDINGPROCESSING,
}

impl AsRef<str> for IssueResolutionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IssueResolutionEnum::RESOLUTIONUNSPECIFIED => "RESOLUTION_UNSPECIFIED",
            IssueResolutionEnum::USERACTION => "USER_ACTION",
            IssueResolutionEnum::PENDINGPROCESSING => "PENDING_PROCESSING",
        }
    }
}

impl std::convert::TryFrom< &str> for IssueResolutionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOLUTION_UNSPECIFIED" => Ok(IssueResolutionEnum::RESOLUTIONUNSPECIFIED),
           "USER_ACTION" => Ok(IssueResolutionEnum::USERACTION),
           "PENDING_PROCESSING" => Ok(IssueResolutionEnum::PENDINGPROCESSING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IssueResolutionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IssueSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity of the issue.
pub enum IssueSeverityEnum {
    

    /// Unspecified severity, never used.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Error severity. The issue prevents the usage of the whole item.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Warning severity. The issue is either one that prevents the usage of the attribute that triggered it or one that will soon prevent the usage of the whole item.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Info severity. The issue is one that doesn't require immediate attention. It is, for example, used to communicate which attributes are still pending review.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
}

impl AsRef<str> for IssueSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IssueSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            IssueSeverityEnum::ERROR => "ERROR",
            IssueSeverityEnum::WARNING => "WARNING",
            IssueSeverityEnum::INFO => "INFO",
        }
    }
}

impl std::convert::TryFrom< &str> for IssueSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(IssueSeverityEnum::SEVERITYUNSPECIFIED),
           "ERROR" => Ok(IssueSeverityEnum::ERROR),
           "WARNING" => Ok(IssueSeverityEnum::WARNING),
           "INFO" => Ok(IssueSeverityEnum::INFO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IssueSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountIncludeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The information to be included in the response. Only sections listed here will be returned.
pub enum AccountIncludeEnum {
    

    /// Unknown, never used.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Include the attributes of the product.
    ///
    /// "ATTRIBUTES"
    #[serde(rename="ATTRIBUTES")]
    ATTRIBUTES,
    

    /// Include the issues of the product.
    ///
    /// "ISSUES"
    #[serde(rename="ISSUES")]
    ISSUES,
    

    /// Include the destination statuses of the product.
    ///
    /// "DESTINATION_STATUSES"
    #[serde(rename="DESTINATION_STATUSES")]
    DESTINATIONSTATUSES,
}

impl AsRef<str> for AccountIncludeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountIncludeEnum::UNKNOWN => "UNKNOWN",
            AccountIncludeEnum::ATTRIBUTES => "ATTRIBUTES",
            AccountIncludeEnum::ISSUES => "ISSUES",
            AccountIncludeEnum::DESTINATIONSTATUSES => "DESTINATION_STATUSES",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountIncludeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(AccountIncludeEnum::UNKNOWN),
           "ATTRIBUTES" => Ok(AccountIncludeEnum::ATTRIBUTES),
           "ISSUES" => Ok(AccountIncludeEnum::ISSUES),
           "DESTINATION_STATUSES" => Ok(AccountIncludeEnum::DESTINATIONSTATUSES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountIncludeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


