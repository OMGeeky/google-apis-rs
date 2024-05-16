use super::*;
/// Creative Click Tag.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClickTag {
    /// Parameter value for the specified click tag. This field contains a click-through url.
    #[serde(rename="clickThroughUrl")]
    
    pub click_through_url: Option<CreativeClickThroughUrl>,
    /// Advertiser event name associated with the click tag. This field is used by DISPLAY_IMAGE_GALLERY and HTML5_BANNER creatives. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    #[serde(rename="eventName")]
    
    pub event_name: Option<String>,
    /// Parameter name for the specified click tag. For DISPLAY_IMAGE_GALLERY creative assets, this field must match the value of the creative asset's creativeAssetId.name field.
    
    pub name: Option<String>,
}

impl client::Part for ClickTag {}


/// Creative Asset ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeAssetId {
    /// Name of the creative asset. This is a required field while inserting an asset. After insertion, this assetIdentifier is used to identify the uploaded asset. Characters in the name must be alphanumeric or one of the following: ".-_ ". Spaces are allowed.
    
    pub name: Option<String>,
    /// Type of asset to upload. This is a required field. FLASH and IMAGE are no longer supported for new uploads. All image assets should use HTML_IMAGE.
    #[serde(rename="type")]
    
    pub type_: Option<CreativeAssetIdTypeEnum>,
}

impl client::Part for CreativeAssetId {}


/// CreativeAssets contains properties of a creative asset file which will be uploaded or has already been uploaded. Refer to the creative sample code for how to upload assets and insert a creative.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload media](MediaUploadCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeAssetMetadata {
    /// ID of the creative asset. This is a required field.
    #[serde(rename="assetIdentifier")]
    
    pub asset_identifier: Option<CreativeAssetId>,
    /// List of detected click tags for assets. This is a read-only, auto-generated field. This field is empty for a rich media asset.
    #[serde(rename="clickTags")]
    
    pub click_tags: Option<Vec<ClickTag>>,
    /// List of counter events configured for the asset. This is a read-only, auto-generated field and only applicable to a rich media asset.
    #[serde(rename="counterCustomEvents")]
    
    pub counter_custom_events: Option<Vec<CreativeCustomEvent>>,
    /// List of feature dependencies for the creative asset that are detected by Campaign Manager. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative correctly. This is a read-only, auto-generated field.
    #[serde(rename="detectedFeatures")]
    
    pub detected_features: Option<Vec<CreativeAssetMetadataDetectedFeaturesEnum>>,
    /// List of exit events configured for the asset. This is a read-only, auto-generated field and only applicable to a rich media asset.
    #[serde(rename="exitCustomEvents")]
    
    pub exit_custom_events: Option<Vec<CreativeCustomEvent>>,
    /// Numeric ID of the asset. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Dimension value for the numeric ID of the asset. This is a read-only, auto-generated field.
    #[serde(rename="idDimensionValue")]
    
    pub id_dimension_value: Option<DimensionValue>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#creativeAssetMetadata".
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="mediaRequestInfo")]
    
    pub media_request_info: Option<MediaRequestInfo>,
    /// no description provided
    #[serde(rename="mediaResponseInfo")]
    
    pub media_response_info: Option<MediaResponseInfo>,
    /// True if the uploaded asset is a rich media asset. This is a read-only, auto-generated field.
    #[serde(rename="richMedia")]
    
    pub rich_media: Option<bool>,
    /// List of timer events configured for the asset. This is a read-only, auto-generated field and only applicable to a rich media asset.
    #[serde(rename="timerCustomEvents")]
    
    pub timer_custom_events: Option<Vec<CreativeCustomEvent>>,
    /// Rules validated during code generation that generated a warning. This is a read-only, auto-generated field. Possible values are: - "ADMOB_REFERENCED" - "ASSET_FORMAT_UNSUPPORTED_DCM" - "ASSET_INVALID" - "CLICK_TAG_HARD_CODED" - "CLICK_TAG_INVALID" - "CLICK_TAG_IN_GWD" - "CLICK_TAG_MISSING" - "CLICK_TAG_MORE_THAN_ONE" - "CLICK_TAG_NON_TOP_LEVEL" - "COMPONENT_UNSUPPORTED_DCM" - "ENABLER_UNSUPPORTED_METHOD_DCM" - "EXTERNAL_FILE_REFERENCED" - "FILE_DETAIL_EMPTY" - "FILE_TYPE_INVALID" - "GWD_PROPERTIES_INVALID" - "HTML5_FEATURE_UNSUPPORTED" - "LINKED_FILE_NOT_FOUND" - "MAX_FLASH_VERSION_11" - "MRAID_REFERENCED" - "NOT_SSL_COMPLIANT" - "ORPHANED_ASSET" - "PRIMARY_HTML_MISSING" - "SVG_INVALID" - "ZIP_INVALID" 
    #[serde(rename="warnedValidationRules")]
    
    pub warned_validation_rules: Option<Vec<CreativeAssetMetadataWarnedValidationRulesEnum>>,
}

