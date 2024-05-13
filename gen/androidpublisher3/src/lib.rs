// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Android Publisher* crate version *5.0.5+20240229*, where *20240229* is the exact revision of the *androidpublisher:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Android Publisher* *v3* API can be found at the
//! [official documentation site](https://developers.google.com/android-publisher).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/androidpublisher3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](AndroidPublisher) ... 
//! 
//! * applications
//!  * [*data safety*](api::ApplicationDataSafetyCall), [*device tier configs create*](api::ApplicationDeviceTierConfigCreateCall), [*device tier configs get*](api::ApplicationDeviceTierConfigGetCall) and [*device tier configs list*](api::ApplicationDeviceTierConfigListCall)
//! * apprecovery
//!  * [*add targeting*](api::ApprecoveryAddTargetingCall), [*app recoveries*](api::ApprecoveryAppRecoveryCall), [*cancel*](api::ApprecoveryCancelCall), [*create*](api::ApprecoveryCreateCall) and [*deploy*](api::ApprecoveryDeployCall)
//! * edits
//!  * [*apks addexternallyhosted*](api::EditApkAddexternallyhostedCall), [*apks list*](api::EditApkListCall), [*apks upload*](api::EditApkUploadCall), [*bundles list*](api::EditBundleListCall), [*bundles upload*](api::EditBundleUploadCall), [*commit*](api::EditCommitCall), [*countryavailability get*](api::EditCountryavailabilityGetCall), [*delete*](api::EditDeleteCall), [*deobfuscationfiles upload*](api::EditDeobfuscationfileUploadCall), [*details get*](api::EditDetailGetCall), [*details patch*](api::EditDetailPatchCall), [*details update*](api::EditDetailUpdateCall), [*expansionfiles get*](api::EditExpansionfileGetCall), [*expansionfiles patch*](api::EditExpansionfilePatchCall), [*expansionfiles update*](api::EditExpansionfileUpdateCall), [*expansionfiles upload*](api::EditExpansionfileUploadCall), [*get*](api::EditGetCall), [*images delete*](api::EditImageDeleteCall), [*images deleteall*](api::EditImageDeleteallCall), [*images list*](api::EditImageListCall), [*images upload*](api::EditImageUploadCall), [*insert*](api::EditInsertCall), [*listings delete*](api::EditListingDeleteCall), [*listings deleteall*](api::EditListingDeleteallCall), [*listings get*](api::EditListingGetCall), [*listings list*](api::EditListingListCall), [*listings patch*](api::EditListingPatchCall), [*listings update*](api::EditListingUpdateCall), [*testers get*](api::EditTesterGetCall), [*testers patch*](api::EditTesterPatchCall), [*testers update*](api::EditTesterUpdateCall), [*tracks create*](api::EditTrackCreateCall), [*tracks get*](api::EditTrackGetCall), [*tracks list*](api::EditTrackListCall), [*tracks patch*](api::EditTrackPatchCall), [*tracks update*](api::EditTrackUpdateCall) and [*validate*](api::EditValidateCall)
//! * externaltransactions
//!  * [*createexternaltransaction*](api::ExternaltransactionCreateexternaltransactionCall), [*getexternaltransaction*](api::ExternaltransactionGetexternaltransactionCall) and [*refundexternaltransaction*](api::ExternaltransactionRefundexternaltransactionCall)
//! * generatedapks
//!  * [*download*](api::GeneratedapkDownloadCall) and [*list*](api::GeneratedapkListCall)
//! * [grants](api::Grant)
//!  * [*create*](api::GrantCreateCall), [*delete*](api::GrantDeleteCall) and [*patch*](api::GrantPatchCall)
//! * inappproducts
//!  * [*batch delete*](api::InappproductBatchDeleteCall), [*batch get*](api::InappproductBatchGetCall), [*batch update*](api::InappproductBatchUpdateCall), [*delete*](api::InappproductDeleteCall), [*get*](api::InappproductGetCall), [*insert*](api::InappproductInsertCall), [*list*](api::InappproductListCall), [*patch*](api::InappproductPatchCall) and [*update*](api::InappproductUpdateCall)
//! * internalappsharingartifacts
//!  * [*uploadapk*](api::InternalappsharingartifactUploadapkCall) and [*uploadbundle*](api::InternalappsharingartifactUploadbundleCall)
//! * monetization
//!  * [*convert region prices*](api::MonetizationConvertRegionPriceCall), [*subscriptions archive*](api::MonetizationSubscriptionArchiveCall), [*subscriptions base plans activate*](api::MonetizationSubscriptionBasePlanActivateCall), [*subscriptions base plans batch migrate prices*](api::MonetizationSubscriptionBasePlanBatchMigratePriceCall), [*subscriptions base plans batch update states*](api::MonetizationSubscriptionBasePlanBatchUpdateStateCall), [*subscriptions base plans deactivate*](api::MonetizationSubscriptionBasePlanDeactivateCall), [*subscriptions base plans delete*](api::MonetizationSubscriptionBasePlanDeleteCall), [*subscriptions base plans migrate prices*](api::MonetizationSubscriptionBasePlanMigratePriceCall), [*subscriptions base plans offers activate*](api::MonetizationSubscriptionBasePlanOfferActivateCall), [*subscriptions base plans offers batch get*](api::MonetizationSubscriptionBasePlanOfferBatchGetCall), [*subscriptions base plans offers batch update*](api::MonetizationSubscriptionBasePlanOfferBatchUpdateCall), [*subscriptions base plans offers batch update states*](api::MonetizationSubscriptionBasePlanOfferBatchUpdateStateCall), [*subscriptions base plans offers create*](api::MonetizationSubscriptionBasePlanOfferCreateCall), [*subscriptions base plans offers deactivate*](api::MonetizationSubscriptionBasePlanOfferDeactivateCall), [*subscriptions base plans offers delete*](api::MonetizationSubscriptionBasePlanOfferDeleteCall), [*subscriptions base plans offers get*](api::MonetizationSubscriptionBasePlanOfferGetCall), [*subscriptions base plans offers list*](api::MonetizationSubscriptionBasePlanOfferListCall), [*subscriptions base plans offers patch*](api::MonetizationSubscriptionBasePlanOfferPatchCall), [*subscriptions batch get*](api::MonetizationSubscriptionBatchGetCall), [*subscriptions batch update*](api::MonetizationSubscriptionBatchUpdateCall), [*subscriptions create*](api::MonetizationSubscriptionCreateCall), [*subscriptions delete*](api::MonetizationSubscriptionDeleteCall), [*subscriptions get*](api::MonetizationSubscriptionGetCall), [*subscriptions list*](api::MonetizationSubscriptionListCall) and [*subscriptions patch*](api::MonetizationSubscriptionPatchCall)
//! * orders
//!  * [*refund*](api::OrderRefundCall)
//! * purchases
//!  * [*products acknowledge*](api::PurchaseProductAcknowledgeCall), [*products consume*](api::PurchaseProductConsumeCall), [*products get*](api::PurchaseProductGetCall), [*subscriptions acknowledge*](api::PurchaseSubscriptionAcknowledgeCall), [*subscriptions cancel*](api::PurchaseSubscriptionCancelCall), [*subscriptions defer*](api::PurchaseSubscriptionDeferCall), [*subscriptions get*](api::PurchaseSubscriptionGetCall), [*subscriptions refund*](api::PurchaseSubscriptionRefundCall), [*subscriptions revoke*](api::PurchaseSubscriptionRevokeCall), [*subscriptionsv2 get*](api::PurchaseSubscriptionsv2GetCall), [*subscriptionsv2 revoke*](api::PurchaseSubscriptionsv2RevokeCall) and [*voidedpurchases list*](api::PurchaseVoidedpurchaseListCall)
//! * [reviews](api::Review)
//!  * [*get*](api::ReviewGetCall), [*list*](api::ReviewListCall) and [*reply*](api::ReviewReplyCall)
//! * systemapks
//!  * [*variants create*](api::SystemapkVariantCreateCall), [*variants download*](api::SystemapkVariantDownloadCall), [*variants get*](api::SystemapkVariantGetCall) and [*variants list*](api::SystemapkVariantListCall)
//! * [users](api::User)
//!  * [*create*](api::UserCreateCall), [*delete*](api::UserDeleteCall), [*list*](api::UserListCall) and [*patch*](api::UserPatchCall)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*apks upload edits*](api::EditApkUploadCall)
//! * [*bundles upload edits*](api::EditBundleUploadCall)
//! * [*deobfuscationfiles upload edits*](api::EditDeobfuscationfileUploadCall)
//! * [*expansionfiles upload edits*](api::EditExpansionfileUploadCall)
//! * [*images upload edits*](api::EditImageUploadCall)
//! * [*uploadapk internalappsharingartifacts*](api::InternalappsharingartifactUploadapkCall)
//! * [*uploadbundle internalappsharingartifacts*](api::InternalappsharingartifactUploadbundleCall)
//! 
//! Download supported by ...
//! 
//! * [*download generatedapks*](api::GeneratedapkDownloadCall)
//! * [*variants download systemapks*](api::SystemapkVariantDownloadCall)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](AndroidPublisher)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.monetization().subscriptions_base_plans_activate(...).doit().await
//! let r = hub.monetization().subscriptions_base_plans_deactivate(...).doit().await
//! let r = hub.monetization().subscriptions_archive(...).doit().await
//! let r = hub.monetization().subscriptions_create(...).doit().await
//! let r = hub.monetization().subscriptions_get(...).doit().await
//! let r = hub.monetization().subscriptions_patch(...).doit().await
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-androidpublisher3 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_androidpublisher3 as androidpublisher3;
//! use androidpublisher3::api::Subscription;
//! use androidpublisher3::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Subscription::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.monetization().subscriptions_patch(req, "packageName", "productId")
//!              .update_mask(&Default::default())
//!              .regions_version_version("amet.")
//!              .latency_tolerance("duo")
//!              .allow_missing(true)
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

// Re-export the hyper and hyper_rustls crate, they are required to build the hub
pub use hyper;
pub use hyper_rustls;
pub extern crate google_apis_common as client;
pub use client::chrono;
pub mod api;

// Re-export the hub type and some basic client structs
pub use api::AndroidPublisher;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;