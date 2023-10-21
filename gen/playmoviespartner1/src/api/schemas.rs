use super::*;
/// An Order tracks the fulfillment of an Edit when delivered using the
/// legacy, non-component-based delivery.
/// 
/// Each Order is uniquely identified by an `order_id`, which is generated
/// by Google.
/// 
/// Externally, Orders can also be identified by partners using its `custom_id`
/// (when provided).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [orders get accounts](AccountOrderGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    /// Countries where the Order is available,
    /// using the "ISO 3166-1 alpha-2" format (example: "US").
    
    pub countries: Option<Vec<String>>,
    /// Detailed status of the order
    #[serde(rename="statusDetail")]
    
    pub status_detail: Option<String>,
    /// High-level status of the order.
    
    pub status: Option<String>,
    /// Timestamp of the earliest start date of the Avails
    /// linked to this Order.
    #[serde(rename="earliestAvailStartTime")]
    
    pub earliest_avail_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Default Edit name,
    /// usually in the language of the country of origin.
    /// Example: "Googlers, The".
    
    pub name: Option<String>,
    /// Name of the studio that owns the Edit ordered.
    #[serde(rename="studioName")]
    
    pub studio_name: Option<String>,
    /// Timestamp when the Order was fulfilled.
    #[serde(rename="receivedTime")]
    
    pub received_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Default Season name,
    /// usually in the language of the country of origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The - A Brave New World".
    #[serde(rename="seasonName")]
    
    pub season_name: Option<String>,
    /// ID that can be used to externally identify an Order.
    /// This ID is provided by partners when submitting the Avails.
    /// Example: 'GOOGLER_2006'
    #[serde(rename="customId")]
    
    pub custom_id: Option<String>,
    /// YouTube Channel Name that should be used to fulfill the Order.
    /// Example: "Google_channel".
    #[serde(rename="channelName")]
    
    pub channel_name: Option<String>,
    /// Timestamp when the Order was approved.
    #[serde(rename="approvedTime")]
    
    pub approved_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Default Show name,
    /// usually in the language of the country of origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The".
    #[serde(rename="showName")]
    
    pub show_name: Option<String>,
    /// A simpler representation of the priority.
    #[serde(rename="normalizedPriority")]
    
    pub normalized_priority: Option<String>,
    /// ID internally generated by Google to uniquely identify an Order.
    /// Example: 'abcde12_x'
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// Type of the Edit linked to the Order.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Field explaining why an Order has been rejected.
    /// Example: "Trailer audio is 2ch mono, please re-deliver in stereo".
    #[serde(rename="rejectionNote")]
    
    pub rejection_note: Option<String>,
    /// YouTube Channel ID that should be used to fulfill the Order.
    /// Example: "UCRG64darCZhb".
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// Legacy Order priority, as defined by Google.
    /// Example: 'P0'
    #[serde(rename="legacyPriority")]
    
    pub legacy_priority: Option<String>,
    /// Name of the post-production house that manages the Edit ordered.
    #[serde(rename="pphName")]
    
    pub pph_name: Option<String>,
    /// Timestamp when the Order was created.
    #[serde(rename="orderedTime")]
    
    pub ordered_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Order priority, as defined by Google.
    /// The higher the value, the higher the priority.
    /// Example: 90
    
    pub priority: Option<f64>,
    /// Google-generated ID identifying the video linked to this Order, once
    /// delivered.
    /// Example: 'gtry456_xc'.
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
    /// Default Episode name,
    /// usually in the language of the country of origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The - Pilot".
    #[serde(rename="episodeName")]
    
    pub episode_name: Option<String>,
}

impl client::ResponseResult for Order {}


/// Response to the ‘ListStoreInfos’ method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [store infos list accounts](AccountStoreInfoListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListStoreInfosResponse {
    /// See 'List methods rules' for info about this field.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
    /// List of StoreInfos that match the request criteria.
    #[serde(rename="storeInfos")]
    
    pub store_infos: Option<Vec<StoreInfo>>,
}

impl client::ResponseResult for ListStoreInfosResponse {}


