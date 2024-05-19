use super::*;
/// (Restricted Access) Contains a signal helping apps differentiating between likely genuine and likely non-genuine user traffic.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountActivity {
    /// Required. Indicates the activity level of the account.
    #[serde(rename="activityLevel")]
    
    pub activity_level: Option<AccountActivityActivityLevelEnum>,
}

impl client::Part for AccountActivity {}


/// Contains the account information such as the licensing status for the user in the scope.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountDetails {
    /// (Restricted Access) Details about the account activity for the user in the scope.
    #[serde(rename="accountActivity")]
    
    pub account_activity: Option<AccountActivity>,
    /// Required. Details about the licensing status of the user for the app in the scope.
    #[serde(rename="appLicensingVerdict")]
    
    pub app_licensing_verdict: Option<AccountDetailAppLicensingVerdictEnum>,
}

impl client::Part for AccountDetails {}


/// Contains signals about others apps on the device which could be used to access or control the requesting app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppAccessRiskVerdict {
    /// Required. App access risk verdict related to apps that are not installed by Google Play, and are not preloaded on the system image by the device manufacturer.
    #[serde(rename="otherApps")]
    
    pub other_apps: Option<AppAccessRiskVerdictOtherAppsEnum>,
    /// Required. App access risk verdict related to apps that are not installed by the Google Play Store, and are not preloaded on the system image by the device manufacturer.
    #[serde(rename="playOrSystemApps")]
    
    pub play_or_system_apps: Option<AppAccessRiskVerdictPlayOrSystemAppsEnum>,
}

impl client::Part for AppAccessRiskVerdict {}


/// Contains the application integrity information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppIntegrity {
    /// Required. Details about the app recognition verdict
    #[serde(rename="appRecognitionVerdict")]
    
    pub app_recognition_verdict: Option<AppIntegrityAppRecognitionVerdictEnum>,
    /// The SHA256 hash of the requesting app's signing certificates (base64 web-safe encoded). Set iff app_recognition_verdict != UNEVALUATED.
    #[serde(rename="certificateSha256Digest")]
    
    pub certificate_sha256_digest: Option<Vec<String>>,
    /// Package name of the application under attestation. Set iff app_recognition_verdict != UNEVALUATED.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Version code of the application. Set iff app_recognition_verdict != UNEVALUATED.
    #[serde(rename="versionCode")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version_code: Option<i64>,
}

impl client::Part for AppIntegrity {}


/// Request to decode the integrity token.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [decode integrity token](MethodDecodeIntegrityTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DecodeIntegrityTokenRequest {
    /// Encoded integrity token.
    #[serde(rename="integrityToken")]
    
    pub integrity_token: Option<String>,
}

impl client::RequestValue for DecodeIntegrityTokenRequest {}


/// Response containing the decoded integrity payload.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [decode integrity token](MethodDecodeIntegrityTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DecodeIntegrityTokenResponse {
    /// Plain token payload generated from the decoded integrity token.
    #[serde(rename="tokenPayloadExternal")]
    
    pub token_payload_external: Option<TokenPayloadExternal>,
}

impl client::ResponseResult for DecodeIntegrityTokenResponse {}


/// Contains the device attestation information. Next tag: 4
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceIntegrity {
    /// Details about the integrity of the device the app is running on.
    #[serde(rename="deviceRecognitionVerdict")]
    
    pub device_recognition_verdict: Option<Vec<DeviceIntegrityDeviceRecognitionVerdictEnum>>,
    /// Details about the device activity of the device the app is running on.
    #[serde(rename="recentDeviceActivity")]
    
    pub recent_device_activity: Option<RecentDeviceActivity>,
}

impl client::Part for DeviceIntegrity {}


/// Contains information about the environment Play Integrity API runs in, e.g. Play Protect verdict.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentDetails {
    /// The evaluation of the App Access Risk verdicts.
    #[serde(rename="appAccessRiskVerdict")]
    
    pub app_access_risk_verdict: Option<AppAccessRiskVerdict>,
    /// The evaluation of Play Protect verdict.
    #[serde(rename="playProtectVerdict")]
    
    pub play_protect_verdict: Option<EnvironmentDetailPlayProtectVerdictEnum>,
}

impl client::Part for EnvironmentDetails {}


/// Recent device activity can help developers identify devices that have exhibited hyperactive attestation activity, which could be a sign of an attack or token farming.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecentDeviceActivity {
    /// Required. Indicates the activity level of the device.
    #[serde(rename="deviceActivityLevel")]
    
    pub device_activity_level: Option<RecentDeviceActivityDeviceActivityLevelEnum>,
}

impl client::Part for RecentDeviceActivity {}


/// Contains the integrity request information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestDetails {
    /// Nonce that was provided in the request (which is base64 web-safe no-wrap).
    
    pub nonce: Option<String>,
    /// Request hash that was provided in the request.
    #[serde(rename="requestHash")]
    
    pub request_hash: Option<String>,
    /// Required. Application package name this attestation was requested for. Note: This field makes no guarantees or promises on the caller integrity. For details on application integrity, check application_integrity.
    #[serde(rename="requestPackageName")]
    
    pub request_package_name: Option<String>,
    /// Required. Timestamp, in milliseconds, of the integrity application request.
    #[serde(rename="timestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp_millis: Option<i64>,
}

impl client::Part for RequestDetails {}


/// Contains additional information generated for testing responses.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestingDetails {
    /// Required. Indicates that the information contained in this payload is a testing response that is statically overridden for a tester.
    #[serde(rename="isTestingResponse")]
    
    pub is_testing_response: Option<bool>,
}

impl client::Part for TestingDetails {}


/// Contains basic app information and integrity signals like device attestation and licensing details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TokenPayloadExternal {
    /// Required. Details about the Play Store account.
    #[serde(rename="accountDetails")]
    
    pub account_details: Option<AccountDetails>,
    /// Required. Details about the application integrity.
    #[serde(rename="appIntegrity")]
    
    pub app_integrity: Option<AppIntegrity>,
    /// Required. Details about the device integrity.
    #[serde(rename="deviceIntegrity")]
    
    pub device_integrity: Option<DeviceIntegrity>,
    /// Details of the environment Play Integrity API runs in.
    #[serde(rename="environmentDetails")]
    
    pub environment_details: Option<EnvironmentDetails>,
    /// Required. Details about the integrity request.
    #[serde(rename="requestDetails")]
    
    pub request_details: Option<RequestDetails>,
    /// Indicates that this payload is generated for testing purposes and contains any additional data that is linked with testing status.
    #[serde(rename="testingDetails")]
    
    pub testing_details: Option<TestingDetails>,
}

impl client::Part for TokenPayloadExternal {}