impl client::RequestValue for CreativeAssetMetadata {}
impl client::ResponseResult for CreativeAssetMetadata {}


/// Click-through URL
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeClickThroughUrl {
    /// Read-only convenience field representing the actual URL that will be used for this click-through. The URL is computed as follows: - If landingPageId is specified then that landing page's URL is assigned to this field. - Otherwise, the customClickThroughUrl is assigned to this field. 
    #[serde(rename="computedClickThroughUrl")]
    
    pub computed_click_through_url: Option<String>,
    /// Custom click-through URL. Applicable if the landingPageId field is left unset.
    #[serde(rename="customClickThroughUrl")]
    
    pub custom_click_through_url: Option<String>,
    /// ID of the landing page for the click-through URL.
    #[serde(rename="landingPageId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub landing_page_id: Option<i64>,
}

impl client::Part for CreativeClickThroughUrl {}


/// Creative Custom Event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeCustomEvent {
    /// Unique ID of this event used by Reporting and Data Transfer. This is a read-only field.
    #[serde(rename="advertiserCustomEventId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_custom_event_id: Option<i64>,
    /// User-entered name for the event.
    #[serde(rename="advertiserCustomEventName")]
    
    pub advertiser_custom_event_name: Option<String>,
    /// Type of the event. This is a read-only field.
    #[serde(rename="advertiserCustomEventType")]
    
    pub advertiser_custom_event_type: Option<CreativeCustomEventAdvertiserCustomEventTypeEnum>,
    /// Artwork label column, used to link events in Campaign Manager back to events in Studio. This is a required field and should not be modified after insertion.
    #[serde(rename="artworkLabel")]
    
    pub artwork_label: Option<String>,
    /// Artwork type used by the creative.This is a read-only field.
    #[serde(rename="artworkType")]
    
    pub artwork_type: Option<CreativeCustomEventArtworkTypeEnum>,
    /// Exit click-through URL for the event. This field is used only for exit events.
    #[serde(rename="exitClickThroughUrl")]
    
    pub exit_click_through_url: Option<CreativeClickThroughUrl>,
    /// ID of this event. This is a required field and should not be modified after insertion.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Properties for rich media popup windows. This field is used only for exit events.
    #[serde(rename="popupWindowProperties")]
    
    pub popup_window_properties: Option<PopupWindowProperties>,
    /// Target type used by the event.
    #[serde(rename="targetType")]
    
    pub target_type: Option<CreativeCustomEventTargetTypeEnum>,
    /// Video reporting ID, used to differentiate multiple videos in a single creative. This is a read-only field.
    #[serde(rename="videoReportingId")]
    
    pub video_reporting_id: Option<String>,
}

impl client::Part for CreativeCustomEvent {}


