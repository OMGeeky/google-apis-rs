use super::*;



// region AndroidInstrumentationTestOrchestratorOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The option of whether running each test within its own invocation of instrumentation with Android Test Orchestrator or not. ** Orchestrator is only compatible with AndroidJUnitRunner version 1.1 or higher! ** Orchestrator offers the following benefits: - No shared state - Crashes are isolated - Logs are scoped per test See for more information about Android Test Orchestrator. If not set, the test will be run without the orchestrator.
pub enum AndroidInstrumentationTestOrchestratorOptionEnum {
    

    /// Default value: the server will choose the mode. Currently implies that the test will run without the orchestrator. In the future, all instrumentation tests will be run with the orchestrator. Using the orchestrator is highly encouraged because of all the benefits it offers.
    ///
    /// "ORCHESTRATOR_OPTION_UNSPECIFIED"
    #[serde(rename="ORCHESTRATOR_OPTION_UNSPECIFIED")]
    ORCHESTRATOROPTIONUNSPECIFIED,
    

    /// Run test using orchestrator. ** Only compatible with AndroidJUnitRunner version 1.1 or higher! ** Recommended.
    ///
    /// "USE_ORCHESTRATOR"
    #[serde(rename="USE_ORCHESTRATOR")]
    USEORCHESTRATOR,
    

    /// Run test without using orchestrator.
    ///
    /// "DO_NOT_USE_ORCHESTRATOR"
    #[serde(rename="DO_NOT_USE_ORCHESTRATOR")]
    DONOTUSEORCHESTRATOR,
}

impl AsRef<str> for AndroidInstrumentationTestOrchestratorOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AndroidInstrumentationTestOrchestratorOptionEnum::ORCHESTRATOROPTIONUNSPECIFIED => "ORCHESTRATOR_OPTION_UNSPECIFIED",
            AndroidInstrumentationTestOrchestratorOptionEnum::USEORCHESTRATOR => "USE_ORCHESTRATOR",
            AndroidInstrumentationTestOrchestratorOptionEnum::DONOTUSEORCHESTRATOR => "DO_NOT_USE_ORCHESTRATOR",
        }
    }
}

impl std::convert::TryFrom< &str> for AndroidInstrumentationTestOrchestratorOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORCHESTRATOR_OPTION_UNSPECIFIED" => Ok(AndroidInstrumentationTestOrchestratorOptionEnum::ORCHESTRATOROPTIONUNSPECIFIED),
           "USE_ORCHESTRATOR" => Ok(AndroidInstrumentationTestOrchestratorOptionEnum::USEORCHESTRATOR),
           "DO_NOT_USE_ORCHESTRATOR" => Ok(AndroidInstrumentationTestOrchestratorOptionEnum::DONOTUSEORCHESTRATOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AndroidInstrumentationTestOrchestratorOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AndroidModelFormEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether this device is virtual or physical.
pub enum AndroidModelFormEnum {
    

    /// Do not use. For proto versioning only.
    ///
    /// "DEVICE_FORM_UNSPECIFIED"
    #[serde(rename="DEVICE_FORM_UNSPECIFIED")]
    DEVICEFORMUNSPECIFIED,
    

    /// Android virtual device using Compute Engine native virtualization. Firebase Test Lab only.
    ///
    /// "VIRTUAL"
    #[serde(rename="VIRTUAL")]
    VIRTUAL,
    

    /// Actual hardware.
    ///
    /// "PHYSICAL"
    #[serde(rename="PHYSICAL")]
    PHYSICAL,
    

    /// Android virtual device using emulator in nested virtualization. Equivalent to Android Studio.
    ///
    /// "EMULATOR"
    #[serde(rename="EMULATOR")]
    EMULATOR,
}

impl AsRef<str> for AndroidModelFormEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AndroidModelFormEnum::DEVICEFORMUNSPECIFIED => "DEVICE_FORM_UNSPECIFIED",
            AndroidModelFormEnum::VIRTUAL => "VIRTUAL",
            AndroidModelFormEnum::PHYSICAL => "PHYSICAL",
            AndroidModelFormEnum::EMULATOR => "EMULATOR",
        }
    }
}

impl std::convert::TryFrom< &str> for AndroidModelFormEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_FORM_UNSPECIFIED" => Ok(AndroidModelFormEnum::DEVICEFORMUNSPECIFIED),
           "VIRTUAL" => Ok(AndroidModelFormEnum::VIRTUAL),
           "PHYSICAL" => Ok(AndroidModelFormEnum::PHYSICAL),
           "EMULATOR" => Ok(AndroidModelFormEnum::EMULATOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AndroidModelFormEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AndroidModelFormFactorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether this device is a phone, tablet, wearable, etc.
pub enum AndroidModelFormFactorEnum {
    

    /// Do not use. For proto versioning only.
    ///
    /// "DEVICE_FORM_FACTOR_UNSPECIFIED"
    #[serde(rename="DEVICE_FORM_FACTOR_UNSPECIFIED")]
    DEVICEFORMFACTORUNSPECIFIED,
    

    /// This device has the shape of a phone.
    ///
    /// "PHONE"
    #[serde(rename="PHONE")]
    PHONE,
    

    /// This device has the shape of a tablet.
    ///
    /// "TABLET"
    #[serde(rename="TABLET")]
    TABLET,
    

    /// This device has the shape of a watch or other wearable.
    ///
    /// "WEARABLE"
    #[serde(rename="WEARABLE")]
    WEARABLE,
}

impl AsRef<str> for AndroidModelFormFactorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AndroidModelFormFactorEnum::DEVICEFORMFACTORUNSPECIFIED => "DEVICE_FORM_FACTOR_UNSPECIFIED",
            AndroidModelFormFactorEnum::PHONE => "PHONE",
            AndroidModelFormFactorEnum::TABLET => "TABLET",
            AndroidModelFormFactorEnum::WEARABLE => "WEARABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for AndroidModelFormFactorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_FORM_FACTOR_UNSPECIFIED" => Ok(AndroidModelFormFactorEnum::DEVICEFORMFACTORUNSPECIFIED),
           "PHONE" => Ok(AndroidModelFormFactorEnum::PHONE),
           "TABLET" => Ok(AndroidModelFormFactorEnum::TABLET),
           "WEARABLE" => Ok(AndroidModelFormFactorEnum::WEARABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AndroidModelFormFactorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AndroidRoboTestRoboModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The mode in which Robo should run. Most clients should allow the server to populate this field automatically.
pub enum AndroidRoboTestRoboModeEnum {
    

    /// This means that the server should choose the mode. Recommended.
    ///
    /// "ROBO_MODE_UNSPECIFIED"
    #[serde(rename="ROBO_MODE_UNSPECIFIED")]
    ROBOMODEUNSPECIFIED,
    

    /// Runs Robo in UIAutomator-only mode without app resigning
    ///
    /// "ROBO_VERSION_1"
    #[serde(rename="ROBO_VERSION_1")]
    ROBOVERSION1,
    

    /// Runs Robo in standard Espresso with UIAutomator fallback
    ///
    /// "ROBO_VERSION_2"
    #[serde(rename="ROBO_VERSION_2")]
    ROBOVERSION2,
}

impl AsRef<str> for AndroidRoboTestRoboModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AndroidRoboTestRoboModeEnum::ROBOMODEUNSPECIFIED => "ROBO_MODE_UNSPECIFIED",
            AndroidRoboTestRoboModeEnum::ROBOVERSION1 => "ROBO_VERSION_1",
            AndroidRoboTestRoboModeEnum::ROBOVERSION2 => "ROBO_VERSION_2",
        }
    }
}

