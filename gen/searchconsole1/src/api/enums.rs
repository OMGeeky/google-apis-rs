use super::*;



// region AmpInspectionResultAmpIndexStatusVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Index status of the AMP URL.
pub enum AmpInspectionResultAmpIndexStatusVerdictEnum {
    

    /// Unknown verdict.
    ///
    /// "VERDICT_UNSPECIFIED"
    #[serde(rename="VERDICT_UNSPECIFIED")]
    VERDICTUNSPECIFIED,
    

    /// Equivalent to "Valid" for the page or item in Search Console.
    ///
    /// "PASS"
    #[serde(rename="PASS")]
    PASS,
    

    /// Reserved, no longer in use.
    ///
    /// "PARTIAL"
    #[serde(rename="PARTIAL")]
    PARTIAL,
    

    /// Equivalent to "Error" or "Invalid" for the page or item in Search Console.
    ///
    /// "FAIL"
    #[serde(rename="FAIL")]
    FAIL,
    

    /// Equivalent to "Excluded" for the page or item in Search Console.
    ///
    /// "NEUTRAL"
    #[serde(rename="NEUTRAL")]
    NEUTRAL,
}

impl AsRef<str> for AmpInspectionResultAmpIndexStatusVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AmpInspectionResultAmpIndexStatusVerdictEnum::VERDICTUNSPECIFIED => "VERDICT_UNSPECIFIED",
            AmpInspectionResultAmpIndexStatusVerdictEnum::PASS => "PASS",
            AmpInspectionResultAmpIndexStatusVerdictEnum::PARTIAL => "PARTIAL",
            AmpInspectionResultAmpIndexStatusVerdictEnum::FAIL => "FAIL",
            AmpInspectionResultAmpIndexStatusVerdictEnum::NEUTRAL => "NEUTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for AmpInspectionResultAmpIndexStatusVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERDICT_UNSPECIFIED" => Ok(AmpInspectionResultAmpIndexStatusVerdictEnum::VERDICTUNSPECIFIED),
           "PASS" => Ok(AmpInspectionResultAmpIndexStatusVerdictEnum::PASS),
           "PARTIAL" => Ok(AmpInspectionResultAmpIndexStatusVerdictEnum::PARTIAL),
           "FAIL" => Ok(AmpInspectionResultAmpIndexStatusVerdictEnum::FAIL),
           "NEUTRAL" => Ok(AmpInspectionResultAmpIndexStatusVerdictEnum::NEUTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AmpInspectionResultAmpIndexStatusVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AmpInspectionResultIndexingStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether or not the page blocks indexing through a noindex rule.
pub enum AmpInspectionResultIndexingStateEnum {
    

    /// Unknown indexing status.
    ///
    /// "AMP_INDEXING_STATE_UNSPECIFIED"
    #[serde(rename="AMP_INDEXING_STATE_UNSPECIFIED")]
    AMPINDEXINGSTATEUNSPECIFIED,
    

    /// Indexing allowed.
    ///
    /// "AMP_INDEXING_ALLOWED"
    #[serde(rename="AMP_INDEXING_ALLOWED")]
    AMPINDEXINGALLOWED,
    

    /// Indexing not allowed, 'noindex' detected.
    ///
    /// "BLOCKED_DUE_TO_NOINDEX"
    #[serde(rename="BLOCKED_DUE_TO_NOINDEX")]
    BLOCKEDDUETONOINDEX,
    

    /// Indexing not allowed, 'unavailable_after' date expired.
    ///
    /// "BLOCKED_DUE_TO_EXPIRED_UNAVAILABLE_AFTER"
    #[serde(rename="BLOCKED_DUE_TO_EXPIRED_UNAVAILABLE_AFTER")]
    BLOCKEDDUETOEXPIREDUNAVAILABLEAFTER,
}

impl AsRef<str> for AmpInspectionResultIndexingStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AmpInspectionResultIndexingStateEnum::AMPINDEXINGSTATEUNSPECIFIED => "AMP_INDEXING_STATE_UNSPECIFIED",
            AmpInspectionResultIndexingStateEnum::AMPINDEXINGALLOWED => "AMP_INDEXING_ALLOWED",
            AmpInspectionResultIndexingStateEnum::BLOCKEDDUETONOINDEX => "BLOCKED_DUE_TO_NOINDEX",
            AmpInspectionResultIndexingStateEnum::BLOCKEDDUETOEXPIREDUNAVAILABLEAFTER => "BLOCKED_DUE_TO_EXPIRED_UNAVAILABLE_AFTER",
        }
    }
}

impl std::convert::TryFrom< &str> for AmpInspectionResultIndexingStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AMP_INDEXING_STATE_UNSPECIFIED" => Ok(AmpInspectionResultIndexingStateEnum::AMPINDEXINGSTATEUNSPECIFIED),
           "AMP_INDEXING_ALLOWED" => Ok(AmpInspectionResultIndexingStateEnum::AMPINDEXINGALLOWED),
           "BLOCKED_DUE_TO_NOINDEX" => Ok(AmpInspectionResultIndexingStateEnum::BLOCKEDDUETONOINDEX),
           "BLOCKED_DUE_TO_EXPIRED_UNAVAILABLE_AFTER" => Ok(AmpInspectionResultIndexingStateEnum::BLOCKEDDUETOEXPIREDUNAVAILABLEAFTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AmpInspectionResultIndexingStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AmpInspectionResultPageFetchStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether or not Google could fetch the AMP.
pub enum AmpInspectionResultPageFetchStateEnum {
    

    /// Unknown fetch state.
    ///
    /// "PAGE_FETCH_STATE_UNSPECIFIED"
    #[serde(rename="PAGE_FETCH_STATE_UNSPECIFIED")]
    PAGEFETCHSTATEUNSPECIFIED,
    

    /// Successful fetch.
    ///
    /// "SUCCESSFUL"
    #[serde(rename="SUCCESSFUL")]
    SUCCESSFUL,
    

    /// Soft 404.
    ///
    /// "SOFT_404"
    #[serde(rename="SOFT_404")]
    SOFT404,
    

    /// Blocked by robots.txt.
    ///
    /// "BLOCKED_ROBOTS_TXT"
    #[serde(rename="BLOCKED_ROBOTS_TXT")]
    BLOCKEDROBOTSTXT,
    

    /// Not found (404).
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
    

    /// Blocked due to unauthorized request (401).
    ///
    /// "ACCESS_DENIED"
    #[serde(rename="ACCESS_DENIED")]
    ACCESSDENIED,
    

    /// Server error (5xx).
    ///
    /// "SERVER_ERROR"
    #[serde(rename="SERVER_ERROR")]
    SERVERERROR,
    

    /// Redirection error.
    ///
    /// "REDIRECT_ERROR"
    #[serde(rename="REDIRECT_ERROR")]
    REDIRECTERROR,
    

    /// Blocked due to access forbidden (403).
    ///
    /// "ACCESS_FORBIDDEN"
    #[serde(rename="ACCESS_FORBIDDEN")]
    ACCESSFORBIDDEN,
    

    /// Blocked due to other 4xx issue (not 403, 404).
    ///
    /// "BLOCKED_4XX"
    #[serde(rename="BLOCKED_4XX")]
    BLOCKED4XX,
    

    /// Internal error.
    ///
    /// "INTERNAL_CRAWL_ERROR"
    #[serde(rename="INTERNAL_CRAWL_ERROR")]
    INTERNALCRAWLERROR,
    

    /// Invalid URL.
    ///
    /// "INVALID_URL"
    #[serde(rename="INVALID_URL")]
    INVALIDURL,
}

impl AsRef<str> for AmpInspectionResultPageFetchStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AmpInspectionResultPageFetchStateEnum::PAGEFETCHSTATEUNSPECIFIED => "PAGE_FETCH_STATE_UNSPECIFIED",
            AmpInspectionResultPageFetchStateEnum::SUCCESSFUL => "SUCCESSFUL",
            AmpInspectionResultPageFetchStateEnum::SOFT404 => "SOFT_404",
            AmpInspectionResultPageFetchStateEnum::BLOCKEDROBOTSTXT => "BLOCKED_ROBOTS_TXT",
            AmpInspectionResultPageFetchStateEnum::NOTFOUND => "NOT_FOUND",
            AmpInspectionResultPageFetchStateEnum::ACCESSDENIED => "ACCESS_DENIED",
            AmpInspectionResultPageFetchStateEnum::SERVERERROR => "SERVER_ERROR",
            AmpInspectionResultPageFetchStateEnum::REDIRECTERROR => "REDIRECT_ERROR",
            AmpInspectionResultPageFetchStateEnum::ACCESSFORBIDDEN => "ACCESS_FORBIDDEN",
            AmpInspectionResultPageFetchStateEnum::BLOCKED4XX => "BLOCKED_4XX",
            AmpInspectionResultPageFetchStateEnum::INTERNALCRAWLERROR => "INTERNAL_CRAWL_ERROR",
            AmpInspectionResultPageFetchStateEnum::INVALIDURL => "INVALID_URL",
        }
    }
}

