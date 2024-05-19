use super::*;



// region BulkCheckResponseBulkErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Error code for the entire request. Present only if the entire request failed. Individual check errors will not trigger the presence of this field.
pub enum BulkCheckResponseBulkErrorCodeEnum {
    
    /// "ERROR_CODE_UNSPECIFIED"
    #[serde(rename="ERROR_CODE_UNSPECIFIED")]
    ERRORCODEUNSPECIFIED,
    

    /// Unable to parse query.
    ///
    /// "ERROR_CODE_INVALID_QUERY"
    #[serde(rename="ERROR_CODE_INVALID_QUERY")]
    ERRORCODEINVALIDQUERY,
    

    /// Unable to fetch the asset links data.
    ///
    /// "ERROR_CODE_FETCH_ERROR"
    #[serde(rename="ERROR_CODE_FETCH_ERROR")]
    ERRORCODEFETCHERROR,
    

    /// Invalid HTTPS certificate .
    ///
    /// "ERROR_CODE_FAILED_SSL_VALIDATION"
    #[serde(rename="ERROR_CODE_FAILED_SSL_VALIDATION")]
    ERRORCODEFAILEDSSLVALIDATION,
    

    /// HTTP redirects (e.g, 301) are not allowed.
    ///
    /// "ERROR_CODE_REDIRECT"
    #[serde(rename="ERROR_CODE_REDIRECT")]
    ERRORCODEREDIRECT,
    

    /// Asset links data exceeds maximum size.
    ///
    /// "ERROR_CODE_TOO_LARGE"
    #[serde(rename="ERROR_CODE_TOO_LARGE")]
    ERRORCODETOOLARGE,
    

    /// Can't parse HTTP response.
    ///
    /// "ERROR_CODE_MALFORMED_HTTP_RESPONSE"
    #[serde(rename="ERROR_CODE_MALFORMED_HTTP_RESPONSE")]
    ERRORCODEMALFORMEDHTTPRESPONSE,
    

    /// HTTP Content-type should be application/json.
    ///
    /// "ERROR_CODE_WRONG_CONTENT_TYPE"
    #[serde(rename="ERROR_CODE_WRONG_CONTENT_TYPE")]
    ERRORCODEWRONGCONTENTTYPE,
    

    /// JSON content is malformed.
    ///
    /// "ERROR_CODE_MALFORMED_CONTENT"
    #[serde(rename="ERROR_CODE_MALFORMED_CONTENT")]
    ERRORCODEMALFORMEDCONTENT,
    

    /// A secure asset includes an insecure asset (security downgrade).
    ///
    /// "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE"
    #[serde(rename="ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE")]
    ERRORCODESECUREASSETINCLUDESINSECURE,
    

    /// Too many includes (maybe a loop).
    ///
    /// "ERROR_CODE_FETCH_BUDGET_EXHAUSTED"
    #[serde(rename="ERROR_CODE_FETCH_BUDGET_EXHAUSTED")]
    ERRORCODEFETCHBUDGETEXHAUSTED,
}

impl AsRef<str> for BulkCheckResponseBulkErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODEUNSPECIFIED => "ERROR_CODE_UNSPECIFIED",
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODEINVALIDQUERY => "ERROR_CODE_INVALID_QUERY",
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODEFETCHERROR => "ERROR_CODE_FETCH_ERROR",
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODEFAILEDSSLVALIDATION => "ERROR_CODE_FAILED_SSL_VALIDATION",
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODEREDIRECT => "ERROR_CODE_REDIRECT",
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODETOOLARGE => "ERROR_CODE_TOO_LARGE",
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODEMALFORMEDHTTPRESPONSE => "ERROR_CODE_MALFORMED_HTTP_RESPONSE",
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODEWRONGCONTENTTYPE => "ERROR_CODE_WRONG_CONTENT_TYPE",
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODEMALFORMEDCONTENT => "ERROR_CODE_MALFORMED_CONTENT",
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODESECUREASSETINCLUDESINSECURE => "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE",
            BulkCheckResponseBulkErrorCodeEnum::ERRORCODEFETCHBUDGETEXHAUSTED => "ERROR_CODE_FETCH_BUDGET_EXHAUSTED",
        }
    }
}

