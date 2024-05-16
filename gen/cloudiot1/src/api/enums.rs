use super::*;



// region DeviceLogLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// **Beta Feature** The logging verbosity for device activity. If unspecified, DeviceRegistry.log_level will be used.
pub enum DeviceLogLevelEnum {
    

    /// No logging specified. If not specified, logging will be disabled.
    ///
    /// "LOG_LEVEL_UNSPECIFIED"
    #[serde(rename="LOG_LEVEL_UNSPECIFIED")]
    LOGLEVELUNSPECIFIED,
    

    /// Disables logging.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Error events will be logged.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Informational events will be logged, such as connections and disconnections.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// All events will be logged.
    ///
    /// "DEBUG"
    #[serde(rename="DEBUG")]
    DEBUG,
}

impl AsRef<str> for DeviceLogLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceLogLevelEnum::LOGLEVELUNSPECIFIED => "LOG_LEVEL_UNSPECIFIED",
            DeviceLogLevelEnum::NONE => "NONE",
            DeviceLogLevelEnum::ERROR => "ERROR",
            DeviceLogLevelEnum::INFO => "INFO",
            DeviceLogLevelEnum::DEBUG => "DEBUG",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceLogLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_LEVEL_UNSPECIFIED" => Ok(DeviceLogLevelEnum::LOGLEVELUNSPECIFIED),
           "NONE" => Ok(DeviceLogLevelEnum::NONE),
           "ERROR" => Ok(DeviceLogLevelEnum::ERROR),
           "INFO" => Ok(DeviceLogLevelEnum::INFO),
           "DEBUG" => Ok(DeviceLogLevelEnum::DEBUG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceLogLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceRegistryLogLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// **Beta Feature** The default logging verbosity for activity from devices in this registry. The verbosity level can be overridden by Device.log_level.
pub enum DeviceRegistryLogLevelEnum {
    

    /// No logging specified. If not specified, logging will be disabled.
    ///
    /// "LOG_LEVEL_UNSPECIFIED"
    #[serde(rename="LOG_LEVEL_UNSPECIFIED")]
    LOGLEVELUNSPECIFIED,
    

    /// Disables logging.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Error events will be logged.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Informational events will be logged, such as connections and disconnections.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// All events will be logged.
    ///
    /// "DEBUG"
    #[serde(rename="DEBUG")]
    DEBUG,
}

impl AsRef<str> for DeviceRegistryLogLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceRegistryLogLevelEnum::LOGLEVELUNSPECIFIED => "LOG_LEVEL_UNSPECIFIED",
            DeviceRegistryLogLevelEnum::NONE => "NONE",
            DeviceRegistryLogLevelEnum::ERROR => "ERROR",
            DeviceRegistryLogLevelEnum::INFO => "INFO",
            DeviceRegistryLogLevelEnum::DEBUG => "DEBUG",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceRegistryLogLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_LEVEL_UNSPECIFIED" => Ok(DeviceRegistryLogLevelEnum::LOGLEVELUNSPECIFIED),
           "NONE" => Ok(DeviceRegistryLogLevelEnum::NONE),
           "ERROR" => Ok(DeviceRegistryLogLevelEnum::ERROR),
           "INFO" => Ok(DeviceRegistryLogLevelEnum::INFO),
           "DEBUG" => Ok(DeviceRegistryLogLevelEnum::DEBUG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceRegistryLogLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GatewayConfigGatewayAuthMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates how to authorize and/or authenticate devices to access the gateway.
pub enum GatewayConfigGatewayAuthMethodEnum {
    

    /// No authentication/authorization method specified. No devices are allowed to access the gateway.
    ///
    /// "GATEWAY_AUTH_METHOD_UNSPECIFIED"
    #[serde(rename="GATEWAY_AUTH_METHOD_UNSPECIFIED")]
    GATEWAYAUTHMETHODUNSPECIFIED,
    

    /// The device is authenticated through the gateway association only. Device credentials are ignored even if provided.
    ///
    /// "ASSOCIATION_ONLY"
    #[serde(rename="ASSOCIATION_ONLY")]
    ASSOCIATIONONLY,
    

    /// The device is authenticated through its own credentials. Gateway association is not checked.
    ///
    /// "DEVICE_AUTH_TOKEN_ONLY"
    #[serde(rename="DEVICE_AUTH_TOKEN_ONLY")]
    DEVICEAUTHTOKENONLY,
    

    /// The device is authenticated through both device credentials and gateway association. The device must be bound to the gateway and must provide its own credentials.
    ///
    /// "ASSOCIATION_AND_DEVICE_AUTH_TOKEN"
    #[serde(rename="ASSOCIATION_AND_DEVICE_AUTH_TOKEN")]
    ASSOCIATIONANDDEVICEAUTHTOKEN,
}

impl AsRef<str> for GatewayConfigGatewayAuthMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GatewayConfigGatewayAuthMethodEnum::GATEWAYAUTHMETHODUNSPECIFIED => "GATEWAY_AUTH_METHOD_UNSPECIFIED",
            GatewayConfigGatewayAuthMethodEnum::ASSOCIATIONONLY => "ASSOCIATION_ONLY",
            GatewayConfigGatewayAuthMethodEnum::DEVICEAUTHTOKENONLY => "DEVICE_AUTH_TOKEN_ONLY",
            GatewayConfigGatewayAuthMethodEnum::ASSOCIATIONANDDEVICEAUTHTOKEN => "ASSOCIATION_AND_DEVICE_AUTH_TOKEN",
        }
    }
}