impl std::convert::TryFrom< &str> for AmpInspectionResultPageFetchStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PAGE_FETCH_STATE_UNSPECIFIED" => Ok(AmpInspectionResultPageFetchStateEnum::PAGEFETCHSTATEUNSPECIFIED),
           "SUCCESSFUL" => Ok(AmpInspectionResultPageFetchStateEnum::SUCCESSFUL),
           "SOFT_404" => Ok(AmpInspectionResultPageFetchStateEnum::SOFT404),
           "BLOCKED_ROBOTS_TXT" => Ok(AmpInspectionResultPageFetchStateEnum::BLOCKEDROBOTSTXT),
           "NOT_FOUND" => Ok(AmpInspectionResultPageFetchStateEnum::NOTFOUND),
           "ACCESS_DENIED" => Ok(AmpInspectionResultPageFetchStateEnum::ACCESSDENIED),
           "SERVER_ERROR" => Ok(AmpInspectionResultPageFetchStateEnum::SERVERERROR),
           "REDIRECT_ERROR" => Ok(AmpInspectionResultPageFetchStateEnum::REDIRECTERROR),
           "ACCESS_FORBIDDEN" => Ok(AmpInspectionResultPageFetchStateEnum::ACCESSFORBIDDEN),
           "BLOCKED_4XX" => Ok(AmpInspectionResultPageFetchStateEnum::BLOCKED4XX),
           "INTERNAL_CRAWL_ERROR" => Ok(AmpInspectionResultPageFetchStateEnum::INTERNALCRAWLERROR),
           "INVALID_URL" => Ok(AmpInspectionResultPageFetchStateEnum::INVALIDURL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AmpInspectionResultPageFetchStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AmpInspectionResultRobotsTxtStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether or not the page is blocked to Google by a robots.txt rule.
pub enum AmpInspectionResultRobotsTxtStateEnum {
    

    /// Unknown robots.txt state, typically because the page wasn't fetched or found, or because robots.txt itself couldn't be reached.
    ///
    /// "ROBOTS_TXT_STATE_UNSPECIFIED"
    #[serde(rename="ROBOTS_TXT_STATE_UNSPECIFIED")]
    ROBOTSTXTSTATEUNSPECIFIED,
    

    /// Crawl allowed by robots.txt.
    ///
    /// "ALLOWED"
    #[serde(rename="ALLOWED")]
    ALLOWED,
    

    /// Crawl blocked by robots.txt.
    ///
    /// "DISALLOWED"
    #[serde(rename="DISALLOWED")]
    DISALLOWED,
}

impl AsRef<str> for AmpInspectionResultRobotsTxtStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AmpInspectionResultRobotsTxtStateEnum::ROBOTSTXTSTATEUNSPECIFIED => "ROBOTS_TXT_STATE_UNSPECIFIED",
            AmpInspectionResultRobotsTxtStateEnum::ALLOWED => "ALLOWED",
            AmpInspectionResultRobotsTxtStateEnum::DISALLOWED => "DISALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for AmpInspectionResultRobotsTxtStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROBOTS_TXT_STATE_UNSPECIFIED" => Ok(AmpInspectionResultRobotsTxtStateEnum::ROBOTSTXTSTATEUNSPECIFIED),
           "ALLOWED" => Ok(AmpInspectionResultRobotsTxtStateEnum::ALLOWED),
           "DISALLOWED" => Ok(AmpInspectionResultRobotsTxtStateEnum::DISALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AmpInspectionResultRobotsTxtStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AmpInspectionResultVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the most severe error on the page. If a page has both warnings and errors, the page status is error. Error status means the page cannot be shown in Search results.
pub enum AmpInspectionResultVerdictEnum {
    

    /// Unknown verdict.
    ///
    /// "VERDICT_UNSPECIFIED"
    #[serde(rename="VERDICT_UNSPECIFIED")]
    VERDICTUNSPECIFIED,
    

    /// Equivalent to "Valid" for the page or item in Search Console.
    ///
    /// "PASS"
    #[serde(rename="PASS")]
    PASS,
    

    /// Reserved, no longer in use.
    ///
    /// "PARTIAL"
    #[serde(rename="PARTIAL")]
    PARTIAL,
    

    /// Equivalent to "Error" or "Invalid" for the page or item in Search Console.
    ///
    /// "FAIL"
    #[serde(rename="FAIL")]
    FAIL,
    

    /// Equivalent to "Excluded" for the page or item in Search Console.
    ///
    /// "NEUTRAL"
    #[serde(rename="NEUTRAL")]
    NEUTRAL,
}

impl AsRef<str> for AmpInspectionResultVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AmpInspectionResultVerdictEnum::VERDICTUNSPECIFIED => "VERDICT_UNSPECIFIED",
            AmpInspectionResultVerdictEnum::PASS => "PASS",
            AmpInspectionResultVerdictEnum::PARTIAL => "PARTIAL",
            AmpInspectionResultVerdictEnum::FAIL => "FAIL",
            AmpInspectionResultVerdictEnum::NEUTRAL => "NEUTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for AmpInspectionResultVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERDICT_UNSPECIFIED" => Ok(AmpInspectionResultVerdictEnum::VERDICTUNSPECIFIED),
           "PASS" => Ok(AmpInspectionResultVerdictEnum::PASS),
           "PARTIAL" => Ok(AmpInspectionResultVerdictEnum::PARTIAL),
           "FAIL" => Ok(AmpInspectionResultVerdictEnum::FAIL),
           "NEUTRAL" => Ok(AmpInspectionResultVerdictEnum::NEUTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AmpInspectionResultVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AmpIssueSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Severity of this issue: WARNING or ERROR.
pub enum AmpIssueSeverityEnum {
    

    /// Unknown severity.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Warning.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Error.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for AmpIssueSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AmpIssueSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            AmpIssueSeverityEnum::WARNING => "WARNING",
            AmpIssueSeverityEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for AmpIssueSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(AmpIssueSeverityEnum::SEVERITYUNSPECIFIED),
           "WARNING" => Ok(AmpIssueSeverityEnum::WARNING),
           "ERROR" => Ok(AmpIssueSeverityEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AmpIssueSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApiDimensionFilterDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiDimensionFilterDimensionEnum {
    
    /// "QUERY"
    #[serde(rename="QUERY")]
    QUERY,
    
    /// "PAGE"
    #[serde(rename="PAGE")]
    PAGE,
    
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    
    /// "DEVICE"
    #[serde(rename="DEVICE")]
    DEVICE,
    
    /// "SEARCH_APPEARANCE"
    #[serde(rename="SEARCH_APPEARANCE")]
    SEARCHAPPEARANCE,
}

impl AsRef<str> for ApiDimensionFilterDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApiDimensionFilterDimensionEnum::QUERY => "QUERY",
            ApiDimensionFilterDimensionEnum::PAGE => "PAGE",
            ApiDimensionFilterDimensionEnum::COUNTRY => "COUNTRY",
            ApiDimensionFilterDimensionEnum::DEVICE => "DEVICE",
            ApiDimensionFilterDimensionEnum::SEARCHAPPEARANCE => "SEARCH_APPEARANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for ApiDimensionFilterDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "QUERY" => Ok(ApiDimensionFilterDimensionEnum::QUERY),
           "PAGE" => Ok(ApiDimensionFilterDimensionEnum::PAGE),
           "COUNTRY" => Ok(ApiDimensionFilterDimensionEnum::COUNTRY),
           "DEVICE" => Ok(ApiDimensionFilterDimensionEnum::DEVICE),
           "SEARCH_APPEARANCE" => Ok(ApiDimensionFilterDimensionEnum::SEARCHAPPEARANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApiDimensionFilterDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApiDimensionFilterOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiDimensionFilterOperatorEnum {
    
    /// "EQUALS"
    #[serde(rename="EQUALS")]
    EQUALS,
    
    /// "NOT_EQUALS"
    #[serde(rename="NOT_EQUALS")]
    NOTEQUALS,
    
    /// "CONTAINS"
    #[serde(rename="CONTAINS")]
    CONTAINS,
    
    /// "NOT_CONTAINS"
    #[serde(rename="NOT_CONTAINS")]
    NOTCONTAINS,
    
    /// "INCLUDING_REGEX"
    #[serde(rename="INCLUDING_REGEX")]
    INCLUDINGREGEX,
    
    /// "EXCLUDING_REGEX"
    #[serde(rename="EXCLUDING_REGEX")]
    EXCLUDINGREGEX,
}

impl AsRef<str> for ApiDimensionFilterOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApiDimensionFilterOperatorEnum::EQUALS => "EQUALS",
            ApiDimensionFilterOperatorEnum::NOTEQUALS => "NOT_EQUALS",
            ApiDimensionFilterOperatorEnum::CONTAINS => "CONTAINS",
            ApiDimensionFilterOperatorEnum::NOTCONTAINS => "NOT_CONTAINS",
            ApiDimensionFilterOperatorEnum::INCLUDINGREGEX => "INCLUDING_REGEX",
            ApiDimensionFilterOperatorEnum::EXCLUDINGREGEX => "EXCLUDING_REGEX",
        }
    }
}

impl std::convert::TryFrom< &str> for ApiDimensionFilterOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EQUALS" => Ok(ApiDimensionFilterOperatorEnum::EQUALS),
           "NOT_EQUALS" => Ok(ApiDimensionFilterOperatorEnum::NOTEQUALS),
           "CONTAINS" => Ok(ApiDimensionFilterOperatorEnum::CONTAINS),
           "NOT_CONTAINS" => Ok(ApiDimensionFilterOperatorEnum::NOTCONTAINS),
           "INCLUDING_REGEX" => Ok(ApiDimensionFilterOperatorEnum::INCLUDINGREGEX),
           "EXCLUDING_REGEX" => Ok(ApiDimensionFilterOperatorEnum::EXCLUDINGREGEX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApiDimensionFilterOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApiDimensionFilterGroupGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiDimensionFilterGroupGroupTypeEnum {
    
    /// "AND"
    #[serde(rename="AND")]
    AND,
}

impl AsRef<str> for ApiDimensionFilterGroupGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApiDimensionFilterGroupGroupTypeEnum::AND => "AND",
        }
    }
}

impl std::convert::TryFrom< &str> for ApiDimensionFilterGroupGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AND" => Ok(ApiDimensionFilterGroupGroupTypeEnum::AND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApiDimensionFilterGroupGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IndexStatusInspectionResultCrawledAsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Primary crawler that was used by Google to crawl your site.
pub enum IndexStatusInspectionResultCrawledAsEnum {
    

    /// Unknown user agent.
    ///
    /// "CRAWLING_USER_AGENT_UNSPECIFIED"
    #[serde(rename="CRAWLING_USER_AGENT_UNSPECIFIED")]
    CRAWLINGUSERAGENTUNSPECIFIED,
    

    /// Desktop user agent.
    ///
    /// "DESKTOP"
    #[serde(rename="DESKTOP")]
    DESKTOP,
    

    /// Mobile user agent.
    ///
    /// "MOBILE"
    #[serde(rename="MOBILE")]
    MOBILE,
}

impl AsRef<str> for IndexStatusInspectionResultCrawledAsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IndexStatusInspectionResultCrawledAsEnum::CRAWLINGUSERAGENTUNSPECIFIED => "CRAWLING_USER_AGENT_UNSPECIFIED",
            IndexStatusInspectionResultCrawledAsEnum::DESKTOP => "DESKTOP",
            IndexStatusInspectionResultCrawledAsEnum::MOBILE => "MOBILE",
        }
    }
}

