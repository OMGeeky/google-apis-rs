use super::*;
/// Account defender risk assessment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment {
    /// Output only. Labels for this request.
    
    pub labels: Option<Vec<GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsEnum>>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment {}


/// Information about account verification, used for identity verification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AccountVerificationInfo {
    /// Optional. Endpoints that can be used for identity verification.
    
    pub endpoints: Option<Vec<GoogleCloudRecaptchaenterpriseV1EndpointVerificationInfo>>,
    /// Optional. Language code preference for the verification message, set as a IETF BCP 47 language code.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Output only. Result of the latest account verification challenge.
    #[serde(rename="latestVerificationResult")]
    
    pub latest_verification_result: Option<GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResultEnum>,
    /// Username of the account that is being verified. Deprecated. Customers should now provide the `account_id` field in `event.user_info`.
    
    pub username: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfo {}


/// Settings specific to keys that can be used by Android apps.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AndroidKeySettings {
    /// Optional. If set to true, allowed_package_names are not enforced.
    #[serde(rename="allowAllPackageNames")]
    
    pub allow_all_package_names: Option<bool>,
    /// Optional. Android package names of apps allowed to use the key. Example: 'com.companyname.appname'
    #[serde(rename="allowedPackageNames")]
    
    pub allowed_package_names: Option<Vec<String>>,
    /// Optional. Set to true for keys that are used in an Android application that is available for download in app stores in addition to the Google Play Store.
    #[serde(rename="supportNonGoogleAppStoreDistribution")]
    
    pub support_non_google_app_store_distribution: Option<bool>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1AndroidKeySettings {}


/// The request message to annotate an Assessment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assessments annotate projects](ProjectAssessmentAnnotateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest {
    /// Optional. A stable account identifier to apply to the assessment. This is an alternative to setting `account_id` in `CreateAssessment`, for example when a stable account identifier is not yet known in the initial request.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Optional. The annotation that will be assigned to the Event. This field can be left empty to provide reasons that apply to an event without concluding whether the event is legitimate or fraudulent.
    
    pub annotation: Option<GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotationEnum>,
    /// Optional. A stable hashed account identifier to apply to the assessment. This is an alternative to setting `hashed_account_id` in `CreateAssessment`, for example when a stable account identifier is not yet known in the initial request.
    #[serde(rename="hashedAccountId")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub hashed_account_id: Option<Vec<u8>>,
    /// Optional. Reasons for the annotation that are assigned to the event.
    
    pub reasons: Option<Vec<GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsEnum>>,
    /// Optional. If the assessment is part of a payment transaction, provide details on payment lifecycle events that occur in the transaction.
    #[serde(rename="transactionEvent")]
    
    pub transaction_event: Option<GoogleCloudRecaptchaenterpriseV1TransactionEvent>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest {}


/// Empty response for AnnotateAssessment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assessments annotate projects](ProjectAssessmentAnnotateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse {}


/// Contains fields that are required to perform Apple-specific integrity checks.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1AppleDeveloperId {
    /// Required. The Apple developer key ID (10-character string).
    #[serde(rename="keyId")]
    
    pub key_id: Option<String>,
    /// Required. Input only. A private key (downloaded as a text file with a .p8 file extension) generated for your Apple Developer account. Ensure that Apple DeviceCheck is enabled for the private key.
    #[serde(rename="privateKey")]
    
    pub private_key: Option<String>,
    /// Required. The Apple team ID (10-character string) owning the provisioning profile used to build your application.
    #[serde(rename="teamId")]
    
    pub team_id: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1AppleDeveloperId {}


/// A reCAPTCHA Enterprise assessment resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assessments create projects](ProjectAssessmentCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1Assessment {
    /// Output only. Assessment returned by account defender when an account identifier is provided.
    #[serde(rename="accountDefenderAssessment")]
    
    pub account_defender_assessment: Option<GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment>,
    /// Optional. Account verification information for identity verification. The assessment event must include a token and site key to use this feature.
    #[serde(rename="accountVerification")]
    
    pub account_verification: Option<GoogleCloudRecaptchaenterpriseV1AccountVerificationInfo>,
    /// Optional. The event being assessed.
    
    pub event: Option<GoogleCloudRecaptchaenterpriseV1Event>,
    /// Output only. Assessment returned when firewall policies belonging to the project are evaluated using the field firewall_policy_evaluation.
    #[serde(rename="firewallPolicyAssessment")]
    
    pub firewall_policy_assessment: Option<GoogleCloudRecaptchaenterpriseV1FirewallPolicyAssessment>,
    /// Output only. Assessment returned by Fraud Prevention when TransactionData is provided.
    #[serde(rename="fraudPreventionAssessment")]
    
    pub fraud_prevention_assessment: Option<GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessment>,
    /// Output only. Fraud Signals specific to the users involved in a payment transaction.
    #[serde(rename="fraudSignals")]
    
    pub fraud_signals: Option<GoogleCloudRecaptchaenterpriseV1FraudSignals>,
    /// Output only. Identifier. The resource name for the Assessment in the format `projects/{project}/assessments/{assessment}`.
    
    pub name: Option<String>,
    /// Optional. The private password leak verification field contains the parameters that are used to to check for leaks privately without sharing user credentials.
    #[serde(rename="privatePasswordLeakVerification")]
    
    pub private_password_leak_verification: Option<GoogleCloudRecaptchaenterpriseV1PrivatePasswordLeakVerification>,
    /// Output only. The risk analysis result for the event being assessed.
    #[serde(rename="riskAnalysis")]
    
    pub risk_analysis: Option<GoogleCloudRecaptchaenterpriseV1RiskAnalysis>,
    /// Output only. Properties of the provided event token.
    #[serde(rename="tokenProperties")]
    
    pub token_properties: Option<GoogleCloudRecaptchaenterpriseV1TokenProperties>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1Assessment {}
impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1Assessment {}


/// Metrics related to challenges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ChallengeMetrics {
    /// Count of submitted challenge solutions that were incorrect or otherwise deemed suspicious such that a subsequent challenge was triggered.
    #[serde(rename="failedCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub failed_count: Option<i64>,
    /// Count of nocaptchas (successful verification without a challenge) issued.
    #[serde(rename="nocaptchaCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub nocaptcha_count: Option<i64>,
    /// Count of reCAPTCHA checkboxes or badges rendered. This is mostly equivalent to a count of pageloads for pages that include reCAPTCHA.
    #[serde(rename="pageloadCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pageload_count: Option<i64>,
    /// Count of nocaptchas (successful verification without a challenge) plus submitted challenge solutions that were correct and resulted in verification.
    #[serde(rename="passedCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub passed_count: Option<i64>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1ChallengeMetrics {}


/// Information about a verification endpoint that can be used for 2FA.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1EndpointVerificationInfo {
    /// Email address for which to trigger a verification request.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Output only. Timestamp of the last successful verification for the endpoint, if any.
    #[serde(rename="lastVerificationTime")]
    
    pub last_verification_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Phone number for which to trigger a verification request. Should be given in E.164 format.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Output only. Token to provide to the client to trigger endpoint verification. It must be used within 15 minutes.
    #[serde(rename="requestToken")]
    
    pub request_token: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1EndpointVerificationInfo {}


/// The event being assessed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1Event {
    /// Optional. The expected action for this type of event. This should be the same action provided at token generation time on client-side platforms already integrated with recaptcha enterprise.
    #[serde(rename="expectedAction")]
    
    pub expected_action: Option<String>,
    /// Optional. Flag for a reCAPTCHA express request for an assessment without a token. If enabled, `site_key` must reference a SCORE key with WAF feature set to EXPRESS.
    
    pub express: Option<bool>,
    /// Optional. Flag for enabling firewall policy config assessment. If this flag is enabled, the firewall policy will be evaluated and a suggested firewall action will be returned in the response.
    #[serde(rename="firewallPolicyEvaluation")]
    
    pub firewall_policy_evaluation: Option<bool>,
    /// Optional. The Fraud Prevention setting for this assessment.
    #[serde(rename="fraudPrevention")]
    
    pub fraud_prevention: Option<GoogleCloudRecaptchaenterpriseV1EventFraudPreventionEnum>,
    /// Optional. Deprecated: use `user_info.account_id` instead. Unique stable hashed user identifier for the request. The identifier must be hashed using hmac-sha256 with stable secret.
    #[serde(rename="hashedAccountId")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub hashed_account_id: Option<Vec<u8>>,
    /// Optional. HTTP header information about the request.
    
    pub headers: Option<Vec<String>>,
    /// Optional. JA3 fingerprint for SSL clients.
    
    pub ja3: Option<String>,
    /// Optional. The URI resource the user requested that triggered an assessment.
    #[serde(rename="requestedUri")]
    
    pub requested_uri: Option<String>,
    /// Optional. The site key that was used to invoke reCAPTCHA Enterprise on your site and generate the token.
    #[serde(rename="siteKey")]
    
    pub site_key: Option<String>,
    /// Optional. The user response token provided by the reCAPTCHA Enterprise client-side integration on your site.
    
    pub token: Option<String>,
    /// Optional. Data describing a payment transaction to be assessed. Sending this data enables reCAPTCHA Enterprise Fraud Prevention and the FraudPreventionAssessment component in the response.
    #[serde(rename="transactionData")]
    
    pub transaction_data: Option<GoogleCloudRecaptchaenterpriseV1TransactionData>,
    /// Optional. The user agent present in the request from the user's device related to this event.
    #[serde(rename="userAgent")]
    
    pub user_agent: Option<String>,
    /// Optional. Information about the user that generates this event, when they can be identified. They are often identified through the use of an account for logged-in requests or login/registration requests, or by providing user identifiers for guest actions like checkout.
    #[serde(rename="userInfo")]
    
    pub user_info: Option<GoogleCloudRecaptchaenterpriseV1UserInfo>,
    /// Optional. The IP address in the request from the user's device related to this event.
    #[serde(rename="userIpAddress")]
    
    pub user_ip_address: Option<String>,
    /// Optional. Flag for running WAF token assessment. If enabled, the token must be specified, and have been created by a WAF-enabled key.
    #[serde(rename="wafTokenAssessment")]
    
    pub waf_token_assessment: Option<bool>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1Event {}


