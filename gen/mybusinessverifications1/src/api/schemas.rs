use super::*;
/// Display data for verifications through postcard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddressVerificationData {
    /// Address that a postcard can be sent to.
    
    pub address: Option<PostalAddress>,
    /// Merchant's business name.
    
    pub business: Option<String>,
    /// Expected number of days it takes to deliver a postcard to the address's region.
    #[serde(rename="expectedDeliveryDaysRegion")]
    
    pub expected_delivery_days_region: Option<i32>,
}

impl client::Part for AddressVerificationData {}


/// Request message for Verifications.CompleteVerificationAction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verifications complete locations](LocationVerificationCompleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompleteVerificationRequest {
    /// Required. PIN code received by the merchant to complete the verification.
    
    pub pin: Option<String>,
}

impl client::RequestValue for CompleteVerificationRequest {}


/// Response message for Verifications.CompleteVerificationAction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verifications complete locations](LocationVerificationCompleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompleteVerificationResponse {
    /// The completed verification.
    
    pub verification: Option<Verification>,
}

impl client::ResponseResult for CompleteVerificationResponse {}


/// Indicates that the location fails to comply with our [guidelines](https://support.google.com/business/answer/3038177).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComplyWithGuidelines {
    /// The reason why the location is being recommended to comply with guidelines.
    #[serde(rename="recommendationReason")]
    
    pub recommendation_reason: Option<ComplyWithGuidelineRecommendationReasonEnum>,
}

impl client::Part for ComplyWithGuidelines {}


/// Display data for verifications through email.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmailVerificationData {
    /// Domain name in the email address. e.g. "gmail.com" in foo@gmail.com
    
    pub domain: Option<String>,
    /// Whether client is allowed to provide a different user name.
    #[serde(rename="isUserNameEditable")]
    
    pub is_user_name_editable: Option<bool>,
    /// User name in the email address. e.g. "foo" in foo@gmail.com
    
    pub user: Option<String>,
}

impl client::Part for EmailVerificationData {}


/// Request message for Verifications.FetchVerificationOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [fetch verification options locations](LocationFetchVerificationOptionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FetchVerificationOptionsRequest {
    /// Optional. Extra context information for the verification of service businesses. Can only be applied to the locations whose business type is CUSTOMER_LOCATION_ONLY. Specifying an accurate address could enable more options. INVALID_ARGUMENT will be thrown if it is set for other business types of locations.
    
    pub context: Option<ServiceBusinessContext>,
    /// Required. The BCP 47 language code representing the language that is to be used for the verification process. Available options vary by language.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::RequestValue for FetchVerificationOptionsRequest {}


/// Response message for Verifications.FetchVerificationOptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [fetch verification options locations](LocationFetchVerificationOptionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FetchVerificationOptionsResponse {
    /// The available verification options.
    
    pub options: Option<Vec<VerificationOption>>,
}

impl client::ResponseResult for FetchVerificationOptionsResponse {}


/// Response message for Verifications.ListVerifications.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verifications list locations](LocationVerificationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVerificationsResponse {
    /// If the number of verifications exceeded the requested page size, this field will be populated with a token to fetch the next page of verification on a subsequent call. If there are no more attributes, this field will not be present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of the verifications.
    
    pub verifications: Option<Vec<Verification>>,
}

impl client::ResponseResult for ListVerificationsResponse {}


/// Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an internationalization-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostalAddress {
    /// Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. "Austin, TX"), it is important that the line order is clear. The order of address lines should be "envelope order" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. "ja" for large-to-small ordering and "ja-Latn" or "en" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas).
    #[serde(rename="addressLines")]
    
    pub address_lines: Option<Vec<String>>,
    /// Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. "Barcelona" and not "Catalonia"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated.
    #[serde(rename="administrativeArea")]
    
    pub administrative_area: Option<String>,
    /// Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: "zh-Hant", "ja", "ja-Latn", "en".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines.
    
    pub locality: Option<String>,
    /// Optional. The name of the organization at the address.
    
    pub organization: Option<String>,
    /// Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.).
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain "care of" information.
    
    pub recipients: Option<Vec<String>>,
    /// Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See https://cldr.unicode.org/ and https://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: "CH" for Switzerland.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions.
    
    pub revision: Option<i32>,
    /// Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like "CEDEX", optionally followed by a number (e.g. "CEDEX 7"), or just a number alone, representing the "sector code" (Jamaica), "delivery area indicator" (Malawi) or "post office indicator" (e.g. Côte d'Ivoire).
    #[serde(rename="sortingCode")]
    
    pub sorting_code: Option<String>,
    /// Optional. Sublocality of the address. For example, this can be neighborhoods, boroughs, districts.
    
    pub sublocality: Option<String>,
}

impl client::Part for PostalAddress {}


/// Indicates that the location duplicates another location that is in good standing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResolveOwnershipConflict { _never_set: Option<bool> }

impl client::Part for ResolveOwnershipConflict {}


/// Additional data for service business verification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceBusinessContext {
    /// The verification address of the location. It is used to either enable more verification options or send a postcard.
    
    pub address: Option<PostalAddress>,
}

impl client::Part for ServiceBusinessContext {}


