use super::*;



// region AppEngineHttpTargetHttpMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The HTTP method to use for the request. PATCH and OPTIONS are not permitted.
pub enum AppEngineHttpTargetHttpMethodEnum {
    

    /// HTTP method unspecified. Defaults to POST.
    ///
    /// "HTTP_METHOD_UNSPECIFIED"
    #[serde(rename="HTTP_METHOD_UNSPECIFIED")]
    HTTPMETHODUNSPECIFIED,
    

    /// HTTP POST
    ///
    /// "POST"
    #[serde(rename="POST")]
    POST,
    

    /// HTTP GET
    ///
    /// "GET"
    #[serde(rename="GET")]
    GET,
    

    /// HTTP HEAD
    ///
    /// "HEAD"
    #[serde(rename="HEAD")]
    HEAD,
    

    /// HTTP PUT
    ///
    /// "PUT"
    #[serde(rename="PUT")]
    PUT,
    

    /// HTTP DELETE
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
    

    /// HTTP PATCH
    ///
    /// "PATCH"
    #[serde(rename="PATCH")]
    PATCH,
    

    /// HTTP OPTIONS
    ///
    /// "OPTIONS"
    #[serde(rename="OPTIONS")]
    OPTIONS,
}

impl AsRef<str> for AppEngineHttpTargetHttpMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppEngineHttpTargetHttpMethodEnum::HTTPMETHODUNSPECIFIED => "HTTP_METHOD_UNSPECIFIED",
            AppEngineHttpTargetHttpMethodEnum::POST => "POST",
            AppEngineHttpTargetHttpMethodEnum::GET => "GET",
            AppEngineHttpTargetHttpMethodEnum::HEAD => "HEAD",
            AppEngineHttpTargetHttpMethodEnum::PUT => "PUT",
            AppEngineHttpTargetHttpMethodEnum::DELETE => "DELETE",
            AppEngineHttpTargetHttpMethodEnum::PATCH => "PATCH",
            AppEngineHttpTargetHttpMethodEnum::OPTIONS => "OPTIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for AppEngineHttpTargetHttpMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HTTP_METHOD_UNSPECIFIED" => Ok(AppEngineHttpTargetHttpMethodEnum::HTTPMETHODUNSPECIFIED),
           "POST" => Ok(AppEngineHttpTargetHttpMethodEnum::POST),
           "GET" => Ok(AppEngineHttpTargetHttpMethodEnum::GET),
           "HEAD" => Ok(AppEngineHttpTargetHttpMethodEnum::HEAD),
           "PUT" => Ok(AppEngineHttpTargetHttpMethodEnum::PUT),
           "DELETE" => Ok(AppEngineHttpTargetHttpMethodEnum::DELETE),
           "PATCH" => Ok(AppEngineHttpTargetHttpMethodEnum::PATCH),
           "OPTIONS" => Ok(AppEngineHttpTargetHttpMethodEnum::OPTIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppEngineHttpTargetHttpMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HttpTargetHttpMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Which HTTP method to use for the request.
pub enum HttpTargetHttpMethodEnum {
    

    /// HTTP method unspecified. Defaults to POST.
    ///
    /// "HTTP_METHOD_UNSPECIFIED"
    #[serde(rename="HTTP_METHOD_UNSPECIFIED")]
    HTTPMETHODUNSPECIFIED,
    

    /// HTTP POST
    ///
    /// "POST"
    #[serde(rename="POST")]
    POST,
    

    /// HTTP GET
    ///
    /// "GET"
    #[serde(rename="GET")]
    GET,
    

    /// HTTP HEAD
    ///
    /// "HEAD"
    #[serde(rename="HEAD")]
    HEAD,
    

    /// HTTP PUT
    ///
    /// "PUT"
    #[serde(rename="PUT")]
    PUT,
    

    /// HTTP DELETE
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
    

    /// HTTP PATCH
    ///
    /// "PATCH"
    #[serde(rename="PATCH")]
    PATCH,
    

    /// HTTP OPTIONS
    ///
    /// "OPTIONS"
    #[serde(rename="OPTIONS")]
    OPTIONS,
}

impl AsRef<str> for HttpTargetHttpMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HttpTargetHttpMethodEnum::HTTPMETHODUNSPECIFIED => "HTTP_METHOD_UNSPECIFIED",
            HttpTargetHttpMethodEnum::POST => "POST",
            HttpTargetHttpMethodEnum::GET => "GET",
            HttpTargetHttpMethodEnum::HEAD => "HEAD",
            HttpTargetHttpMethodEnum::PUT => "PUT",
            HttpTargetHttpMethodEnum::DELETE => "DELETE",
            HttpTargetHttpMethodEnum::PATCH => "PATCH",
            HttpTargetHttpMethodEnum::OPTIONS => "OPTIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for HttpTargetHttpMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HTTP_METHOD_UNSPECIFIED" => Ok(HttpTargetHttpMethodEnum::HTTPMETHODUNSPECIFIED),
           "POST" => Ok(HttpTargetHttpMethodEnum::POST),
           "GET" => Ok(HttpTargetHttpMethodEnum::GET),
           "HEAD" => Ok(HttpTargetHttpMethodEnum::HEAD),
           "PUT" => Ok(HttpTargetHttpMethodEnum::PUT),
           "DELETE" => Ok(HttpTargetHttpMethodEnum::DELETE),
           "PATCH" => Ok(HttpTargetHttpMethodEnum::PATCH),
           "OPTIONS" => Ok(HttpTargetHttpMethodEnum::OPTIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HttpTargetHttpMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the job.
pub enum JobStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The job is executing normally.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// The job is paused by the user. It will not execute. A user can intentionally pause the job using PauseJobRequest.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// The job is disabled by the system due to error. The user cannot directly set a job to be disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// The job state resulting from a failed CloudScheduler.UpdateJob operation. To recover a job from this state, retry CloudScheduler.UpdateJob until a successful response is received.
    ///
    /// "UPDATE_FAILED"
    #[serde(rename="UPDATE_FAILED")]
    UPDATEFAILED,
}

impl AsRef<str> for JobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            JobStateEnum::ENABLED => "ENABLED",
            JobStateEnum::PAUSED => "PAUSED",
            JobStateEnum::DISABLED => "DISABLED",
            JobStateEnum::UPDATEFAILED => "UPDATE_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for JobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(JobStateEnum::STATEUNSPECIFIED),
           "ENABLED" => Ok(JobStateEnum::ENABLED),
           "PAUSED" => Ok(JobStateEnum::PAUSED),
           "DISABLED" => Ok(JobStateEnum::DISABLED),
           "UPDATE_FAILED" => Ok(JobStateEnum::UPDATEFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