/// An individual action. Each action represents what to do if a policy matches.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FirewallAction {
    /// The user request did not match any policy and should be allowed access to the requested resource.
    
    pub allow: Option<GoogleCloudRecaptchaenterpriseV1FirewallActionAllowAction>,
    /// This action will deny access to a given page. The user will get an HTTP error code.
    
    pub block: Option<GoogleCloudRecaptchaenterpriseV1FirewallActionBlockAction>,
    /// This action will inject reCAPTCHA JavaScript code into the HTML page returned by the site backend.
    #[serde(rename="includeRecaptchaScript")]
    
    pub include_recaptcha_script: Option<GoogleCloudRecaptchaenterpriseV1FirewallActionIncludeRecaptchaScriptAction>,
    /// This action will redirect the request to a ReCaptcha interstitial to attach a token.
    
    pub redirect: Option<GoogleCloudRecaptchaenterpriseV1FirewallActionRedirectAction>,
    /// This action will set a custom header but allow the request to continue to the customer backend.
    #[serde(rename="setHeader")]
    
    pub set_header: Option<GoogleCloudRecaptchaenterpriseV1FirewallActionSetHeaderAction>,
    /// This action will transparently serve a different page to an offending user.
    
    pub substitute: Option<GoogleCloudRecaptchaenterpriseV1FirewallActionSubstituteAction>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FirewallAction {}


/// An allow action continues processing a request unimpeded.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FirewallActionAllowAction { _never_set: Option<bool> }

impl client::Part for GoogleCloudRecaptchaenterpriseV1FirewallActionAllowAction {}


/// A block action serves an HTTP error code a prevents the request from hitting the backend.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FirewallActionBlockAction { _never_set: Option<bool> }

impl client::Part for GoogleCloudRecaptchaenterpriseV1FirewallActionBlockAction {}


/// An include reCAPTCHA script action involves injecting reCAPTCHA JavaScript code into the HTML returned by the site backend. This reCAPTCHA script is tasked with collecting user signals on the requested web page, issuing tokens as a cookie within the site domain, and enabling their utilization in subsequent page requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FirewallActionIncludeRecaptchaScriptAction { _never_set: Option<bool> }

impl client::Part for GoogleCloudRecaptchaenterpriseV1FirewallActionIncludeRecaptchaScriptAction {}


/// A redirect action returns a 307 (temporary redirect) response, pointing the user to a ReCaptcha interstitial page to attach a token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FirewallActionRedirectAction { _never_set: Option<bool> }

impl client::Part for GoogleCloudRecaptchaenterpriseV1FirewallActionRedirectAction {}


/// A set header action sets a header and forwards the request to the backend. This can be used to trigger custom protection implemented on the backend.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FirewallActionSetHeaderAction {
    /// Optional. The header key to set in the request to the backend server.
    
    pub key: Option<String>,
    /// Optional. The header value to set in the request to the backend server.
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FirewallActionSetHeaderAction {}


/// A substitute action transparently serves a different page than the one requested.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FirewallActionSubstituteAction {
    /// Optional. The address to redirect to. The target is a relative path in the current host. Example: "/blog/404.html".
    
    pub path: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FirewallActionSubstituteAction {}


/// A FirewallPolicy represents a single matching pattern and resulting actions to take.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firewallpolicies create projects](ProjectFirewallpolicyCreateCall) (request|response)
/// * [firewallpolicies get projects](ProjectFirewallpolicyGetCall) (response)
/// * [firewallpolicies patch projects](ProjectFirewallpolicyPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FirewallPolicy {
    /// Optional. The actions that the caller should take regarding user access. There should be at most one terminal action. A terminal action is any action that forces a response, such as `AllowAction`, `BlockAction` or `SubstituteAction`. Zero or more non-terminal actions such as `SetHeader` might be specified. A single policy can contain up to 16 actions.
    
    pub actions: Option<Vec<GoogleCloudRecaptchaenterpriseV1FirewallAction>>,
    /// Optional. A CEL (Common Expression Language) conditional expression that specifies if this policy applies to an incoming user request. If this condition evaluates to true and the requested path matched the path pattern, the associated actions should be executed by the caller. The condition string is checked for CEL syntax correctness on creation. For more information, see the [CEL spec](https://github.com/google/cel-spec) and its [language definition](https://github.com/google/cel-spec/blob/master/doc/langdef.md). A condition has a max length of 500 characters.
    
    pub condition: Option<String>,
    /// Optional. A description of what this policy aims to achieve, for convenience purposes. The description can at most include 256 UTF-8 characters.
    
    pub description: Option<String>,
    /// Identifier. The resource name for the FirewallPolicy in the format `projects/{project}/firewallpolicies/{firewallpolicy}`.
    
    pub name: Option<String>,
    /// Optional. The path for which this policy applies, specified as a glob pattern. For more information on glob, see the [manual page](https://man7.org/linux/man-pages/man7/glob.7.html). A path has a max length of 200 characters.
    
    pub path: Option<String>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1FirewallPolicy {}
impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1FirewallPolicy {}


/// Policy config assessment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FirewallPolicyAssessment {
    /// Output only. If the processing of a policy config fails, an error will be populated and the firewall_policy will be left empty.
    
    pub error: Option<GoogleRpcStatus>,
    /// Output only. The policy that matched the request. If more than one policy may match, this is the first match. If no policy matches the incoming request, the policy field will be left empty.
    #[serde(rename="firewallPolicy")]
    
    pub firewall_policy: Option<GoogleCloudRecaptchaenterpriseV1FirewallPolicy>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FirewallPolicyAssessment {}


