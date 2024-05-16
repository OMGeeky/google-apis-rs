use super::*;
/// A builder providing access to all methods supported on *media* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d5 as dfareporting3d5;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d5::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `upload(...)`
/// // to build up your call.
/// let rb = hub.media();
/// # }
/// ```
pub struct MediaMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for MediaMethods<'a, S> {}

impl<'a, S> MediaMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new creative asset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `advertiserId` - Advertiser ID of this creative. This is a required field.
    pub fn upload(&self, request: CreativeAssetMetadata, profile_id: i64, advertiser_id: i64) -> MediaUploadCall<'a, S> {
        MediaUploadCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