impl std::convert::TryFrom< &str> for IndexStatusInspectionResultCrawledAsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRAWLING_USER_AGENT_UNSPECIFIED" => Ok(IndexStatusInspectionResultCrawledAsEnum::CRAWLINGUSERAGENTUNSPECIFIED),
           "DESKTOP" => Ok(IndexStatusInspectionResultCrawledAsEnum::DESKTOP),
           "MOBILE" => Ok(IndexStatusInspectionResultCrawledAsEnum::MOBILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IndexStatusInspectionResultCrawledAsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IndexStatusInspectionResultIndexingStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether or not the page blocks indexing through a noindex rule.
pub enum IndexStatusInspectionResultIndexingStateEnum {
    

    /// Unknown indexing status.
    ///
    /// "INDEXING_STATE_UNSPECIFIED"
    #[serde(rename="INDEXING_STATE_UNSPECIFIED")]
    INDEXINGSTATEUNSPECIFIED,
    

    /// Indexing allowed.
    ///
    /// "INDEXING_ALLOWED"
    #[serde(rename="INDEXING_ALLOWED")]
    INDEXINGALLOWED,
    

    /// Indexing not allowed, 'noindex' detected in 'robots' meta tag.
    ///
    /// "BLOCKED_BY_META_TAG"
    #[serde(rename="BLOCKED_BY_META_TAG")]
    BLOCKEDBYMETATAG,
    

    /// Indexing not allowed, 'noindex' detected in 'X-Robots-Tag' http header.
    ///
    /// "BLOCKED_BY_HTTP_HEADER"
    #[serde(rename="BLOCKED_BY_HTTP_HEADER")]
    BLOCKEDBYHTTPHEADER,
    

    /// Reserved, no longer in use.
    ///
    /// "BLOCKED_BY_ROBOTS_TXT"
    #[serde(rename="BLOCKED_BY_ROBOTS_TXT")]
    BLOCKEDBYROBOTSTXT,
}

impl AsRef<str> for IndexStatusInspectionResultIndexingStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IndexStatusInspectionResultIndexingStateEnum::INDEXINGSTATEUNSPECIFIED => "INDEXING_STATE_UNSPECIFIED",
            IndexStatusInspectionResultIndexingStateEnum::INDEXINGALLOWED => "INDEXING_ALLOWED",
            IndexStatusInspectionResultIndexingStateEnum::BLOCKEDBYMETATAG => "BLOCKED_BY_META_TAG",
            IndexStatusInspectionResultIndexingStateEnum::BLOCKEDBYHTTPHEADER => "BLOCKED_BY_HTTP_HEADER",
            IndexStatusInspectionResultIndexingStateEnum::BLOCKEDBYROBOTSTXT => "BLOCKED_BY_ROBOTS_TXT",
        }
    }
}

impl std::convert::TryFrom< &str> for IndexStatusInspectionResultIndexingStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INDEXING_STATE_UNSPECIFIED" => Ok(IndexStatusInspectionResultIndexingStateEnum::INDEXINGSTATEUNSPECIFIED),
           "INDEXING_ALLOWED" => Ok(IndexStatusInspectionResultIndexingStateEnum::INDEXINGALLOWED),
           "BLOCKED_BY_META_TAG" => Ok(IndexStatusInspectionResultIndexingStateEnum::BLOCKEDBYMETATAG),
           "BLOCKED_BY_HTTP_HEADER" => Ok(IndexStatusInspectionResultIndexingStateEnum::BLOCKEDBYHTTPHEADER),
           "BLOCKED_BY_ROBOTS_TXT" => Ok(IndexStatusInspectionResultIndexingStateEnum::BLOCKEDBYROBOTSTXT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IndexStatusInspectionResultIndexingStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IndexStatusInspectionResultPageFetchStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether or not Google could retrieve the page from your server. Equivalent to ["page fetch"](https://support.google.com/webmasters/answer/9012289#index_coverage) in the URL inspection report.
pub enum IndexStatusInspectionResultPageFetchStateEnum {
    

    /// Unknown fetch state.
    ///
    /// "PAGE_FETCH_STATE_UNSPECIFIED"
    #[serde(rename="PAGE_FETCH_STATE_UNSPECIFIED")]
    PAGEFETCHSTATEUNSPECIFIED,
    

    /// Successful fetch.
    ///
    /// "SUCCESSFUL"
    #[serde(rename="SUCCESSFUL")]
    SUCCESSFUL,
    

    /// Soft 404.
    ///
    /// "SOFT_404"
    #[serde(rename="SOFT_404")]
    SOFT404,
    

    /// Blocked by robots.txt.
    ///
    /// "BLOCKED_ROBOTS_TXT"
    #[serde(rename="BLOCKED_ROBOTS_TXT")]
    BLOCKEDROBOTSTXT,
    

    /// Not found (404).
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
    

    /// Blocked due to unauthorized request (401).
    ///
    /// "ACCESS_DENIED"
    #[serde(rename="ACCESS_DENIED")]
    ACCESSDENIED,
    

    /// Server error (5xx).
    ///
    /// "SERVER_ERROR"
    #[serde(rename="SERVER_ERROR")]
    SERVERERROR,
    

    /// Redirection error.
    ///
    /// "REDIRECT_ERROR"
    #[serde(rename="REDIRECT_ERROR")]
    REDIRECTERROR,
    

    /// Blocked due to access forbidden (403).
    ///
    /// "ACCESS_FORBIDDEN"
    #[serde(rename="ACCESS_FORBIDDEN")]
    ACCESSFORBIDDEN,
    

    /// Blocked due to other 4xx issue (not 403, 404).
    ///
    /// "BLOCKED_4XX"
    #[serde(rename="BLOCKED_4XX")]
    BLOCKED4XX,
    

    /// Internal error.
    ///
    /// "INTERNAL_CRAWL_ERROR"
    #[serde(rename="INTERNAL_CRAWL_ERROR")]
    INTERNALCRAWLERROR,
    

    /// Invalid URL.
    ///
    /// "INVALID_URL"
    #[serde(rename="INVALID_URL")]
    INVALIDURL,
}

impl AsRef<str> for IndexStatusInspectionResultPageFetchStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IndexStatusInspectionResultPageFetchStateEnum::PAGEFETCHSTATEUNSPECIFIED => "PAGE_FETCH_STATE_UNSPECIFIED",
            IndexStatusInspectionResultPageFetchStateEnum::SUCCESSFUL => "SUCCESSFUL",
            IndexStatusInspectionResultPageFetchStateEnum::SOFT404 => "SOFT_404",
            IndexStatusInspectionResultPageFetchStateEnum::BLOCKEDROBOTSTXT => "BLOCKED_ROBOTS_TXT",
            IndexStatusInspectionResultPageFetchStateEnum::NOTFOUND => "NOT_FOUND",
            IndexStatusInspectionResultPageFetchStateEnum::ACCESSDENIED => "ACCESS_DENIED",
            IndexStatusInspectionResultPageFetchStateEnum::SERVERERROR => "SERVER_ERROR",
            IndexStatusInspectionResultPageFetchStateEnum::REDIRECTERROR => "REDIRECT_ERROR",
            IndexStatusInspectionResultPageFetchStateEnum::ACCESSFORBIDDEN => "ACCESS_FORBIDDEN",
            IndexStatusInspectionResultPageFetchStateEnum::BLOCKED4XX => "BLOCKED_4XX",
            IndexStatusInspectionResultPageFetchStateEnum::INTERNALCRAWLERROR => "INTERNAL_CRAWL_ERROR",
            IndexStatusInspectionResultPageFetchStateEnum::INVALIDURL => "INVALID_URL",
        }
    }
}