/// Assessment for Fraud Prevention.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessment {
    /// Output only. Assessment of this transaction for behavioral trust.
    #[serde(rename="behavioralTrustVerdict")]
    
    pub behavioral_trust_verdict: Option<GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessmentBehavioralTrustVerdict>,
    /// Output only. Assessment of this transaction for risk of being part of a card testing attack.
    #[serde(rename="cardTestingVerdict")]
    
    pub card_testing_verdict: Option<GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessmentCardTestingVerdict>,
    /// Output only. Assessment of this transaction for risk of a stolen instrument.
    #[serde(rename="stolenInstrumentVerdict")]
    
    pub stolen_instrument_verdict: Option<GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessmentStolenInstrumentVerdict>,
    /// Output only. Probability of this transaction being fraudulent. Summarizes the combined risk of attack vectors below. Values are from 0.0 (lowest) to 1.0 (highest).
    #[serde(rename="transactionRisk")]
    
    pub transaction_risk: Option<f32>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessment {}


/// Information about behavioral trust of the transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessmentBehavioralTrustVerdict {
    /// Output only. Probability of this transaction attempt being executed in a behaviorally trustworthy way. Values are from 0.0 (lowest) to 1.0 (highest).
    
    pub trust: Option<f32>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessmentBehavioralTrustVerdict {}


/// Information about card testing fraud, where an adversary is testing fraudulently obtained cards or brute forcing their details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessmentCardTestingVerdict {
    /// Output only. Probability of this transaction attempt being part of a card testing attack. Values are from 0.0 (lowest) to 1.0 (highest).
    
    pub risk: Option<f32>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessmentCardTestingVerdict {}


/// Information about stolen instrument fraud, where the user is not the legitimate owner of the instrument being used for the purchase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessmentStolenInstrumentVerdict {
    /// Output only. Probability of this transaction being executed with a stolen instrument. Values are from 0.0 (lowest) to 1.0 (highest).
    
    pub risk: Option<f32>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FraudPreventionAssessmentStolenInstrumentVerdict {}


/// Fraud signals describing users and cards involved in the transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FraudSignals {
    /// Output only. Signals describing the payment card or cards used in this transaction.
    #[serde(rename="cardSignals")]
    
    pub card_signals: Option<GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignals>,
    /// Output only. Signals describing the end user in this transaction.
    #[serde(rename="userSignals")]
    
    pub user_signals: Option<GoogleCloudRecaptchaenterpriseV1FraudSignalsUserSignals>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FraudSignals {}


/// Signals describing the payment card used in this transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignals {
    /// Output only. The labels for the payment card in this transaction.
    #[serde(rename="cardLabels")]
    
    pub card_labels: Option<Vec<GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignalCardLabelsEnum>>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FraudSignalsCardSignals {}


/// Signals describing the user involved in this transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1FraudSignalsUserSignals {
    /// Output only. This user (based on email, phone, and other identifiers) has been seen on the internet for at least this number of days.
    #[serde(rename="activeDaysLowerBound")]
    
    pub active_days_lower_bound: Option<i32>,
    /// Output only. Likelihood (from 0.0 to 1.0) this user includes synthetic components in their identity, such as a randomly generated email address, temporary phone number, or fake shipping address.
    #[serde(rename="syntheticRisk")]
    
    pub synthetic_risk: Option<f32>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1FraudSignalsUserSignals {}


/// Settings specific to keys that can be used by iOS apps.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1IOSKeySettings {
    /// Optional. If set to true, allowed_bundle_ids are not enforced.
    #[serde(rename="allowAllBundleIds")]
    
    pub allow_all_bundle_ids: Option<bool>,
    /// Optional. iOS bundle ids of apps allowed to use the key. Example: 'com.companyname.productname.appname'
    #[serde(rename="allowedBundleIds")]
    
    pub allowed_bundle_ids: Option<Vec<String>>,
    /// Optional. Apple Developer account details for the app that is protected by the reCAPTCHA Key. reCAPTCHA Enterprise leverages platform-specific checks like Apple App Attest and Apple DeviceCheck to protect your app from abuse. Providing these fields allows reCAPTCHA Enterprise to get a better assessment of the integrity of your app.
    #[serde(rename="appleDeveloperId")]
    
    pub apple_developer_id: Option<GoogleCloudRecaptchaenterpriseV1AppleDeveloperId>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1IOSKeySettings {}


/// A key used to identify and configure applications (web and/or mobile) that use reCAPTCHA Enterprise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys create projects](ProjectKeyCreateCall) (request|response)
/// * [keys get projects](ProjectKeyGetCall) (response)
/// * [keys migrate projects](ProjectKeyMigrateCall) (response)
/// * [keys patch projects](ProjectKeyPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1Key {
    /// Settings for keys that can be used by Android apps.
    #[serde(rename="androidSettings")]
    
    pub android_settings: Option<GoogleCloudRecaptchaenterpriseV1AndroidKeySettings>,
    /// Output only. The timestamp corresponding to the creation of this key.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Human-readable display name of this key. Modifiable by user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Settings for keys that can be used by iOS apps.
    #[serde(rename="iosSettings")]
    
    pub ios_settings: Option<GoogleCloudRecaptchaenterpriseV1IOSKeySettings>,
    /// Optional. See [Creating and managing labels] (https://cloud.google.com/recaptcha-enterprise/docs/labels).
    
    pub labels: Option<HashMap<String, String>>,
    /// Identifier. The resource name for the Key in the format `projects/{project}/keys/{key}`.
    
    pub name: Option<String>,
    /// Optional. Options for user acceptance testing.
    #[serde(rename="testingOptions")]
    
    pub testing_options: Option<GoogleCloudRecaptchaenterpriseV1TestingOptions>,
    /// Optional. Settings for WAF
    #[serde(rename="wafSettings")]
    
    pub waf_settings: Option<GoogleCloudRecaptchaenterpriseV1WafSettings>,
    /// Settings for keys that can be used by websites.
    #[serde(rename="webSettings")]
    
    pub web_settings: Option<GoogleCloudRecaptchaenterpriseV1WebKeySettings>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1Key {}
impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1Key {}


/// Response to request to list firewall policies belonging to a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firewallpolicies list projects](ProjectFirewallpolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ListFirewallPoliciesResponse {
    /// Policy details.
    #[serde(rename="firewallPolicies")]
    
    pub firewall_policies: Option<Vec<GoogleCloudRecaptchaenterpriseV1FirewallPolicy>>,
    /// Token to retrieve the next page of results. It is set to empty if no policies remain in results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1ListFirewallPoliciesResponse {}


/// Response to request to list keys in a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys list projects](ProjectKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ListKeysResponse {
    /// Key details.
    
    pub keys: Option<Vec<GoogleCloudRecaptchaenterpriseV1Key>>,
    /// Token to retrieve the next page of results. It is set to empty if no keys remain in results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1ListKeysResponse {}


/// The response to a `ListRelatedAccountGroupMemberships` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [relatedaccountgroups memberships list projects](ProjectRelatedaccountgroupMembershipListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The memberships listed by the query.
    #[serde(rename="relatedAccountGroupMemberships")]
    
    pub related_account_group_memberships: Option<Vec<GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership>>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse {}


/// The response to a `ListRelatedAccountGroups` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [relatedaccountgroups list projects](ProjectRelatedaccountgroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The groups of related accounts listed by the query.
    #[serde(rename="relatedAccountGroups")]
    
    pub related_account_groups: Option<Vec<GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup>>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse {}


/// Metrics for a single Key.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys get metrics projects](ProjectKeyGetMetricCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1Metrics {
    /// Metrics will be continuous and in order by dates, and in the granularity of day. Only challenge-based keys (CHECKBOX, INVISIBLE), will have challenge-based data.
    #[serde(rename="challengeMetrics")]
    
    pub challenge_metrics: Option<Vec<GoogleCloudRecaptchaenterpriseV1ChallengeMetrics>>,
    /// Output only. Identifier. The name of the metrics, in the format `projects/{project}/keys/{key}/metrics`.
    
    pub name: Option<String>,
    /// Metrics will be continuous and in order by dates, and in the granularity of day. All Key types should have score-based data.
    #[serde(rename="scoreMetrics")]
    
    pub score_metrics: Option<Vec<GoogleCloudRecaptchaenterpriseV1ScoreMetrics>>,
    /// Inclusive start time aligned to a day (UTC).
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1Metrics {}


/// The migrate key request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys migrate projects](ProjectKeyMigrateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest {
    /// Optional. If true, skips the billing check. A reCAPTCHA Enterprise key or migrated key behaves differently than a reCAPTCHA (non-Enterprise version) key when you reach a quota limit (see https://cloud.google.com/recaptcha-enterprise/quotas#quota_limit). To avoid any disruption of your usage, we check that a billing account is present. If your usage of reCAPTCHA is under the free quota, you can safely skip the billing check and proceed with the migration. See https://cloud.google.com/recaptcha-enterprise/docs/billing-information.
    #[serde(rename="skipBillingCheck")]
    
    pub skip_billing_check: Option<bool>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest {}


/// Private password leak verification info.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1PrivatePasswordLeakVerification {
    /// Output only. List of prefixes of the encrypted potential password leaks that matched the given parameters. They must be compared with the client-side decryption prefix of `reencrypted_user_credentials_hash`
    #[serde(rename="encryptedLeakMatchPrefixes")]
    
    #[serde_as(as = "Option<Vec<::client::serde::standard_base64::Wrapper>>")]
    pub encrypted_leak_match_prefixes: Option<Vec<Vec<u8>>>,
    /// Optional. Encrypted Scrypt hash of the canonicalized username+password. It is re-encrypted by the server and returned through `reencrypted_user_credentials_hash`.
    #[serde(rename="encryptedUserCredentialsHash")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub encrypted_user_credentials_hash: Option<Vec<u8>>,
    /// Required. Exactly 26-bit prefix of the SHA-256 hash of the canonicalized username. It is used to look up password leaks associated with that hash prefix.
    #[serde(rename="lookupHashPrefix")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub lookup_hash_prefix: Option<Vec<u8>>,
    /// Output only. Corresponds to the re-encryption of the `encrypted_user_credentials_hash` field. It is used to match potential password leaks within `encrypted_leak_match_prefixes`.
    #[serde(rename="reencryptedUserCredentialsHash")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub reencrypted_user_credentials_hash: Option<Vec<u8>>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1PrivatePasswordLeakVerification {}