impl std::convert::TryFrom< &str> for BulkCheckResponseBulkErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_CODE_UNSPECIFIED" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODEUNSPECIFIED),
           "ERROR_CODE_INVALID_QUERY" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODEINVALIDQUERY),
           "ERROR_CODE_FETCH_ERROR" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODEFETCHERROR),
           "ERROR_CODE_FAILED_SSL_VALIDATION" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODEFAILEDSSLVALIDATION),
           "ERROR_CODE_REDIRECT" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODEREDIRECT),
           "ERROR_CODE_TOO_LARGE" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODETOOLARGE),
           "ERROR_CODE_MALFORMED_HTTP_RESPONSE" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODEMALFORMEDHTTPRESPONSE),
           "ERROR_CODE_WRONG_CONTENT_TYPE" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODEWRONGCONTENTTYPE),
           "ERROR_CODE_MALFORMED_CONTENT" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODEMALFORMEDCONTENT),
           "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODESECUREASSETINCLUDESINSECURE),
           "ERROR_CODE_FETCH_BUDGET_EXHAUSTED" => Ok(BulkCheckResponseBulkErrorCodeEnum::ERRORCODEFETCHBUDGETEXHAUSTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BulkCheckResponseBulkErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CheckResponseErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Error codes that describe the result of the Check operation.
pub enum CheckResponseErrorCodeEnum {
    
    /// "ERROR_CODE_UNSPECIFIED"
    #[serde(rename="ERROR_CODE_UNSPECIFIED")]
    ERRORCODEUNSPECIFIED,
    

    /// Unable to parse query.
    ///
    /// "ERROR_CODE_INVALID_QUERY"
    #[serde(rename="ERROR_CODE_INVALID_QUERY")]
    ERRORCODEINVALIDQUERY,
    

    /// Unable to fetch the asset links data.
    ///
    /// "ERROR_CODE_FETCH_ERROR"
    #[serde(rename="ERROR_CODE_FETCH_ERROR")]
    ERRORCODEFETCHERROR,
    

    /// Invalid HTTPS certificate .
    ///
    /// "ERROR_CODE_FAILED_SSL_VALIDATION"
    #[serde(rename="ERROR_CODE_FAILED_SSL_VALIDATION")]
    ERRORCODEFAILEDSSLVALIDATION,
    

    /// HTTP redirects (e.g, 301) are not allowed.
    ///
    /// "ERROR_CODE_REDIRECT"
    #[serde(rename="ERROR_CODE_REDIRECT")]
    ERRORCODEREDIRECT,
    

    /// Asset links data exceeds maximum size.
    ///
    /// "ERROR_CODE_TOO_LARGE"
    #[serde(rename="ERROR_CODE_TOO_LARGE")]
    ERRORCODETOOLARGE,
    

    /// Can't parse HTTP response.
    ///
    /// "ERROR_CODE_MALFORMED_HTTP_RESPONSE"
    #[serde(rename="ERROR_CODE_MALFORMED_HTTP_RESPONSE")]
    ERRORCODEMALFORMEDHTTPRESPONSE,
    

    /// HTTP Content-type should be application/json.
    ///
    /// "ERROR_CODE_WRONG_CONTENT_TYPE"
    #[serde(rename="ERROR_CODE_WRONG_CONTENT_TYPE")]
    ERRORCODEWRONGCONTENTTYPE,
    

    /// JSON content is malformed.
    ///
    /// "ERROR_CODE_MALFORMED_CONTENT"
    #[serde(rename="ERROR_CODE_MALFORMED_CONTENT")]
    ERRORCODEMALFORMEDCONTENT,
    

    /// A secure asset includes an insecure asset (security downgrade).
    ///
    /// "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE"
    #[serde(rename="ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE")]
    ERRORCODESECUREASSETINCLUDESINSECURE,
    

    /// Too many includes (maybe a loop).
    ///
    /// "ERROR_CODE_FETCH_BUDGET_EXHAUSTED"
    #[serde(rename="ERROR_CODE_FETCH_BUDGET_EXHAUSTED")]
    ERRORCODEFETCHBUDGETEXHAUSTED,
}

impl AsRef<str> for CheckResponseErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CheckResponseErrorCodeEnum::ERRORCODEUNSPECIFIED => "ERROR_CODE_UNSPECIFIED",
            CheckResponseErrorCodeEnum::ERRORCODEINVALIDQUERY => "ERROR_CODE_INVALID_QUERY",
            CheckResponseErrorCodeEnum::ERRORCODEFETCHERROR => "ERROR_CODE_FETCH_ERROR",
            CheckResponseErrorCodeEnum::ERRORCODEFAILEDSSLVALIDATION => "ERROR_CODE_FAILED_SSL_VALIDATION",
            CheckResponseErrorCodeEnum::ERRORCODEREDIRECT => "ERROR_CODE_REDIRECT",
            CheckResponseErrorCodeEnum::ERRORCODETOOLARGE => "ERROR_CODE_TOO_LARGE",
            CheckResponseErrorCodeEnum::ERRORCODEMALFORMEDHTTPRESPONSE => "ERROR_CODE_MALFORMED_HTTP_RESPONSE",
            CheckResponseErrorCodeEnum::ERRORCODEWRONGCONTENTTYPE => "ERROR_CODE_WRONG_CONTENT_TYPE",
            CheckResponseErrorCodeEnum::ERRORCODEMALFORMEDCONTENT => "ERROR_CODE_MALFORMED_CONTENT",
            CheckResponseErrorCodeEnum::ERRORCODESECUREASSETINCLUDESINSECURE => "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE",
            CheckResponseErrorCodeEnum::ERRORCODEFETCHBUDGETEXHAUSTED => "ERROR_CODE_FETCH_BUDGET_EXHAUSTED",
        }
    }
}