impl std::convert::TryFrom< &str> for GatewayConfigGatewayAuthMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GATEWAY_AUTH_METHOD_UNSPECIFIED" => Ok(GatewayConfigGatewayAuthMethodEnum::GATEWAYAUTHMETHODUNSPECIFIED),
           "ASSOCIATION_ONLY" => Ok(GatewayConfigGatewayAuthMethodEnum::ASSOCIATIONONLY),
           "DEVICE_AUTH_TOKEN_ONLY" => Ok(GatewayConfigGatewayAuthMethodEnum::DEVICEAUTHTOKENONLY),
           "ASSOCIATION_AND_DEVICE_AUTH_TOKEN" => Ok(GatewayConfigGatewayAuthMethodEnum::ASSOCIATIONANDDEVICEAUTHTOKEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GatewayConfigGatewayAuthMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GatewayConfigGatewayTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the device is a gateway.
pub enum GatewayConfigGatewayTypeEnum {
    

    /// If unspecified, the device is considered a non-gateway device.
    ///
    /// "GATEWAY_TYPE_UNSPECIFIED"
    #[serde(rename="GATEWAY_TYPE_UNSPECIFIED")]
    GATEWAYTYPEUNSPECIFIED,
    

    /// The device is a gateway.
    ///
    /// "GATEWAY"
    #[serde(rename="GATEWAY")]
    GATEWAY,
    

    /// The device is not a gateway.
    ///
    /// "NON_GATEWAY"
    #[serde(rename="NON_GATEWAY")]
    NONGATEWAY,
}

impl AsRef<str> for GatewayConfigGatewayTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GatewayConfigGatewayTypeEnum::GATEWAYTYPEUNSPECIFIED => "GATEWAY_TYPE_UNSPECIFIED",
            GatewayConfigGatewayTypeEnum::GATEWAY => "GATEWAY",
            GatewayConfigGatewayTypeEnum::NONGATEWAY => "NON_GATEWAY",
        }
    }
}