/// A group of related accounts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup {
    /// Required. Identifier. The resource name for the related account group in the format `projects/{project}/relatedaccountgroups/{related_account_group}`.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup {}


/// A membership in a group of related accounts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership {
    /// The unique stable account identifier of the member. The identifier corresponds to an `account_id` provided in a previous `CreateAssessment` or `AnnotateAssessment` call.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Deprecated: use `account_id` instead. The unique stable hashed account identifier of the member. The identifier corresponds to a `hashed_account_id` provided in a previous `CreateAssessment` or `AnnotateAssessment` call.
    #[serde(rename="hashedAccountId")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub hashed_account_id: Option<Vec<u8>>,
    /// Required. Identifier. The resource name for this membership in the format `projects/{project}/relatedaccountgroups/{relatedaccountgroup}/memberships/{membership}`.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership {}


/// The reorder firewall policies request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firewallpolicies reorder projects](ProjectFirewallpolicyReorderCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ReorderFirewallPoliciesRequest {
    /// Required. A list containing all policy names, in the new order. Each name is in the format `projects/{project}/firewallpolicies/{firewallpolicy}`.
    
    pub names: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1ReorderFirewallPoliciesRequest {}


/// The reorder firewall policies response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firewallpolicies reorder projects](ProjectFirewallpolicyReorderCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ReorderFirewallPoliciesResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1ReorderFirewallPoliciesResponse {}


/// Secret key is used only in legacy reCAPTCHA. It must be used in a 3rd party integration with legacy reCAPTCHA.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [keys retrieve legacy secret key projects](ProjectKeyRetrieveLegacySecretKeyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1RetrieveLegacySecretKeyResponse {
    /// The secret key (also known as shared secret) authorizes communication between your application backend and the reCAPTCHA Enterprise server to create an assessment. The secret key needs to be kept safe for security purposes.
    #[serde(rename="legacySecretKey")]
    
    pub legacy_secret_key: Option<String>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1RetrieveLegacySecretKeyResponse {}


/// Risk analysis result for an event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1RiskAnalysis {
    /// Output only. Extended verdict reasons to be used for experimentation only. The set of possible reasons is subject to change.
    #[serde(rename="extendedVerdictReasons")]
    
    pub extended_verdict_reasons: Option<Vec<String>>,
    /// Output only. Reasons contributing to the risk analysis verdict.
    
    pub reasons: Option<Vec<GoogleCloudRecaptchaenterpriseV1RiskAnalysiReasonsEnum>>,
    /// Output only. Legitimate event score from 0.0 to 1.0. (1.0 means very likely legitimate traffic while 0.0 means very likely non-legitimate traffic).
    
    pub score: Option<f32>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1RiskAnalysis {}


/// Score distribution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ScoreDistribution {
    /// Map key is score value multiplied by 100. The scores are discrete values between [0, 1]. The maximum number of buckets is on order of a few dozen, but typically much lower (ie. 10).
    #[serde(rename="scoreBuckets")]
    
    #[serde_as(as = "Option<HashMap<_, ::client::serde_with::DisplayFromStr>>")]
    pub score_buckets: Option<HashMap<String, i64>>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1ScoreDistribution {}


/// Metrics related to scoring.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1ScoreMetrics {
    /// Action-based metrics. The map key is the action name which specified by the site owners at time of the "execute" client-side call.
    #[serde(rename="actionMetrics")]
    
    pub action_metrics: Option<HashMap<String, GoogleCloudRecaptchaenterpriseV1ScoreDistribution>>,
    /// Aggregated score metrics for all traffic.
    #[serde(rename="overallMetrics")]
    
    pub overall_metrics: Option<GoogleCloudRecaptchaenterpriseV1ScoreDistribution>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1ScoreMetrics {}


/// The request message to search related account group memberships.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [relatedaccountgroupmemberships search projects](ProjectRelatedaccountgroupmembershipSearchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest {
    /// Optional. The unique stable account identifier used to search connections. The identifier should correspond to an `account_id` provided in a previous `CreateAssessment` or `AnnotateAssessment` call. Either hashed_account_id or account_id must be set, but not both.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Optional. Deprecated: use `account_id` instead. The unique stable hashed account identifier used to search connections. The identifier should correspond to a `hashed_account_id` provided in a previous `CreateAssessment` or `AnnotateAssessment` call. Either hashed_account_id or account_id must be set, but not both.
    #[serde(rename="hashedAccountId")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub hashed_account_id: Option<Vec<u8>>,
    /// Optional. The maximum number of groups to return. The service might return fewer than this value. If unspecified, at most 50 groups are returned. The maximum value is 1000; values above 1000 are coerced to 1000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. A page token, received from a previous `SearchRelatedAccountGroupMemberships` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `SearchRelatedAccountGroupMemberships` must match the call that provided the page token.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest {}


/// The response to a `SearchRelatedAccountGroupMemberships` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [relatedaccountgroupmemberships search projects](ProjectRelatedaccountgroupmembershipSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The queried memberships.
    #[serde(rename="relatedAccountGroupMemberships")]
    
    pub related_account_group_memberships: Option<Vec<GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership>>,
}

impl client::ResponseResult for GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse {}