/// Response to the ‘ListAvails’ method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [avails list accounts](AccountAvailListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAvailsResponse {
    /// List of Avails that match the request criteria.
    
    pub avails: Option<Vec<Avail>>,
    /// See _List methods rules_ for info about this field.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListAvailsResponse {}


/// Information about a playable sequence (video) associated with an Edit
/// and available at the Google Play Store.
/// 
/// Internally, each StoreInfo is uniquely identified by a `video_id`
/// and `country`.
/// 
/// Externally, Title-level EIDR or Edit-level EIDR, if provided,
/// can also be used to identify a specific title or edit in a country.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [store infos country get accounts](AccountStoreInfoCountryGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StoreInfo {
    /// Timestamp when the Edit went live on the Store.
    #[serde(rename="liveTime")]
    
    pub live_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Google-generated ID identifying the video linked to the Edit.
    /// Example: 'gtry456_xc'
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
    /// Whether the Edit has info cards.
    #[serde(rename="hasInfoCards")]
    
    pub has_info_cards: Option<bool>,
    /// Whether the Edit has a VOD offer.
    #[serde(rename="hasVodOffer")]
    
    pub has_vod_offer: Option<bool>,
    /// Name of the post-production houses that manage the Edit.
    #[serde(rename="pphNames")]
    
    pub pph_names: Option<Vec<String>>,
    /// The number assigned to the episode within a season.
    /// Only available on TV Edits.
    /// Example: "1".
    #[serde(rename="episodeNumber")]
    
    pub episode_number: Option<String>,
    /// Name of the studio that owns the Edit ordered.
    #[serde(rename="studioName")]
    
    pub studio_name: Option<String>,
    /// Subtitles available for this Edit.
    
    pub subtitles: Option<Vec<String>>,
    /// Audio tracks available for this Edit.
    #[serde(rename="audioTracks")]
    
    pub audio_tracks: Option<Vec<String>>,
    /// Default Show name, usually in the language of the country of
    /// origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The".
    #[serde(rename="showName")]
    
    pub show_name: Option<String>,
    /// Country where Edit is available in ISO 3166-1 alpha-2 country
    /// code.
    /// Example: "US".
    
    pub country: Option<String>,
    /// Google-generated ID identifying the show linked to the Edit.
    /// Only available for TV Edits.
    /// Example: 'et2hsue_x'
    #[serde(rename="showId")]
    
    pub show_id: Option<String>,
    /// Edit type, like Movie, Episode or Season.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Google-generated ID identifying the trailer linked to the Edit.
    /// Example: 'bhd_4e_cx'
    #[serde(rename="trailerId")]
    
    pub trailer_id: Option<String>,
    /// Whether the Edit has a HD offer.
    #[serde(rename="hasHdOffer")]
    
    pub has_hd_offer: Option<bool>,
    /// Knowledge Graph ID associated to this Edit, if available.
    /// This ID links the Edit to its knowledge entity, externally accessible
    /// at http://freebase.com.
    /// In the absense of Title EIDR or Edit EIDR, this ID helps link together
    /// multiple Edits across countries.
    /// Example: '/m/0ffx29'
    
    pub mid: Option<String>,
    /// Whether the Edit has a 5.1 channel audio track.
    #[serde(rename="hasAudio51")]
    
    pub has_audio51: Option<bool>,
    /// Default Edit name, usually in the language of the country of
    /// origin.
    /// Example: "Googlers, The".
    
    pub name: Option<String>,
    /// Google-generated ID identifying the season linked to the Edit.
    /// Only available for TV Edits.
    /// Example: 'ster23ex'
    #[serde(rename="seasonId")]
    
    pub season_id: Option<String>,
    /// Title-level EIDR ID.
    /// Example: "10.5240/1489-49A2-3956-4B2D-FE16-5".
    #[serde(rename="titleLevelEidr")]
    
    pub title_level_eidr: Option<String>,
    /// Default Season name, usually in the language of the country of
    /// origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The - A Brave New World".
    #[serde(rename="seasonName")]
    
    pub season_name: Option<String>,
    /// The number assigned to the season within a show.
    /// Only available on TV Edits.
    /// Example: "1".
    #[serde(rename="seasonNumber")]
    
    pub season_number: Option<String>,
    /// Whether the Edit has a EST offer.
    #[serde(rename="hasEstOffer")]
    
    pub has_est_offer: Option<bool>,
    /// Edit-level EIDR ID.
    /// Example: "10.5240/1489-49A2-3956-4B2D-FE16-6".
    #[serde(rename="editLevelEidr")]
    
    pub edit_level_eidr: Option<String>,
    /// Whether the Edit has a SD offer.
    #[serde(rename="hasSdOffer")]
    
    pub has_sd_offer: Option<bool>,
}