impl std::convert::TryFrom< &str> for CheckResponseErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_CODE_UNSPECIFIED" => Ok(CheckResponseErrorCodeEnum::ERRORCODEUNSPECIFIED),
           "ERROR_CODE_INVALID_QUERY" => Ok(CheckResponseErrorCodeEnum::ERRORCODEINVALIDQUERY),
           "ERROR_CODE_FETCH_ERROR" => Ok(CheckResponseErrorCodeEnum::ERRORCODEFETCHERROR),
           "ERROR_CODE_FAILED_SSL_VALIDATION" => Ok(CheckResponseErrorCodeEnum::ERRORCODEFAILEDSSLVALIDATION),
           "ERROR_CODE_REDIRECT" => Ok(CheckResponseErrorCodeEnum::ERRORCODEREDIRECT),
           "ERROR_CODE_TOO_LARGE" => Ok(CheckResponseErrorCodeEnum::ERRORCODETOOLARGE),
           "ERROR_CODE_MALFORMED_HTTP_RESPONSE" => Ok(CheckResponseErrorCodeEnum::ERRORCODEMALFORMEDHTTPRESPONSE),
           "ERROR_CODE_WRONG_CONTENT_TYPE" => Ok(CheckResponseErrorCodeEnum::ERRORCODEWRONGCONTENTTYPE),
           "ERROR_CODE_MALFORMED_CONTENT" => Ok(CheckResponseErrorCodeEnum::ERRORCODEMALFORMEDCONTENT),
           "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE" => Ok(CheckResponseErrorCodeEnum::ERRORCODESECUREASSETINCLUDESINSECURE),
           "ERROR_CODE_FETCH_BUDGET_EXHAUSTED" => Ok(CheckResponseErrorCodeEnum::ERRORCODEFETCHBUDGETEXHAUSTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CheckResponseErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ListResponseErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Error codes that describe the result of the List operation.
pub enum ListResponseErrorCodeEnum {
    
    /// "ERROR_CODE_UNSPECIFIED"
    #[serde(rename="ERROR_CODE_UNSPECIFIED")]
    ERRORCODEUNSPECIFIED,
    

    /// Unable to parse query.
    ///
    /// "ERROR_CODE_INVALID_QUERY"
    #[serde(rename="ERROR_CODE_INVALID_QUERY")]
    ERRORCODEINVALIDQUERY,
    

    /// Unable to fetch the asset links data.
    ///
    /// "ERROR_CODE_FETCH_ERROR"
    #[serde(rename="ERROR_CODE_FETCH_ERROR")]
    ERRORCODEFETCHERROR,
    

    /// Invalid HTTPS certificate .
    ///
    /// "ERROR_CODE_FAILED_SSL_VALIDATION"
    #[serde(rename="ERROR_CODE_FAILED_SSL_VALIDATION")]
    ERRORCODEFAILEDSSLVALIDATION,
    

    /// HTTP redirects (e.g, 301) are not allowed.
    ///
    /// "ERROR_CODE_REDIRECT"
    #[serde(rename="ERROR_CODE_REDIRECT")]
    ERRORCODEREDIRECT,
    

    /// Asset links data exceeds maximum size.
    ///
    /// "ERROR_CODE_TOO_LARGE"
    #[serde(rename="ERROR_CODE_TOO_LARGE")]
    ERRORCODETOOLARGE,
    

    /// Can't parse HTTP response.
    ///
    /// "ERROR_CODE_MALFORMED_HTTP_RESPONSE"
    #[serde(rename="ERROR_CODE_MALFORMED_HTTP_RESPONSE")]
    ERRORCODEMALFORMEDHTTPRESPONSE,
    

    /// HTTP Content-type should be application/json.
    ///
    /// "ERROR_CODE_WRONG_CONTENT_TYPE"
    #[serde(rename="ERROR_CODE_WRONG_CONTENT_TYPE")]
    ERRORCODEWRONGCONTENTTYPE,
    

    /// JSON content is malformed.
    ///
    /// "ERROR_CODE_MALFORMED_CONTENT"
    #[serde(rename="ERROR_CODE_MALFORMED_CONTENT")]
    ERRORCODEMALFORMEDCONTENT,
    

    /// A secure asset includes an insecure asset (security downgrade).
    ///
    /// "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE"
    #[serde(rename="ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE")]
    ERRORCODESECUREASSETINCLUDESINSECURE,
    

    /// Too many includes (maybe a loop).
    ///
    /// "ERROR_CODE_FETCH_BUDGET_EXHAUSTED"
    #[serde(rename="ERROR_CODE_FETCH_BUDGET_EXHAUSTED")]
    ERRORCODEFETCHBUDGETEXHAUSTED,
}

impl AsRef<str> for ListResponseErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ListResponseErrorCodeEnum::ERRORCODEUNSPECIFIED => "ERROR_CODE_UNSPECIFIED",
            ListResponseErrorCodeEnum::ERRORCODEINVALIDQUERY => "ERROR_CODE_INVALID_QUERY",
            ListResponseErrorCodeEnum::ERRORCODEFETCHERROR => "ERROR_CODE_FETCH_ERROR",
            ListResponseErrorCodeEnum::ERRORCODEFAILEDSSLVALIDATION => "ERROR_CODE_FAILED_SSL_VALIDATION",
            ListResponseErrorCodeEnum::ERRORCODEREDIRECT => "ERROR_CODE_REDIRECT",
            ListResponseErrorCodeEnum::ERRORCODETOOLARGE => "ERROR_CODE_TOO_LARGE",
            ListResponseErrorCodeEnum::ERRORCODEMALFORMEDHTTPRESPONSE => "ERROR_CODE_MALFORMED_HTTP_RESPONSE",
            ListResponseErrorCodeEnum::ERRORCODEWRONGCONTENTTYPE => "ERROR_CODE_WRONG_CONTENT_TYPE",
            ListResponseErrorCodeEnum::ERRORCODEMALFORMEDCONTENT => "ERROR_CODE_MALFORMED_CONTENT",
            ListResponseErrorCodeEnum::ERRORCODESECUREASSETINCLUDESINSECURE => "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE",
            ListResponseErrorCodeEnum::ERRORCODEFETCHBUDGETEXHAUSTED => "ERROR_CODE_FETCH_BUDGET_EXHAUSTED",
        }
    }
}