impl std::convert::TryFrom< &str> for IndexStatusInspectionResultPageFetchStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PAGE_FETCH_STATE_UNSPECIFIED" => Ok(IndexStatusInspectionResultPageFetchStateEnum::PAGEFETCHSTATEUNSPECIFIED),
           "SUCCESSFUL" => Ok(IndexStatusInspectionResultPageFetchStateEnum::SUCCESSFUL),
           "SOFT_404" => Ok(IndexStatusInspectionResultPageFetchStateEnum::SOFT404),
           "BLOCKED_ROBOTS_TXT" => Ok(IndexStatusInspectionResultPageFetchStateEnum::BLOCKEDROBOTSTXT),
           "NOT_FOUND" => Ok(IndexStatusInspectionResultPageFetchStateEnum::NOTFOUND),
           "ACCESS_DENIED" => Ok(IndexStatusInspectionResultPageFetchStateEnum::ACCESSDENIED),
           "SERVER_ERROR" => Ok(IndexStatusInspectionResultPageFetchStateEnum::SERVERERROR),
           "REDIRECT_ERROR" => Ok(IndexStatusInspectionResultPageFetchStateEnum::REDIRECTERROR),
           "ACCESS_FORBIDDEN" => Ok(IndexStatusInspectionResultPageFetchStateEnum::ACCESSFORBIDDEN),
           "BLOCKED_4XX" => Ok(IndexStatusInspectionResultPageFetchStateEnum::BLOCKED4XX),
           "INTERNAL_CRAWL_ERROR" => Ok(IndexStatusInspectionResultPageFetchStateEnum::INTERNALCRAWLERROR),
           "INVALID_URL" => Ok(IndexStatusInspectionResultPageFetchStateEnum::INVALIDURL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IndexStatusInspectionResultPageFetchStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IndexStatusInspectionResultRobotsTxtStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether or not the page is blocked to Google by a robots.txt rule.
pub enum IndexStatusInspectionResultRobotsTxtStateEnum {
    

    /// Unknown robots.txt state, typically because the page wasn't fetched or found, or because robots.txt itself couldn't be reached.
    ///
    /// "ROBOTS_TXT_STATE_UNSPECIFIED"
    #[serde(rename="ROBOTS_TXT_STATE_UNSPECIFIED")]
    ROBOTSTXTSTATEUNSPECIFIED,
    

    /// Crawl allowed by robots.txt.
    ///
    /// "ALLOWED"
    #[serde(rename="ALLOWED")]
    ALLOWED,
    

    /// Crawl blocked by robots.txt.
    ///
    /// "DISALLOWED"
    #[serde(rename="DISALLOWED")]
    DISALLOWED,
}

impl AsRef<str> for IndexStatusInspectionResultRobotsTxtStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IndexStatusInspectionResultRobotsTxtStateEnum::ROBOTSTXTSTATEUNSPECIFIED => "ROBOTS_TXT_STATE_UNSPECIFIED",
            IndexStatusInspectionResultRobotsTxtStateEnum::ALLOWED => "ALLOWED",
            IndexStatusInspectionResultRobotsTxtStateEnum::DISALLOWED => "DISALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for IndexStatusInspectionResultRobotsTxtStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROBOTS_TXT_STATE_UNSPECIFIED" => Ok(IndexStatusInspectionResultRobotsTxtStateEnum::ROBOTSTXTSTATEUNSPECIFIED),
           "ALLOWED" => Ok(IndexStatusInspectionResultRobotsTxtStateEnum::ALLOWED),
           "DISALLOWED" => Ok(IndexStatusInspectionResultRobotsTxtStateEnum::DISALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IndexStatusInspectionResultRobotsTxtStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IndexStatusInspectionResultVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// High level verdict about whether the URL *is* indexed (indexed status), or *can be* indexed (live inspection).
pub enum IndexStatusInspectionResultVerdictEnum {
    

    /// Unknown verdict.
    ///
    /// "VERDICT_UNSPECIFIED"
    #[serde(rename="VERDICT_UNSPECIFIED")]
    VERDICTUNSPECIFIED,
    

    /// Equivalent to "Valid" for the page or item in Search Console.
    ///
    /// "PASS"
    #[serde(rename="PASS")]
    PASS,
    

    /// Reserved, no longer in use.
    ///
    /// "PARTIAL"
    #[serde(rename="PARTIAL")]
    PARTIAL,
    

    /// Equivalent to "Error" or "Invalid" for the page or item in Search Console.
    ///
    /// "FAIL"
    #[serde(rename="FAIL")]
    FAIL,
    

    /// Equivalent to "Excluded" for the page or item in Search Console.
    ///
    /// "NEUTRAL"
    #[serde(rename="NEUTRAL")]
    NEUTRAL,
}

impl AsRef<str> for IndexStatusInspectionResultVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IndexStatusInspectionResultVerdictEnum::VERDICTUNSPECIFIED => "VERDICT_UNSPECIFIED",
            IndexStatusInspectionResultVerdictEnum::PASS => "PASS",
            IndexStatusInspectionResultVerdictEnum::PARTIAL => "PARTIAL",
            IndexStatusInspectionResultVerdictEnum::FAIL => "FAIL",
            IndexStatusInspectionResultVerdictEnum::NEUTRAL => "NEUTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for IndexStatusInspectionResultVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERDICT_UNSPECIFIED" => Ok(IndexStatusInspectionResultVerdictEnum::VERDICTUNSPECIFIED),
           "PASS" => Ok(IndexStatusInspectionResultVerdictEnum::PASS),
           "PARTIAL" => Ok(IndexStatusInspectionResultVerdictEnum::PARTIAL),
           "FAIL" => Ok(IndexStatusInspectionResultVerdictEnum::FAIL),
           "NEUTRAL" => Ok(IndexStatusInspectionResultVerdictEnum::NEUTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IndexStatusInspectionResultVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MobileFriendlyIssueRuleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rule violated.
pub enum MobileFriendlyIssueRuleEnum {
    

    /// Unknown rule. Sorry, we don't have any description for the rule that was broken.
    ///
    /// "MOBILE_FRIENDLY_RULE_UNSPECIFIED"
    #[serde(rename="MOBILE_FRIENDLY_RULE_UNSPECIFIED")]
    MOBILEFRIENDLYRULEUNSPECIFIED,
    

    /// Plugins incompatible with mobile devices are being used. [Learn more] (https://support.google.com/webmasters/answer/6352293#flash_usage).
    ///
    /// "USES_INCOMPATIBLE_PLUGINS"
    #[serde(rename="USES_INCOMPATIBLE_PLUGINS")]
    USESINCOMPATIBLEPLUGINS,
    

    /// Viewport is not specified using the meta viewport tag. [Learn more] (https://support.google.com/webmasters/answer/6352293#viewport_not_configured).
    ///
    /// "CONFIGURE_VIEWPORT"
    #[serde(rename="CONFIGURE_VIEWPORT")]
    CONFIGUREVIEWPORT,
    

    /// Viewport defined to a fixed width. [Learn more] (https://support.google.com/webmasters/answer/6352293#fixed-width_viewport).
    ///
    /// "FIXED_WIDTH_VIEWPORT"
    #[serde(rename="FIXED_WIDTH_VIEWPORT")]
    FIXEDWIDTHVIEWPORT,
    

    /// Content not sized to viewport. [Learn more] (https://support.google.com/webmasters/answer/6352293#content_not_sized_to_viewport).
    ///
    /// "SIZE_CONTENT_TO_VIEWPORT"
    #[serde(rename="SIZE_CONTENT_TO_VIEWPORT")]
    SIZECONTENTTOVIEWPORT,
    

    /// Font size is too small for easy reading on a small screen. [Learn More] (https://support.google.com/webmasters/answer/6352293#small_font_size).
    ///
    /// "USE_LEGIBLE_FONT_SIZES"
    #[serde(rename="USE_LEGIBLE_FONT_SIZES")]
    USELEGIBLEFONTSIZES,
    

    /// Touch elements are too close to each other. [Learn more] (https://support.google.com/webmasters/answer/6352293#touch_elements_too_close).
    ///
    /// "TAP_TARGETS_TOO_CLOSE"
    #[serde(rename="TAP_TARGETS_TOO_CLOSE")]
    TAPTARGETSTOOCLOSE,
}

impl AsRef<str> for MobileFriendlyIssueRuleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MobileFriendlyIssueRuleEnum::MOBILEFRIENDLYRULEUNSPECIFIED => "MOBILE_FRIENDLY_RULE_UNSPECIFIED",
            MobileFriendlyIssueRuleEnum::USESINCOMPATIBLEPLUGINS => "USES_INCOMPATIBLE_PLUGINS",
            MobileFriendlyIssueRuleEnum::CONFIGUREVIEWPORT => "CONFIGURE_VIEWPORT",
            MobileFriendlyIssueRuleEnum::FIXEDWIDTHVIEWPORT => "FIXED_WIDTH_VIEWPORT",
            MobileFriendlyIssueRuleEnum::SIZECONTENTTOVIEWPORT => "SIZE_CONTENT_TO_VIEWPORT",
            MobileFriendlyIssueRuleEnum::USELEGIBLEFONTSIZES => "USE_LEGIBLE_FONT_SIZES",
            MobileFriendlyIssueRuleEnum::TAPTARGETSTOOCLOSE => "TAP_TARGETS_TOO_CLOSE",
        }
    }
}

