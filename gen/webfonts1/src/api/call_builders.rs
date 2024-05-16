use super::*;
/// Retrieves the list of fonts currently served by the Google Fonts Developer API.
///
/// A builder for the *list* method supported by a *webfont* resource.
/// It is not used directly, but through a [`WebfontMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_webfonts1 as webfonts1;
/// use webfonts1::api::enums::*;
/// # async fn dox() {
/// # use std::default::Default;
/// # use webfonts1::{Webfonts, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Webfonts::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.webfonts().list()
///              .subset("sanctus")
///              .sort(&WebfontSortEnum::SORTUNDEFINED)
///              .add_family("sed")
///              .add_capability(&WebfontCapabilityEnum::CAPABILITYUNSPECIFIED)
///              .doit().await;
/// # }
/// ```
pub struct WebfontListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Webfonts<S>,
   pub(super) _subset: Option<String>,
   pub(super) _sort: Option<WebfontSortEnum>,
   pub(super) _family: Vec<String>,
   pub(super) _capability: Option<WebfontCapabilityEnum>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for WebfontListCall<'a, S> {}

impl<'a, S> WebfontListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, WebfontList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "webfonts.webfonts.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "subset", "sort", "family", "capability"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._subset.as_ref() {
            params.push("subset", value);
        }
        if let Some(value) = self._sort.as_ref() {
            params.push("sort", value);
        }
        if self._family.len() > 0 {
            for f in self._family.iter() {
                params.push("family", f);
            }
        }
        if self._capability.len() > 0 {
            for f in self._capability.iter() {
                params.push("capability", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/webfonts";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }


        let url = params.parse_with_url(&url);



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Filters by Webfont.subset, if subset is found in Webfont.subsets. If not set, returns all families.
    ///
    /// Sets the *subset* query property to the given value.
    pub fn subset(mut self, new_value: &str) -> WebfontListCall<'a, S> {
        self._subset = Some(new_value.to_string());
        self
    }
    /// Enables sorting of the list.
    ///
    /// Sets the *sort* query property to the given value.
    pub fn sort(mut self, new_value: &WebfontSortEnum) -> WebfontListCall<'a, S> {
        self._sort = Some(new_value.clone());
        self
    }
    /// Filters by Webfont.family, using literal match. If not set, returns all families
    ///
    /// Append the given value to the *family* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_family(mut self, new_value: &str) -> WebfontListCall<'a, S> {
        self._family.push(new_value.to_string());
        self
    }
    /// Controls the font urls in `Webfont.files`, by default, static ttf fonts are sent.
    ///
    /// Append the given value to the *capability* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_capability(mut self, new_value: &WebfontCapabilityEnum) -> WebfontListCall<'a, S> {
        self._capability.push(new_value.clone());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> WebfontListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> WebfontListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


