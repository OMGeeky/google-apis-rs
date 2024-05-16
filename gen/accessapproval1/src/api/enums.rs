use super::*;



// region AccessReasonTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of access justification.
pub enum AccessReasonTypeEnum {
    

    /// Default value for proto, shouldn't be used.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Customer made a request or raised an issue that required the principal to access customer data. `detail` is of the form ("#####" is the issue ID): * "Feedback Report: #####" * "Case Number: #####" * "Case ID: #####" * "E-PIN Reference: #####" * "Google-#####" * "T-#####"
    ///
    /// "CUSTOMER_INITIATED_SUPPORT"
    #[serde(rename="CUSTOMER_INITIATED_SUPPORT")]
    CUSTOMERINITIATEDSUPPORT,
    

    /// The principal accessed customer data in order to diagnose or resolve a suspected issue in services. Often this access is used to confirm that customers are not affected by a suspected service issue or to remediate a reversible system issue.
    ///
    /// "GOOGLE_INITIATED_SERVICE"
    #[serde(rename="GOOGLE_INITIATED_SERVICE")]
    GOOGLEINITIATEDSERVICE,
    

    /// Google initiated service for security, fraud, abuse, or compliance purposes.
    ///
    /// "GOOGLE_INITIATED_REVIEW"
    #[serde(rename="GOOGLE_INITIATED_REVIEW")]
    GOOGLEINITIATEDREVIEW,
    

    /// The principal was compelled to access customer data in order to respond to a legal third party data request or process, including legal processes from customers themselves.
    ///
    /// "THIRD_PARTY_DATA_REQUEST"
    #[serde(rename="THIRD_PARTY_DATA_REQUEST")]
    THIRDPARTYDATAREQUEST,
    

    /// The principal accessed customer data in order to diagnose or resolve a suspected issue in services or a known outage.
    ///
    /// "GOOGLE_RESPONSE_TO_PRODUCTION_ALERT"
    #[serde(rename="GOOGLE_RESPONSE_TO_PRODUCTION_ALERT")]
    GOOGLERESPONSETOPRODUCTIONALERT,
    

    /// Similar to 'GOOGLE_INITIATED_SERVICE' or 'GOOGLE_INITIATED_REVIEW', but with universe agnostic naming. The principal accessed customer data in order to diagnose or resolve a suspected issue in services or a known outage, or for security, fraud, abuse, or compliance review purposes.
    ///
    /// "CLOUD_INITIATED_ACCESS"
    #[serde(rename="CLOUD_INITIATED_ACCESS")]
    CLOUDINITIATEDACCESS,
}

impl AsRef<str> for AccessReasonTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccessReasonTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            AccessReasonTypeEnum::CUSTOMERINITIATEDSUPPORT => "CUSTOMER_INITIATED_SUPPORT",
            AccessReasonTypeEnum::GOOGLEINITIATEDSERVICE => "GOOGLE_INITIATED_SERVICE",
            AccessReasonTypeEnum::GOOGLEINITIATEDREVIEW => "GOOGLE_INITIATED_REVIEW",
            AccessReasonTypeEnum::THIRDPARTYDATAREQUEST => "THIRD_PARTY_DATA_REQUEST",
            AccessReasonTypeEnum::GOOGLERESPONSETOPRODUCTIONALERT => "GOOGLE_RESPONSE_TO_PRODUCTION_ALERT",
            AccessReasonTypeEnum::CLOUDINITIATEDACCESS => "CLOUD_INITIATED_ACCESS",
        }
    }
}