impl client::ResponseResult for StoreInfo {}


/// An Avail describes the Availability Window of a specific Edit in a given
/// country, which means the period Google is allowed to sell or rent the Edit.
/// 
/// Avails are exposed in EMA format Version 1.6b (available at
/// http://www.movielabs.com/md/avails/)
/// 
/// Studios can see the Avails for the Titles they own.
/// Post-production houses cannot see any Avails.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [avails get accounts](AccountAvailGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Avail {
    /// Title used by involved parties to refer to this series.
    /// Only available on TV Avails.
    /// Example: "Googlers, The".
    #[serde(rename="seriesTitleInternalAlias")]
    
    pub series_title_internal_alias: Option<String>,
    /// Indicates the format profile covered by the transaction.
    #[serde(rename="formatProfile")]
    
    pub format_profile: Option<String>,
    /// Title Identifier. This should be the Title Level EIDR.
    /// Example: "10.5240/1489-49A2-3956-4B2D-FE16-5".
    #[serde(rename="contentId")]
    
    pub content_id: Option<String>,
    /// Title used by involved parties to refer to this content.
    /// Example: "Googlers, The".
    /// Only available on Movie Avails.
    #[serde(rename="titleInternalAlias")]
    
    pub title_internal_alias: Option<String>,
    /// Value representing the rating.
    /// Ratings should be formatted as per http://www.movielabs.com/md/ratings/
    /// Example: "PG"
    #[serde(rename="ratingValue")]
    
    pub rating_value: Option<String>,
    /// Spoken language of the intended audience.
    /// Language shall be encoded in accordance with RFC 5646.
    /// Example: "fr".
    #[serde(rename="storeLanguage")]
    
    pub store_language: Option<String>,
    /// Communicating an exempt category as defined by FCC regulations.
    /// It is not required for non-US Avails.
    /// Example: "1"
    #[serde(rename="captionExemption")]
    
    pub caption_exemption: Option<String>,
    /// The name of the studio that owns the Edit referred in the Avail.
    /// This is the equivalent of `studio_name` in other resources, but it follows
    /// the EMA nomenclature.
    /// Example: "Google Films".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Edit Identifier. This should be the Edit Level EIDR.
    /// Example: "10.2340/1489-49A2-3956-4B2D-FE16-6"
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Title used by involved parties to refer to this season.
    /// Only available on TV Avails.
    /// Example: "Googlers, The".
    #[serde(rename="seasonTitleInternalAlias")]
    
    pub season_title_internal_alias: Option<String>,
    /// Other identifier referring to the episode, as defined by partner.
    /// Only available on TV avails.
    /// Example: "rs_googlers_s1_3".
    #[serde(rename="episodeAltId")]
    
    pub episode_alt_id: Option<String>,
    /// Value to be applied to the pricing type.
    /// Example: "4" or "2.99"
    #[serde(rename="priceValue")]
    
    pub price_value: Option<String>,
    /// ISO 3166-1 alpha-2 country code for the country or territory
    /// of this Avail.
    /// For Avails, we use Territory in lieu of Country to comply with
    /// EMA specifications.
    /// But please note that Territory and Country identify the same thing.
    /// Example: "US".
    
    pub territory: Option<String>,
    /// Work type as enumerated in EMA.
    #[serde(rename="workType")]
    
    pub work_type: Option<String>,
    /// ID internally generated by Google to uniquely identify an Avail.
    /// Not part of EMA Specs.
    #[serde(rename="availId")]
    
    pub avail_id: Option<String>,
    /// Value representing the rating reason.
    /// Rating reasons should be formatted as per
    /// [EMA ratings spec](http://www.movielabs.com/md/ratings/)
    /// and comma-separated for inclusion of multiple reasons.
    /// Example: "L, S, V"
    #[serde(rename="ratingReason")]
    
    pub rating_reason: Option<String>,
    /// OPTIONAL.TV Only. Title used by involved parties to refer to this episode.
    /// Only available on TV Avails.
    /// Example: "Coding at Google".
    #[serde(rename="episodeTitleInternalAlias")]
    
    pub episode_title_internal_alias: Option<String>,
    /// First date an Edit could be publically announced as becoming
    /// available at a specific future date in territory of Avail.
    /// *Not* the Avail start date or pre-order start date.
    /// Format is YYYY-MM-DD.
    /// Only available for pre-orders.
    /// Example: "2012-12-10"
    #[serde(rename="suppressionLiftDate")]
    
    pub suppression_lift_date: Option<String>,
    /// Other identifier referring to the season, as defined by partner.
    /// Only available on TV avails.
    /// Example: "rs_googlers_s1".
    #[serde(rename="seasonAltId")]
    
    pub season_alt_id: Option<String>,
    /// Manifestation Identifier. This should be the Manifestation
    /// Level EIDR.
    /// Example: "10.2340/1489-49A2-3956-4B2D-FE16-7"
    #[serde(rename="encodeId")]
    
    pub encode_id: Option<String>,
    /// Type of pricing that should be applied to this Avail
    /// based on how the partner classify them.
    /// Example: "Tier", "WSP", "SRP", or "Category".
    #[serde(rename="priceType")]
    
    pub price_type: Option<String>,
    /// Communicating if caption file will be delivered.
    #[serde(rename="captionIncluded")]
    
    pub caption_included: Option<bool>,
    /// Type of transaction.
    #[serde(rename="licenseType")]
    
    pub license_type: Option<String>,
    /// The number assigned to the season within a series.
    /// Only available on TV Avails.
    /// Example: "1".
    #[serde(rename="seasonNumber")]
    
    pub season_number: Option<String>,
    /// Release date of the Title in earliest released territory.
    /// Typically it is just the year, but it is free-form as per EMA spec.
    /// Examples: "1979", "Oct 2014"
    #[serde(rename="releaseDate")]
    
    pub release_date: Option<String>,
    /// End of term in YYYY-MM-DD format in the timezone of the country
    /// of the Avail.
    /// "Open" if no end date is available.
    /// Example: "2019-02-17"
    
    pub end: Option<String>,
    /// Google-generated ID identifying the video linked to this Avail, once
    /// delivered.
    /// Not part of EMA Specs.
    /// Example: 'gtry456_xc'
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
    /// Start of term in YYYY-MM-DD format in the timezone of the
    /// country of the Avail.
    /// Example: "2013-05-14".
    
    pub start: Option<String>,
    /// Rating system applied to the version of title within territory
    /// of Avail.
    /// Rating systems should be formatted as per
    /// [EMA ratings spec](http://www.movielabs.com/md/ratings/)
    /// Example: "MPAA"
    #[serde(rename="ratingSystem")]
    
    pub rating_system: Option<String>,
    /// Name of the post-production houses that manage the Avail.
    /// Not part of EMA Specs.
    #[serde(rename="pphNames")]
    
    pub pph_names: Option<Vec<String>>,
    /// Other identifier referring to the series, as defined by partner.
    /// Only available on TV avails.
    /// Example: "rs_googlers".
    #[serde(rename="seriesAltId")]
    
    pub series_alt_id: Option<String>,
    /// Other identifier referring to the Edit, as defined by partner.
    /// Example: "GOOGLER_2006"
    #[serde(rename="altId")]
    
    pub alt_id: Option<String>,
    /// The number assigned to the episode within a season.
    /// Only available on TV Avails.
    /// Example: "3".
    #[serde(rename="episodeNumber")]
    
    pub episode_number: Option<String>,
}

impl client::ResponseResult for Avail {}


/// Response to the ‘ListOrders’ method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [orders list accounts](AccountOrderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOrdersResponse {
    /// List of Orders that match the request criteria.
    
    pub orders: Option<Vec<Order>>,
    /// See _List methods rules_ for info about this field.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListOrdersResponse {}


