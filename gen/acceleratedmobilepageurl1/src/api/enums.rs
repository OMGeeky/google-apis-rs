use super::*;



// region AmpUrlErrorErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The error code of an API call.
pub enum AmpUrlErrorErrorCodeEnum {
    

    /// Not specified error.
    ///
    /// "ERROR_CODE_UNSPECIFIED"
    #[serde(rename="ERROR_CODE_UNSPECIFIED")]
    ERRORCODEUNSPECIFIED,
    

    /// Indicates the requested URL is not found in the index, possibly because it's unable to be found, not able to be accessed by Googlebot, or some other error.
    ///
    /// "INPUT_URL_NOT_FOUND"
    #[serde(rename="INPUT_URL_NOT_FOUND")]
    INPUTURLNOTFOUND,
    

    /// Indicates no AMP URL has been found that corresponds to the requested URL.
    ///
    /// "NO_AMP_URL"
    #[serde(rename="NO_AMP_URL")]
    NOAMPURL,
    

    /// Indicates some kind of application error occurred at the server. Client advised to retry.
    ///
    /// "APPLICATION_ERROR"
    #[serde(rename="APPLICATION_ERROR")]
    APPLICATIONERROR,
    

    /// DEPRECATED: Indicates the requested URL is a valid AMP URL. This is a non-error state, should not be relied upon as a sign of success or failure. It will be removed in future versions of the API.
    ///
    /// "URL_IS_VALID_AMP"
    #[serde(rename="URL_IS_VALID_AMP")]
    URLISVALIDAMP,
    

    /// Indicates that an AMP URL has been found that corresponds to the request URL, but it is not valid AMP HTML.
    ///
    /// "URL_IS_INVALID_AMP"
    #[serde(rename="URL_IS_INVALID_AMP")]
    URLISINVALIDAMP,
}

impl AsRef<str> for AmpUrlErrorErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AmpUrlErrorErrorCodeEnum::ERRORCODEUNSPECIFIED => "ERROR_CODE_UNSPECIFIED",
            AmpUrlErrorErrorCodeEnum::INPUTURLNOTFOUND => "INPUT_URL_NOT_FOUND",
            AmpUrlErrorErrorCodeEnum::NOAMPURL => "NO_AMP_URL",
            AmpUrlErrorErrorCodeEnum::APPLICATIONERROR => "APPLICATION_ERROR",
            AmpUrlErrorErrorCodeEnum::URLISVALIDAMP => "URL_IS_VALID_AMP",
            AmpUrlErrorErrorCodeEnum::URLISINVALIDAMP => "URL_IS_INVALID_AMP",
        }
    }
}

impl std::convert::TryFrom< &str> for AmpUrlErrorErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_CODE_UNSPECIFIED" => Ok(AmpUrlErrorErrorCodeEnum::ERRORCODEUNSPECIFIED),
           "INPUT_URL_NOT_FOUND" => Ok(AmpUrlErrorErrorCodeEnum::INPUTURLNOTFOUND),
           "NO_AMP_URL" => Ok(AmpUrlErrorErrorCodeEnum::NOAMPURL),
           "APPLICATION_ERROR" => Ok(AmpUrlErrorErrorCodeEnum::APPLICATIONERROR),
           "URL_IS_VALID_AMP" => Ok(AmpUrlErrorErrorCodeEnum::URLISVALIDAMP),
           "URL_IS_INVALID_AMP" => Ok(AmpUrlErrorErrorCodeEnum::URLISINVALIDAMP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AmpUrlErrorErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchGetAmpUrlsRequestLookupStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The lookup_strategy being requested.
pub enum BatchGetAmpUrlsRequestLookupStrategyEnum {
    

    /// FETCH_LIVE_DOC strategy involves live document fetch of URLs not found in the index. Any request URL not found in the index is crawled in realtime to validate if there is a corresponding AMP URL. This strategy has higher coverage but with extra latency introduced by realtime crawling. This is the default strategy. Applications using this strategy should set higher HTTP timeouts of the API calls.
    ///
    /// "FETCH_LIVE_DOC"
    #[serde(rename="FETCH_LIVE_DOC")]
    FETCHLIVEDOC,
    

    /// IN_INDEX_DOC strategy skips fetching live documents of URL(s) not found in index. For applications which need low latency use of IN_INDEX_DOC strategy is recommended.
    ///
    /// "IN_INDEX_DOC"
    #[serde(rename="IN_INDEX_DOC")]
    ININDEXDOC,
}

impl AsRef<str> for BatchGetAmpUrlsRequestLookupStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchGetAmpUrlsRequestLookupStrategyEnum::FETCHLIVEDOC => "FETCH_LIVE_DOC",
            BatchGetAmpUrlsRequestLookupStrategyEnum::ININDEXDOC => "IN_INDEX_DOC",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchGetAmpUrlsRequestLookupStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FETCH_LIVE_DOC" => Ok(BatchGetAmpUrlsRequestLookupStrategyEnum::FETCHLIVEDOC),
           "IN_INDEX_DOC" => Ok(BatchGetAmpUrlsRequestLookupStrategyEnum::ININDEXDOC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchGetAmpUrlsRequestLookupStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