impl std::convert::TryFrom< &str> for ListResponseErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_CODE_UNSPECIFIED" => Ok(ListResponseErrorCodeEnum::ERRORCODEUNSPECIFIED),
           "ERROR_CODE_INVALID_QUERY" => Ok(ListResponseErrorCodeEnum::ERRORCODEINVALIDQUERY),
           "ERROR_CODE_FETCH_ERROR" => Ok(ListResponseErrorCodeEnum::ERRORCODEFETCHERROR),
           "ERROR_CODE_FAILED_SSL_VALIDATION" => Ok(ListResponseErrorCodeEnum::ERRORCODEFAILEDSSLVALIDATION),
           "ERROR_CODE_REDIRECT" => Ok(ListResponseErrorCodeEnum::ERRORCODEREDIRECT),
           "ERROR_CODE_TOO_LARGE" => Ok(ListResponseErrorCodeEnum::ERRORCODETOOLARGE),
           "ERROR_CODE_MALFORMED_HTTP_RESPONSE" => Ok(ListResponseErrorCodeEnum::ERRORCODEMALFORMEDHTTPRESPONSE),
           "ERROR_CODE_WRONG_CONTENT_TYPE" => Ok(ListResponseErrorCodeEnum::ERRORCODEWRONGCONTENTTYPE),
           "ERROR_CODE_MALFORMED_CONTENT" => Ok(ListResponseErrorCodeEnum::ERRORCODEMALFORMEDCONTENT),
           "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE" => Ok(ListResponseErrorCodeEnum::ERRORCODESECUREASSETINCLUDESINSECURE),
           "ERROR_CODE_FETCH_BUDGET_EXHAUSTED" => Ok(ListResponseErrorCodeEnum::ERRORCODEFETCHBUDGETEXHAUSTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ListResponseErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