/// Represents a DimensionValue resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionValue {
    /// The name of the dimension.
    #[serde(rename="dimensionName")]
    
    pub dimension_name: Option<String>,
    /// The eTag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The ID associated with the value if available.
    
    pub id: Option<String>,
    /// The kind of resource this is, in this case dfareporting#dimensionValue.
    
    pub kind: Option<String>,
    /// Determines how the 'value' field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, '*' is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions ('dfa:paidSearch*') allow a matchType other than EXACT.
    #[serde(rename="matchType")]
    
    pub match_type: Option<DimensionValueMatchTypeEnum>,
    /// The value of the dimension.
    
    pub value: Option<String>,
}

impl client::Part for DimensionValue {}


/// Extra information added to operations that support Scotty media requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaRequestInfo {
    /// The number of current bytes uploaded or downloaded.
    #[serde(rename="currentBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_bytes: Option<i64>,
    /// Data to be copied to backend requests. Custom data is returned to Scotty in the agent_state field, which Scotty will then provide in subsequent upload notifications.
    #[serde(rename="customData")]
    
    pub custom_data: Option<String>,
    /// Set if the http request info is diff encoded. The value of this field is the version number of the base revision. This is corresponding to Apiary's mediaDiffObjectVersion (//depot/google3/java/com/google/api/server/media/variable/DiffObjectVersionVariable.java). See go/esf-scotty-diff-upload for more information.
    #[serde(rename="diffObjectVersion")]
    
    pub diff_object_version: Option<String>,
    /// The existence of the final_status field indicates that this is the last call to the agent for this request_id. http://google3/uploader/agent/scotty_agent.proto?l=737&rcl=347601929
    #[serde(rename="finalStatus")]
    
    pub final_status: Option<i32>,
    /// The type of notification received from Scotty.
    #[serde(rename="notificationType")]
    
    pub notification_type: Option<MediaRequestInfoNotificationTypeEnum>,
    /// The Scotty request ID.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// The partition of the Scotty server handling this request. type is uploader_service.RequestReceivedParamsServingInfo LINT.IfChange(request_received_params_serving_info_annotations) LINT.ThenChange()
    #[serde(rename="requestReceivedParamsServingInfo")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub request_received_params_serving_info: Option<Vec<u8>>,
    /// The total size of the file.
    #[serde(rename="totalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_bytes: Option<i64>,
    /// Whether the total bytes field contains an estimated data.
    #[serde(rename="totalBytesIsEstimated")]
    
    pub total_bytes_is_estimated: Option<bool>,
}

impl client::Part for MediaRequestInfo {}