/// Options for user acceptance testing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1TestingOptions {
    /// Optional. For challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests for this site will return nocaptcha if NOCAPTCHA, or an unsolvable challenge if CHALLENGE.
    #[serde(rename="testingChallenge")]
    
    pub testing_challenge: Option<GoogleCloudRecaptchaenterpriseV1TestingOptionTestingChallengeEnum>,
    /// Optional. All assessments for this Key will return this score. Must be between 0 (likely not legitimate) and 1 (likely legitimate) inclusive.
    #[serde(rename="testingScore")]
    
    pub testing_score: Option<f32>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1TestingOptions {}


/// Properties of the provided event token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1TokenProperties {
    /// Output only. Action name provided at token generation.
    
    pub action: Option<String>,
    /// Output only. The name of the Android package with which the token was generated (Android keys only).
    #[serde(rename="androidPackageName")]
    
    pub android_package_name: Option<String>,
    /// Output only. The timestamp corresponding to the generation of the token.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The hostname of the page on which the token was generated (Web keys only).
    
    pub hostname: Option<String>,
    /// Output only. Reason associated with the response when valid = false.
    #[serde(rename="invalidReason")]
    
    pub invalid_reason: Option<GoogleCloudRecaptchaenterpriseV1TokenPropertyInvalidReasonEnum>,
    /// Output only. The ID of the iOS bundle with which the token was generated (iOS keys only).
    #[serde(rename="iosBundleId")]
    
    pub ios_bundle_id: Option<String>,
    /// Output only. Whether the provided user response token is valid. When valid = false, the reason could be specified in invalid_reason or it could also be due to a user failing to solve a challenge or a sitekey mismatch (i.e the sitekey used to generate the token was different than the one specified in the assessment).
    
    pub valid: Option<bool>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1TokenProperties {}


/// Transaction data associated with a payment protected by reCAPTCHA Enterprise.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1TransactionData {
    /// Optional. Address associated with the payment method when applicable.
    #[serde(rename="billingAddress")]
    
    pub billing_address: Option<GoogleCloudRecaptchaenterpriseV1TransactionDataAddress>,
    /// Optional. The Bank Identification Number - generally the first 6 or 8 digits of the card.
    #[serde(rename="cardBin")]
    
    pub card_bin: Option<String>,
    /// Optional. The last four digits of the card.
    #[serde(rename="cardLastFour")]
    
    pub card_last_four: Option<String>,
    /// Optional. The currency code in ISO-4217 format.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Optional. Information about the payment gateway's response to the transaction.
    #[serde(rename="gatewayInfo")]
    
    pub gateway_info: Option<GoogleCloudRecaptchaenterpriseV1TransactionDataGatewayInfo>,
    /// Optional. Items purchased in this transaction.
    
    pub items: Option<Vec<GoogleCloudRecaptchaenterpriseV1TransactionDataItem>>,
    /// Optional. Information about the user or users fulfilling the transaction.
    
    pub merchants: Option<Vec<GoogleCloudRecaptchaenterpriseV1TransactionDataUser>>,
    /// Optional. The payment method for the transaction. The allowed values are: * credit-card * debit-card * gift-card * processor-{name} (If a third-party is used, for example, processor-paypal) * custom-{name} (If an alternative method is used, for example, custom-crypto)
    #[serde(rename="paymentMethod")]
    
    pub payment_method: Option<String>,
    /// Optional. Destination address if this transaction involves shipping a physical item.
    #[serde(rename="shippingAddress")]
    
    pub shipping_address: Option<GoogleCloudRecaptchaenterpriseV1TransactionDataAddress>,
    /// Optional. The value of shipping in the specified currency. 0 for free or no shipping.
    #[serde(rename="shippingValue")]
    
    pub shipping_value: Option<f64>,
    /// Unique identifier for the transaction. This custom identifier can be used to reference this transaction in the future, for example, labeling a refund or chargeback event. Two attempts at the same transaction should use the same transaction id.
    #[serde(rename="transactionId")]
    
    pub transaction_id: Option<String>,
    /// Optional. Information about the user paying/initiating the transaction.
    
    pub user: Option<GoogleCloudRecaptchaenterpriseV1TransactionDataUser>,
    /// Optional. The decimal value of the transaction in the specified currency.
    
    pub value: Option<f64>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1TransactionData {}


/// Structured address format for billing and shipping addresses.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1TransactionDataAddress {
    /// Optional. The first lines of the address. The first line generally contains the street name and number, and further lines may include information such as an apartment number.
    
    pub address: Option<Vec<String>>,
    /// Optional. The state, province, or otherwise administrative area of the address.
    #[serde(rename="administrativeArea")]
    
    pub administrative_area: Option<String>,
    /// Optional. The town/city of the address.
    
    pub locality: Option<String>,
    /// Optional. The postal or ZIP code of the address.
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Optional. The recipient name, potentially including information such as "care of".
    
    pub recipient: Option<String>,
    /// Optional. The CLDR country/region of the address.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1TransactionDataAddress {}


/// Details about the transaction from the gateway.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1TransactionDataGatewayInfo {
    /// Optional. AVS response code from the gateway (available only when reCAPTCHA Enterprise is called after authorization).
    #[serde(rename="avsResponseCode")]
    
    pub avs_response_code: Option<String>,
    /// Optional. CVV response code from the gateway (available only when reCAPTCHA Enterprise is called after authorization).
    #[serde(rename="cvvResponseCode")]
    
    pub cvv_response_code: Option<String>,
    /// Optional. Gateway response code describing the state of the transaction.
    #[serde(rename="gatewayResponseCode")]
    
    pub gateway_response_code: Option<String>,
    /// Optional. Name of the gateway service (for example, stripe, square, paypal).
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1TransactionDataGatewayInfo {}