/// A verification represents a verification attempt on a location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Verification {
    /// Optional. Response announcement set only if the method is VETTED_PARTNER.
    
    pub announcement: Option<String>,
    /// The timestamp when the verification is requested.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The method of the verification.
    
    pub method: Option<VerificationMethodEnum>,
    /// Resource name of the verification.
    
    pub name: Option<String>,
    /// The state of the verification.
    
    pub state: Option<VerificationStateEnum>,
}

impl client::Part for Verification {}


/// The verification option represents how to verify the location (indicated by verification method) and where the verification will be sent to (indicated by display data).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerificationOption {
    /// Set only if the method is MAIL.
    #[serde(rename="addressData")]
    
    pub address_data: Option<AddressVerificationData>,
    /// Set only if the method is VETTED_PARTNER.
    
    pub announcement: Option<String>,
    /// Set only if the method is EMAIL.
    #[serde(rename="emailData")]
    
    pub email_data: Option<EmailVerificationData>,
    /// Set only if the method is PHONE_CALL or SMS. Phone number that the PIN will be sent to.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Method to verify the location.
    #[serde(rename="verificationMethod")]
    
    pub verification_method: Option<VerificationOptionVerificationMethodEnum>,
}

impl client::Part for VerificationOption {}


/// Token generated by a vetted [partner](https://support.google.com/business/answer/7674102).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerificationToken {
    /// The token string.
    #[serde(rename="tokenString")]
    
    pub token_string: Option<String>,
}

impl client::Part for VerificationToken {}


/// Indicates that the location requires verification. Contains information about the current verification actions performed on the location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Verify {
    /// Indicates whether a verification process has already started, and can be completed by the location.
    #[serde(rename="hasPendingVerification")]
    
    pub has_pending_verification: Option<bool>,
}

impl client::Part for Verify {}


/// Request message for Verifications.VerifyLocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify locations](LocationVerifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyLocationRequest {
    /// Optional. Extra context information for the verification of service businesses. It is only required for the locations whose business type is CUSTOMER_LOCATION_ONLY. For ADDRESS verification, the address will be used to send out postcard. For other methods, it should be the same as the one that is passed to GetVerificationOptions. INVALID_ARGUMENT will be thrown if it is set for other types of business locations.
    
    pub context: Option<ServiceBusinessContext>,
    /// Optional. The input for EMAIL method. Email address where the PIN should be sent to. An email address is accepted only if it is one of the addresses provided by FetchVerificationOptions. If the EmailVerificationData has is_user_name_editable set to true, the client may specify a different user name (local-part) but must match the domain name.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Optional. The BCP 47 language code representing the language that is to be used for the verification process.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. The input for ADDRESS method. Contact name the mail should be sent to.
    #[serde(rename="mailerContact")]
    
    pub mailer_contact: Option<String>,
    /// Required. Verification method.
    
    pub method: Option<VerifyLocationRequestMethodEnum>,
    /// Optional. The input for PHONE_CALL/SMS method The phone number that should be called or be sent SMS to. It must be one of the phone numbers in the eligible options.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Optional. The input for VETTED_PARTNER method available to select [partners.](https://support.google.com/business/answer/7674102) The input is not needed for a vetted account. Token that is associated to the location. Token that is associated to the location.
    
    pub token: Option<VerificationToken>,
}

impl client::RequestValue for VerifyLocationRequest {}


/// Response message for Verifications.VerifyLocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify locations](LocationVerifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyLocationResponse {
    /// The created verification request.
    
    pub verification: Option<Verification>,
}

impl client::ResponseResult for VerifyLocationResponse {}


/// Response message for VoiceOfMerchant.GetVoiceOfMerchantState.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get voice of merchant state locations](LocationGetVoiceOfMerchantStateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VoiceOfMerchantState {
    /// The location fails to comply with our [guidelines](https://support.google.com/business/answer/3038177) and requires additional steps for reinstatement. To fix this issue, consult the [Help Center Article](https://support.google.com/business/answer/4569145).
    #[serde(rename="complyWithGuidelines")]
    
    pub comply_with_guidelines: Option<ComplyWithGuidelines>,
    /// Indicates whether the location has the authority (ownership) over the business on Google. If true, another location cannot take over and become the dominant listing on Maps. However, edits will not become live unless Voice of Merchant is gained (i.e. has_voice_of_merchant is true).
    #[serde(rename="hasBusinessAuthority")]
    
    pub has_business_authority: Option<bool>,
    /// Indicates whether the location is in good standing and has control over the business on Google. Any edits made to the location will propagate to Maps after passing the review phase.
    #[serde(rename="hasVoiceOfMerchant")]
    
    pub has_voice_of_merchant: Option<bool>,
    /// This location duplicates another location that is in good standing. If you have access to the location in good standing, use that location's id to perform operations. Otherwise, request access from the current owner.
    #[serde(rename="resolveOwnershipConflict")]
    
    pub resolve_ownership_conflict: Option<ResolveOwnershipConflict>,
    /// Start or continue the verification process.
    
    pub verify: Option<Verify>,
    /// Wait to gain Voice of Merchant. The location is under review for quality purposes.
    #[serde(rename="waitForVoiceOfMerchant")]
    
    pub wait_for_voice_of_merchant: Option<WaitForVoiceOfMerchant>,
}

impl client::ResponseResult for VoiceOfMerchantState {}


/// Indicates that the location will gain voice of merchant after passing review.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaitForVoiceOfMerchant { _never_set: Option<bool> }

impl client::Part for WaitForVoiceOfMerchant {}


