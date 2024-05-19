use super::*;



// region AppEngineHttpRequestHttpMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The HTTP method to use for the request. The default is POST. The app's request handler for the task's target URL must be able to handle HTTP requests with this http_method, otherwise the task attempt fails with error code 405 (Method Not Allowed). See [Writing a push task request handler](https://cloud.google.com/appengine/docs/java/taskqueue/push/creating-handlers#writing_a_push_task_request_handler) and the App Engine documentation for your runtime on [How Requests are Handled](https://cloud.google.com/appengine/docs/standard/python3/how-requests-are-handled).
pub enum AppEngineHttpRequestHttpMethodEnum {
    

    /// HTTP method unspecified
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

impl AsRef<str> for AppEngineHttpRequestHttpMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppEngineHttpRequestHttpMethodEnum::HTTPMETHODUNSPECIFIED => "HTTP_METHOD_UNSPECIFIED",
            AppEngineHttpRequestHttpMethodEnum::POST => "POST",
            AppEngineHttpRequestHttpMethodEnum::GET => "GET",
            AppEngineHttpRequestHttpMethodEnum::HEAD => "HEAD",
            AppEngineHttpRequestHttpMethodEnum::PUT => "PUT",
            AppEngineHttpRequestHttpMethodEnum::DELETE => "DELETE",
            AppEngineHttpRequestHttpMethodEnum::PATCH => "PATCH",
            AppEngineHttpRequestHttpMethodEnum::OPTIONS => "OPTIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for AppEngineHttpRequestHttpMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HTTP_METHOD_UNSPECIFIED" => Ok(AppEngineHttpRequestHttpMethodEnum::HTTPMETHODUNSPECIFIED),
           "POST" => Ok(AppEngineHttpRequestHttpMethodEnum::POST),
           "GET" => Ok(AppEngineHttpRequestHttpMethodEnum::GET),
           "HEAD" => Ok(AppEngineHttpRequestHttpMethodEnum::HEAD),
           "PUT" => Ok(AppEngineHttpRequestHttpMethodEnum::PUT),
           "DELETE" => Ok(AppEngineHttpRequestHttpMethodEnum::DELETE),
           "PATCH" => Ok(AppEngineHttpRequestHttpMethodEnum::PATCH),
           "OPTIONS" => Ok(AppEngineHttpRequestHttpMethodEnum::OPTIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppEngineHttpRequestHttpMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreateTaskRequestResponseViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The response_view specifies which subset of the Task will be returned. By default response_view is BASIC; not all information is retrieved by default because some data, such as payloads, might be desirable to return only when needed because of its large size or because of the sensitivity of data that it contains. Authorization for FULL requires `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/) permission on the Task resource.
pub enum CreateTaskRequestResponseViewEnum {
    

    /// Unspecified. Defaults to BASIC.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// The basic view omits fields which can be large or can contain sensitive data. This view does not include the body in AppEngineHttpRequest. Bodies are desirable to return only when needed, because they can be large and because of the sensitivity of the data that you choose to store in it.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// All information is returned. Authorization for FULL requires `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/) permission on the Queue resource.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for CreateTaskRequestResponseViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateTaskRequestResponseViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            CreateTaskRequestResponseViewEnum::BASIC => "BASIC",
            CreateTaskRequestResponseViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateTaskRequestResponseViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(CreateTaskRequestResponseViewEnum::VIEWUNSPECIFIED),
           "BASIC" => Ok(CreateTaskRequestResponseViewEnum::BASIC),
           "FULL" => Ok(CreateTaskRequestResponseViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateTaskRequestResponseViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HttpRequestHttpMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The HTTP method to use for the request. The default is POST.
pub enum HttpRequestHttpMethodEnum {
    

    /// HTTP method unspecified
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

impl AsRef<str> for HttpRequestHttpMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HttpRequestHttpMethodEnum::HTTPMETHODUNSPECIFIED => "HTTP_METHOD_UNSPECIFIED",
            HttpRequestHttpMethodEnum::POST => "POST",
            HttpRequestHttpMethodEnum::GET => "GET",
            HttpRequestHttpMethodEnum::HEAD => "HEAD",
            HttpRequestHttpMethodEnum::PUT => "PUT",
            HttpRequestHttpMethodEnum::DELETE => "DELETE",
            HttpRequestHttpMethodEnum::PATCH => "PATCH",
            HttpRequestHttpMethodEnum::OPTIONS => "OPTIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for HttpRequestHttpMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HTTP_METHOD_UNSPECIFIED" => Ok(HttpRequestHttpMethodEnum::HTTPMETHODUNSPECIFIED),
           "POST" => Ok(HttpRequestHttpMethodEnum::POST),
           "GET" => Ok(HttpRequestHttpMethodEnum::GET),
           "HEAD" => Ok(HttpRequestHttpMethodEnum::HEAD),
           "PUT" => Ok(HttpRequestHttpMethodEnum::PUT),
           "DELETE" => Ok(HttpRequestHttpMethodEnum::DELETE),
           "PATCH" => Ok(HttpRequestHttpMethodEnum::PATCH),
           "OPTIONS" => Ok(HttpRequestHttpMethodEnum::OPTIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HttpRequestHttpMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HttpTargetHttpMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The HTTP method to use for the request. When specified, it overrides HttpRequest for the task. Note that if the value is set to HttpMethod the HttpRequest of the task will be ignored at execution time.
pub enum HttpTargetHttpMethodEnum {
    

    /// HTTP method unspecified
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


// region QueueStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the queue. `state` can only be changed by called PauseQueue, ResumeQueue, or uploading [queue.yaml/xml](https://cloud.google.com/appengine/docs/python/config/queueref). UpdateQueue cannot be used to change `state`.
pub enum QueueStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The queue is running. Tasks can be dispatched. If the queue was created using Cloud Tasks and the queue has had no activity (method calls or task dispatches) for 30 days, the queue may take a few minutes to re-activate. Some method calls may return NOT_FOUND and tasks may not be dispatched for a few minutes until the queue has been re-activated.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// Tasks are paused by the user. If the queue is paused then Cloud Tasks will stop delivering tasks from it, but more tasks can still be added to it by the user.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// The queue is disabled. A queue becomes `DISABLED` when [queue.yaml](https://cloud.google.com/appengine/docs/python/config/queueref) or [queue.xml](https://cloud.google.com/appengine/docs/standard/java/config/queueref) is uploaded which does not contain the queue. You cannot directly disable a queue. When a queue is disabled, tasks can still be added to a queue but the tasks are not dispatched. To permanently delete this queue and all of its tasks, call DeleteQueue.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for QueueStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueueStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            QueueStateEnum::RUNNING => "RUNNING",
            QueueStateEnum::PAUSED => "PAUSED",
            QueueStateEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for QueueStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(QueueStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(QueueStateEnum::RUNNING),
           "PAUSED" => Ok(QueueStateEnum::PAUSED),
           "DISABLED" => Ok(QueueStateEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueueStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The type of a queue (push or pull). `Queue.type` is an immutable property of the queue that is set at the queue creation time. When left unspecified, the default value of `PUSH` is selected.
pub enum QueueTypeEnum {
    

    /// Default value.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// A pull queue.
    ///
    /// "PULL"
    #[serde(rename="PULL")]
    PULL,
    

    /// A push queue.
    ///
    /// "PUSH"
    #[serde(rename="PUSH")]
    PUSH,
}

impl AsRef<str> for QueueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueueTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            QueueTypeEnum::PULL => "PULL",
            QueueTypeEnum::PUSH => "PUSH",
        }
    }
}

impl std::convert::TryFrom< &str> for QueueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(QueueTypeEnum::TYPEUNSPECIFIED),
           "PULL" => Ok(QueueTypeEnum::PULL),
           "PUSH" => Ok(QueueTypeEnum::PUSH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RunTaskRequestResponseViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The response_view specifies which subset of the Task will be returned. By default response_view is BASIC; not all information is retrieved by default because some data, such as payloads, might be desirable to return only when needed because of its large size or because of the sensitivity of data that it contains. Authorization for FULL requires `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/) permission on the Task resource.
pub enum RunTaskRequestResponseViewEnum {
    

    /// Unspecified. Defaults to BASIC.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// The basic view omits fields which can be large or can contain sensitive data. This view does not include the body in AppEngineHttpRequest. Bodies are desirable to return only when needed, because they can be large and because of the sensitivity of the data that you choose to store in it.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// All information is returned. Authorization for FULL requires `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/) permission on the Queue resource.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for RunTaskRequestResponseViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RunTaskRequestResponseViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            RunTaskRequestResponseViewEnum::BASIC => "BASIC",
            RunTaskRequestResponseViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for RunTaskRequestResponseViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(RunTaskRequestResponseViewEnum::VIEWUNSPECIFIED),
           "BASIC" => Ok(RunTaskRequestResponseViewEnum::BASIC),
           "FULL" => Ok(RunTaskRequestResponseViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RunTaskRequestResponseViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TaskViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The view specifies which subset of the Task has been returned.
pub enum TaskViewEnum {
    

    /// Unspecified. Defaults to BASIC.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// The basic view omits fields which can be large or can contain sensitive data. This view does not include the body in AppEngineHttpRequest. Bodies are desirable to return only when needed, because they can be large and because of the sensitivity of the data that you choose to store in it.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// All information is returned. Authorization for FULL requires `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/) permission on the Queue resource.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for TaskViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TaskViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            TaskViewEnum::BASIC => "BASIC",
            TaskViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for TaskViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(TaskViewEnum::VIEWUNSPECIFIED),
           "BASIC" => Ok(TaskViewEnum::BASIC),
           "FULL" => Ok(TaskViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TaskViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UriOverrideSchemeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Scheme override. When specified, the task URI scheme is replaced by the provided value (HTTP or HTTPS).
pub enum UriOverrideSchemeEnum {
    

    /// Scheme unspecified. Defaults to HTTPS.
    ///
    /// "SCHEME_UNSPECIFIED"
    #[serde(rename="SCHEME_UNSPECIFIED")]
    SCHEMEUNSPECIFIED,
    

    /// Convert the scheme to HTTP, e.g., https://www.google.ca will change to http://www.google.ca.
    ///
    /// "HTTP"
    #[serde(rename="HTTP")]
    HTTP,
    

    /// Convert the scheme to HTTPS, e.g., http://www.google.ca will change to https://www.google.ca.
    ///
    /// "HTTPS"
    #[serde(rename="HTTPS")]
    HTTPS,
}

impl AsRef<str> for UriOverrideSchemeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UriOverrideSchemeEnum::SCHEMEUNSPECIFIED => "SCHEME_UNSPECIFIED",
            UriOverrideSchemeEnum::HTTP => "HTTP",
            UriOverrideSchemeEnum::HTTPS => "HTTPS",
        }
    }
}

impl std::convert::TryFrom< &str> for UriOverrideSchemeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCHEME_UNSPECIFIED" => Ok(UriOverrideSchemeEnum::SCHEMEUNSPECIFIED),
           "HTTP" => Ok(UriOverrideSchemeEnum::HTTP),
           "HTTPS" => Ok(UriOverrideSchemeEnum::HTTPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UriOverrideSchemeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UriOverrideUriOverrideEnforceModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// URI Override Enforce Mode When specified, determines the Target UriOverride mode. If not specified, it defaults to ALWAYS.
pub enum UriOverrideUriOverrideEnforceModeEnum {
    

    /// OverrideMode Unspecified. Defaults to ALWAYS.
    ///
    /// "URI_OVERRIDE_ENFORCE_MODE_UNSPECIFIED"
    #[serde(rename="URI_OVERRIDE_ENFORCE_MODE_UNSPECIFIED")]
    URIOVERRIDEENFORCEMODEUNSPECIFIED,
    

    /// In the IF_NOT_EXISTS mode, queue-level configuration is only applied where task-level configuration does not exist.
    ///
    /// "IF_NOT_EXISTS"
    #[serde(rename="IF_NOT_EXISTS")]
    IFNOTEXISTS,
    

    /// In the ALWAYS mode, queue-level configuration overrides all task-level configuration
    ///
    /// "ALWAYS"
    #[serde(rename="ALWAYS")]
    ALWAYS,
}

impl AsRef<str> for UriOverrideUriOverrideEnforceModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UriOverrideUriOverrideEnforceModeEnum::URIOVERRIDEENFORCEMODEUNSPECIFIED => "URI_OVERRIDE_ENFORCE_MODE_UNSPECIFIED",
            UriOverrideUriOverrideEnforceModeEnum::IFNOTEXISTS => "IF_NOT_EXISTS",
            UriOverrideUriOverrideEnforceModeEnum::ALWAYS => "ALWAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for UriOverrideUriOverrideEnforceModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "URI_OVERRIDE_ENFORCE_MODE_UNSPECIFIED" => Ok(UriOverrideUriOverrideEnforceModeEnum::URIOVERRIDEENFORCEMODEUNSPECIFIED),
           "IF_NOT_EXISTS" => Ok(UriOverrideUriOverrideEnforceModeEnum::IFNOTEXISTS),
           "ALWAYS" => Ok(UriOverrideUriOverrideEnforceModeEnum::ALWAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UriOverrideUriOverrideEnforceModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectResponseViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The response_view specifies which subset of the Task will be returned. By default response_view is BASIC; not all information is retrieved by default because some data, such as payloads, might be desirable to return only when needed because of its large size or because of the sensitivity of data that it contains. Authorization for FULL requires `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/) permission on the Task resource.
pub enum ProjectResponseViewEnum {
    

    /// Unspecified. Defaults to BASIC.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// The basic view omits fields which can be large or can contain sensitive data. This view does not include the body in AppEngineHttpRequest. Bodies are desirable to return only when needed, because they can be large and because of the sensitivity of the data that you choose to store in it.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// All information is returned. Authorization for FULL requires `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/) permission on the Queue resource.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectResponseViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectResponseViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            ProjectResponseViewEnum::BASIC => "BASIC",
            ProjectResponseViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectResponseViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(ProjectResponseViewEnum::VIEWUNSPECIFIED),
           "BASIC" => Ok(ProjectResponseViewEnum::BASIC),
           "FULL" => Ok(ProjectResponseViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectResponseViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