impl std::convert::TryFrom< &str> for MobileFriendlyIssueRuleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MOBILE_FRIENDLY_RULE_UNSPECIFIED" => Ok(MobileFriendlyIssueRuleEnum::MOBILEFRIENDLYRULEUNSPECIFIED),
           "USES_INCOMPATIBLE_PLUGINS" => Ok(MobileFriendlyIssueRuleEnum::USESINCOMPATIBLEPLUGINS),
           "CONFIGURE_VIEWPORT" => Ok(MobileFriendlyIssueRuleEnum::CONFIGUREVIEWPORT),
           "FIXED_WIDTH_VIEWPORT" => Ok(MobileFriendlyIssueRuleEnum::FIXEDWIDTHVIEWPORT),
           "SIZE_CONTENT_TO_VIEWPORT" => Ok(MobileFriendlyIssueRuleEnum::SIZECONTENTTOVIEWPORT),
           "USE_LEGIBLE_FONT_SIZES" => Ok(MobileFriendlyIssueRuleEnum::USELEGIBLEFONTSIZES),
           "TAP_TARGETS_TOO_CLOSE" => Ok(MobileFriendlyIssueRuleEnum::TAPTARGETSTOOCLOSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MobileFriendlyIssueRuleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MobileUsabilityInspectionResultVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// High-level mobile-usability inspection result for this URL.
pub enum MobileUsabilityInspectionResultVerdictEnum {
    

    /// Unknown verdict.
    ///
    /// "VERDICT_UNSPECIFIED"
    #[serde(rename="VERDICT_UNSPECIFIED")]
    VERDICTUNSPECIFIED,
    

    /// Equivalent to "Valid" for the page or item in Search Console.
    ///
    /// "PASS"
    #[serde(rename="PASS")]
    PASS,
    

    /// Reserved, no longer in use.
    ///
    /// "PARTIAL"
    #[serde(rename="PARTIAL")]
    PARTIAL,
    

    /// Equivalent to "Error" or "Invalid" for the page or item in Search Console.
    ///
    /// "FAIL"
    #[serde(rename="FAIL")]
    FAIL,
    

    /// Equivalent to "Excluded" for the page or item in Search Console.
    ///
    /// "NEUTRAL"
    #[serde(rename="NEUTRAL")]
    NEUTRAL,
}

impl AsRef<str> for MobileUsabilityInspectionResultVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MobileUsabilityInspectionResultVerdictEnum::VERDICTUNSPECIFIED => "VERDICT_UNSPECIFIED",
            MobileUsabilityInspectionResultVerdictEnum::PASS => "PASS",
            MobileUsabilityInspectionResultVerdictEnum::PARTIAL => "PARTIAL",
            MobileUsabilityInspectionResultVerdictEnum::FAIL => "FAIL",
            MobileUsabilityInspectionResultVerdictEnum::NEUTRAL => "NEUTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for MobileUsabilityInspectionResultVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERDICT_UNSPECIFIED" => Ok(MobileUsabilityInspectionResultVerdictEnum::VERDICTUNSPECIFIED),
           "PASS" => Ok(MobileUsabilityInspectionResultVerdictEnum::PASS),
           "PARTIAL" => Ok(MobileUsabilityInspectionResultVerdictEnum::PARTIAL),
           "FAIL" => Ok(MobileUsabilityInspectionResultVerdictEnum::FAIL),
           "NEUTRAL" => Ok(MobileUsabilityInspectionResultVerdictEnum::NEUTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MobileUsabilityInspectionResultVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MobileUsabilityIssueIssueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobile-usability issue type.
pub enum MobileUsabilityIssueIssueTypeEnum {
    

    /// Unknown issue. Sorry, we don't have any description for the rule that was broken.
    ///
    /// "MOBILE_USABILITY_ISSUE_TYPE_UNSPECIFIED"
    #[serde(rename="MOBILE_USABILITY_ISSUE_TYPE_UNSPECIFIED")]
    MOBILEUSABILITYISSUETYPEUNSPECIFIED,
    

    /// Plugins incompatible with mobile devices are being used. [Learn more] (https://support.google.com/webmasters/answer/6352293#flash_usage#error-list).
    ///
    /// "USES_INCOMPATIBLE_PLUGINS"
    #[serde(rename="USES_INCOMPATIBLE_PLUGINS")]
    USESINCOMPATIBLEPLUGINS,
    

    /// Viewport is not specified using the meta viewport tag. [Learn more] (https://support.google.com/webmasters/answer/6352293#viewport_not_configured#error-list).
    ///
    /// "CONFIGURE_VIEWPORT"
    #[serde(rename="CONFIGURE_VIEWPORT")]
    CONFIGUREVIEWPORT,
    

    /// Viewport defined to a fixed width. [Learn more] (https://support.google.com/webmasters/answer/6352293#fixed-width_viewport#error-list).
    ///
    /// "FIXED_WIDTH_VIEWPORT"
    #[serde(rename="FIXED_WIDTH_VIEWPORT")]
    FIXEDWIDTHVIEWPORT,
    

    /// Content not sized to viewport. [Learn more] (https://support.google.com/webmasters/answer/6352293#content_not_sized_to_viewport#error-list).
    ///
    /// "SIZE_CONTENT_TO_VIEWPORT"
    #[serde(rename="SIZE_CONTENT_TO_VIEWPORT")]
    SIZECONTENTTOVIEWPORT,
    

    /// Font size is too small for easy reading on a small screen. [Learn More] (https://support.google.com/webmasters/answer/6352293#small_font_size#error-list).
    ///
    /// "USE_LEGIBLE_FONT_SIZES"
    #[serde(rename="USE_LEGIBLE_FONT_SIZES")]
    USELEGIBLEFONTSIZES,
    

    /// Touch elements are too close to each other. [Learn more] (https://support.google.com/webmasters/answer/6352293#touch_elements_too_close#error-list).
    ///
    /// "TAP_TARGETS_TOO_CLOSE"
    #[serde(rename="TAP_TARGETS_TOO_CLOSE")]
    TAPTARGETSTOOCLOSE,
}

impl AsRef<str> for MobileUsabilityIssueIssueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MobileUsabilityIssueIssueTypeEnum::MOBILEUSABILITYISSUETYPEUNSPECIFIED => "MOBILE_USABILITY_ISSUE_TYPE_UNSPECIFIED",
            MobileUsabilityIssueIssueTypeEnum::USESINCOMPATIBLEPLUGINS => "USES_INCOMPATIBLE_PLUGINS",
            MobileUsabilityIssueIssueTypeEnum::CONFIGUREVIEWPORT => "CONFIGURE_VIEWPORT",
            MobileUsabilityIssueIssueTypeEnum::FIXEDWIDTHVIEWPORT => "FIXED_WIDTH_VIEWPORT",
            MobileUsabilityIssueIssueTypeEnum::SIZECONTENTTOVIEWPORT => "SIZE_CONTENT_TO_VIEWPORT",
            MobileUsabilityIssueIssueTypeEnum::USELEGIBLEFONTSIZES => "USE_LEGIBLE_FONT_SIZES",
            MobileUsabilityIssueIssueTypeEnum::TAPTARGETSTOOCLOSE => "TAP_TARGETS_TOO_CLOSE",
        }
    }
}