impl std::convert::TryFrom< &str> for AndroidRoboTestRoboModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROBO_MODE_UNSPECIFIED" => Ok(AndroidRoboTestRoboModeEnum::ROBOMODEUNSPECIFIED),
           "ROBO_VERSION_1" => Ok(AndroidRoboTestRoboModeEnum::ROBOVERSION1),
           "ROBO_VERSION_2" => Ok(AndroidRoboTestRoboModeEnum::ROBOVERSION2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AndroidRoboTestRoboModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CancelTestMatrixResponseTestStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current rolled-up state of the test matrix. If this state is already final, then the cancelation request will have no effect.
pub enum CancelTestMatrixResponseTestStateEnum {
    

    /// Do not use. For proto versioning only.
    ///
    /// "TEST_STATE_UNSPECIFIED"
    #[serde(rename="TEST_STATE_UNSPECIFIED")]
    TESTSTATEUNSPECIFIED,
    

    /// The execution or matrix is being validated.
    ///
    /// "VALIDATING"
    #[serde(rename="VALIDATING")]
    VALIDATING,
    

    /// The execution or matrix is waiting for resources to become available.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The execution is currently being processed. Can only be set on an execution.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The execution or matrix has terminated normally. On a matrix this means that the matrix level processing completed normally, but individual executions may be in an ERROR state.
    ///
    /// "FINISHED"
    #[serde(rename="FINISHED")]
    FINISHED,
    

    /// The execution or matrix has stopped because it encountered an infrastructure failure.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// The execution was not run because it corresponds to a unsupported environment. Can only be set on an execution.
    ///
    /// "UNSUPPORTED_ENVIRONMENT"
    #[serde(rename="UNSUPPORTED_ENVIRONMENT")]
    UNSUPPORTEDENVIRONMENT,
    

    /// The execution was not run because the provided inputs are incompatible with the requested environment. Example: requested AndroidVersion is lower than APK's minSdkVersion Can only be set on an execution.
    ///
    /// "INCOMPATIBLE_ENVIRONMENT"
    #[serde(rename="INCOMPATIBLE_ENVIRONMENT")]
    INCOMPATIBLEENVIRONMENT,
    

    /// The execution was not run because the provided inputs are incompatible with the requested architecture. Example: requested device does not support running the native code in the supplied APK Can only be set on an execution.
    ///
    /// "INCOMPATIBLE_ARCHITECTURE"
    #[serde(rename="INCOMPATIBLE_ARCHITECTURE")]
    INCOMPATIBLEARCHITECTURE,
    

    /// The user cancelled the execution. Can only be set on an execution.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The execution or matrix was not run because the provided inputs are not valid. Examples: input file is not of the expected type, is malformed/corrupt, or was flagged as malware
    ///
    /// "INVALID"
    #[serde(rename="INVALID")]
    INVALID,
}

impl AsRef<str> for CancelTestMatrixResponseTestStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CancelTestMatrixResponseTestStateEnum::TESTSTATEUNSPECIFIED => "TEST_STATE_UNSPECIFIED",
            CancelTestMatrixResponseTestStateEnum::VALIDATING => "VALIDATING",
            CancelTestMatrixResponseTestStateEnum::PENDING => "PENDING",
            CancelTestMatrixResponseTestStateEnum::RUNNING => "RUNNING",
            CancelTestMatrixResponseTestStateEnum::FINISHED => "FINISHED",
            CancelTestMatrixResponseTestStateEnum::ERROR => "ERROR",
            CancelTestMatrixResponseTestStateEnum::UNSUPPORTEDENVIRONMENT => "UNSUPPORTED_ENVIRONMENT",
            CancelTestMatrixResponseTestStateEnum::INCOMPATIBLEENVIRONMENT => "INCOMPATIBLE_ENVIRONMENT",
            CancelTestMatrixResponseTestStateEnum::INCOMPATIBLEARCHITECTURE => "INCOMPATIBLE_ARCHITECTURE",
            CancelTestMatrixResponseTestStateEnum::CANCELLED => "CANCELLED",
            CancelTestMatrixResponseTestStateEnum::INVALID => "INVALID",
        }
    }
}