/// Line items being purchased in this transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1TransactionDataItem {
    /// Optional. When a merchant is specified, its corresponding account_id. Necessary to populate marketplace-style transactions.
    #[serde(rename="merchantAccountId")]
    
    pub merchant_account_id: Option<String>,
    /// Optional. The full name of the item.
    
    pub name: Option<String>,
    /// Optional. The quantity of this item that is being purchased.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quantity: Option<i64>,
    /// Optional. The value per item that the user is paying, in the transaction currency, after discounts.
    
    pub value: Option<f64>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1TransactionDataItem {}


/// Details about a user's account involved in the transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1TransactionDataUser {
    /// Optional. Unique account identifier for this user. If using account defender, this should match the hashed_account_id field. Otherwise, a unique and persistent identifier for this account.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Optional. The epoch milliseconds of the user's account creation.
    #[serde(rename="creationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_ms: Option<i64>,
    /// Optional. The email address of the user.
    
    pub email: Option<String>,
    /// Optional. Whether the email has been verified to be accessible by the user (OTP or similar).
    #[serde(rename="emailVerified")]
    
    pub email_verified: Option<bool>,
    /// Optional. The phone number of the user, with country code.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Optional. Whether the phone number has been verified to be accessible by the user (OTP or similar).
    #[serde(rename="phoneVerified")]
    
    pub phone_verified: Option<bool>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1TransactionDataUser {}


/// Describes an event in the lifecycle of a payment transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1TransactionEvent {
    /// Optional. Timestamp when this transaction event occurred; otherwise assumed to be the time of the API call.
    #[serde(rename="eventTime")]
    
    pub event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The type of this transaction event.
    #[serde(rename="eventType")]
    
    pub event_type: Option<GoogleCloudRecaptchaenterpriseV1TransactionEventEventTypeEnum>,
    /// Optional. The reason or standardized code that corresponds with this transaction event, if one exists. For example, a CHARGEBACK event with code 6005.
    
    pub reason: Option<String>,
    /// Optional. The value that corresponds with this transaction event, if one exists. For example, a refund event where $5.00 was refunded. Currency is obtained from the original transaction data.
    
    pub value: Option<f64>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1TransactionEvent {}


/// An identifier associated with a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1UserId {
    /// Optional. An email address.
    
    pub email: Option<String>,
    /// Optional. A phone number. Should use the E.164 format.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Optional. A unique username, if different from all the other identifiers and `account_id` that are provided. Can be a unique login handle or display name for a user.
    
    pub username: Option<String>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1UserId {}


/// User information associated with a request protected by reCAPTCHA Enterprise.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1UserInfo {
    /// Optional. For logged-in requests or login/registration requests, the unique account identifier associated with this user. You can use the username if it is stable (meaning it is the same for every request associated with the same user), or any stable user ID of your choice. Leave blank for non logged-in actions or guest checkout.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Optional. Creation time for this account associated with this user. Leave blank for non logged-in actions, guest checkout, or when there is no account associated with the current user.
    #[serde(rename="createAccountTime")]
    
    pub create_account_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Identifiers associated with this user or request.
    #[serde(rename="userIds")]
    
    pub user_ids: Option<Vec<GoogleCloudRecaptchaenterpriseV1UserId>>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1UserInfo {}


/// Settings specific to keys that can be used for WAF (Web Application Firewall).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1WafSettings {
    /// Required. The WAF feature for which this key is enabled.
    #[serde(rename="wafFeature")]
    
    pub waf_feature: Option<GoogleCloudRecaptchaenterpriseV1WafSettingWafFeatureEnum>,
    /// Required. The WAF service that uses this key.
    #[serde(rename="wafService")]
    
    pub waf_service: Option<GoogleCloudRecaptchaenterpriseV1WafSettingWafServiceEnum>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1WafSettings {}


/// Settings specific to keys that can be used by websites.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecaptchaenterpriseV1WebKeySettings {
    /// Optional. If set to true, it means allowed_domains will not be enforced.
    #[serde(rename="allowAllDomains")]
    
    pub allow_all_domains: Option<bool>,
    /// Optional. If set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type.
    #[serde(rename="allowAmpTraffic")]
    
    pub allow_amp_traffic: Option<bool>,
    /// Optional. Domains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: 'example.com' or 'subdomain.example.com'
    #[serde(rename="allowedDomains")]
    
    pub allowed_domains: Option<Vec<String>>,
    /// Optional. Settings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE.
    #[serde(rename="challengeSecurityPreference")]
    
    pub challenge_security_preference: Option<GoogleCloudRecaptchaenterpriseV1WebKeySettingChallengeSecurityPreferenceEnum>,
    /// Required. Describes how this key is integrated with the website.
    #[serde(rename="integrationType")]
    
    pub integration_type: Option<GoogleCloudRecaptchaenterpriseV1WebKeySettingIntegrationTypeEnum>,
}

impl client::Part for GoogleCloudRecaptchaenterpriseV1WebKeySettings {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [firewallpolicies delete projects](ProjectFirewallpolicyDeleteCall) (response)
/// * [keys delete projects](ProjectKeyDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