impl std::convert::TryFrom< &str> for MobileUsabilityIssueIssueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MOBILE_USABILITY_ISSUE_TYPE_UNSPECIFIED" => Ok(MobileUsabilityIssueIssueTypeEnum::MOBILEUSABILITYISSUETYPEUNSPECIFIED),
           "USES_INCOMPATIBLE_PLUGINS" => Ok(MobileUsabilityIssueIssueTypeEnum::USESINCOMPATIBLEPLUGINS),
           "CONFIGURE_VIEWPORT" => Ok(MobileUsabilityIssueIssueTypeEnum::CONFIGUREVIEWPORT),
           "FIXED_WIDTH_VIEWPORT" => Ok(MobileUsabilityIssueIssueTypeEnum::FIXEDWIDTHVIEWPORT),
           "SIZE_CONTENT_TO_VIEWPORT" => Ok(MobileUsabilityIssueIssueTypeEnum::SIZECONTENTTOVIEWPORT),
           "USE_LEGIBLE_FONT_SIZES" => Ok(MobileUsabilityIssueIssueTypeEnum::USELEGIBLEFONTSIZES),
           "TAP_TARGETS_TOO_CLOSE" => Ok(MobileUsabilityIssueIssueTypeEnum::TAPTARGETSTOOCLOSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MobileUsabilityIssueIssueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MobileUsabilityIssueSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Not returned; reserved for future use.
pub enum MobileUsabilityIssueSeverityEnum {
    

    /// Unknown severity.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Warning.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Error.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for MobileUsabilityIssueSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MobileUsabilityIssueSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            MobileUsabilityIssueSeverityEnum::WARNING => "WARNING",
            MobileUsabilityIssueSeverityEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for MobileUsabilityIssueSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(MobileUsabilityIssueSeverityEnum::SEVERITYUNSPECIFIED),
           "WARNING" => Ok(MobileUsabilityIssueSeverityEnum::WARNING),
           "ERROR" => Ok(MobileUsabilityIssueSeverityEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MobileUsabilityIssueSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RichResultsInspectionResultVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// High-level rich results inspection result for this URL.
pub enum RichResultsInspectionResultVerdictEnum {
    

    /// Unknown verdict.
    ///
    /// "VERDICT_UNSPECIFIED"
    #[serde(rename="VERDICT_UNSPECIFIED")]
    VERDICTUNSPECIFIED,
    

    /// Equivalent to "Valid" for the page or item in Search Console.
    ///
    /// "PASS"
    #[serde(rename="PASS")]
    PASS,
    

    /// Reserved, no longer in use.
    ///
    /// "PARTIAL"
    #[serde(rename="PARTIAL")]
    PARTIAL,
    

    /// Equivalent to "Error" or "Invalid" for the page or item in Search Console.
    ///
    /// "FAIL"
    #[serde(rename="FAIL")]
    FAIL,
    

    /// Equivalent to "Excluded" for the page or item in Search Console.
    ///
    /// "NEUTRAL"
    #[serde(rename="NEUTRAL")]
    NEUTRAL,
}

impl AsRef<str> for RichResultsInspectionResultVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RichResultsInspectionResultVerdictEnum::VERDICTUNSPECIFIED => "VERDICT_UNSPECIFIED",
            RichResultsInspectionResultVerdictEnum::PASS => "PASS",
            RichResultsInspectionResultVerdictEnum::PARTIAL => "PARTIAL",
            RichResultsInspectionResultVerdictEnum::FAIL => "FAIL",
            RichResultsInspectionResultVerdictEnum::NEUTRAL => "NEUTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for RichResultsInspectionResultVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERDICT_UNSPECIFIED" => Ok(RichResultsInspectionResultVerdictEnum::VERDICTUNSPECIFIED),
           "PASS" => Ok(RichResultsInspectionResultVerdictEnum::PASS),
           "PARTIAL" => Ok(RichResultsInspectionResultVerdictEnum::PARTIAL),
           "FAIL" => Ok(RichResultsInspectionResultVerdictEnum::FAIL),
           "NEUTRAL" => Ok(RichResultsInspectionResultVerdictEnum::NEUTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RichResultsInspectionResultVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RichResultsIssueSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Severity of this issue: WARNING, or ERROR. Items with an issue of status ERROR cannot appear with rich result features in Google Search results.
pub enum RichResultsIssueSeverityEnum {
    

    /// Unknown severity.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Warning.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Error.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for RichResultsIssueSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RichResultsIssueSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            RichResultsIssueSeverityEnum::WARNING => "WARNING",
            RichResultsIssueSeverityEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for RichResultsIssueSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(RichResultsIssueSeverityEnum::SEVERITYUNSPECIFIED),
           "WARNING" => Ok(RichResultsIssueSeverityEnum::WARNING),
           "ERROR" => Ok(RichResultsIssueSeverityEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RichResultsIssueSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RunMobileFriendlyTestResponseMobileFriendlinessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Test verdict, whether the page is mobile friendly or not.
pub enum RunMobileFriendlyTestResponseMobileFriendlinessEnum {
    

    /// Internal error when running this test. Please try running the test again.
    ///
    /// "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED"
    #[serde(rename="MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED")]
    MOBILEFRIENDLYTESTRESULTUNSPECIFIED,
    

    /// The page is mobile friendly.
    ///
    /// "MOBILE_FRIENDLY"
    #[serde(rename="MOBILE_FRIENDLY")]
    MOBILEFRIENDLY,
    

    /// The page is not mobile friendly.
    ///
    /// "NOT_MOBILE_FRIENDLY"
    #[serde(rename="NOT_MOBILE_FRIENDLY")]
    NOTMOBILEFRIENDLY,
}

impl AsRef<str> for RunMobileFriendlyTestResponseMobileFriendlinessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RunMobileFriendlyTestResponseMobileFriendlinessEnum::MOBILEFRIENDLYTESTRESULTUNSPECIFIED => "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED",
            RunMobileFriendlyTestResponseMobileFriendlinessEnum::MOBILEFRIENDLY => "MOBILE_FRIENDLY",
            RunMobileFriendlyTestResponseMobileFriendlinessEnum::NOTMOBILEFRIENDLY => "NOT_MOBILE_FRIENDLY",
        }
    }
}

impl std::convert::TryFrom< &str> for RunMobileFriendlyTestResponseMobileFriendlinessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED" => Ok(RunMobileFriendlyTestResponseMobileFriendlinessEnum::MOBILEFRIENDLYTESTRESULTUNSPECIFIED),
           "MOBILE_FRIENDLY" => Ok(RunMobileFriendlyTestResponseMobileFriendlinessEnum::MOBILEFRIENDLY),
           "NOT_MOBILE_FRIENDLY" => Ok(RunMobileFriendlyTestResponseMobileFriendlinessEnum::NOTMOBILEFRIENDLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RunMobileFriendlyTestResponseMobileFriendlinessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchAnalyticsQueryRequestAggregationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Optional; Default is \"auto\"] How data is aggregated. If aggregated by property, all data for the same property is aggregated; if aggregated by page, all data is aggregated by canonical URI. If you filter or group by page, choose AUTO; otherwise you can aggregate either by property or by page, depending on how you want your data calculated; see the help documentation to learn how data is calculated differently by site versus by page. **Note:** If you group or filter by page, you cannot aggregate by property. If you specify any value other than AUTO, the aggregation type in the result will match the requested type, or if you request an invalid type, you will get an error. The API will never change your aggregation type if the requested type is invalid.
pub enum SearchAnalyticsQueryRequestAggregationTypeEnum {
    
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
    
    /// "BY_PROPERTY"
    #[serde(rename="BY_PROPERTY")]
    BYPROPERTY,
    
    /// "BY_PAGE"
    #[serde(rename="BY_PAGE")]
    BYPAGE,
    
    /// "BY_NEWS_SHOWCASE_PANEL"
    #[serde(rename="BY_NEWS_SHOWCASE_PANEL")]
    BYNEWSSHOWCASEPANEL,
}

impl AsRef<str> for SearchAnalyticsQueryRequestAggregationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchAnalyticsQueryRequestAggregationTypeEnum::AUTO => "AUTO",
            SearchAnalyticsQueryRequestAggregationTypeEnum::BYPROPERTY => "BY_PROPERTY",
            SearchAnalyticsQueryRequestAggregationTypeEnum::BYPAGE => "BY_PAGE",
            SearchAnalyticsQueryRequestAggregationTypeEnum::BYNEWSSHOWCASEPANEL => "BY_NEWS_SHOWCASE_PANEL",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchAnalyticsQueryRequestAggregationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTO" => Ok(SearchAnalyticsQueryRequestAggregationTypeEnum::AUTO),
           "BY_PROPERTY" => Ok(SearchAnalyticsQueryRequestAggregationTypeEnum::BYPROPERTY),
           "BY_PAGE" => Ok(SearchAnalyticsQueryRequestAggregationTypeEnum::BYPAGE),
           "BY_NEWS_SHOWCASE_PANEL" => Ok(SearchAnalyticsQueryRequestAggregationTypeEnum::BYNEWSSHOWCASEPANEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchAnalyticsQueryRequestAggregationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchAnalyticsQueryRequestDataStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The data state to be fetched, can be full or all, the latter including full and partial data.
pub enum SearchAnalyticsQueryRequestDataStateEnum {
    

    /// Default value, should not be used.
    ///
    /// "DATA_STATE_UNSPECIFIED"
    #[serde(rename="DATA_STATE_UNSPECIFIED")]
    DATASTATEUNSPECIFIED,
    

    /// Include full final data only, without partial.
    ///
    /// "FINAL"
    #[serde(rename="FINAL")]
    FINAL,
    

    /// Include all data, full and partial.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
}

impl AsRef<str> for SearchAnalyticsQueryRequestDataStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchAnalyticsQueryRequestDataStateEnum::DATASTATEUNSPECIFIED => "DATA_STATE_UNSPECIFIED",
            SearchAnalyticsQueryRequestDataStateEnum::FINAL => "FINAL",
            SearchAnalyticsQueryRequestDataStateEnum::ALL => "ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchAnalyticsQueryRequestDataStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_STATE_UNSPECIFIED" => Ok(SearchAnalyticsQueryRequestDataStateEnum::DATASTATEUNSPECIFIED),
           "FINAL" => Ok(SearchAnalyticsQueryRequestDataStateEnum::FINAL),
           "ALL" => Ok(SearchAnalyticsQueryRequestDataStateEnum::ALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchAnalyticsQueryRequestDataStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchAnalyticsQueryRequestDimensionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Optional] Zero or more dimensions to group results by. Dimensions are the group-by values in the Search Analytics page. Dimensions are combined to create a unique row key for each row. Results are grouped in the order that you supply these dimensions.
pub enum SearchAnalyticsQueryRequestDimensionsEnum {
    
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    
    /// "QUERY"
    #[serde(rename="QUERY")]
    QUERY,
    
    /// "PAGE"
    #[serde(rename="PAGE")]
    PAGE,
    
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    
    /// "DEVICE"
    #[serde(rename="DEVICE")]
    DEVICE,
    
    /// "SEARCH_APPEARANCE"
    #[serde(rename="SEARCH_APPEARANCE")]
    SEARCHAPPEARANCE,
}

impl AsRef<str> for SearchAnalyticsQueryRequestDimensionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchAnalyticsQueryRequestDimensionsEnum::DATE => "DATE",
            SearchAnalyticsQueryRequestDimensionsEnum::QUERY => "QUERY",
            SearchAnalyticsQueryRequestDimensionsEnum::PAGE => "PAGE",
            SearchAnalyticsQueryRequestDimensionsEnum::COUNTRY => "COUNTRY",
            SearchAnalyticsQueryRequestDimensionsEnum::DEVICE => "DEVICE",
            SearchAnalyticsQueryRequestDimensionsEnum::SEARCHAPPEARANCE => "SEARCH_APPEARANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchAnalyticsQueryRequestDimensionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATE" => Ok(SearchAnalyticsQueryRequestDimensionsEnum::DATE),
           "QUERY" => Ok(SearchAnalyticsQueryRequestDimensionsEnum::QUERY),
           "PAGE" => Ok(SearchAnalyticsQueryRequestDimensionsEnum::PAGE),
           "COUNTRY" => Ok(SearchAnalyticsQueryRequestDimensionsEnum::COUNTRY),
           "DEVICE" => Ok(SearchAnalyticsQueryRequestDimensionsEnum::DEVICE),
           "SEARCH_APPEARANCE" => Ok(SearchAnalyticsQueryRequestDimensionsEnum::SEARCHAPPEARANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchAnalyticsQueryRequestDimensionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchAnalyticsQueryRequestSearchTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Optional; Default is \"web\"] The search type to filter for.
pub enum SearchAnalyticsQueryRequestSearchTypeEnum {
    
    /// "WEB"
    #[serde(rename="WEB")]
    WEB,
    
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// News tab in search.
    ///
    /// "NEWS"
    #[serde(rename="NEWS")]
    NEWS,
    

    /// Discover.
    ///
    /// "DISCOVER"
    #[serde(rename="DISCOVER")]
    DISCOVER,
    

    /// Google News (news.google.com or mobile app).
    ///
    /// "GOOGLE_NEWS"
    #[serde(rename="GOOGLE_NEWS")]
    GOOGLENEWS,
}

impl AsRef<str> for SearchAnalyticsQueryRequestSearchTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchAnalyticsQueryRequestSearchTypeEnum::WEB => "WEB",
            SearchAnalyticsQueryRequestSearchTypeEnum::IMAGE => "IMAGE",
            SearchAnalyticsQueryRequestSearchTypeEnum::VIDEO => "VIDEO",
            SearchAnalyticsQueryRequestSearchTypeEnum::NEWS => "NEWS",
            SearchAnalyticsQueryRequestSearchTypeEnum::DISCOVER => "DISCOVER",
            SearchAnalyticsQueryRequestSearchTypeEnum::GOOGLENEWS => "GOOGLE_NEWS",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchAnalyticsQueryRequestSearchTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEB" => Ok(SearchAnalyticsQueryRequestSearchTypeEnum::WEB),
           "IMAGE" => Ok(SearchAnalyticsQueryRequestSearchTypeEnum::IMAGE),
           "VIDEO" => Ok(SearchAnalyticsQueryRequestSearchTypeEnum::VIDEO),
           "NEWS" => Ok(SearchAnalyticsQueryRequestSearchTypeEnum::NEWS),
           "DISCOVER" => Ok(SearchAnalyticsQueryRequestSearchTypeEnum::DISCOVER),
           "GOOGLE_NEWS" => Ok(SearchAnalyticsQueryRequestSearchTypeEnum::GOOGLENEWS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchAnalyticsQueryRequestSearchTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchAnalyticsQueryRequestTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. [Optional; Default is \"web\"] Type of report: search type, or either Discover or Gnews.
pub enum SearchAnalyticsQueryRequestTypeEnum {
    
    /// "WEB"
    #[serde(rename="WEB")]
    WEB,
    
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// News tab in search.
    ///
    /// "NEWS"
    #[serde(rename="NEWS")]
    NEWS,
    

    /// Discover.
    ///
    /// "DISCOVER"
    #[serde(rename="DISCOVER")]
    DISCOVER,
    

    /// Google News (news.google.com or mobile app).
    ///
    /// "GOOGLE_NEWS"
    #[serde(rename="GOOGLE_NEWS")]
    GOOGLENEWS,
}

impl AsRef<str> for SearchAnalyticsQueryRequestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchAnalyticsQueryRequestTypeEnum::WEB => "WEB",
            SearchAnalyticsQueryRequestTypeEnum::IMAGE => "IMAGE",
            SearchAnalyticsQueryRequestTypeEnum::VIDEO => "VIDEO",
            SearchAnalyticsQueryRequestTypeEnum::NEWS => "NEWS",
            SearchAnalyticsQueryRequestTypeEnum::DISCOVER => "DISCOVER",
            SearchAnalyticsQueryRequestTypeEnum::GOOGLENEWS => "GOOGLE_NEWS",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchAnalyticsQueryRequestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEB" => Ok(SearchAnalyticsQueryRequestTypeEnum::WEB),
           "IMAGE" => Ok(SearchAnalyticsQueryRequestTypeEnum::IMAGE),
           "VIDEO" => Ok(SearchAnalyticsQueryRequestTypeEnum::VIDEO),
           "NEWS" => Ok(SearchAnalyticsQueryRequestTypeEnum::NEWS),
           "DISCOVER" => Ok(SearchAnalyticsQueryRequestTypeEnum::DISCOVER),
           "GOOGLE_NEWS" => Ok(SearchAnalyticsQueryRequestTypeEnum::GOOGLENEWS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchAnalyticsQueryRequestTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchAnalyticsQueryResponseResponseAggregationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the results were aggregated.
pub enum SearchAnalyticsQueryResponseResponseAggregationTypeEnum {
    
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
    
    /// "BY_PROPERTY"
    #[serde(rename="BY_PROPERTY")]
    BYPROPERTY,
    
    /// "BY_PAGE"
    #[serde(rename="BY_PAGE")]
    BYPAGE,
    
    /// "BY_NEWS_SHOWCASE_PANEL"
    #[serde(rename="BY_NEWS_SHOWCASE_PANEL")]
    BYNEWSSHOWCASEPANEL,
}

impl AsRef<str> for SearchAnalyticsQueryResponseResponseAggregationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchAnalyticsQueryResponseResponseAggregationTypeEnum::AUTO => "AUTO",
            SearchAnalyticsQueryResponseResponseAggregationTypeEnum::BYPROPERTY => "BY_PROPERTY",
            SearchAnalyticsQueryResponseResponseAggregationTypeEnum::BYPAGE => "BY_PAGE",
            SearchAnalyticsQueryResponseResponseAggregationTypeEnum::BYNEWSSHOWCASEPANEL => "BY_NEWS_SHOWCASE_PANEL",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchAnalyticsQueryResponseResponseAggregationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTO" => Ok(SearchAnalyticsQueryResponseResponseAggregationTypeEnum::AUTO),
           "BY_PROPERTY" => Ok(SearchAnalyticsQueryResponseResponseAggregationTypeEnum::BYPROPERTY),
           "BY_PAGE" => Ok(SearchAnalyticsQueryResponseResponseAggregationTypeEnum::BYPAGE),
           "BY_NEWS_SHOWCASE_PANEL" => Ok(SearchAnalyticsQueryResponseResponseAggregationTypeEnum::BYNEWSSHOWCASEPANEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchAnalyticsQueryResponseResponseAggregationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TestStatusStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the test.
pub enum TestStatusStatusEnum {
    

    /// Internal error when running this test. Please try running the test again.
    ///
    /// "TEST_STATUS_UNSPECIFIED"
    #[serde(rename="TEST_STATUS_UNSPECIFIED")]
    TESTSTATUSUNSPECIFIED,
    

    /// Inspection has completed without errors.
    ///
    /// "COMPLETE"
    #[serde(rename="COMPLETE")]
    COMPLETE,
    

    /// Inspection terminated in an error state. This indicates a problem in Google's infrastructure, not a user error. Please try again later.
    ///
    /// "INTERNAL_ERROR"
    #[serde(rename="INTERNAL_ERROR")]
    INTERNALERROR,
    

    /// Google can not access the URL because of a user error such as a robots.txt blockage, a 403 or 500 code etc. Please make sure that the URL provided is accessible by Googlebot and is not password protected.
    ///
    /// "PAGE_UNREACHABLE"
    #[serde(rename="PAGE_UNREACHABLE")]
    PAGEUNREACHABLE,
}

impl AsRef<str> for TestStatusStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TestStatusStatusEnum::TESTSTATUSUNSPECIFIED => "TEST_STATUS_UNSPECIFIED",
            TestStatusStatusEnum::COMPLETE => "COMPLETE",
            TestStatusStatusEnum::INTERNALERROR => "INTERNAL_ERROR",
            TestStatusStatusEnum::PAGEUNREACHABLE => "PAGE_UNREACHABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for TestStatusStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEST_STATUS_UNSPECIFIED" => Ok(TestStatusStatusEnum::TESTSTATUSUNSPECIFIED),
           "COMPLETE" => Ok(TestStatusStatusEnum::COMPLETE),
           "INTERNAL_ERROR" => Ok(TestStatusStatusEnum::INTERNALERROR),
           "PAGE_UNREACHABLE" => Ok(TestStatusStatusEnum::PAGEUNREACHABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TestStatusStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WmxSitePermissionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The user's permission level for the site.
pub enum WmxSitePermissionLevelEnum {
    
    /// "SITE_PERMISSION_LEVEL_UNSPECIFIED"
    #[serde(rename="SITE_PERMISSION_LEVEL_UNSPECIFIED")]
    SITEPERMISSIONLEVELUNSPECIFIED,
    

    /// Owner has complete access to the site.
    ///
    /// "SITE_OWNER"
    #[serde(rename="SITE_OWNER")]
    SITEOWNER,
    

    /// Full users can access all data, and perform most of the operations.
    ///
    /// "SITE_FULL_USER"
    #[serde(rename="SITE_FULL_USER")]
    SITEFULLUSER,
    

    /// Restricted users can access most of the data, and perform some operations.
    ///
    /// "SITE_RESTRICTED_USER"
    #[serde(rename="SITE_RESTRICTED_USER")]
    SITERESTRICTEDUSER,
    

    /// Unverified user has no access to site's data.
    ///
    /// "SITE_UNVERIFIED_USER"
    #[serde(rename="SITE_UNVERIFIED_USER")]
    SITEUNVERIFIEDUSER,
}

impl AsRef<str> for WmxSitePermissionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WmxSitePermissionLevelEnum::SITEPERMISSIONLEVELUNSPECIFIED => "SITE_PERMISSION_LEVEL_UNSPECIFIED",
            WmxSitePermissionLevelEnum::SITEOWNER => "SITE_OWNER",
            WmxSitePermissionLevelEnum::SITEFULLUSER => "SITE_FULL_USER",
            WmxSitePermissionLevelEnum::SITERESTRICTEDUSER => "SITE_RESTRICTED_USER",
            WmxSitePermissionLevelEnum::SITEUNVERIFIEDUSER => "SITE_UNVERIFIED_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for WmxSitePermissionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SITE_PERMISSION_LEVEL_UNSPECIFIED" => Ok(WmxSitePermissionLevelEnum::SITEPERMISSIONLEVELUNSPECIFIED),
           "SITE_OWNER" => Ok(WmxSitePermissionLevelEnum::SITEOWNER),
           "SITE_FULL_USER" => Ok(WmxSitePermissionLevelEnum::SITEFULLUSER),
           "SITE_RESTRICTED_USER" => Ok(WmxSitePermissionLevelEnum::SITERESTRICTEDUSER),
           "SITE_UNVERIFIED_USER" => Ok(WmxSitePermissionLevelEnum::SITEUNVERIFIEDUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WmxSitePermissionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WmxSitemapTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the sitemap. For example: `rssFeed`.
pub enum WmxSitemapTypeEnum {
    
    /// "NOT_SITEMAP"
    #[serde(rename="NOT_SITEMAP")]
    NOTSITEMAP,
    
    /// "URL_LIST"
    #[serde(rename="URL_LIST")]
    URLLIST,
    
    /// "SITEMAP"
    #[serde(rename="SITEMAP")]
    SITEMAP,
    
    /// "RSS_FEED"
    #[serde(rename="RSS_FEED")]
    RSSFEED,
    
    /// "ATOM_FEED"
    #[serde(rename="ATOM_FEED")]
    ATOMFEED,
    

    /// Unsupported sitemap types.
    ///
    /// "PATTERN_SITEMAP"
    #[serde(rename="PATTERN_SITEMAP")]
    PATTERNSITEMAP,
    
    /// "OCEANFRONT"
    #[serde(rename="OCEANFRONT")]
    OCEANFRONT,
}

impl AsRef<str> for WmxSitemapTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WmxSitemapTypeEnum::NOTSITEMAP => "NOT_SITEMAP",
            WmxSitemapTypeEnum::URLLIST => "URL_LIST",
            WmxSitemapTypeEnum::SITEMAP => "SITEMAP",
            WmxSitemapTypeEnum::RSSFEED => "RSS_FEED",
            WmxSitemapTypeEnum::ATOMFEED => "ATOM_FEED",
            WmxSitemapTypeEnum::PATTERNSITEMAP => "PATTERN_SITEMAP",
            WmxSitemapTypeEnum::OCEANFRONT => "OCEANFRONT",
        }
    }
}

impl std::convert::TryFrom< &str> for WmxSitemapTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOT_SITEMAP" => Ok(WmxSitemapTypeEnum::NOTSITEMAP),
           "URL_LIST" => Ok(WmxSitemapTypeEnum::URLLIST),
           "SITEMAP" => Ok(WmxSitemapTypeEnum::SITEMAP),
           "RSS_FEED" => Ok(WmxSitemapTypeEnum::RSSFEED),
           "ATOM_FEED" => Ok(WmxSitemapTypeEnum::ATOMFEED),
           "PATTERN_SITEMAP" => Ok(WmxSitemapTypeEnum::PATTERNSITEMAP),
           "OCEANFRONT" => Ok(WmxSitemapTypeEnum::OCEANFRONT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WmxSitemapTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WmxSitemapContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The specific type of content in this sitemap. For example: `web`.
pub enum WmxSitemapContentTypeEnum {
    
    /// "WEB"
    #[serde(rename="WEB")]
    WEB,
    
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    
    /// "NEWS"
    #[serde(rename="NEWS")]
    NEWS,
    
    /// "MOBILE"
    #[serde(rename="MOBILE")]
    MOBILE,
    
    /// "ANDROID_APP"
    #[serde(rename="ANDROID_APP")]
    ANDROIDAPP,
    

    /// Unsupported content type.
    ///
    /// "PATTERN"
    #[serde(rename="PATTERN")]
    PATTERN,
    
    /// "IOS_APP"
    #[serde(rename="IOS_APP")]
    IOSAPP,
    

    /// Unsupported content type.
    ///
    /// "DATA_FEED_ELEMENT"
    #[serde(rename="DATA_FEED_ELEMENT")]
    DATAFEEDELEMENT,
}

impl AsRef<str> for WmxSitemapContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WmxSitemapContentTypeEnum::WEB => "WEB",
            WmxSitemapContentTypeEnum::IMAGE => "IMAGE",
            WmxSitemapContentTypeEnum::VIDEO => "VIDEO",
            WmxSitemapContentTypeEnum::NEWS => "NEWS",
            WmxSitemapContentTypeEnum::MOBILE => "MOBILE",
            WmxSitemapContentTypeEnum::ANDROIDAPP => "ANDROID_APP",
            WmxSitemapContentTypeEnum::PATTERN => "PATTERN",
            WmxSitemapContentTypeEnum::IOSAPP => "IOS_APP",
            WmxSitemapContentTypeEnum::DATAFEEDELEMENT => "DATA_FEED_ELEMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for WmxSitemapContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEB" => Ok(WmxSitemapContentTypeEnum::WEB),
           "IMAGE" => Ok(WmxSitemapContentTypeEnum::IMAGE),
           "VIDEO" => Ok(WmxSitemapContentTypeEnum::VIDEO),
           "NEWS" => Ok(WmxSitemapContentTypeEnum::NEWS),
           "MOBILE" => Ok(WmxSitemapContentTypeEnum::MOBILE),
           "ANDROID_APP" => Ok(WmxSitemapContentTypeEnum::ANDROIDAPP),
           "PATTERN" => Ok(WmxSitemapContentTypeEnum::PATTERN),
           "IOS_APP" => Ok(WmxSitemapContentTypeEnum::IOSAPP),
           "DATA_FEED_ELEMENT" => Ok(WmxSitemapContentTypeEnum::DATAFEEDELEMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WmxSitemapContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