impl std::convert::TryFrom< &str> for CancelTestMatrixResponseTestStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEST_STATE_UNSPECIFIED" => Ok(CancelTestMatrixResponseTestStateEnum::TESTSTATEUNSPECIFIED),
           "VALIDATING" => Ok(CancelTestMatrixResponseTestStateEnum::VALIDATING),
           "PENDING" => Ok(CancelTestMatrixResponseTestStateEnum::PENDING),
           "RUNNING" => Ok(CancelTestMatrixResponseTestStateEnum::RUNNING),
           "FINISHED" => Ok(CancelTestMatrixResponseTestStateEnum::FINISHED),
           "ERROR" => Ok(CancelTestMatrixResponseTestStateEnum::ERROR),
           "UNSUPPORTED_ENVIRONMENT" => Ok(CancelTestMatrixResponseTestStateEnum::UNSUPPORTEDENVIRONMENT),
           "INCOMPATIBLE_ENVIRONMENT" => Ok(CancelTestMatrixResponseTestStateEnum::INCOMPATIBLEENVIRONMENT),
           "INCOMPATIBLE_ARCHITECTURE" => Ok(CancelTestMatrixResponseTestStateEnum::INCOMPATIBLEARCHITECTURE),
           "CANCELLED" => Ok(CancelTestMatrixResponseTestStateEnum::CANCELLED),
           "INVALID" => Ok(CancelTestMatrixResponseTestStateEnum::INVALID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CancelTestMatrixResponseTestStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceIpBlockFormEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether this block is used by physical or virtual devices
pub enum DeviceIpBlockFormEnum {
    

    /// Do not use. For proto versioning only.
    ///
    /// "DEVICE_FORM_UNSPECIFIED"
    #[serde(rename="DEVICE_FORM_UNSPECIFIED")]
    DEVICEFORMUNSPECIFIED,
    

    /// Android virtual device using Compute Engine native virtualization. Firebase Test Lab only.
    ///
    /// "VIRTUAL"
    #[serde(rename="VIRTUAL")]
    VIRTUAL,
    

    /// Actual hardware.
    ///
    /// "PHYSICAL"
    #[serde(rename="PHYSICAL")]
    PHYSICAL,
    

    /// Android virtual device using emulator in nested virtualization. Equivalent to Android Studio.
    ///
    /// "EMULATOR"
    #[serde(rename="EMULATOR")]
    EMULATOR,
}

impl AsRef<str> for DeviceIpBlockFormEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceIpBlockFormEnum::DEVICEFORMUNSPECIFIED => "DEVICE_FORM_UNSPECIFIED",
            DeviceIpBlockFormEnum::VIRTUAL => "VIRTUAL",
            DeviceIpBlockFormEnum::PHYSICAL => "PHYSICAL",
            DeviceIpBlockFormEnum::EMULATOR => "EMULATOR",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceIpBlockFormEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_FORM_UNSPECIFIED" => Ok(DeviceIpBlockFormEnum::DEVICEFORMUNSPECIFIED),
           "VIRTUAL" => Ok(DeviceIpBlockFormEnum::VIRTUAL),
           "PHYSICAL" => Ok(DeviceIpBlockFormEnum::PHYSICAL),
           "EMULATOR" => Ok(DeviceIpBlockFormEnum::EMULATOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceIpBlockFormEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceSessionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the DeviceSession.
pub enum DeviceSessionStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "SESSION_STATE_UNSPECIFIED"
    #[serde(rename="SESSION_STATE_UNSPECIFIED")]
    SESSIONSTATEUNSPECIFIED,
    

    /// Initial state of a session request. The session is being validated for correctness and a device is not yet requested.
    ///
    /// "REQUESTED"
    #[serde(rename="REQUESTED")]
    REQUESTED,
    

    /// The session has been validated and is in the queue for a device.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The session has been granted and the device is accepting connections.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The session duration exceeded the device’s reservation time period and timed out automatically.
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// The user is finished with the session and it was canceled by the user while the request was still getting allocated or after allocation and during device usage period.
    ///
    /// "FINISHED"
    #[serde(rename="FINISHED")]
    FINISHED,
    

    /// Unable to complete the session because the device was unavailable and it failed to allocate through the scheduler. For example, a device not in the catalog was requested or the request expired in the allocation queue.
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
    

    /// Unable to complete the session for an internal reason, such as an infrastructure failure.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for DeviceSessionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceSessionStateEnum::SESSIONSTATEUNSPECIFIED => "SESSION_STATE_UNSPECIFIED",
            DeviceSessionStateEnum::REQUESTED => "REQUESTED",
            DeviceSessionStateEnum::PENDING => "PENDING",
            DeviceSessionStateEnum::ACTIVE => "ACTIVE",
            DeviceSessionStateEnum::EXPIRED => "EXPIRED",
            DeviceSessionStateEnum::FINISHED => "FINISHED",
            DeviceSessionStateEnum::UNAVAILABLE => "UNAVAILABLE",
            DeviceSessionStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceSessionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SESSION_STATE_UNSPECIFIED" => Ok(DeviceSessionStateEnum::SESSIONSTATEUNSPECIFIED),
           "REQUESTED" => Ok(DeviceSessionStateEnum::REQUESTED),
           "PENDING" => Ok(DeviceSessionStateEnum::PENDING),
           "ACTIVE" => Ok(DeviceSessionStateEnum::ACTIVE),
           "EXPIRED" => Ok(DeviceSessionStateEnum::EXPIRED),
           "FINISHED" => Ok(DeviceSessionStateEnum::FINISHED),
           "UNAVAILABLE" => Ok(DeviceSessionStateEnum::UNAVAILABLE),
           "ERROR" => Ok(DeviceSessionStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceSessionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IosModelFormFactorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether this device is a phone, tablet, wearable, etc.
pub enum IosModelFormFactorEnum {
    

    /// Do not use. For proto versioning only.
    ///
    /// "DEVICE_FORM_FACTOR_UNSPECIFIED"
    #[serde(rename="DEVICE_FORM_FACTOR_UNSPECIFIED")]
    DEVICEFORMFACTORUNSPECIFIED,
    

    /// This device has the shape of a phone.
    ///
    /// "PHONE"
    #[serde(rename="PHONE")]
    PHONE,
    

    /// This device has the shape of a tablet.
    ///
    /// "TABLET"
    #[serde(rename="TABLET")]
    TABLET,
    

    /// This device has the shape of a watch or other wearable.
    ///
    /// "WEARABLE"
    #[serde(rename="WEARABLE")]
    WEARABLE,
}

impl AsRef<str> for IosModelFormFactorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IosModelFormFactorEnum::DEVICEFORMFACTORUNSPECIFIED => "DEVICE_FORM_FACTOR_UNSPECIFIED",
            IosModelFormFactorEnum::PHONE => "PHONE",
            IosModelFormFactorEnum::TABLET => "TABLET",
            IosModelFormFactorEnum::WEARABLE => "WEARABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for IosModelFormFactorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_FORM_FACTOR_UNSPECIFIED" => Ok(IosModelFormFactorEnum::DEVICEFORMFACTORUNSPECIFIED),
           "PHONE" => Ok(IosModelFormFactorEnum::PHONE),
           "TABLET" => Ok(IosModelFormFactorEnum::TABLET),
           "WEARABLE" => Ok(IosModelFormFactorEnum::WEARABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IosModelFormFactorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PerAndroidVersionInfoDeviceCapacityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The number of online devices for an Android version.
pub enum PerAndroidVersionInfoDeviceCapacityEnum {
    

    /// The value of device capacity is unknown or unset.
    ///
    /// "DEVICE_CAPACITY_UNSPECIFIED"
    #[serde(rename="DEVICE_CAPACITY_UNSPECIFIED")]
    DEVICECAPACITYUNSPECIFIED,
    

    /// Devices that are high in capacity (The lab has a large number of these devices). These devices are generally suggested for running a large number of simultaneous tests (e.g. more than 100 tests). Please note that high capacity devices do not guarantee short wait times due to several factors: 1. Traffic (how heavily they are used at any given moment) 2. High capacity devices are prioritized for certain usages, which may cause user tests to be slower than selecting other similar device types.
    ///
    /// "DEVICE_CAPACITY_HIGH"
    #[serde(rename="DEVICE_CAPACITY_HIGH")]
    DEVICECAPACITYHIGH,
    

    /// Devices that are medium in capacity (The lab has a decent number of these devices, though not as many as high capacity devices). These devices are suitable for fewer test runs (e.g. fewer than 100 tests) and only for low shard counts (e.g. less than 10 shards).
    ///
    /// "DEVICE_CAPACITY_MEDIUM"
    #[serde(rename="DEVICE_CAPACITY_MEDIUM")]
    DEVICECAPACITYMEDIUM,
    

    /// Devices that are low in capacity (The lab has a small number of these devices). These devices may be used if users need to test on this specific device model and version. Please note that due to low capacity, the tests may take much longer to finish, especially if a large number of tests are invoked at once. These devices are not suitable for test sharding.
    ///
    /// "DEVICE_CAPACITY_LOW"
    #[serde(rename="DEVICE_CAPACITY_LOW")]
    DEVICECAPACITYLOW,
    

    /// Devices that are completely missing from the lab. These devices are unavailable either temporarily or permanently and should not be requested. If the device is also marked as deprecated, this state is very likely permanent.
    ///
    /// "DEVICE_CAPACITY_NONE"
    #[serde(rename="DEVICE_CAPACITY_NONE")]
    DEVICECAPACITYNONE,
}

impl AsRef<str> for PerAndroidVersionInfoDeviceCapacityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PerAndroidVersionInfoDeviceCapacityEnum::DEVICECAPACITYUNSPECIFIED => "DEVICE_CAPACITY_UNSPECIFIED",
            PerAndroidVersionInfoDeviceCapacityEnum::DEVICECAPACITYHIGH => "DEVICE_CAPACITY_HIGH",
            PerAndroidVersionInfoDeviceCapacityEnum::DEVICECAPACITYMEDIUM => "DEVICE_CAPACITY_MEDIUM",
            PerAndroidVersionInfoDeviceCapacityEnum::DEVICECAPACITYLOW => "DEVICE_CAPACITY_LOW",
            PerAndroidVersionInfoDeviceCapacityEnum::DEVICECAPACITYNONE => "DEVICE_CAPACITY_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for PerAndroidVersionInfoDeviceCapacityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_CAPACITY_UNSPECIFIED" => Ok(PerAndroidVersionInfoDeviceCapacityEnum::DEVICECAPACITYUNSPECIFIED),
           "DEVICE_CAPACITY_HIGH" => Ok(PerAndroidVersionInfoDeviceCapacityEnum::DEVICECAPACITYHIGH),
           "DEVICE_CAPACITY_MEDIUM" => Ok(PerAndroidVersionInfoDeviceCapacityEnum::DEVICECAPACITYMEDIUM),
           "DEVICE_CAPACITY_LOW" => Ok(PerAndroidVersionInfoDeviceCapacityEnum::DEVICECAPACITYLOW),
           "DEVICE_CAPACITY_NONE" => Ok(PerAndroidVersionInfoDeviceCapacityEnum::DEVICECAPACITYNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PerAndroidVersionInfoDeviceCapacityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PerIosVersionInfoDeviceCapacityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The number of online devices for an iOS version.
pub enum PerIosVersionInfoDeviceCapacityEnum {
    

    /// The value of device capacity is unknown or unset.
    ///
    /// "DEVICE_CAPACITY_UNSPECIFIED"
    #[serde(rename="DEVICE_CAPACITY_UNSPECIFIED")]
    DEVICECAPACITYUNSPECIFIED,
    

    /// Devices that are high in capacity (The lab has a large number of these devices). These devices are generally suggested for running a large number of simultaneous tests (e.g. more than 100 tests). Please note that high capacity devices do not guarantee short wait times due to several factors: 1. Traffic (how heavily they are used at any given moment) 2. High capacity devices are prioritized for certain usages, which may cause user tests to be slower than selecting other similar device types.
    ///
    /// "DEVICE_CAPACITY_HIGH"
    #[serde(rename="DEVICE_CAPACITY_HIGH")]
    DEVICECAPACITYHIGH,
    

    /// Devices that are medium in capacity (The lab has a decent number of these devices, though not as many as high capacity devices). These devices are suitable for fewer test runs (e.g. fewer than 100 tests) and only for low shard counts (e.g. less than 10 shards).
    ///
    /// "DEVICE_CAPACITY_MEDIUM"
    #[serde(rename="DEVICE_CAPACITY_MEDIUM")]
    DEVICECAPACITYMEDIUM,
    

    /// Devices that are low in capacity (The lab has a small number of these devices). These devices may be used if users need to test on this specific device model and version. Please note that due to low capacity, the tests may take much longer to finish, especially if a large number of tests are invoked at once. These devices are not suitable for test sharding.
    ///
    /// "DEVICE_CAPACITY_LOW"
    #[serde(rename="DEVICE_CAPACITY_LOW")]
    DEVICECAPACITYLOW,
    

    /// Devices that are completely missing from the lab. These devices are unavailable either temporarily or permanently and should not be requested. If the device is also marked as deprecated, this state is very likely permanent.
    ///
    /// "DEVICE_CAPACITY_NONE"
    #[serde(rename="DEVICE_CAPACITY_NONE")]
    DEVICECAPACITYNONE,
}

impl AsRef<str> for PerIosVersionInfoDeviceCapacityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PerIosVersionInfoDeviceCapacityEnum::DEVICECAPACITYUNSPECIFIED => "DEVICE_CAPACITY_UNSPECIFIED",
            PerIosVersionInfoDeviceCapacityEnum::DEVICECAPACITYHIGH => "DEVICE_CAPACITY_HIGH",
            PerIosVersionInfoDeviceCapacityEnum::DEVICECAPACITYMEDIUM => "DEVICE_CAPACITY_MEDIUM",
            PerIosVersionInfoDeviceCapacityEnum::DEVICECAPACITYLOW => "DEVICE_CAPACITY_LOW",
            PerIosVersionInfoDeviceCapacityEnum::DEVICECAPACITYNONE => "DEVICE_CAPACITY_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for PerIosVersionInfoDeviceCapacityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_CAPACITY_UNSPECIFIED" => Ok(PerIosVersionInfoDeviceCapacityEnum::DEVICECAPACITYUNSPECIFIED),
           "DEVICE_CAPACITY_HIGH" => Ok(PerIosVersionInfoDeviceCapacityEnum::DEVICECAPACITYHIGH),
           "DEVICE_CAPACITY_MEDIUM" => Ok(PerIosVersionInfoDeviceCapacityEnum::DEVICECAPACITYMEDIUM),
           "DEVICE_CAPACITY_LOW" => Ok(PerIosVersionInfoDeviceCapacityEnum::DEVICECAPACITYLOW),
           "DEVICE_CAPACITY_NONE" => Ok(PerIosVersionInfoDeviceCapacityEnum::DEVICECAPACITYNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PerIosVersionInfoDeviceCapacityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RoboDirectiveActionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of action that Robo should perform on the specified element.
pub enum RoboDirectiveActionTypeEnum {
    

    /// DO NOT USE. For proto versioning only.
    ///
    /// "ACTION_TYPE_UNSPECIFIED"
    #[serde(rename="ACTION_TYPE_UNSPECIFIED")]
    ACTIONTYPEUNSPECIFIED,
    

    /// Direct Robo to click on the specified element. No-op if specified element is not clickable.
    ///
    /// "SINGLE_CLICK"
    #[serde(rename="SINGLE_CLICK")]
    SINGLECLICK,
    

    /// Direct Robo to enter text on the specified element. No-op if specified element is not enabled or does not allow text entry.
    ///
    /// "ENTER_TEXT"
    #[serde(rename="ENTER_TEXT")]
    ENTERTEXT,
    

    /// Direct Robo to ignore interactions with a specific element.
    ///
    /// "IGNORE"
    #[serde(rename="IGNORE")]
    IGNORE,
}

impl AsRef<str> for RoboDirectiveActionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RoboDirectiveActionTypeEnum::ACTIONTYPEUNSPECIFIED => "ACTION_TYPE_UNSPECIFIED",
            RoboDirectiveActionTypeEnum::SINGLECLICK => "SINGLE_CLICK",
            RoboDirectiveActionTypeEnum::ENTERTEXT => "ENTER_TEXT",
            RoboDirectiveActionTypeEnum::IGNORE => "IGNORE",
        }
    }
}

impl std::convert::TryFrom< &str> for RoboDirectiveActionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_TYPE_UNSPECIFIED" => Ok(RoboDirectiveActionTypeEnum::ACTIONTYPEUNSPECIFIED),
           "SINGLE_CLICK" => Ok(RoboDirectiveActionTypeEnum::SINGLECLICK),
           "ENTER_TEXT" => Ok(RoboDirectiveActionTypeEnum::ENTERTEXT),
           "IGNORE" => Ok(RoboDirectiveActionTypeEnum::IGNORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RoboDirectiveActionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SessionStateEventSessionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The session_state tracked by this event
pub enum SessionStateEventSessionStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "SESSION_STATE_UNSPECIFIED"
    #[serde(rename="SESSION_STATE_UNSPECIFIED")]
    SESSIONSTATEUNSPECIFIED,
    

    /// Initial state of a session request. The session is being validated for correctness and a device is not yet requested.
    ///
    /// "REQUESTED"
    #[serde(rename="REQUESTED")]
    REQUESTED,
    

    /// The session has been validated and is in the queue for a device.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The session has been granted and the device is accepting connections.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The session duration exceeded the device’s reservation time period and timed out automatically.
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// The user is finished with the session and it was canceled by the user while the request was still getting allocated or after allocation and during device usage period.
    ///
    /// "FINISHED"
    #[serde(rename="FINISHED")]
    FINISHED,
    

    /// Unable to complete the session because the device was unavailable and it failed to allocate through the scheduler. For example, a device not in the catalog was requested or the request expired in the allocation queue.
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
    

    /// Unable to complete the session for an internal reason, such as an infrastructure failure.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for SessionStateEventSessionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SessionStateEventSessionStateEnum::SESSIONSTATEUNSPECIFIED => "SESSION_STATE_UNSPECIFIED",
            SessionStateEventSessionStateEnum::REQUESTED => "REQUESTED",
            SessionStateEventSessionStateEnum::PENDING => "PENDING",
            SessionStateEventSessionStateEnum::ACTIVE => "ACTIVE",
            SessionStateEventSessionStateEnum::EXPIRED => "EXPIRED",
            SessionStateEventSessionStateEnum::FINISHED => "FINISHED",
            SessionStateEventSessionStateEnum::UNAVAILABLE => "UNAVAILABLE",
            SessionStateEventSessionStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for SessionStateEventSessionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SESSION_STATE_UNSPECIFIED" => Ok(SessionStateEventSessionStateEnum::SESSIONSTATEUNSPECIFIED),
           "REQUESTED" => Ok(SessionStateEventSessionStateEnum::REQUESTED),
           "PENDING" => Ok(SessionStateEventSessionStateEnum::PENDING),
           "ACTIVE" => Ok(SessionStateEventSessionStateEnum::ACTIVE),
           "EXPIRED" => Ok(SessionStateEventSessionStateEnum::EXPIRED),
           "FINISHED" => Ok(SessionStateEventSessionStateEnum::FINISHED),
           "UNAVAILABLE" => Ok(SessionStateEventSessionStateEnum::UNAVAILABLE),
           "ERROR" => Ok(SessionStateEventSessionStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SessionStateEventSessionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TestExecutionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Indicates the current progress of the test execution (e.g., FINISHED).
pub enum TestExecutionStateEnum {
    

    /// Do not use. For proto versioning only.
    ///
    /// "TEST_STATE_UNSPECIFIED"
    #[serde(rename="TEST_STATE_UNSPECIFIED")]
    TESTSTATEUNSPECIFIED,
    

    /// The execution or matrix is being validated.
    ///
    /// "VALIDATING"
    #[serde(rename="VALIDATING")]
    VALIDATING,
    

    /// The execution or matrix is waiting for resources to become available.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The execution is currently being processed. Can only be set on an execution.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The execution or matrix has terminated normally. On a matrix this means that the matrix level processing completed normally, but individual executions may be in an ERROR state.
    ///
    /// "FINISHED"
    #[serde(rename="FINISHED")]
    FINISHED,
    

    /// The execution or matrix has stopped because it encountered an infrastructure failure.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// The execution was not run because it corresponds to a unsupported environment. Can only be set on an execution.
    ///
    /// "UNSUPPORTED_ENVIRONMENT"
    #[serde(rename="UNSUPPORTED_ENVIRONMENT")]
    UNSUPPORTEDENVIRONMENT,
    

    /// The execution was not run because the provided inputs are incompatible with the requested environment. Example: requested AndroidVersion is lower than APK's minSdkVersion Can only be set on an execution.
    ///
    /// "INCOMPATIBLE_ENVIRONMENT"
    #[serde(rename="INCOMPATIBLE_ENVIRONMENT")]
    INCOMPATIBLEENVIRONMENT,
    

    /// The execution was not run because the provided inputs are incompatible with the requested architecture. Example: requested device does not support running the native code in the supplied APK Can only be set on an execution.
    ///
    /// "INCOMPATIBLE_ARCHITECTURE"
    #[serde(rename="INCOMPATIBLE_ARCHITECTURE")]
    INCOMPATIBLEARCHITECTURE,
    

    /// The user cancelled the execution. Can only be set on an execution.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The execution or matrix was not run because the provided inputs are not valid. Examples: input file is not of the expected type, is malformed/corrupt, or was flagged as malware
    ///
    /// "INVALID"
    #[serde(rename="INVALID")]
    INVALID,
}

impl AsRef<str> for TestExecutionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TestExecutionStateEnum::TESTSTATEUNSPECIFIED => "TEST_STATE_UNSPECIFIED",
            TestExecutionStateEnum::VALIDATING => "VALIDATING",
            TestExecutionStateEnum::PENDING => "PENDING",
            TestExecutionStateEnum::RUNNING => "RUNNING",
            TestExecutionStateEnum::FINISHED => "FINISHED",
            TestExecutionStateEnum::ERROR => "ERROR",
            TestExecutionStateEnum::UNSUPPORTEDENVIRONMENT => "UNSUPPORTED_ENVIRONMENT",
            TestExecutionStateEnum::INCOMPATIBLEENVIRONMENT => "INCOMPATIBLE_ENVIRONMENT",
            TestExecutionStateEnum::INCOMPATIBLEARCHITECTURE => "INCOMPATIBLE_ARCHITECTURE",
            TestExecutionStateEnum::CANCELLED => "CANCELLED",
            TestExecutionStateEnum::INVALID => "INVALID",
        }
    }
}

impl std::convert::TryFrom< &str> for TestExecutionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEST_STATE_UNSPECIFIED" => Ok(TestExecutionStateEnum::TESTSTATEUNSPECIFIED),
           "VALIDATING" => Ok(TestExecutionStateEnum::VALIDATING),
           "PENDING" => Ok(TestExecutionStateEnum::PENDING),
           "RUNNING" => Ok(TestExecutionStateEnum::RUNNING),
           "FINISHED" => Ok(TestExecutionStateEnum::FINISHED),
           "ERROR" => Ok(TestExecutionStateEnum::ERROR),
           "UNSUPPORTED_ENVIRONMENT" => Ok(TestExecutionStateEnum::UNSUPPORTEDENVIRONMENT),
           "INCOMPATIBLE_ENVIRONMENT" => Ok(TestExecutionStateEnum::INCOMPATIBLEENVIRONMENT),
           "INCOMPATIBLE_ARCHITECTURE" => Ok(TestExecutionStateEnum::INCOMPATIBLEARCHITECTURE),
           "CANCELLED" => Ok(TestExecutionStateEnum::CANCELLED),
           "INVALID" => Ok(TestExecutionStateEnum::INVALID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TestExecutionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TestMatrixInvalidMatrixDetailsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Describes why the matrix is considered invalid. Only useful for matrices in the INVALID state.
pub enum TestMatrixInvalidMatrixDetailsEnum {
    

    /// Do not use. For proto versioning only.
    ///
    /// "INVALID_MATRIX_DETAILS_UNSPECIFIED"
    #[serde(rename="INVALID_MATRIX_DETAILS_UNSPECIFIED")]
    INVALIDMATRIXDETAILSUNSPECIFIED,
    

    /// The matrix is INVALID, but there are no further details available.
    ///
    /// "DETAILS_UNAVAILABLE"
    #[serde(rename="DETAILS_UNAVAILABLE")]
    DETAILSUNAVAILABLE,
    

    /// The input app APK could not be parsed.
    ///
    /// "MALFORMED_APK"
    #[serde(rename="MALFORMED_APK")]
    MALFORMEDAPK,
    

    /// The input test APK could not be parsed.
    ///
    /// "MALFORMED_TEST_APK"
    #[serde(rename="MALFORMED_TEST_APK")]
    MALFORMEDTESTAPK,
    

    /// The AndroidManifest.xml could not be found.
    ///
    /// "NO_MANIFEST"
    #[serde(rename="NO_MANIFEST")]
    NOMANIFEST,
    

    /// The APK manifest does not declare a package name.
    ///
    /// "NO_PACKAGE_NAME"
    #[serde(rename="NO_PACKAGE_NAME")]
    NOPACKAGENAME,
    

    /// The APK application ID (aka package name) is invalid. See also https://developer.android.com/studio/build/application-id
    ///
    /// "INVALID_PACKAGE_NAME"
    #[serde(rename="INVALID_PACKAGE_NAME")]
    INVALIDPACKAGENAME,
    

    /// The test package and app package are the same.
    ///
    /// "TEST_SAME_AS_APP"
    #[serde(rename="TEST_SAME_AS_APP")]
    TESTSAMEASAPP,
    

    /// The test apk does not declare an instrumentation.
    ///
    /// "NO_INSTRUMENTATION"
    #[serde(rename="NO_INSTRUMENTATION")]
    NOINSTRUMENTATION,
    

    /// The input app apk does not have a signature.
    ///
    /// "NO_SIGNATURE"
    #[serde(rename="NO_SIGNATURE")]
    NOSIGNATURE,
    

    /// The test runner class specified by user or in the test APK's manifest file is not compatible with Android Test Orchestrator. Orchestrator is only compatible with AndroidJUnitRunner version 1.1 or higher. Orchestrator can be disabled by using DO_NOT_USE_ORCHESTRATOR OrchestratorOption.
    ///
    /// "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE"
    #[serde(rename="INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE")]
    INSTRUMENTATIONORCHESTRATORINCOMPATIBLE,
    

    /// The test APK does not contain the test runner class specified by the user or in the manifest file. This can be caused by one of the following reasons: - the user provided a runner class name that's incorrect, or - the test runner isn't built into the test APK (might be in the app APK instead).
    ///
    /// "NO_TEST_RUNNER_CLASS"
    #[serde(rename="NO_TEST_RUNNER_CLASS")]
    NOTESTRUNNERCLASS,
    

    /// A main launcher activity could not be found.
    ///
    /// "NO_LAUNCHER_ACTIVITY"
    #[serde(rename="NO_LAUNCHER_ACTIVITY")]
    NOLAUNCHERACTIVITY,
    

    /// The app declares one or more permissions that are not allowed.
    ///
    /// "FORBIDDEN_PERMISSIONS"
    #[serde(rename="FORBIDDEN_PERMISSIONS")]
    FORBIDDENPERMISSIONS,
    

    /// There is a conflict in the provided robo_directives.
    ///
    /// "INVALID_ROBO_DIRECTIVES"
    #[serde(rename="INVALID_ROBO_DIRECTIVES")]
    INVALIDROBODIRECTIVES,
    

    /// There is at least one invalid resource name in the provided robo directives
    ///
    /// "INVALID_RESOURCE_NAME"
    #[serde(rename="INVALID_RESOURCE_NAME")]
    INVALIDRESOURCENAME,
    

    /// Invalid definition of action in the robo directives (e.g. a click or ignore action includes an input text field)
    ///
    /// "INVALID_DIRECTIVE_ACTION"
    #[serde(rename="INVALID_DIRECTIVE_ACTION")]
    INVALIDDIRECTIVEACTION,
    

    /// There is no test loop intent filter, or the one that is given is not formatted correctly.
    ///
    /// "TEST_LOOP_INTENT_FILTER_NOT_FOUND"
    #[serde(rename="TEST_LOOP_INTENT_FILTER_NOT_FOUND")]
    TESTLOOPINTENTFILTERNOTFOUND,
    

    /// The request contains a scenario label that was not declared in the manifest.
    ///
    /// "SCENARIO_LABEL_NOT_DECLARED"
    #[serde(rename="SCENARIO_LABEL_NOT_DECLARED")]
    SCENARIOLABELNOTDECLARED,
    

    /// There was an error when parsing a label's value.
    ///
    /// "SCENARIO_LABEL_MALFORMED"
    #[serde(rename="SCENARIO_LABEL_MALFORMED")]
    SCENARIOLABELMALFORMED,
    

    /// The request contains a scenario number that was not declared in the manifest.
    ///
    /// "SCENARIO_NOT_DECLARED"
    #[serde(rename="SCENARIO_NOT_DECLARED")]
    SCENARIONOTDECLARED,
    

    /// Device administrator applications are not allowed.
    ///
    /// "DEVICE_ADMIN_RECEIVER"
    #[serde(rename="DEVICE_ADMIN_RECEIVER")]
    DEVICEADMINRECEIVER,
    

    /// The zipped XCTest was malformed. The zip did not contain a single .xctestrun file and the contents of the DerivedData/Build/Products directory.
    ///
    /// "MALFORMED_XC_TEST_ZIP"
    #[serde(rename="MALFORMED_XC_TEST_ZIP")]
    MALFORMEDXCTESTZIP,
    

    /// The zipped XCTest was built for the iOS simulator rather than for a physical device.
    ///
    /// "BUILT_FOR_IOS_SIMULATOR"
    #[serde(rename="BUILT_FOR_IOS_SIMULATOR")]
    BUILTFORIOSSIMULATOR,
    

    /// The .xctestrun file did not specify any test targets.
    ///
    /// "NO_TESTS_IN_XC_TEST_ZIP"
    #[serde(rename="NO_TESTS_IN_XC_TEST_ZIP")]
    NOTESTSINXCTESTZIP,
    

    /// One or more of the test targets defined in the .xctestrun file specifies "UseDestinationArtifacts", which is disallowed.
    ///
    /// "USE_DESTINATION_ARTIFACTS"
    #[serde(rename="USE_DESTINATION_ARTIFACTS")]
    USEDESTINATIONARTIFACTS,
    

    /// XC tests which run on physical devices must have "IsAppHostedTestBundle" == "true" in the xctestrun file.
    ///
    /// "TEST_NOT_APP_HOSTED"
    #[serde(rename="TEST_NOT_APP_HOSTED")]
    TESTNOTAPPHOSTED,
    

    /// An Info.plist file in the XCTest zip could not be parsed.
    ///
    /// "PLIST_CANNOT_BE_PARSED"
    #[serde(rename="PLIST_CANNOT_BE_PARSED")]
    PLISTCANNOTBEPARSED,
    

    /// The APK is marked as "testOnly". Deprecated and not currently used.
    ///
    /// "TEST_ONLY_APK"
    #[serde(rename="TEST_ONLY_APK")]
    TESTONLYAPK,
    

    /// The input IPA could not be parsed.
    ///
    /// "MALFORMED_IPA"
    #[serde(rename="MALFORMED_IPA")]
    MALFORMEDIPA,
    

    /// The application doesn't register the game loop URL scheme.
    ///
    /// "MISSING_URL_SCHEME"
    #[serde(rename="MISSING_URL_SCHEME")]
    MISSINGURLSCHEME,
    

    /// The iOS application bundle (.app) couldn't be processed.
    ///
    /// "MALFORMED_APP_BUNDLE"
    #[serde(rename="MALFORMED_APP_BUNDLE")]
    MALFORMEDAPPBUNDLE,
    

    /// APK contains no code. See also https://developer.android.com/guide/topics/manifest/application-element.html#code
    ///
    /// "NO_CODE_APK"
    #[serde(rename="NO_CODE_APK")]
    NOCODEAPK,
    

    /// Either the provided input APK path was malformed, the APK file does not exist, or the user does not have permission to access the APK file.
    ///
    /// "INVALID_INPUT_APK"
    #[serde(rename="INVALID_INPUT_APK")]
    INVALIDINPUTAPK,
    

    /// APK is built for a preview SDK which is unsupported
    ///
    /// "INVALID_APK_PREVIEW_SDK"
    #[serde(rename="INVALID_APK_PREVIEW_SDK")]
    INVALIDAPKPREVIEWSDK,
    

    /// The matrix expanded to contain too many executions.
    ///
    /// "MATRIX_TOO_LARGE"
    #[serde(rename="MATRIX_TOO_LARGE")]
    MATRIXTOOLARGE,
    

    /// Not enough test quota to run the executions in this matrix.
    ///
    /// "TEST_QUOTA_EXCEEDED"
    #[serde(rename="TEST_QUOTA_EXCEEDED")]
    TESTQUOTAEXCEEDED,
    

    /// A required cloud service api is not activated. See: https://firebase.google.com/docs/test-lab/android/continuous#requirements
    ///
    /// "SERVICE_NOT_ACTIVATED"
    #[serde(rename="SERVICE_NOT_ACTIVATED")]
    SERVICENOTACTIVATED,
    

    /// There was an unknown permission issue running this test.
    ///
    /// "UNKNOWN_PERMISSION_ERROR"
    #[serde(rename="UNKNOWN_PERMISSION_ERROR")]
    UNKNOWNPERMISSIONERROR,
}

impl AsRef<str> for TestMatrixInvalidMatrixDetailsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TestMatrixInvalidMatrixDetailsEnum::INVALIDMATRIXDETAILSUNSPECIFIED => "INVALID_MATRIX_DETAILS_UNSPECIFIED",
            TestMatrixInvalidMatrixDetailsEnum::DETAILSUNAVAILABLE => "DETAILS_UNAVAILABLE",
            TestMatrixInvalidMatrixDetailsEnum::MALFORMEDAPK => "MALFORMED_APK",
            TestMatrixInvalidMatrixDetailsEnum::MALFORMEDTESTAPK => "MALFORMED_TEST_APK",
            TestMatrixInvalidMatrixDetailsEnum::NOMANIFEST => "NO_MANIFEST",
            TestMatrixInvalidMatrixDetailsEnum::NOPACKAGENAME => "NO_PACKAGE_NAME",
            TestMatrixInvalidMatrixDetailsEnum::INVALIDPACKAGENAME => "INVALID_PACKAGE_NAME",
            TestMatrixInvalidMatrixDetailsEnum::TESTSAMEASAPP => "TEST_SAME_AS_APP",
            TestMatrixInvalidMatrixDetailsEnum::NOINSTRUMENTATION => "NO_INSTRUMENTATION",
            TestMatrixInvalidMatrixDetailsEnum::NOSIGNATURE => "NO_SIGNATURE",
            TestMatrixInvalidMatrixDetailsEnum::INSTRUMENTATIONORCHESTRATORINCOMPATIBLE => "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE",
            TestMatrixInvalidMatrixDetailsEnum::NOTESTRUNNERCLASS => "NO_TEST_RUNNER_CLASS",
            TestMatrixInvalidMatrixDetailsEnum::NOLAUNCHERACTIVITY => "NO_LAUNCHER_ACTIVITY",
            TestMatrixInvalidMatrixDetailsEnum::FORBIDDENPERMISSIONS => "FORBIDDEN_PERMISSIONS",
            TestMatrixInvalidMatrixDetailsEnum::INVALIDROBODIRECTIVES => "INVALID_ROBO_DIRECTIVES",
            TestMatrixInvalidMatrixDetailsEnum::INVALIDRESOURCENAME => "INVALID_RESOURCE_NAME",
            TestMatrixInvalidMatrixDetailsEnum::INVALIDDIRECTIVEACTION => "INVALID_DIRECTIVE_ACTION",
            TestMatrixInvalidMatrixDetailsEnum::TESTLOOPINTENTFILTERNOTFOUND => "TEST_LOOP_INTENT_FILTER_NOT_FOUND",
            TestMatrixInvalidMatrixDetailsEnum::SCENARIOLABELNOTDECLARED => "SCENARIO_LABEL_NOT_DECLARED",
            TestMatrixInvalidMatrixDetailsEnum::SCENARIOLABELMALFORMED => "SCENARIO_LABEL_MALFORMED",
            TestMatrixInvalidMatrixDetailsEnum::SCENARIONOTDECLARED => "SCENARIO_NOT_DECLARED",
            TestMatrixInvalidMatrixDetailsEnum::DEVICEADMINRECEIVER => "DEVICE_ADMIN_RECEIVER",
            TestMatrixInvalidMatrixDetailsEnum::MALFORMEDXCTESTZIP => "MALFORMED_XC_TEST_ZIP",
            TestMatrixInvalidMatrixDetailsEnum::BUILTFORIOSSIMULATOR => "BUILT_FOR_IOS_SIMULATOR",
            TestMatrixInvalidMatrixDetailsEnum::NOTESTSINXCTESTZIP => "NO_TESTS_IN_XC_TEST_ZIP",
            TestMatrixInvalidMatrixDetailsEnum::USEDESTINATIONARTIFACTS => "USE_DESTINATION_ARTIFACTS",
            TestMatrixInvalidMatrixDetailsEnum::TESTNOTAPPHOSTED => "TEST_NOT_APP_HOSTED",
            TestMatrixInvalidMatrixDetailsEnum::PLISTCANNOTBEPARSED => "PLIST_CANNOT_BE_PARSED",
            TestMatrixInvalidMatrixDetailsEnum::TESTONLYAPK => "TEST_ONLY_APK",
            TestMatrixInvalidMatrixDetailsEnum::MALFORMEDIPA => "MALFORMED_IPA",
            TestMatrixInvalidMatrixDetailsEnum::MISSINGURLSCHEME => "MISSING_URL_SCHEME",
            TestMatrixInvalidMatrixDetailsEnum::MALFORMEDAPPBUNDLE => "MALFORMED_APP_BUNDLE",
            TestMatrixInvalidMatrixDetailsEnum::NOCODEAPK => "NO_CODE_APK",
            TestMatrixInvalidMatrixDetailsEnum::INVALIDINPUTAPK => "INVALID_INPUT_APK",
            TestMatrixInvalidMatrixDetailsEnum::INVALIDAPKPREVIEWSDK => "INVALID_APK_PREVIEW_SDK",
            TestMatrixInvalidMatrixDetailsEnum::MATRIXTOOLARGE => "MATRIX_TOO_LARGE",
            TestMatrixInvalidMatrixDetailsEnum::TESTQUOTAEXCEEDED => "TEST_QUOTA_EXCEEDED",
            TestMatrixInvalidMatrixDetailsEnum::SERVICENOTACTIVATED => "SERVICE_NOT_ACTIVATED",
            TestMatrixInvalidMatrixDetailsEnum::UNKNOWNPERMISSIONERROR => "UNKNOWN_PERMISSION_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for TestMatrixInvalidMatrixDetailsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVALID_MATRIX_DETAILS_UNSPECIFIED" => Ok(TestMatrixInvalidMatrixDetailsEnum::INVALIDMATRIXDETAILSUNSPECIFIED),
           "DETAILS_UNAVAILABLE" => Ok(TestMatrixInvalidMatrixDetailsEnum::DETAILSUNAVAILABLE),
           "MALFORMED_APK" => Ok(TestMatrixInvalidMatrixDetailsEnum::MALFORMEDAPK),
           "MALFORMED_TEST_APK" => Ok(TestMatrixInvalidMatrixDetailsEnum::MALFORMEDTESTAPK),
           "NO_MANIFEST" => Ok(TestMatrixInvalidMatrixDetailsEnum::NOMANIFEST),
           "NO_PACKAGE_NAME" => Ok(TestMatrixInvalidMatrixDetailsEnum::NOPACKAGENAME),
           "INVALID_PACKAGE_NAME" => Ok(TestMatrixInvalidMatrixDetailsEnum::INVALIDPACKAGENAME),
           "TEST_SAME_AS_APP" => Ok(TestMatrixInvalidMatrixDetailsEnum::TESTSAMEASAPP),
           "NO_INSTRUMENTATION" => Ok(TestMatrixInvalidMatrixDetailsEnum::NOINSTRUMENTATION),
           "NO_SIGNATURE" => Ok(TestMatrixInvalidMatrixDetailsEnum::NOSIGNATURE),
           "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE" => Ok(TestMatrixInvalidMatrixDetailsEnum::INSTRUMENTATIONORCHESTRATORINCOMPATIBLE),
           "NO_TEST_RUNNER_CLASS" => Ok(TestMatrixInvalidMatrixDetailsEnum::NOTESTRUNNERCLASS),
           "NO_LAUNCHER_ACTIVITY" => Ok(TestMatrixInvalidMatrixDetailsEnum::NOLAUNCHERACTIVITY),
           "FORBIDDEN_PERMISSIONS" => Ok(TestMatrixInvalidMatrixDetailsEnum::FORBIDDENPERMISSIONS),
           "INVALID_ROBO_DIRECTIVES" => Ok(TestMatrixInvalidMatrixDetailsEnum::INVALIDROBODIRECTIVES),
           "INVALID_RESOURCE_NAME" => Ok(TestMatrixInvalidMatrixDetailsEnum::INVALIDRESOURCENAME),
           "INVALID_DIRECTIVE_ACTION" => Ok(TestMatrixInvalidMatrixDetailsEnum::INVALIDDIRECTIVEACTION),
           "TEST_LOOP_INTENT_FILTER_NOT_FOUND" => Ok(TestMatrixInvalidMatrixDetailsEnum::TESTLOOPINTENTFILTERNOTFOUND),
           "SCENARIO_LABEL_NOT_DECLARED" => Ok(TestMatrixInvalidMatrixDetailsEnum::SCENARIOLABELNOTDECLARED),
           "SCENARIO_LABEL_MALFORMED" => Ok(TestMatrixInvalidMatrixDetailsEnum::SCENARIOLABELMALFORMED),
           "SCENARIO_NOT_DECLARED" => Ok(TestMatrixInvalidMatrixDetailsEnum::SCENARIONOTDECLARED),
           "DEVICE_ADMIN_RECEIVER" => Ok(TestMatrixInvalidMatrixDetailsEnum::DEVICEADMINRECEIVER),
           "MALFORMED_XC_TEST_ZIP" => Ok(TestMatrixInvalidMatrixDetailsEnum::MALFORMEDXCTESTZIP),
           "BUILT_FOR_IOS_SIMULATOR" => Ok(TestMatrixInvalidMatrixDetailsEnum::BUILTFORIOSSIMULATOR),
           "NO_TESTS_IN_XC_TEST_ZIP" => Ok(TestMatrixInvalidMatrixDetailsEnum::NOTESTSINXCTESTZIP),
           "USE_DESTINATION_ARTIFACTS" => Ok(TestMatrixInvalidMatrixDetailsEnum::USEDESTINATIONARTIFACTS),
           "TEST_NOT_APP_HOSTED" => Ok(TestMatrixInvalidMatrixDetailsEnum::TESTNOTAPPHOSTED),
           "PLIST_CANNOT_BE_PARSED" => Ok(TestMatrixInvalidMatrixDetailsEnum::PLISTCANNOTBEPARSED),
           "TEST_ONLY_APK" => Ok(TestMatrixInvalidMatrixDetailsEnum::TESTONLYAPK),
           "MALFORMED_IPA" => Ok(TestMatrixInvalidMatrixDetailsEnum::MALFORMEDIPA),
           "MISSING_URL_SCHEME" => Ok(TestMatrixInvalidMatrixDetailsEnum::MISSINGURLSCHEME),
           "MALFORMED_APP_BUNDLE" => Ok(TestMatrixInvalidMatrixDetailsEnum::MALFORMEDAPPBUNDLE),
           "NO_CODE_APK" => Ok(TestMatrixInvalidMatrixDetailsEnum::NOCODEAPK),
           "INVALID_INPUT_APK" => Ok(TestMatrixInvalidMatrixDetailsEnum::INVALIDINPUTAPK),
           "INVALID_APK_PREVIEW_SDK" => Ok(TestMatrixInvalidMatrixDetailsEnum::INVALIDAPKPREVIEWSDK),
           "MATRIX_TOO_LARGE" => Ok(TestMatrixInvalidMatrixDetailsEnum::MATRIXTOOLARGE),
           "TEST_QUOTA_EXCEEDED" => Ok(TestMatrixInvalidMatrixDetailsEnum::TESTQUOTAEXCEEDED),
           "SERVICE_NOT_ACTIVATED" => Ok(TestMatrixInvalidMatrixDetailsEnum::SERVICENOTACTIVATED),
           "UNKNOWN_PERMISSION_ERROR" => Ok(TestMatrixInvalidMatrixDetailsEnum::UNKNOWNPERMISSIONERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TestMatrixInvalidMatrixDetailsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TestMatrixOutcomeSummaryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output Only. The overall outcome of the test. Only set when the test matrix state is FINISHED.
pub enum TestMatrixOutcomeSummaryEnum {
    

    /// Do not use. For proto versioning only.
    ///
    /// "OUTCOME_SUMMARY_UNSPECIFIED"
    #[serde(rename="OUTCOME_SUMMARY_UNSPECIFIED")]
    OUTCOMESUMMARYUNSPECIFIED,
    

    /// The test matrix run was successful, for instance: - All the test cases passed. - Robo did not detect a crash of the application under test.
    ///
    /// "SUCCESS"
    #[serde(rename="SUCCESS")]
    SUCCESS,
    

    /// A run failed, for instance: - One or more test cases failed. - A test timed out. - The application under test crashed.
    ///
    /// "FAILURE"
    #[serde(rename="FAILURE")]
    FAILURE,
    

    /// Something unexpected happened. The run should still be considered unsuccessful but this is likely a transient problem and re-running the test might be successful.
    ///
    /// "INCONCLUSIVE"
    #[serde(rename="INCONCLUSIVE")]
    INCONCLUSIVE,
    

    /// All tests were skipped, for instance: - All device configurations were incompatible.
    ///
    /// "SKIPPED"
    #[serde(rename="SKIPPED")]
    SKIPPED,
}

impl AsRef<str> for TestMatrixOutcomeSummaryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TestMatrixOutcomeSummaryEnum::OUTCOMESUMMARYUNSPECIFIED => "OUTCOME_SUMMARY_UNSPECIFIED",
            TestMatrixOutcomeSummaryEnum::SUCCESS => "SUCCESS",
            TestMatrixOutcomeSummaryEnum::FAILURE => "FAILURE",
            TestMatrixOutcomeSummaryEnum::INCONCLUSIVE => "INCONCLUSIVE",
            TestMatrixOutcomeSummaryEnum::SKIPPED => "SKIPPED",
        }
    }
}

impl std::convert::TryFrom< &str> for TestMatrixOutcomeSummaryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OUTCOME_SUMMARY_UNSPECIFIED" => Ok(TestMatrixOutcomeSummaryEnum::OUTCOMESUMMARYUNSPECIFIED),
           "SUCCESS" => Ok(TestMatrixOutcomeSummaryEnum::SUCCESS),
           "FAILURE" => Ok(TestMatrixOutcomeSummaryEnum::FAILURE),
           "INCONCLUSIVE" => Ok(TestMatrixOutcomeSummaryEnum::INCONCLUSIVE),
           "SKIPPED" => Ok(TestMatrixOutcomeSummaryEnum::SKIPPED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TestMatrixOutcomeSummaryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TestMatrixStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Indicates the current progress of the test matrix.
pub enum TestMatrixStateEnum {
    

    /// Do not use. For proto versioning only.
    ///
    /// "TEST_STATE_UNSPECIFIED"
    #[serde(rename="TEST_STATE_UNSPECIFIED")]
    TESTSTATEUNSPECIFIED,
    

    /// The execution or matrix is being validated.
    ///
    /// "VALIDATING"
    #[serde(rename="VALIDATING")]
    VALIDATING,
    

    /// The execution or matrix is waiting for resources to become available.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The execution is currently being processed. Can only be set on an execution.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The execution or matrix has terminated normally. On a matrix this means that the matrix level processing completed normally, but individual executions may be in an ERROR state.
    ///
    /// "FINISHED"
    #[serde(rename="FINISHED")]
    FINISHED,
    

    /// The execution or matrix has stopped because it encountered an infrastructure failure.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// The execution was not run because it corresponds to a unsupported environment. Can only be set on an execution.
    ///
    /// "UNSUPPORTED_ENVIRONMENT"
    #[serde(rename="UNSUPPORTED_ENVIRONMENT")]
    UNSUPPORTEDENVIRONMENT,
    

    /// The execution was not run because the provided inputs are incompatible with the requested environment. Example: requested AndroidVersion is lower than APK's minSdkVersion Can only be set on an execution.
    ///
    /// "INCOMPATIBLE_ENVIRONMENT"
    #[serde(rename="INCOMPATIBLE_ENVIRONMENT")]
    INCOMPATIBLEENVIRONMENT,
    

    /// The execution was not run because the provided inputs are incompatible with the requested architecture. Example: requested device does not support running the native code in the supplied APK Can only be set on an execution.
    ///
    /// "INCOMPATIBLE_ARCHITECTURE"
    #[serde(rename="INCOMPATIBLE_ARCHITECTURE")]
    INCOMPATIBLEARCHITECTURE,
    

    /// The user cancelled the execution. Can only be set on an execution.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The execution or matrix was not run because the provided inputs are not valid. Examples: input file is not of the expected type, is malformed/corrupt, or was flagged as malware
    ///
    /// "INVALID"
    #[serde(rename="INVALID")]
    INVALID,
}

impl AsRef<str> for TestMatrixStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TestMatrixStateEnum::TESTSTATEUNSPECIFIED => "TEST_STATE_UNSPECIFIED",
            TestMatrixStateEnum::VALIDATING => "VALIDATING",
            TestMatrixStateEnum::PENDING => "PENDING",
            TestMatrixStateEnum::RUNNING => "RUNNING",
            TestMatrixStateEnum::FINISHED => "FINISHED",
            TestMatrixStateEnum::ERROR => "ERROR",
            TestMatrixStateEnum::UNSUPPORTEDENVIRONMENT => "UNSUPPORTED_ENVIRONMENT",
            TestMatrixStateEnum::INCOMPATIBLEENVIRONMENT => "INCOMPATIBLE_ENVIRONMENT",
            TestMatrixStateEnum::INCOMPATIBLEARCHITECTURE => "INCOMPATIBLE_ARCHITECTURE",
            TestMatrixStateEnum::CANCELLED => "CANCELLED",
            TestMatrixStateEnum::INVALID => "INVALID",
        }
    }
}