impl std::convert::TryFrom< &str> for GatewayConfigGatewayTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GATEWAY_TYPE_UNSPECIFIED" => Ok(GatewayConfigGatewayTypeEnum::GATEWAYTYPEUNSPECIFIED),
           "GATEWAY" => Ok(GatewayConfigGatewayTypeEnum::GATEWAY),
           "NON_GATEWAY" => Ok(GatewayConfigGatewayTypeEnum::NONGATEWAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GatewayConfigGatewayTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HttpConfigHttpEnabledStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If enabled, allows devices to use DeviceService via the HTTP protocol. Otherwise, any requests to DeviceService will fail for this registry.
pub enum HttpConfigHttpEnabledStateEnum {
    

    /// No HTTP state specified. If not specified, DeviceService will be enabled by default.
    ///
    /// "HTTP_STATE_UNSPECIFIED"
    #[serde(rename="HTTP_STATE_UNSPECIFIED")]
    HTTPSTATEUNSPECIFIED,
    

    /// Enables DeviceService (HTTP) service for the registry.
    ///
    /// "HTTP_ENABLED"
    #[serde(rename="HTTP_ENABLED")]
    HTTPENABLED,
    

    /// Disables DeviceService (HTTP) service for the registry.
    ///
    /// "HTTP_DISABLED"
    #[serde(rename="HTTP_DISABLED")]
    HTTPDISABLED,
}

impl AsRef<str> for HttpConfigHttpEnabledStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HttpConfigHttpEnabledStateEnum::HTTPSTATEUNSPECIFIED => "HTTP_STATE_UNSPECIFIED",
            HttpConfigHttpEnabledStateEnum::HTTPENABLED => "HTTP_ENABLED",
            HttpConfigHttpEnabledStateEnum::HTTPDISABLED => "HTTP_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for HttpConfigHttpEnabledStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HTTP_STATE_UNSPECIFIED" => Ok(HttpConfigHttpEnabledStateEnum::HTTPSTATEUNSPECIFIED),
           "HTTP_ENABLED" => Ok(HttpConfigHttpEnabledStateEnum::HTTPENABLED),
           "HTTP_DISABLED" => Ok(HttpConfigHttpEnabledStateEnum::HTTPDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HttpConfigHttpEnabledStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MqttConfigMqttEnabledStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If enabled, allows connections using the MQTT protocol. Otherwise, MQTT connections to this registry will fail.
pub enum MqttConfigMqttEnabledStateEnum {
    

    /// No MQTT state specified. If not specified, MQTT will be enabled by default.
    ///
    /// "MQTT_STATE_UNSPECIFIED"
    #[serde(rename="MQTT_STATE_UNSPECIFIED")]
    MQTTSTATEUNSPECIFIED,
    

    /// Enables a MQTT connection.
    ///
    /// "MQTT_ENABLED"
    #[serde(rename="MQTT_ENABLED")]
    MQTTENABLED,
    

    /// Disables a MQTT connection.
    ///
    /// "MQTT_DISABLED"
    #[serde(rename="MQTT_DISABLED")]
    MQTTDISABLED,
}

impl AsRef<str> for MqttConfigMqttEnabledStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MqttConfigMqttEnabledStateEnum::MQTTSTATEUNSPECIFIED => "MQTT_STATE_UNSPECIFIED",
            MqttConfigMqttEnabledStateEnum::MQTTENABLED => "MQTT_ENABLED",
            MqttConfigMqttEnabledStateEnum::MQTTDISABLED => "MQTT_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for MqttConfigMqttEnabledStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MQTT_STATE_UNSPECIFIED" => Ok(MqttConfigMqttEnabledStateEnum::MQTTSTATEUNSPECIFIED),
           "MQTT_ENABLED" => Ok(MqttConfigMqttEnabledStateEnum::MQTTENABLED),
           "MQTT_DISABLED" => Ok(MqttConfigMqttEnabledStateEnum::MQTTDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MqttConfigMqttEnabledStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublicKeyCertificateFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The certificate format.
pub enum PublicKeyCertificateFormatEnum {
    

    /// The format has not been specified. This is an invalid default value and must not be used.
    ///
    /// "UNSPECIFIED_PUBLIC_KEY_CERTIFICATE_FORMAT"
    #[serde(rename="UNSPECIFIED_PUBLIC_KEY_CERTIFICATE_FORMAT")]
    UNSPECIFIEDPUBLICKEYCERTIFICATEFORMAT,
    

    /// An X.509v3 certificate ([RFC5280](https://www.ietf.org/rfc/rfc5280.txt)), encoded in base64, and wrapped by `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----`.
    ///
    /// "X509_CERTIFICATE_PEM"
    #[serde(rename="X509_CERTIFICATE_PEM")]
    X509CERTIFICATEPEM,
}

impl AsRef<str> for PublicKeyCertificateFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublicKeyCertificateFormatEnum::UNSPECIFIEDPUBLICKEYCERTIFICATEFORMAT => "UNSPECIFIED_PUBLIC_KEY_CERTIFICATE_FORMAT",
            PublicKeyCertificateFormatEnum::X509CERTIFICATEPEM => "X509_CERTIFICATE_PEM",
        }
    }
}