/// This message is for backends to pass their scotty media specific fields to ESF. Backend will include this in their response message to ESF. Example: ExportFile is an rpc defined for upload using scotty from ESF. rpc ExportFile(ExportFileRequest) returns (ExportFileResponse) Message ExportFileResponse will include apiserving.MediaResponseInfo to tell ESF about data like dynamic_dropzone it needs to pass to Scotty. message ExportFileResponse { optional gdata.Media blob = 1; optional apiserving.MediaResponseInfo media_response_info = 2 }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaResponseInfo {
    /// Data to copy from backend response to the next backend requests. Custom data is returned to Scotty in the agent_state field, which Scotty will then provide in subsequent upload notifications.
    #[serde(rename="customData")]
    
    pub custom_data: Option<String>,
    /// Specifies any transformation to be applied to data before persisting it or retrieving from storage. E.g., encryption options for blobstore2. This should be of the form uploader_service.DataStorageTransform.
    #[serde(rename="dataStorageTransform")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub data_storage_transform: Option<Vec<u8>>,
    /// Specifies the Scotty Drop Target to use for uploads. If present in a media response, Scotty does not upload to a standard drop zone. Instead, Scotty saves the upload directly to the location specified in this drop target. Unlike drop zones, the drop target is the final storage location for an upload. So, the agent does not need to clone the blob at the end of the upload. The agent is responsible for garbage collecting any orphaned blobs that may occur due to aborted uploads. For more information, see the drop target design doc here: http://goto/ScottyDropTarget This field will be preferred to dynamicDropzone. If provided, the identified field in the response must be of the type uploader.agent.DropTarget.
    #[serde(rename="dynamicDropTarget")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub dynamic_drop_target: Option<Vec<u8>>,
    /// Specifies the Scotty dropzone to use for uploads.
    #[serde(rename="dynamicDropzone")]
    
    pub dynamic_dropzone: Option<String>,
    /// Request class to use for all Blobstore operations for this request.
    #[serde(rename="requestClass")]
    
    pub request_class: Option<MediaResponseInfoRequestClassEnum>,
    /// Requester ID passed along to be recorded in the Scotty logs
    #[serde(rename="scottyAgentUserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub scotty_agent_user_id: Option<i64>,
    /// Customer-specific data to be recorded in the Scotty logs type is logs_proto_scotty.CustomerLog
    #[serde(rename="scottyCustomerLog")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub scotty_customer_log: Option<Vec<u8>>,
    /// Specifies the TrafficClass that Scotty should use for any RPCs to fetch the response bytes. Will override the traffic class GTOS of the incoming http request. This is a temporary field to facilitate whitelisting and experimentation by the bigstore agent only. For instance, this does not apply to RTMP reads. WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.
    #[serde(rename="trafficClassField")]
    
    pub traffic_class_field: Option<MediaResponseInfoTrafficClassFieldEnum>,
    /// Tells Scotty to verify hashes on the agent's behalf by parsing out the X-Goog-Hash header.
    #[serde(rename="verifyHashFromHeader")]
    
    pub verify_hash_from_header: Option<bool>,
}

impl client::Part for MediaResponseInfo {}


/// Offset Position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OffsetPosition {
    /// Offset distance from left side of an asset or a window.
    
    pub left: Option<i32>,
    /// Offset distance from top side of an asset or a window.
    
    pub top: Option<i32>,
}

impl client::Part for OffsetPosition {}


/// Popup Window Properties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PopupWindowProperties {
    /// Popup dimension for a creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA and all VPAID
    
    pub dimension: Option<Size>,
    /// Upper-left corner coordinates of the popup window. Applicable if positionType is COORDINATES.
    
    pub offset: Option<OffsetPosition>,
    /// Popup window position either centered or at specific coordinate.
    #[serde(rename="positionType")]
    
    pub position_type: Option<PopupWindowPropertyPositionTypeEnum>,
    /// Whether to display the browser address bar.
    #[serde(rename="showAddressBar")]
    
    pub show_address_bar: Option<bool>,
    /// Whether to display the browser menu bar.
    #[serde(rename="showMenuBar")]
    
    pub show_menu_bar: Option<bool>,
    /// Whether to display the browser scroll bar.
    #[serde(rename="showScrollBar")]
    
    pub show_scroll_bar: Option<bool>,
    /// Whether to display the browser status bar.
    #[serde(rename="showStatusBar")]
    
    pub show_status_bar: Option<bool>,
    /// Whether to display the browser tool bar.
    #[serde(rename="showToolBar")]
    
    pub show_tool_bar: Option<bool>,
    /// Title of popup window.
    
    pub title: Option<String>,
}

impl client::Part for PopupWindowProperties {}


/// Represents the dimensions of ads, placements, creatives, or creative assets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Size {
    /// Height of this size. Acceptable values are 0 to 32767, inclusive.
    
    pub height: Option<i32>,
    /// IAB standard size. This is a read-only, auto-generated field.
    
    pub iab: Option<bool>,
    /// ID of this size. This is a read-only, auto-generated field.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies what kind of resource this is. Value: the fixed string "dfareporting#size".
    
    pub kind: Option<String>,
    /// Width of this size. Acceptable values are 0 to 32767, inclusive.
    
    pub width: Option<i32>,
}

impl client::Part for Size {}