impl std::convert::TryFrom< &str> for TestMatrixStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEST_STATE_UNSPECIFIED" => Ok(TestMatrixStateEnum::TESTSTATEUNSPECIFIED),
           "VALIDATING" => Ok(TestMatrixStateEnum::VALIDATING),
           "PENDING" => Ok(TestMatrixStateEnum::PENDING),
           "RUNNING" => Ok(TestMatrixStateEnum::RUNNING),
           "FINISHED" => Ok(TestMatrixStateEnum::FINISHED),
           "ERROR" => Ok(TestMatrixStateEnum::ERROR),
           "UNSUPPORTED_ENVIRONMENT" => Ok(TestMatrixStateEnum::UNSUPPORTEDENVIRONMENT),
           "INCOMPATIBLE_ENVIRONMENT" => Ok(TestMatrixStateEnum::INCOMPATIBLEENVIRONMENT),
           "INCOMPATIBLE_ARCHITECTURE" => Ok(TestMatrixStateEnum::INCOMPATIBLEARCHITECTURE),
           "CANCELLED" => Ok(TestMatrixStateEnum::CANCELLED),
           "INVALID" => Ok(TestMatrixStateEnum::INVALID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TestMatrixStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TestEnvironmentCatalogEnvironmentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of environment that should be listed.
pub enum TestEnvironmentCatalogEnvironmentTypeEnum {
    

    /// Do not use. For proto versioning only.
    ///
    /// "ENVIRONMENT_TYPE_UNSPECIFIED"
    #[serde(rename="ENVIRONMENT_TYPE_UNSPECIFIED")]
    ENVIRONMENTTYPEUNSPECIFIED,
    

    /// A device running a version of the Android OS.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// A device running a version of iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// A network configuration to use when running a test.
    ///
    /// "NETWORK_CONFIGURATION"
    #[serde(rename="NETWORK_CONFIGURATION")]
    NETWORKCONFIGURATION,
    

    /// The software environment provided by TestExecutionService.
    ///
    /// "PROVIDED_SOFTWARE"
    #[serde(rename="PROVIDED_SOFTWARE")]
    PROVIDEDSOFTWARE,
    

    /// The IP blocks used by devices in the test environment.
    ///
    /// "DEVICE_IP_BLOCKS"
    #[serde(rename="DEVICE_IP_BLOCKS")]
    DEVICEIPBLOCKS,
}

impl AsRef<str> for TestEnvironmentCatalogEnvironmentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TestEnvironmentCatalogEnvironmentTypeEnum::ENVIRONMENTTYPEUNSPECIFIED => "ENVIRONMENT_TYPE_UNSPECIFIED",
            TestEnvironmentCatalogEnvironmentTypeEnum::ANDROID => "ANDROID",
            TestEnvironmentCatalogEnvironmentTypeEnum::IOS => "IOS",
            TestEnvironmentCatalogEnvironmentTypeEnum::NETWORKCONFIGURATION => "NETWORK_CONFIGURATION",
            TestEnvironmentCatalogEnvironmentTypeEnum::PROVIDEDSOFTWARE => "PROVIDED_SOFTWARE",
            TestEnvironmentCatalogEnvironmentTypeEnum::DEVICEIPBLOCKS => "DEVICE_IP_BLOCKS",
        }
    }
}

impl std::convert::TryFrom< &str> for TestEnvironmentCatalogEnvironmentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVIRONMENT_TYPE_UNSPECIFIED" => Ok(TestEnvironmentCatalogEnvironmentTypeEnum::ENVIRONMENTTYPEUNSPECIFIED),
           "ANDROID" => Ok(TestEnvironmentCatalogEnvironmentTypeEnum::ANDROID),
           "IOS" => Ok(TestEnvironmentCatalogEnvironmentTypeEnum::IOS),
           "NETWORK_CONFIGURATION" => Ok(TestEnvironmentCatalogEnvironmentTypeEnum::NETWORKCONFIGURATION),
           "PROVIDED_SOFTWARE" => Ok(TestEnvironmentCatalogEnvironmentTypeEnum::PROVIDEDSOFTWARE),
           "DEVICE_IP_BLOCKS" => Ok(TestEnvironmentCatalogEnvironmentTypeEnum::DEVICEIPBLOCKS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TestEnvironmentCatalogEnvironmentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