impl std::convert::TryFrom< &str> for PublicKeyCertificateFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_PUBLIC_KEY_CERTIFICATE_FORMAT" => Ok(PublicKeyCertificateFormatEnum::UNSPECIFIEDPUBLICKEYCERTIFICATEFORMAT),
           "X509_CERTIFICATE_PEM" => Ok(PublicKeyCertificateFormatEnum::X509CERTIFICATEPEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublicKeyCertificateFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublicKeyCredentialFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The format of the key.
pub enum PublicKeyCredentialFormatEnum {
    

    /// The format has not been specified. This is an invalid default value and must not be used.
    ///
    /// "UNSPECIFIED_PUBLIC_KEY_FORMAT"
    #[serde(rename="UNSPECIFIED_PUBLIC_KEY_FORMAT")]
    UNSPECIFIEDPUBLICKEYFORMAT,
    

    /// An RSA public key encoded in base64, and wrapped by `-----BEGIN PUBLIC KEY-----` and `-----END PUBLIC KEY-----`. This can be used to verify `RS256` signatures in JWT tokens ([RFC7518]( https://www.ietf.org/rfc/rfc7518.txt)).
    ///
    /// "RSA_PEM"
    #[serde(rename="RSA_PEM")]
    RSAPEM,
    

    /// As RSA_PEM, but wrapped in an X.509v3 certificate ([RFC5280]( https://www.ietf.org/rfc/rfc5280.txt)), encoded in base64, and wrapped by `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----`.
    ///
    /// "RSA_X509_PEM"
    #[serde(rename="RSA_X509_PEM")]
    RSAX509PEM,
    

    /// Public key for the ECDSA algorithm using P-256 and SHA-256, encoded in base64, and wrapped by `-----BEGIN PUBLIC KEY-----` and `-----END PUBLIC KEY-----`. This can be used to verify JWT tokens with the `ES256` algorithm ([RFC7518](https://www.ietf.org/rfc/rfc7518.txt)). This curve is defined in [OpenSSL](https://www.openssl.org/) as the `prime256v1` curve.
    ///
    /// "ES256_PEM"
    #[serde(rename="ES256_PEM")]
    ES256PEM,
    

    /// As ES256_PEM, but wrapped in an X.509v3 certificate ([RFC5280]( https://www.ietf.org/rfc/rfc5280.txt)), encoded in base64, and wrapped by `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----`.
    ///
    /// "ES256_X509_PEM"
    #[serde(rename="ES256_X509_PEM")]
    ES256X509PEM,
}

impl AsRef<str> for PublicKeyCredentialFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublicKeyCredentialFormatEnum::UNSPECIFIEDPUBLICKEYFORMAT => "UNSPECIFIED_PUBLIC_KEY_FORMAT",
            PublicKeyCredentialFormatEnum::RSAPEM => "RSA_PEM",
            PublicKeyCredentialFormatEnum::RSAX509PEM => "RSA_X509_PEM",
            PublicKeyCredentialFormatEnum::ES256PEM => "ES256_PEM",
            PublicKeyCredentialFormatEnum::ES256X509PEM => "ES256_X509_PEM",
        }
    }
}

impl std::convert::TryFrom< &str> for PublicKeyCredentialFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_PUBLIC_KEY_FORMAT" => Ok(PublicKeyCredentialFormatEnum::UNSPECIFIEDPUBLICKEYFORMAT),
           "RSA_PEM" => Ok(PublicKeyCredentialFormatEnum::RSAPEM),
           "RSA_X509_PEM" => Ok(PublicKeyCredentialFormatEnum::RSAX509PEM),
           "ES256_PEM" => Ok(PublicKeyCredentialFormatEnum::ES256PEM),
           "ES256_X509_PEM" => Ok(PublicKeyCredentialFormatEnum::ES256X509PEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublicKeyCredentialFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectGatewayListOptionsGatewayTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If `GATEWAY` is specified, only gateways are returned. If `NON_GATEWAY` is specified, only non-gateway devices are returned. If `GATEWAY_TYPE_UNSPECIFIED` is specified, all devices are returned.
pub enum ProjectGatewayListOptionsGatewayTypeEnum {
    

    /// If unspecified, the device is considered a non-gateway device.
    ///
    /// "GATEWAY_TYPE_UNSPECIFIED"
    #[serde(rename="GATEWAY_TYPE_UNSPECIFIED")]
    GATEWAYTYPEUNSPECIFIED,
    

    /// The device is a gateway.
    ///
    /// "GATEWAY"
    #[serde(rename="GATEWAY")]
    GATEWAY,
    

    /// The device is not a gateway.
    ///
    /// "NON_GATEWAY"
    #[serde(rename="NON_GATEWAY")]
    NONGATEWAY,
}

impl AsRef<str> for ProjectGatewayListOptionsGatewayTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectGatewayListOptionsGatewayTypeEnum::GATEWAYTYPEUNSPECIFIED => "GATEWAY_TYPE_UNSPECIFIED",
            ProjectGatewayListOptionsGatewayTypeEnum::GATEWAY => "GATEWAY",
            ProjectGatewayListOptionsGatewayTypeEnum::NONGATEWAY => "NON_GATEWAY",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectGatewayListOptionsGatewayTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GATEWAY_TYPE_UNSPECIFIED" => Ok(ProjectGatewayListOptionsGatewayTypeEnum::GATEWAYTYPEUNSPECIFIED),
           "GATEWAY" => Ok(ProjectGatewayListOptionsGatewayTypeEnum::GATEWAY),
           "NON_GATEWAY" => Ok(ProjectGatewayListOptionsGatewayTypeEnum::NONGATEWAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectGatewayListOptionsGatewayTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