impl std::convert::TryFrom< &str> for AccessReasonTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(AccessReasonTypeEnum::TYPEUNSPECIFIED),
           "CUSTOMER_INITIATED_SUPPORT" => Ok(AccessReasonTypeEnum::CUSTOMERINITIATEDSUPPORT),
           "GOOGLE_INITIATED_SERVICE" => Ok(AccessReasonTypeEnum::GOOGLEINITIATEDSERVICE),
           "GOOGLE_INITIATED_REVIEW" => Ok(AccessReasonTypeEnum::GOOGLEINITIATEDREVIEW),
           "THIRD_PARTY_DATA_REQUEST" => Ok(AccessReasonTypeEnum::THIRDPARTYDATAREQUEST),
           "GOOGLE_RESPONSE_TO_PRODUCTION_ALERT" => Ok(AccessReasonTypeEnum::GOOGLERESPONSETOPRODUCTIONALERT),
           "CLOUD_INITIATED_ACCESS" => Ok(AccessReasonTypeEnum::CLOUDINITIATEDACCESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccessReasonTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnrolledServiceEnrollmentLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The enrollment level of the service.
pub enum EnrolledServiceEnrollmentLevelEnum {
    

    /// Default value for proto, shouldn't be used.
    ///
    /// "ENROLLMENT_LEVEL_UNSPECIFIED"
    #[serde(rename="ENROLLMENT_LEVEL_UNSPECIFIED")]
    ENROLLMENTLEVELUNSPECIFIED,
    

    /// Service is enrolled in Access Approval for all requests
    ///
    /// "BLOCK_ALL"
    #[serde(rename="BLOCK_ALL")]
    BLOCKALL,
}

impl AsRef<str> for EnrolledServiceEnrollmentLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnrolledServiceEnrollmentLevelEnum::ENROLLMENTLEVELUNSPECIFIED => "ENROLLMENT_LEVEL_UNSPECIFIED",
            EnrolledServiceEnrollmentLevelEnum::BLOCKALL => "BLOCK_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for EnrolledServiceEnrollmentLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENROLLMENT_LEVEL_UNSPECIFIED" => Ok(EnrolledServiceEnrollmentLevelEnum::ENROLLMENTLEVELUNSPECIFIED),
           "BLOCK_ALL" => Ok(EnrolledServiceEnrollmentLevelEnum::BLOCKALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnrolledServiceEnrollmentLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SignatureInfoGoogleKeyAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The hashing algorithm used for signature verification. It will only be present in the case of Google managed keys.
pub enum SignatureInfoGoogleKeyAlgorithmEnum {
    

    /// Not specified.
    ///
    /// "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED"
    #[serde(rename="CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED")]
    CRYPTOKEYVERSIONALGORITHMUNSPECIFIED,
    

    /// Creates symmetric encryption keys.
    ///
    /// "GOOGLE_SYMMETRIC_ENCRYPTION"
    #[serde(rename="GOOGLE_SYMMETRIC_ENCRYPTION")]
    GOOGLESYMMETRICENCRYPTION,
    

    /// AES-GCM (Galois Counter Mode) using 128-bit keys.
    ///
    /// "AES_128_GCM"
    #[serde(rename="AES_128_GCM")]
    AES128GCM,
    

    /// AES-GCM (Galois Counter Mode) using 256-bit keys.
    ///
    /// "AES_256_GCM"
    #[serde(rename="AES_256_GCM")]
    AES256GCM,
    

    /// AES-CBC (Cipher Block Chaining Mode) using 128-bit keys.
    ///
    /// "AES_128_CBC"
    #[serde(rename="AES_128_CBC")]
    AES128CBC,
    

    /// AES-CBC (Cipher Block Chaining Mode) using 256-bit keys.
    ///
    /// "AES_256_CBC"
    #[serde(rename="AES_256_CBC")]
    AES256CBC,
    

    /// AES-CTR (Counter Mode) using 128-bit keys.
    ///
    /// "AES_128_CTR"
    #[serde(rename="AES_128_CTR")]
    AES128CTR,
    

    /// AES-CTR (Counter Mode) using 256-bit keys.
    ///
    /// "AES_256_CTR"
    #[serde(rename="AES_256_CTR")]
    AES256CTR,
    

    /// RSASSA-PSS 2048 bit key with a SHA256 digest.
    ///
    /// "RSA_SIGN_PSS_2048_SHA256"
    #[serde(rename="RSA_SIGN_PSS_2048_SHA256")]
    RSASIGNPSS2048SHA256,
    

    /// RSASSA-PSS 3072 bit key with a SHA256 digest.
    ///
    /// "RSA_SIGN_PSS_3072_SHA256"
    #[serde(rename="RSA_SIGN_PSS_3072_SHA256")]
    RSASIGNPSS3072SHA256,
    

    /// RSASSA-PSS 4096 bit key with a SHA256 digest.
    ///
    /// "RSA_SIGN_PSS_4096_SHA256"
    #[serde(rename="RSA_SIGN_PSS_4096_SHA256")]
    RSASIGNPSS4096SHA256,
    

    /// RSASSA-PSS 4096 bit key with a SHA512 digest.
    ///
    /// "RSA_SIGN_PSS_4096_SHA512"
    #[serde(rename="RSA_SIGN_PSS_4096_SHA512")]
    RSASIGNPSS4096SHA512,
    

    /// RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest.
    ///
    /// "RSA_SIGN_PKCS1_2048_SHA256"
    #[serde(rename="RSA_SIGN_PKCS1_2048_SHA256")]
    RSASIGNPKCS12048SHA256,
    

    /// RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest.
    ///
    /// "RSA_SIGN_PKCS1_3072_SHA256"
    #[serde(rename="RSA_SIGN_PKCS1_3072_SHA256")]
    RSASIGNPKCS13072SHA256,
    

    /// RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest.
    ///
    /// "RSA_SIGN_PKCS1_4096_SHA256"
    #[serde(rename="RSA_SIGN_PKCS1_4096_SHA256")]
    RSASIGNPKCS14096SHA256,
    

    /// RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest.
    ///
    /// "RSA_SIGN_PKCS1_4096_SHA512"
    #[serde(rename="RSA_SIGN_PKCS1_4096_SHA512")]
    RSASIGNPKCS14096SHA512,
    

    /// RSASSA-PKCS1-v1_5 signing without encoding, with a 2048 bit key.
    ///
    /// "RSA_SIGN_RAW_PKCS1_2048"
    #[serde(rename="RSA_SIGN_RAW_PKCS1_2048")]
    RSASIGNRAWPKCS12048,
    

    /// RSASSA-PKCS1-v1_5 signing without encoding, with a 3072 bit key.
    ///
    /// "RSA_SIGN_RAW_PKCS1_3072"
    #[serde(rename="RSA_SIGN_RAW_PKCS1_3072")]
    RSASIGNRAWPKCS13072,
    

    /// RSASSA-PKCS1-v1_5 signing without encoding, with a 4096 bit key.
    ///
    /// "RSA_SIGN_RAW_PKCS1_4096"
    #[serde(rename="RSA_SIGN_RAW_PKCS1_4096")]
    RSASIGNRAWPKCS14096,
    

    /// RSAES-OAEP 2048 bit key with a SHA256 digest.
    ///
    /// "RSA_DECRYPT_OAEP_2048_SHA256"
    #[serde(rename="RSA_DECRYPT_OAEP_2048_SHA256")]
    RSADECRYPTOAEP2048SHA256,
    

    /// RSAES-OAEP 3072 bit key with a SHA256 digest.
    ///
    /// "RSA_DECRYPT_OAEP_3072_SHA256"
    #[serde(rename="RSA_DECRYPT_OAEP_3072_SHA256")]
    RSADECRYPTOAEP3072SHA256,
    

    /// RSAES-OAEP 4096 bit key with a SHA256 digest.
    ///
    /// "RSA_DECRYPT_OAEP_4096_SHA256"
    #[serde(rename="RSA_DECRYPT_OAEP_4096_SHA256")]
    RSADECRYPTOAEP4096SHA256,
    

    /// RSAES-OAEP 4096 bit key with a SHA512 digest.
    ///
    /// "RSA_DECRYPT_OAEP_4096_SHA512"
    #[serde(rename="RSA_DECRYPT_OAEP_4096_SHA512")]
    RSADECRYPTOAEP4096SHA512,
    

    /// RSAES-OAEP 2048 bit key with a SHA1 digest.
    ///
    /// "RSA_DECRYPT_OAEP_2048_SHA1"
    #[serde(rename="RSA_DECRYPT_OAEP_2048_SHA1")]
    RSADECRYPTOAEP2048SHA1,
    

    /// RSAES-OAEP 3072 bit key with a SHA1 digest.
    ///
    /// "RSA_DECRYPT_OAEP_3072_SHA1"
    #[serde(rename="RSA_DECRYPT_OAEP_3072_SHA1")]
    RSADECRYPTOAEP3072SHA1,
    

    /// RSAES-OAEP 4096 bit key with a SHA1 digest.
    ///
    /// "RSA_DECRYPT_OAEP_4096_SHA1"
    #[serde(rename="RSA_DECRYPT_OAEP_4096_SHA1")]
    RSADECRYPTOAEP4096SHA1,
    

    /// ECDSA on the NIST P-256 curve with a SHA256 digest. Other hash functions can also be used: https://cloud.google.com/kms/docs/create-validate-signatures#ecdsa_support_for_other_hash_algorithms
    ///
    /// "EC_SIGN_P256_SHA256"
    #[serde(rename="EC_SIGN_P256_SHA256")]
    ECSIGNP256SHA256,
    

    /// ECDSA on the NIST P-384 curve with a SHA384 digest. Other hash functions can also be used: https://cloud.google.com/kms/docs/create-validate-signatures#ecdsa_support_for_other_hash_algorithms
    ///
    /// "EC_SIGN_P384_SHA384"
    #[serde(rename="EC_SIGN_P384_SHA384")]
    ECSIGNP384SHA384,
    

    /// ECDSA on the non-NIST secp256k1 curve. This curve is only supported for HSM protection level. Other hash functions can also be used: https://cloud.google.com/kms/docs/create-validate-signatures#ecdsa_support_for_other_hash_algorithms
    ///
    /// "EC_SIGN_SECP256K1_SHA256"
    #[serde(rename="EC_SIGN_SECP256K1_SHA256")]
    ECSIGNSECP256K1SHA256,
    

    /// HMAC-SHA256 signing with a 256 bit key.
    ///
    /// "HMAC_SHA256"
    #[serde(rename="HMAC_SHA256")]
    HMACSHA256,
    

    /// HMAC-SHA1 signing with a 160 bit key.
    ///
    /// "HMAC_SHA1"
    #[serde(rename="HMAC_SHA1")]
    HMACSHA1,
    

    /// HMAC-SHA384 signing with a 384 bit key.
    ///
    /// "HMAC_SHA384"
    #[serde(rename="HMAC_SHA384")]
    HMACSHA384,
    

    /// HMAC-SHA512 signing with a 512 bit key.
    ///
    /// "HMAC_SHA512"
    #[serde(rename="HMAC_SHA512")]
    HMACSHA512,
    

    /// HMAC-SHA224 signing with a 224 bit key.
    ///
    /// "HMAC_SHA224"
    #[serde(rename="HMAC_SHA224")]
    HMACSHA224,
    

    /// Algorithm representing symmetric encryption by an external key manager.
    ///
    /// "EXTERNAL_SYMMETRIC_ENCRYPTION"
    #[serde(rename="EXTERNAL_SYMMETRIC_ENCRYPTION")]
    EXTERNALSYMMETRICENCRYPTION,
}

impl AsRef<str> for SignatureInfoGoogleKeyAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SignatureInfoGoogleKeyAlgorithmEnum::CRYPTOKEYVERSIONALGORITHMUNSPECIFIED => "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED",
            SignatureInfoGoogleKeyAlgorithmEnum::GOOGLESYMMETRICENCRYPTION => "GOOGLE_SYMMETRIC_ENCRYPTION",
            SignatureInfoGoogleKeyAlgorithmEnum::AES128GCM => "AES_128_GCM",
            SignatureInfoGoogleKeyAlgorithmEnum::AES256GCM => "AES_256_GCM",
            SignatureInfoGoogleKeyAlgorithmEnum::AES128CBC => "AES_128_CBC",
            SignatureInfoGoogleKeyAlgorithmEnum::AES256CBC => "AES_256_CBC",
            SignatureInfoGoogleKeyAlgorithmEnum::AES128CTR => "AES_128_CTR",
            SignatureInfoGoogleKeyAlgorithmEnum::AES256CTR => "AES_256_CTR",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPSS2048SHA256 => "RSA_SIGN_PSS_2048_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPSS3072SHA256 => "RSA_SIGN_PSS_3072_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPSS4096SHA256 => "RSA_SIGN_PSS_4096_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPSS4096SHA512 => "RSA_SIGN_PSS_4096_SHA512",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPKCS12048SHA256 => "RSA_SIGN_PKCS1_2048_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPKCS13072SHA256 => "RSA_SIGN_PKCS1_3072_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPKCS14096SHA256 => "RSA_SIGN_PKCS1_4096_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPKCS14096SHA512 => "RSA_SIGN_PKCS1_4096_SHA512",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNRAWPKCS12048 => "RSA_SIGN_RAW_PKCS1_2048",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNRAWPKCS13072 => "RSA_SIGN_RAW_PKCS1_3072",
            SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNRAWPKCS14096 => "RSA_SIGN_RAW_PKCS1_4096",
            SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP2048SHA256 => "RSA_DECRYPT_OAEP_2048_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP3072SHA256 => "RSA_DECRYPT_OAEP_3072_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA256 => "RSA_DECRYPT_OAEP_4096_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA512 => "RSA_DECRYPT_OAEP_4096_SHA512",
            SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP2048SHA1 => "RSA_DECRYPT_OAEP_2048_SHA1",
            SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP3072SHA1 => "RSA_DECRYPT_OAEP_3072_SHA1",
            SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA1 => "RSA_DECRYPT_OAEP_4096_SHA1",
            SignatureInfoGoogleKeyAlgorithmEnum::ECSIGNP256SHA256 => "EC_SIGN_P256_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::ECSIGNP384SHA384 => "EC_SIGN_P384_SHA384",
            SignatureInfoGoogleKeyAlgorithmEnum::ECSIGNSECP256K1SHA256 => "EC_SIGN_SECP256K1_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::HMACSHA256 => "HMAC_SHA256",
            SignatureInfoGoogleKeyAlgorithmEnum::HMACSHA1 => "HMAC_SHA1",
            SignatureInfoGoogleKeyAlgorithmEnum::HMACSHA384 => "HMAC_SHA384",
            SignatureInfoGoogleKeyAlgorithmEnum::HMACSHA512 => "HMAC_SHA512",
            SignatureInfoGoogleKeyAlgorithmEnum::HMACSHA224 => "HMAC_SHA224",
            SignatureInfoGoogleKeyAlgorithmEnum::EXTERNALSYMMETRICENCRYPTION => "EXTERNAL_SYMMETRIC_ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for SignatureInfoGoogleKeyAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::CRYPTOKEYVERSIONALGORITHMUNSPECIFIED),
           "GOOGLE_SYMMETRIC_ENCRYPTION" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::GOOGLESYMMETRICENCRYPTION),
           "AES_128_GCM" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::AES128GCM),
           "AES_256_GCM" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::AES256GCM),
           "AES_128_CBC" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::AES128CBC),
           "AES_256_CBC" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::AES256CBC),
           "AES_128_CTR" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::AES128CTR),
           "AES_256_CTR" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::AES256CTR),
           "RSA_SIGN_PSS_2048_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPSS2048SHA256),
           "RSA_SIGN_PSS_3072_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPSS3072SHA256),
           "RSA_SIGN_PSS_4096_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPSS4096SHA256),
           "RSA_SIGN_PSS_4096_SHA512" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPSS4096SHA512),
           "RSA_SIGN_PKCS1_2048_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPKCS12048SHA256),
           "RSA_SIGN_PKCS1_3072_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPKCS13072SHA256),
           "RSA_SIGN_PKCS1_4096_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPKCS14096SHA256),
           "RSA_SIGN_PKCS1_4096_SHA512" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNPKCS14096SHA512),
           "RSA_SIGN_RAW_PKCS1_2048" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNRAWPKCS12048),
           "RSA_SIGN_RAW_PKCS1_3072" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNRAWPKCS13072),
           "RSA_SIGN_RAW_PKCS1_4096" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSASIGNRAWPKCS14096),
           "RSA_DECRYPT_OAEP_2048_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP2048SHA256),
           "RSA_DECRYPT_OAEP_3072_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP3072SHA256),
           "RSA_DECRYPT_OAEP_4096_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA256),
           "RSA_DECRYPT_OAEP_4096_SHA512" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA512),
           "RSA_DECRYPT_OAEP_2048_SHA1" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP2048SHA1),
           "RSA_DECRYPT_OAEP_3072_SHA1" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP3072SHA1),
           "RSA_DECRYPT_OAEP_4096_SHA1" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA1),
           "EC_SIGN_P256_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::ECSIGNP256SHA256),
           "EC_SIGN_P384_SHA384" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::ECSIGNP384SHA384),
           "EC_SIGN_SECP256K1_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::ECSIGNSECP256K1SHA256),
           "HMAC_SHA256" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::HMACSHA256),
           "HMAC_SHA1" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::HMACSHA1),
           "HMAC_SHA384" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::HMACSHA384),
           "HMAC_SHA512" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::HMACSHA512),
           "HMAC_SHA224" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::HMACSHA224),
           "EXTERNAL_SYMMETRIC_ENCRYPTION" => Ok(SignatureInfoGoogleKeyAlgorithmEnum::EXTERNALSYMMETRICENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SignatureInfoGoogleKeyAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


