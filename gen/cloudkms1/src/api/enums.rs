use super::*;



// region AsymmetricDecryptResponseProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ProtectionLevel of the CryptoKeyVersion used in decryption.
pub enum AsymmetricDecryptResponseProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for AsymmetricDecryptResponseProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AsymmetricDecryptResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            AsymmetricDecryptResponseProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            AsymmetricDecryptResponseProtectionLevelEnum::HSM => "HSM",
            AsymmetricDecryptResponseProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            AsymmetricDecryptResponseProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for AsymmetricDecryptResponseProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(AsymmetricDecryptResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(AsymmetricDecryptResponseProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(AsymmetricDecryptResponseProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(AsymmetricDecryptResponseProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(AsymmetricDecryptResponseProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AsymmetricDecryptResponseProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AsymmetricSignResponseProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ProtectionLevel of the CryptoKeyVersion used for signing.
pub enum AsymmetricSignResponseProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for AsymmetricSignResponseProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AsymmetricSignResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            AsymmetricSignResponseProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            AsymmetricSignResponseProtectionLevelEnum::HSM => "HSM",
            AsymmetricSignResponseProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            AsymmetricSignResponseProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for AsymmetricSignResponseProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(AsymmetricSignResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(AsymmetricSignResponseProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(AsymmetricSignResponseProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(AsymmetricSignResponseProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(AsymmetricSignResponseProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AsymmetricSignResponseProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuditLogConfigLogTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log type that this config enables.
pub enum AuditLogConfigLogTypeEnum {
    

    /// Default case. Should never be this.
    ///
    /// "LOG_TYPE_UNSPECIFIED"
    #[serde(rename="LOG_TYPE_UNSPECIFIED")]
    LOGTYPEUNSPECIFIED,
    

    /// Admin reads. Example: CloudIAM getIamPolicy
    ///
    /// "ADMIN_READ"
    #[serde(rename="ADMIN_READ")]
    ADMINREAD,
    

    /// Data writes. Example: CloudSQL Users create
    ///
    /// "DATA_WRITE"
    #[serde(rename="DATA_WRITE")]
    DATAWRITE,
    

    /// Data reads. Example: CloudSQL Users list
    ///
    /// "DATA_READ"
    #[serde(rename="DATA_READ")]
    DATAREAD,
}

impl AsRef<str> for AuditLogConfigLogTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            AuditLogConfigLogTypeEnum::ADMINREAD => "ADMIN_READ",
            AuditLogConfigLogTypeEnum::DATAWRITE => "DATA_WRITE",
            AuditLogConfigLogTypeEnum::DATAREAD => "DATA_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for AuditLogConfigLogTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(AuditLogConfigLogTypeEnum::ADMINREAD),
           "DATA_WRITE" => Ok(AuditLogConfigLogTypeEnum::DATAWRITE),
           "DATA_READ" => Ok(AuditLogConfigLogTypeEnum::DATAREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuditLogConfigLogTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CryptoKeyPurposeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The immutable purpose of this CryptoKey.
pub enum CryptoKeyPurposeEnum {
    

    /// Not specified.
    ///
    /// "CRYPTO_KEY_PURPOSE_UNSPECIFIED"
    #[serde(rename="CRYPTO_KEY_PURPOSE_UNSPECIFIED")]
    CRYPTOKEYPURPOSEUNSPECIFIED,
    

    /// CryptoKeys with this purpose may be used with Encrypt and Decrypt.
    ///
    /// "ENCRYPT_DECRYPT"
    #[serde(rename="ENCRYPT_DECRYPT")]
    ENCRYPTDECRYPT,
    

    /// CryptoKeys with this purpose may be used with AsymmetricSign and GetPublicKey.
    ///
    /// "ASYMMETRIC_SIGN"
    #[serde(rename="ASYMMETRIC_SIGN")]
    ASYMMETRICSIGN,
    

    /// CryptoKeys with this purpose may be used with AsymmetricDecrypt and GetPublicKey.
    ///
    /// "ASYMMETRIC_DECRYPT"
    #[serde(rename="ASYMMETRIC_DECRYPT")]
    ASYMMETRICDECRYPT,
    

    /// CryptoKeys with this purpose may be used with RawEncrypt and RawDecrypt. This purpose is meant to be used for interoperable symmetric encryption and does not support automatic CryptoKey rotation.
    ///
    /// "RAW_ENCRYPT_DECRYPT"
    #[serde(rename="RAW_ENCRYPT_DECRYPT")]
    RAWENCRYPTDECRYPT,
    

    /// CryptoKeys with this purpose may be used with MacSign.
    ///
    /// "MAC"
    #[serde(rename="MAC")]
    MAC,
}

impl AsRef<str> for CryptoKeyPurposeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CryptoKeyPurposeEnum::CRYPTOKEYPURPOSEUNSPECIFIED => "CRYPTO_KEY_PURPOSE_UNSPECIFIED",
            CryptoKeyPurposeEnum::ENCRYPTDECRYPT => "ENCRYPT_DECRYPT",
            CryptoKeyPurposeEnum::ASYMMETRICSIGN => "ASYMMETRIC_SIGN",
            CryptoKeyPurposeEnum::ASYMMETRICDECRYPT => "ASYMMETRIC_DECRYPT",
            CryptoKeyPurposeEnum::RAWENCRYPTDECRYPT => "RAW_ENCRYPT_DECRYPT",
            CryptoKeyPurposeEnum::MAC => "MAC",
        }
    }
}

impl std::convert::TryFrom< &str> for CryptoKeyPurposeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_PURPOSE_UNSPECIFIED" => Ok(CryptoKeyPurposeEnum::CRYPTOKEYPURPOSEUNSPECIFIED),
           "ENCRYPT_DECRYPT" => Ok(CryptoKeyPurposeEnum::ENCRYPTDECRYPT),
           "ASYMMETRIC_SIGN" => Ok(CryptoKeyPurposeEnum::ASYMMETRICSIGN),
           "ASYMMETRIC_DECRYPT" => Ok(CryptoKeyPurposeEnum::ASYMMETRICDECRYPT),
           "RAW_ENCRYPT_DECRYPT" => Ok(CryptoKeyPurposeEnum::RAWENCRYPTDECRYPT),
           "MAC" => Ok(CryptoKeyPurposeEnum::MAC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CryptoKeyPurposeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CryptoKeyVersionAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports.
pub enum CryptoKeyVersionAlgorithmEnum {
    

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

impl AsRef<str> for CryptoKeyVersionAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CryptoKeyVersionAlgorithmEnum::CRYPTOKEYVERSIONALGORITHMUNSPECIFIED => "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED",
            CryptoKeyVersionAlgorithmEnum::GOOGLESYMMETRICENCRYPTION => "GOOGLE_SYMMETRIC_ENCRYPTION",
            CryptoKeyVersionAlgorithmEnum::AES128GCM => "AES_128_GCM",
            CryptoKeyVersionAlgorithmEnum::AES256GCM => "AES_256_GCM",
            CryptoKeyVersionAlgorithmEnum::AES128CBC => "AES_128_CBC",
            CryptoKeyVersionAlgorithmEnum::AES256CBC => "AES_256_CBC",
            CryptoKeyVersionAlgorithmEnum::AES128CTR => "AES_128_CTR",
            CryptoKeyVersionAlgorithmEnum::AES256CTR => "AES_256_CTR",
            CryptoKeyVersionAlgorithmEnum::RSASIGNPSS2048SHA256 => "RSA_SIGN_PSS_2048_SHA256",
            CryptoKeyVersionAlgorithmEnum::RSASIGNPSS3072SHA256 => "RSA_SIGN_PSS_3072_SHA256",
            CryptoKeyVersionAlgorithmEnum::RSASIGNPSS4096SHA256 => "RSA_SIGN_PSS_4096_SHA256",
            CryptoKeyVersionAlgorithmEnum::RSASIGNPSS4096SHA512 => "RSA_SIGN_PSS_4096_SHA512",
            CryptoKeyVersionAlgorithmEnum::RSASIGNPKCS12048SHA256 => "RSA_SIGN_PKCS1_2048_SHA256",
            CryptoKeyVersionAlgorithmEnum::RSASIGNPKCS13072SHA256 => "RSA_SIGN_PKCS1_3072_SHA256",
            CryptoKeyVersionAlgorithmEnum::RSASIGNPKCS14096SHA256 => "RSA_SIGN_PKCS1_4096_SHA256",
            CryptoKeyVersionAlgorithmEnum::RSASIGNPKCS14096SHA512 => "RSA_SIGN_PKCS1_4096_SHA512",
            CryptoKeyVersionAlgorithmEnum::RSASIGNRAWPKCS12048 => "RSA_SIGN_RAW_PKCS1_2048",
            CryptoKeyVersionAlgorithmEnum::RSASIGNRAWPKCS13072 => "RSA_SIGN_RAW_PKCS1_3072",
            CryptoKeyVersionAlgorithmEnum::RSASIGNRAWPKCS14096 => "RSA_SIGN_RAW_PKCS1_4096",
            CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP2048SHA256 => "RSA_DECRYPT_OAEP_2048_SHA256",
            CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP3072SHA256 => "RSA_DECRYPT_OAEP_3072_SHA256",
            CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP4096SHA256 => "RSA_DECRYPT_OAEP_4096_SHA256",
            CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP4096SHA512 => "RSA_DECRYPT_OAEP_4096_SHA512",
            CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP2048SHA1 => "RSA_DECRYPT_OAEP_2048_SHA1",
            CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP3072SHA1 => "RSA_DECRYPT_OAEP_3072_SHA1",
            CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP4096SHA1 => "RSA_DECRYPT_OAEP_4096_SHA1",
            CryptoKeyVersionAlgorithmEnum::ECSIGNP256SHA256 => "EC_SIGN_P256_SHA256",
            CryptoKeyVersionAlgorithmEnum::ECSIGNP384SHA384 => "EC_SIGN_P384_SHA384",
            CryptoKeyVersionAlgorithmEnum::ECSIGNSECP256K1SHA256 => "EC_SIGN_SECP256K1_SHA256",
            CryptoKeyVersionAlgorithmEnum::HMACSHA256 => "HMAC_SHA256",
            CryptoKeyVersionAlgorithmEnum::HMACSHA1 => "HMAC_SHA1",
            CryptoKeyVersionAlgorithmEnum::HMACSHA384 => "HMAC_SHA384",
            CryptoKeyVersionAlgorithmEnum::HMACSHA512 => "HMAC_SHA512",
            CryptoKeyVersionAlgorithmEnum::HMACSHA224 => "HMAC_SHA224",
            CryptoKeyVersionAlgorithmEnum::EXTERNALSYMMETRICENCRYPTION => "EXTERNAL_SYMMETRIC_ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for CryptoKeyVersionAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => Ok(CryptoKeyVersionAlgorithmEnum::CRYPTOKEYVERSIONALGORITHMUNSPECIFIED),
           "GOOGLE_SYMMETRIC_ENCRYPTION" => Ok(CryptoKeyVersionAlgorithmEnum::GOOGLESYMMETRICENCRYPTION),
           "AES_128_GCM" => Ok(CryptoKeyVersionAlgorithmEnum::AES128GCM),
           "AES_256_GCM" => Ok(CryptoKeyVersionAlgorithmEnum::AES256GCM),
           "AES_128_CBC" => Ok(CryptoKeyVersionAlgorithmEnum::AES128CBC),
           "AES_256_CBC" => Ok(CryptoKeyVersionAlgorithmEnum::AES256CBC),
           "AES_128_CTR" => Ok(CryptoKeyVersionAlgorithmEnum::AES128CTR),
           "AES_256_CTR" => Ok(CryptoKeyVersionAlgorithmEnum::AES256CTR),
           "RSA_SIGN_PSS_2048_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNPSS2048SHA256),
           "RSA_SIGN_PSS_3072_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNPSS3072SHA256),
           "RSA_SIGN_PSS_4096_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNPSS4096SHA256),
           "RSA_SIGN_PSS_4096_SHA512" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNPSS4096SHA512),
           "RSA_SIGN_PKCS1_2048_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNPKCS12048SHA256),
           "RSA_SIGN_PKCS1_3072_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNPKCS13072SHA256),
           "RSA_SIGN_PKCS1_4096_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNPKCS14096SHA256),
           "RSA_SIGN_PKCS1_4096_SHA512" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNPKCS14096SHA512),
           "RSA_SIGN_RAW_PKCS1_2048" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNRAWPKCS12048),
           "RSA_SIGN_RAW_PKCS1_3072" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNRAWPKCS13072),
           "RSA_SIGN_RAW_PKCS1_4096" => Ok(CryptoKeyVersionAlgorithmEnum::RSASIGNRAWPKCS14096),
           "RSA_DECRYPT_OAEP_2048_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP2048SHA256),
           "RSA_DECRYPT_OAEP_3072_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP3072SHA256),
           "RSA_DECRYPT_OAEP_4096_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP4096SHA256),
           "RSA_DECRYPT_OAEP_4096_SHA512" => Ok(CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP4096SHA512),
           "RSA_DECRYPT_OAEP_2048_SHA1" => Ok(CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP2048SHA1),
           "RSA_DECRYPT_OAEP_3072_SHA1" => Ok(CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP3072SHA1),
           "RSA_DECRYPT_OAEP_4096_SHA1" => Ok(CryptoKeyVersionAlgorithmEnum::RSADECRYPTOAEP4096SHA1),
           "EC_SIGN_P256_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::ECSIGNP256SHA256),
           "EC_SIGN_P384_SHA384" => Ok(CryptoKeyVersionAlgorithmEnum::ECSIGNP384SHA384),
           "EC_SIGN_SECP256K1_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::ECSIGNSECP256K1SHA256),
           "HMAC_SHA256" => Ok(CryptoKeyVersionAlgorithmEnum::HMACSHA256),
           "HMAC_SHA1" => Ok(CryptoKeyVersionAlgorithmEnum::HMACSHA1),
           "HMAC_SHA384" => Ok(CryptoKeyVersionAlgorithmEnum::HMACSHA384),
           "HMAC_SHA512" => Ok(CryptoKeyVersionAlgorithmEnum::HMACSHA512),
           "HMAC_SHA224" => Ok(CryptoKeyVersionAlgorithmEnum::HMACSHA224),
           "EXTERNAL_SYMMETRIC_ENCRYPTION" => Ok(CryptoKeyVersionAlgorithmEnum::EXTERNALSYMMETRICENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CryptoKeyVersionAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CryptoKeyVersionProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion.
pub enum CryptoKeyVersionProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for CryptoKeyVersionProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CryptoKeyVersionProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            CryptoKeyVersionProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            CryptoKeyVersionProtectionLevelEnum::HSM => "HSM",
            CryptoKeyVersionProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            CryptoKeyVersionProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for CryptoKeyVersionProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(CryptoKeyVersionProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(CryptoKeyVersionProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(CryptoKeyVersionProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(CryptoKeyVersionProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(CryptoKeyVersionProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CryptoKeyVersionProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CryptoKeyVersionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the CryptoKeyVersion.
pub enum CryptoKeyVersionStateEnum {
    

    /// Not specified.
    ///
    /// "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED"
    #[serde(rename="CRYPTO_KEY_VERSION_STATE_UNSPECIFIED")]
    CRYPTOKEYVERSIONSTATEUNSPECIFIED,
    

    /// This version is still being generated. It may not be used, enabled, disabled, or destroyed yet. Cloud KMS will automatically mark this version ENABLED as soon as the version is ready.
    ///
    /// "PENDING_GENERATION"
    #[serde(rename="PENDING_GENERATION")]
    PENDINGGENERATION,
    

    /// This version may be used for cryptographic operations.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// This version may not be used, but the key material is still available, and the version can be placed back into the ENABLED state.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// This version is destroyed, and the key material is no longer stored. This version may only become ENABLED again if this version is reimport_eligible and the original key material is reimported with a call to KeyManagementService.ImportCryptoKeyVersion.
    ///
    /// "DESTROYED"
    #[serde(rename="DESTROYED")]
    DESTROYED,
    

    /// This version is scheduled for destruction, and will be destroyed soon. Call RestoreCryptoKeyVersion to put it back into the DISABLED state.
    ///
    /// "DESTROY_SCHEDULED"
    #[serde(rename="DESTROY_SCHEDULED")]
    DESTROYSCHEDULED,
    

    /// This version is still being imported. It may not be used, enabled, disabled, or destroyed yet. Cloud KMS will automatically mark this version ENABLED as soon as the version is ready.
    ///
    /// "PENDING_IMPORT"
    #[serde(rename="PENDING_IMPORT")]
    PENDINGIMPORT,
    

    /// This version was not imported successfully. It may not be used, enabled, disabled, or destroyed. The submitted key material has been discarded. Additional details can be found in CryptoKeyVersion.import_failure_reason.
    ///
    /// "IMPORT_FAILED"
    #[serde(rename="IMPORT_FAILED")]
    IMPORTFAILED,
    

    /// This version was not generated successfully. It may not be used, enabled, disabled, or destroyed. Additional details can be found in CryptoKeyVersion.generation_failure_reason.
    ///
    /// "GENERATION_FAILED"
    #[serde(rename="GENERATION_FAILED")]
    GENERATIONFAILED,
    

    /// This version was destroyed, and it may not be used or enabled again. Cloud KMS is waiting for the corresponding key material residing in an external key manager to be destroyed.
    ///
    /// "PENDING_EXTERNAL_DESTRUCTION"
    #[serde(rename="PENDING_EXTERNAL_DESTRUCTION")]
    PENDINGEXTERNALDESTRUCTION,
    

    /// This version was destroyed, and it may not be used or enabled again. However, Cloud KMS could not confirm that the corresponding key material residing in an external key manager was destroyed. Additional details can be found in CryptoKeyVersion.external_destruction_failure_reason.
    ///
    /// "EXTERNAL_DESTRUCTION_FAILED"
    #[serde(rename="EXTERNAL_DESTRUCTION_FAILED")]
    EXTERNALDESTRUCTIONFAILED,
}

impl AsRef<str> for CryptoKeyVersionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CryptoKeyVersionStateEnum::CRYPTOKEYVERSIONSTATEUNSPECIFIED => "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED",
            CryptoKeyVersionStateEnum::PENDINGGENERATION => "PENDING_GENERATION",
            CryptoKeyVersionStateEnum::ENABLED => "ENABLED",
            CryptoKeyVersionStateEnum::DISABLED => "DISABLED",
            CryptoKeyVersionStateEnum::DESTROYED => "DESTROYED",
            CryptoKeyVersionStateEnum::DESTROYSCHEDULED => "DESTROY_SCHEDULED",
            CryptoKeyVersionStateEnum::PENDINGIMPORT => "PENDING_IMPORT",
            CryptoKeyVersionStateEnum::IMPORTFAILED => "IMPORT_FAILED",
            CryptoKeyVersionStateEnum::GENERATIONFAILED => "GENERATION_FAILED",
            CryptoKeyVersionStateEnum::PENDINGEXTERNALDESTRUCTION => "PENDING_EXTERNAL_DESTRUCTION",
            CryptoKeyVersionStateEnum::EXTERNALDESTRUCTIONFAILED => "EXTERNAL_DESTRUCTION_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for CryptoKeyVersionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED" => Ok(CryptoKeyVersionStateEnum::CRYPTOKEYVERSIONSTATEUNSPECIFIED),
           "PENDING_GENERATION" => Ok(CryptoKeyVersionStateEnum::PENDINGGENERATION),
           "ENABLED" => Ok(CryptoKeyVersionStateEnum::ENABLED),
           "DISABLED" => Ok(CryptoKeyVersionStateEnum::DISABLED),
           "DESTROYED" => Ok(CryptoKeyVersionStateEnum::DESTROYED),
           "DESTROY_SCHEDULED" => Ok(CryptoKeyVersionStateEnum::DESTROYSCHEDULED),
           "PENDING_IMPORT" => Ok(CryptoKeyVersionStateEnum::PENDINGIMPORT),
           "IMPORT_FAILED" => Ok(CryptoKeyVersionStateEnum::IMPORTFAILED),
           "GENERATION_FAILED" => Ok(CryptoKeyVersionStateEnum::GENERATIONFAILED),
           "PENDING_EXTERNAL_DESTRUCTION" => Ok(CryptoKeyVersionStateEnum::PENDINGEXTERNALDESTRUCTION),
           "EXTERNAL_DESTRUCTION_FAILED" => Ok(CryptoKeyVersionStateEnum::EXTERNALDESTRUCTIONFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CryptoKeyVersionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CryptoKeyVersionTemplateAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Algorithm to use when creating a CryptoKeyVersion based on this template. For backwards compatibility, GOOGLE_SYMMETRIC_ENCRYPTION is implied if both this field is omitted and CryptoKey.purpose is ENCRYPT_DECRYPT.
pub enum CryptoKeyVersionTemplateAlgorithmEnum {
    

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

impl AsRef<str> for CryptoKeyVersionTemplateAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CryptoKeyVersionTemplateAlgorithmEnum::CRYPTOKEYVERSIONALGORITHMUNSPECIFIED => "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED",
            CryptoKeyVersionTemplateAlgorithmEnum::GOOGLESYMMETRICENCRYPTION => "GOOGLE_SYMMETRIC_ENCRYPTION",
            CryptoKeyVersionTemplateAlgorithmEnum::AES128GCM => "AES_128_GCM",
            CryptoKeyVersionTemplateAlgorithmEnum::AES256GCM => "AES_256_GCM",
            CryptoKeyVersionTemplateAlgorithmEnum::AES128CBC => "AES_128_CBC",
            CryptoKeyVersionTemplateAlgorithmEnum::AES256CBC => "AES_256_CBC",
            CryptoKeyVersionTemplateAlgorithmEnum::AES128CTR => "AES_128_CTR",
            CryptoKeyVersionTemplateAlgorithmEnum::AES256CTR => "AES_256_CTR",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPSS2048SHA256 => "RSA_SIGN_PSS_2048_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPSS3072SHA256 => "RSA_SIGN_PSS_3072_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPSS4096SHA256 => "RSA_SIGN_PSS_4096_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPSS4096SHA512 => "RSA_SIGN_PSS_4096_SHA512",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPKCS12048SHA256 => "RSA_SIGN_PKCS1_2048_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPKCS13072SHA256 => "RSA_SIGN_PKCS1_3072_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPKCS14096SHA256 => "RSA_SIGN_PKCS1_4096_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPKCS14096SHA512 => "RSA_SIGN_PKCS1_4096_SHA512",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNRAWPKCS12048 => "RSA_SIGN_RAW_PKCS1_2048",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNRAWPKCS13072 => "RSA_SIGN_RAW_PKCS1_3072",
            CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNRAWPKCS14096 => "RSA_SIGN_RAW_PKCS1_4096",
            CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP2048SHA256 => "RSA_DECRYPT_OAEP_2048_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP3072SHA256 => "RSA_DECRYPT_OAEP_3072_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP4096SHA256 => "RSA_DECRYPT_OAEP_4096_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP4096SHA512 => "RSA_DECRYPT_OAEP_4096_SHA512",
            CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP2048SHA1 => "RSA_DECRYPT_OAEP_2048_SHA1",
            CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP3072SHA1 => "RSA_DECRYPT_OAEP_3072_SHA1",
            CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP4096SHA1 => "RSA_DECRYPT_OAEP_4096_SHA1",
            CryptoKeyVersionTemplateAlgorithmEnum::ECSIGNP256SHA256 => "EC_SIGN_P256_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::ECSIGNP384SHA384 => "EC_SIGN_P384_SHA384",
            CryptoKeyVersionTemplateAlgorithmEnum::ECSIGNSECP256K1SHA256 => "EC_SIGN_SECP256K1_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::HMACSHA256 => "HMAC_SHA256",
            CryptoKeyVersionTemplateAlgorithmEnum::HMACSHA1 => "HMAC_SHA1",
            CryptoKeyVersionTemplateAlgorithmEnum::HMACSHA384 => "HMAC_SHA384",
            CryptoKeyVersionTemplateAlgorithmEnum::HMACSHA512 => "HMAC_SHA512",
            CryptoKeyVersionTemplateAlgorithmEnum::HMACSHA224 => "HMAC_SHA224",
            CryptoKeyVersionTemplateAlgorithmEnum::EXTERNALSYMMETRICENCRYPTION => "EXTERNAL_SYMMETRIC_ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for CryptoKeyVersionTemplateAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::CRYPTOKEYVERSIONALGORITHMUNSPECIFIED),
           "GOOGLE_SYMMETRIC_ENCRYPTION" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::GOOGLESYMMETRICENCRYPTION),
           "AES_128_GCM" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::AES128GCM),
           "AES_256_GCM" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::AES256GCM),
           "AES_128_CBC" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::AES128CBC),
           "AES_256_CBC" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::AES256CBC),
           "AES_128_CTR" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::AES128CTR),
           "AES_256_CTR" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::AES256CTR),
           "RSA_SIGN_PSS_2048_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPSS2048SHA256),
           "RSA_SIGN_PSS_3072_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPSS3072SHA256),
           "RSA_SIGN_PSS_4096_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPSS4096SHA256),
           "RSA_SIGN_PSS_4096_SHA512" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPSS4096SHA512),
           "RSA_SIGN_PKCS1_2048_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPKCS12048SHA256),
           "RSA_SIGN_PKCS1_3072_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPKCS13072SHA256),
           "RSA_SIGN_PKCS1_4096_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPKCS14096SHA256),
           "RSA_SIGN_PKCS1_4096_SHA512" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNPKCS14096SHA512),
           "RSA_SIGN_RAW_PKCS1_2048" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNRAWPKCS12048),
           "RSA_SIGN_RAW_PKCS1_3072" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNRAWPKCS13072),
           "RSA_SIGN_RAW_PKCS1_4096" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSASIGNRAWPKCS14096),
           "RSA_DECRYPT_OAEP_2048_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP2048SHA256),
           "RSA_DECRYPT_OAEP_3072_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP3072SHA256),
           "RSA_DECRYPT_OAEP_4096_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP4096SHA256),
           "RSA_DECRYPT_OAEP_4096_SHA512" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP4096SHA512),
           "RSA_DECRYPT_OAEP_2048_SHA1" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP2048SHA1),
           "RSA_DECRYPT_OAEP_3072_SHA1" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP3072SHA1),
           "RSA_DECRYPT_OAEP_4096_SHA1" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::RSADECRYPTOAEP4096SHA1),
           "EC_SIGN_P256_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::ECSIGNP256SHA256),
           "EC_SIGN_P384_SHA384" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::ECSIGNP384SHA384),
           "EC_SIGN_SECP256K1_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::ECSIGNSECP256K1SHA256),
           "HMAC_SHA256" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::HMACSHA256),
           "HMAC_SHA1" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::HMACSHA1),
           "HMAC_SHA384" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::HMACSHA384),
           "HMAC_SHA512" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::HMACSHA512),
           "HMAC_SHA224" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::HMACSHA224),
           "EXTERNAL_SYMMETRIC_ENCRYPTION" => Ok(CryptoKeyVersionTemplateAlgorithmEnum::EXTERNALSYMMETRICENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CryptoKeyVersionTemplateAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CryptoKeyVersionTemplateProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// ProtectionLevel to use when creating a CryptoKeyVersion based on this template. Immutable. Defaults to SOFTWARE.
pub enum CryptoKeyVersionTemplateProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for CryptoKeyVersionTemplateProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CryptoKeyVersionTemplateProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            CryptoKeyVersionTemplateProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            CryptoKeyVersionTemplateProtectionLevelEnum::HSM => "HSM",
            CryptoKeyVersionTemplateProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            CryptoKeyVersionTemplateProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for CryptoKeyVersionTemplateProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(CryptoKeyVersionTemplateProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(CryptoKeyVersionTemplateProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(CryptoKeyVersionTemplateProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(CryptoKeyVersionTemplateProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(CryptoKeyVersionTemplateProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CryptoKeyVersionTemplateProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DecryptResponseProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ProtectionLevel of the CryptoKeyVersion used in decryption.
pub enum DecryptResponseProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for DecryptResponseProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DecryptResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            DecryptResponseProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            DecryptResponseProtectionLevelEnum::HSM => "HSM",
            DecryptResponseProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            DecryptResponseProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for DecryptResponseProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(DecryptResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(DecryptResponseProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(DecryptResponseProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(DecryptResponseProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(DecryptResponseProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DecryptResponseProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EkmConnectionKeyManagementModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL.
pub enum EkmConnectionKeyManagementModeEnum {
    

    /// Not specified.
    ///
    /// "KEY_MANAGEMENT_MODE_UNSPECIFIED"
    #[serde(rename="KEY_MANAGEMENT_MODE_UNSPECIFIED")]
    KEYMANAGEMENTMODEUNSPECIFIED,
    

    /// EKM-side key management operations on CryptoKeys created with this EkmConnection must be initiated from the EKM directly and cannot be performed from Cloud KMS. This means that: * When creating a CryptoKeyVersion associated with this EkmConnection, the caller must supply the key path of pre-existing external key material that will be linked to the CryptoKeyVersion. * Destruction of external key material cannot be requested via the Cloud KMS API and must be performed directly in the EKM. * Automatic rotation of key material is not supported.
    ///
    /// "MANUAL"
    #[serde(rename="MANUAL")]
    MANUAL,
    

    /// All CryptoKeys created with this EkmConnection use EKM-side key management operations initiated from Cloud KMS. This means that: * When a CryptoKeyVersion associated with this EkmConnection is created, the EKM automatically generates new key material and a new key path. The caller cannot supply the key path of pre-existing external key material. * Destruction of external key material associated with this EkmConnection can be requested by calling DestroyCryptoKeyVersion. * Automatic rotation of key material is supported.
    ///
    /// "CLOUD_KMS"
    #[serde(rename="CLOUD_KMS")]
    CLOUDKMS,
}

impl AsRef<str> for EkmConnectionKeyManagementModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EkmConnectionKeyManagementModeEnum::KEYMANAGEMENTMODEUNSPECIFIED => "KEY_MANAGEMENT_MODE_UNSPECIFIED",
            EkmConnectionKeyManagementModeEnum::MANUAL => "MANUAL",
            EkmConnectionKeyManagementModeEnum::CLOUDKMS => "CLOUD_KMS",
        }
    }
}

impl std::convert::TryFrom< &str> for EkmConnectionKeyManagementModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_MANAGEMENT_MODE_UNSPECIFIED" => Ok(EkmConnectionKeyManagementModeEnum::KEYMANAGEMENTMODEUNSPECIFIED),
           "MANUAL" => Ok(EkmConnectionKeyManagementModeEnum::MANUAL),
           "CLOUD_KMS" => Ok(EkmConnectionKeyManagementModeEnum::CLOUDKMS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EkmConnectionKeyManagementModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EncryptResponseProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ProtectionLevel of the CryptoKeyVersion used in encryption.
pub enum EncryptResponseProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for EncryptResponseProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EncryptResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            EncryptResponseProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            EncryptResponseProtectionLevelEnum::HSM => "HSM",
            EncryptResponseProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            EncryptResponseProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for EncryptResponseProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(EncryptResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(EncryptResponseProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(EncryptResponseProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(EncryptResponseProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(EncryptResponseProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EncryptResponseProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GenerateRandomBytesRequestProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ProtectionLevel to use when generating the random data. Currently, only HSM protection level is supported.
pub enum GenerateRandomBytesRequestProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for GenerateRandomBytesRequestProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GenerateRandomBytesRequestProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            GenerateRandomBytesRequestProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            GenerateRandomBytesRequestProtectionLevelEnum::HSM => "HSM",
            GenerateRandomBytesRequestProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            GenerateRandomBytesRequestProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for GenerateRandomBytesRequestProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(GenerateRandomBytesRequestProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(GenerateRandomBytesRequestProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(GenerateRandomBytesRequestProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(GenerateRandomBytesRequestProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(GenerateRandomBytesRequestProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GenerateRandomBytesRequestProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImportCryptoKeyVersionRequestAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The algorithm of the key being imported. This does not need to match the version_template of the CryptoKey this version imports into.
pub enum ImportCryptoKeyVersionRequestAlgorithmEnum {
    

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

impl AsRef<str> for ImportCryptoKeyVersionRequestAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImportCryptoKeyVersionRequestAlgorithmEnum::CRYPTOKEYVERSIONALGORITHMUNSPECIFIED => "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED",
            ImportCryptoKeyVersionRequestAlgorithmEnum::GOOGLESYMMETRICENCRYPTION => "GOOGLE_SYMMETRIC_ENCRYPTION",
            ImportCryptoKeyVersionRequestAlgorithmEnum::AES128GCM => "AES_128_GCM",
            ImportCryptoKeyVersionRequestAlgorithmEnum::AES256GCM => "AES_256_GCM",
            ImportCryptoKeyVersionRequestAlgorithmEnum::AES128CBC => "AES_128_CBC",
            ImportCryptoKeyVersionRequestAlgorithmEnum::AES256CBC => "AES_256_CBC",
            ImportCryptoKeyVersionRequestAlgorithmEnum::AES128CTR => "AES_128_CTR",
            ImportCryptoKeyVersionRequestAlgorithmEnum::AES256CTR => "AES_256_CTR",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPSS2048SHA256 => "RSA_SIGN_PSS_2048_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPSS3072SHA256 => "RSA_SIGN_PSS_3072_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPSS4096SHA256 => "RSA_SIGN_PSS_4096_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPSS4096SHA512 => "RSA_SIGN_PSS_4096_SHA512",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPKCS12048SHA256 => "RSA_SIGN_PKCS1_2048_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPKCS13072SHA256 => "RSA_SIGN_PKCS1_3072_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPKCS14096SHA256 => "RSA_SIGN_PKCS1_4096_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPKCS14096SHA512 => "RSA_SIGN_PKCS1_4096_SHA512",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNRAWPKCS12048 => "RSA_SIGN_RAW_PKCS1_2048",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNRAWPKCS13072 => "RSA_SIGN_RAW_PKCS1_3072",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNRAWPKCS14096 => "RSA_SIGN_RAW_PKCS1_4096",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP2048SHA256 => "RSA_DECRYPT_OAEP_2048_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP3072SHA256 => "RSA_DECRYPT_OAEP_3072_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP4096SHA256 => "RSA_DECRYPT_OAEP_4096_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP4096SHA512 => "RSA_DECRYPT_OAEP_4096_SHA512",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP2048SHA1 => "RSA_DECRYPT_OAEP_2048_SHA1",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP3072SHA1 => "RSA_DECRYPT_OAEP_3072_SHA1",
            ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP4096SHA1 => "RSA_DECRYPT_OAEP_4096_SHA1",
            ImportCryptoKeyVersionRequestAlgorithmEnum::ECSIGNP256SHA256 => "EC_SIGN_P256_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::ECSIGNP384SHA384 => "EC_SIGN_P384_SHA384",
            ImportCryptoKeyVersionRequestAlgorithmEnum::ECSIGNSECP256K1SHA256 => "EC_SIGN_SECP256K1_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::HMACSHA256 => "HMAC_SHA256",
            ImportCryptoKeyVersionRequestAlgorithmEnum::HMACSHA1 => "HMAC_SHA1",
            ImportCryptoKeyVersionRequestAlgorithmEnum::HMACSHA384 => "HMAC_SHA384",
            ImportCryptoKeyVersionRequestAlgorithmEnum::HMACSHA512 => "HMAC_SHA512",
            ImportCryptoKeyVersionRequestAlgorithmEnum::HMACSHA224 => "HMAC_SHA224",
            ImportCryptoKeyVersionRequestAlgorithmEnum::EXTERNALSYMMETRICENCRYPTION => "EXTERNAL_SYMMETRIC_ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for ImportCryptoKeyVersionRequestAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::CRYPTOKEYVERSIONALGORITHMUNSPECIFIED),
           "GOOGLE_SYMMETRIC_ENCRYPTION" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::GOOGLESYMMETRICENCRYPTION),
           "AES_128_GCM" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::AES128GCM),
           "AES_256_GCM" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::AES256GCM),
           "AES_128_CBC" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::AES128CBC),
           "AES_256_CBC" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::AES256CBC),
           "AES_128_CTR" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::AES128CTR),
           "AES_256_CTR" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::AES256CTR),
           "RSA_SIGN_PSS_2048_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPSS2048SHA256),
           "RSA_SIGN_PSS_3072_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPSS3072SHA256),
           "RSA_SIGN_PSS_4096_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPSS4096SHA256),
           "RSA_SIGN_PSS_4096_SHA512" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPSS4096SHA512),
           "RSA_SIGN_PKCS1_2048_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPKCS12048SHA256),
           "RSA_SIGN_PKCS1_3072_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPKCS13072SHA256),
           "RSA_SIGN_PKCS1_4096_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPKCS14096SHA256),
           "RSA_SIGN_PKCS1_4096_SHA512" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNPKCS14096SHA512),
           "RSA_SIGN_RAW_PKCS1_2048" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNRAWPKCS12048),
           "RSA_SIGN_RAW_PKCS1_3072" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNRAWPKCS13072),
           "RSA_SIGN_RAW_PKCS1_4096" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSASIGNRAWPKCS14096),
           "RSA_DECRYPT_OAEP_2048_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP2048SHA256),
           "RSA_DECRYPT_OAEP_3072_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP3072SHA256),
           "RSA_DECRYPT_OAEP_4096_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP4096SHA256),
           "RSA_DECRYPT_OAEP_4096_SHA512" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP4096SHA512),
           "RSA_DECRYPT_OAEP_2048_SHA1" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP2048SHA1),
           "RSA_DECRYPT_OAEP_3072_SHA1" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP3072SHA1),
           "RSA_DECRYPT_OAEP_4096_SHA1" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::RSADECRYPTOAEP4096SHA1),
           "EC_SIGN_P256_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::ECSIGNP256SHA256),
           "EC_SIGN_P384_SHA384" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::ECSIGNP384SHA384),
           "EC_SIGN_SECP256K1_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::ECSIGNSECP256K1SHA256),
           "HMAC_SHA256" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::HMACSHA256),
           "HMAC_SHA1" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::HMACSHA1),
           "HMAC_SHA384" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::HMACSHA384),
           "HMAC_SHA512" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::HMACSHA512),
           "HMAC_SHA224" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::HMACSHA224),
           "EXTERNAL_SYMMETRIC_ENCRYPTION" => Ok(ImportCryptoKeyVersionRequestAlgorithmEnum::EXTERNALSYMMETRICENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImportCryptoKeyVersionRequestAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImportJobImportMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The wrapping method to be used for incoming key material.
pub enum ImportJobImportMethodEnum {
    

    /// Not specified.
    ///
    /// "IMPORT_METHOD_UNSPECIFIED"
    #[serde(rename="IMPORT_METHOD_UNSPECIFIED")]
    IMPORTMETHODUNSPECIFIED,
    

    /// This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping scheme defined in the PKCS #11 standard. In summary, this involves wrapping the raw key with an ephemeral AES key, and wrapping the ephemeral AES key with a 3072 bit RSA key. For more details, see [RSA AES key wrap mechanism](http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908).
    ///
    /// "RSA_OAEP_3072_SHA1_AES_256"
    #[serde(rename="RSA_OAEP_3072_SHA1_AES_256")]
    RSAOAEP3072SHA1AES256,
    

    /// This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping scheme defined in the PKCS #11 standard. In summary, this involves wrapping the raw key with an ephemeral AES key, and wrapping the ephemeral AES key with a 4096 bit RSA key. For more details, see [RSA AES key wrap mechanism](http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908).
    ///
    /// "RSA_OAEP_4096_SHA1_AES_256"
    #[serde(rename="RSA_OAEP_4096_SHA1_AES_256")]
    RSAOAEP4096SHA1AES256,
    

    /// This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping scheme defined in the PKCS #11 standard. In summary, this involves wrapping the raw key with an ephemeral AES key, and wrapping the ephemeral AES key with a 3072 bit RSA key. For more details, see [RSA AES key wrap mechanism](http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908).
    ///
    /// "RSA_OAEP_3072_SHA256_AES_256"
    #[serde(rename="RSA_OAEP_3072_SHA256_AES_256")]
    RSAOAEP3072SHA256AES256,
    

    /// This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping scheme defined in the PKCS #11 standard. In summary, this involves wrapping the raw key with an ephemeral AES key, and wrapping the ephemeral AES key with a 4096 bit RSA key. For more details, see [RSA AES key wrap mechanism](http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908).
    ///
    /// "RSA_OAEP_4096_SHA256_AES_256"
    #[serde(rename="RSA_OAEP_4096_SHA256_AES_256")]
    RSAOAEP4096SHA256AES256,
    

    /// This ImportMethod represents RSAES-OAEP with a 3072 bit RSA key. The key material to be imported is wrapped directly with the RSA key. Due to technical limitations of RSA wrapping, this method cannot be used to wrap RSA keys for import.
    ///
    /// "RSA_OAEP_3072_SHA256"
    #[serde(rename="RSA_OAEP_3072_SHA256")]
    RSAOAEP3072SHA256,
    

    /// This ImportMethod represents RSAES-OAEP with a 4096 bit RSA key. The key material to be imported is wrapped directly with the RSA key. Due to technical limitations of RSA wrapping, this method cannot be used to wrap RSA keys for import.
    ///
    /// "RSA_OAEP_4096_SHA256"
    #[serde(rename="RSA_OAEP_4096_SHA256")]
    RSAOAEP4096SHA256,
}

impl AsRef<str> for ImportJobImportMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImportJobImportMethodEnum::IMPORTMETHODUNSPECIFIED => "IMPORT_METHOD_UNSPECIFIED",
            ImportJobImportMethodEnum::RSAOAEP3072SHA1AES256 => "RSA_OAEP_3072_SHA1_AES_256",
            ImportJobImportMethodEnum::RSAOAEP4096SHA1AES256 => "RSA_OAEP_4096_SHA1_AES_256",
            ImportJobImportMethodEnum::RSAOAEP3072SHA256AES256 => "RSA_OAEP_3072_SHA256_AES_256",
            ImportJobImportMethodEnum::RSAOAEP4096SHA256AES256 => "RSA_OAEP_4096_SHA256_AES_256",
            ImportJobImportMethodEnum::RSAOAEP3072SHA256 => "RSA_OAEP_3072_SHA256",
            ImportJobImportMethodEnum::RSAOAEP4096SHA256 => "RSA_OAEP_4096_SHA256",
        }
    }
}

impl std::convert::TryFrom< &str> for ImportJobImportMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPORT_METHOD_UNSPECIFIED" => Ok(ImportJobImportMethodEnum::IMPORTMETHODUNSPECIFIED),
           "RSA_OAEP_3072_SHA1_AES_256" => Ok(ImportJobImportMethodEnum::RSAOAEP3072SHA1AES256),
           "RSA_OAEP_4096_SHA1_AES_256" => Ok(ImportJobImportMethodEnum::RSAOAEP4096SHA1AES256),
           "RSA_OAEP_3072_SHA256_AES_256" => Ok(ImportJobImportMethodEnum::RSAOAEP3072SHA256AES256),
           "RSA_OAEP_4096_SHA256_AES_256" => Ok(ImportJobImportMethodEnum::RSAOAEP4096SHA256AES256),
           "RSA_OAEP_3072_SHA256" => Ok(ImportJobImportMethodEnum::RSAOAEP3072SHA256),
           "RSA_OAEP_4096_SHA256" => Ok(ImportJobImportMethodEnum::RSAOAEP4096SHA256),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImportJobImportMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImportJobProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The protection level of the ImportJob. This must match the protection_level of the version_template on the CryptoKey you attempt to import into.
pub enum ImportJobProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for ImportJobProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImportJobProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            ImportJobProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            ImportJobProtectionLevelEnum::HSM => "HSM",
            ImportJobProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            ImportJobProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for ImportJobProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(ImportJobProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(ImportJobProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(ImportJobProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(ImportJobProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(ImportJobProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImportJobProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImportJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the ImportJob, indicating if it can be used.
pub enum ImportJobStateEnum {
    

    /// Not specified.
    ///
    /// "IMPORT_JOB_STATE_UNSPECIFIED"
    #[serde(rename="IMPORT_JOB_STATE_UNSPECIFIED")]
    IMPORTJOBSTATEUNSPECIFIED,
    

    /// The wrapping key for this job is still being generated. It may not be used. Cloud KMS will automatically mark this job as ACTIVE as soon as the wrapping key is generated.
    ///
    /// "PENDING_GENERATION"
    #[serde(rename="PENDING_GENERATION")]
    PENDINGGENERATION,
    

    /// This job may be used in CreateCryptoKey and CreateCryptoKeyVersion requests.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// This job can no longer be used and may not leave this state once entered.
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
}

impl AsRef<str> for ImportJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImportJobStateEnum::IMPORTJOBSTATEUNSPECIFIED => "IMPORT_JOB_STATE_UNSPECIFIED",
            ImportJobStateEnum::PENDINGGENERATION => "PENDING_GENERATION",
            ImportJobStateEnum::ACTIVE => "ACTIVE",
            ImportJobStateEnum::EXPIRED => "EXPIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for ImportJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPORT_JOB_STATE_UNSPECIFIED" => Ok(ImportJobStateEnum::IMPORTJOBSTATEUNSPECIFIED),
           "PENDING_GENERATION" => Ok(ImportJobStateEnum::PENDINGGENERATION),
           "ACTIVE" => Ok(ImportJobStateEnum::ACTIVE),
           "EXPIRED" => Ok(ImportJobStateEnum::EXPIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImportJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KeyOperationAttestationFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The format of the attestation data.
pub enum KeyOperationAttestationFormatEnum {
    

    /// Not specified.
    ///
    /// "ATTESTATION_FORMAT_UNSPECIFIED"
    #[serde(rename="ATTESTATION_FORMAT_UNSPECIFIED")]
    ATTESTATIONFORMATUNSPECIFIED,
    

    /// Cavium HSM attestation compressed with gzip. Note that this format is defined by Cavium and subject to change at any time. See https://www.marvell.com/products/security-solutions/nitrox-hs-adapters/software-key-attestation.html.
    ///
    /// "CAVIUM_V1_COMPRESSED"
    #[serde(rename="CAVIUM_V1_COMPRESSED")]
    CAVIUMV1COMPRESSED,
    

    /// Cavium HSM attestation V2 compressed with gzip. This is a new format introduced in Cavium's version 3.2-08.
    ///
    /// "CAVIUM_V2_COMPRESSED"
    #[serde(rename="CAVIUM_V2_COMPRESSED")]
    CAVIUMV2COMPRESSED,
}

impl AsRef<str> for KeyOperationAttestationFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KeyOperationAttestationFormatEnum::ATTESTATIONFORMATUNSPECIFIED => "ATTESTATION_FORMAT_UNSPECIFIED",
            KeyOperationAttestationFormatEnum::CAVIUMV1COMPRESSED => "CAVIUM_V1_COMPRESSED",
            KeyOperationAttestationFormatEnum::CAVIUMV2COMPRESSED => "CAVIUM_V2_COMPRESSED",
        }
    }
}

impl std::convert::TryFrom< &str> for KeyOperationAttestationFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTESTATION_FORMAT_UNSPECIFIED" => Ok(KeyOperationAttestationFormatEnum::ATTESTATIONFORMATUNSPECIFIED),
           "CAVIUM_V1_COMPRESSED" => Ok(KeyOperationAttestationFormatEnum::CAVIUMV1COMPRESSED),
           "CAVIUM_V2_COMPRESSED" => Ok(KeyOperationAttestationFormatEnum::CAVIUMV2COMPRESSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KeyOperationAttestationFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MacSignResponseProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ProtectionLevel of the CryptoKeyVersion used for signing.
pub enum MacSignResponseProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for MacSignResponseProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MacSignResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            MacSignResponseProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            MacSignResponseProtectionLevelEnum::HSM => "HSM",
            MacSignResponseProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            MacSignResponseProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for MacSignResponseProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(MacSignResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(MacSignResponseProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(MacSignResponseProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(MacSignResponseProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(MacSignResponseProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MacSignResponseProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MacVerifyResponseProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ProtectionLevel of the CryptoKeyVersion used for verification.
pub enum MacVerifyResponseProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for MacVerifyResponseProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MacVerifyResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            MacVerifyResponseProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            MacVerifyResponseProtectionLevelEnum::HSM => "HSM",
            MacVerifyResponseProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            MacVerifyResponseProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for MacVerifyResponseProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(MacVerifyResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(MacVerifyResponseProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(MacVerifyResponseProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(MacVerifyResponseProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(MacVerifyResponseProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MacVerifyResponseProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublicKeyAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Algorithm associated with this key.
pub enum PublicKeyAlgorithmEnum {
    

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

impl AsRef<str> for PublicKeyAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublicKeyAlgorithmEnum::CRYPTOKEYVERSIONALGORITHMUNSPECIFIED => "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED",
            PublicKeyAlgorithmEnum::GOOGLESYMMETRICENCRYPTION => "GOOGLE_SYMMETRIC_ENCRYPTION",
            PublicKeyAlgorithmEnum::AES128GCM => "AES_128_GCM",
            PublicKeyAlgorithmEnum::AES256GCM => "AES_256_GCM",
            PublicKeyAlgorithmEnum::AES128CBC => "AES_128_CBC",
            PublicKeyAlgorithmEnum::AES256CBC => "AES_256_CBC",
            PublicKeyAlgorithmEnum::AES128CTR => "AES_128_CTR",
            PublicKeyAlgorithmEnum::AES256CTR => "AES_256_CTR",
            PublicKeyAlgorithmEnum::RSASIGNPSS2048SHA256 => "RSA_SIGN_PSS_2048_SHA256",
            PublicKeyAlgorithmEnum::RSASIGNPSS3072SHA256 => "RSA_SIGN_PSS_3072_SHA256",
            PublicKeyAlgorithmEnum::RSASIGNPSS4096SHA256 => "RSA_SIGN_PSS_4096_SHA256",
            PublicKeyAlgorithmEnum::RSASIGNPSS4096SHA512 => "RSA_SIGN_PSS_4096_SHA512",
            PublicKeyAlgorithmEnum::RSASIGNPKCS12048SHA256 => "RSA_SIGN_PKCS1_2048_SHA256",
            PublicKeyAlgorithmEnum::RSASIGNPKCS13072SHA256 => "RSA_SIGN_PKCS1_3072_SHA256",
            PublicKeyAlgorithmEnum::RSASIGNPKCS14096SHA256 => "RSA_SIGN_PKCS1_4096_SHA256",
            PublicKeyAlgorithmEnum::RSASIGNPKCS14096SHA512 => "RSA_SIGN_PKCS1_4096_SHA512",
            PublicKeyAlgorithmEnum::RSASIGNRAWPKCS12048 => "RSA_SIGN_RAW_PKCS1_2048",
            PublicKeyAlgorithmEnum::RSASIGNRAWPKCS13072 => "RSA_SIGN_RAW_PKCS1_3072",
            PublicKeyAlgorithmEnum::RSASIGNRAWPKCS14096 => "RSA_SIGN_RAW_PKCS1_4096",
            PublicKeyAlgorithmEnum::RSADECRYPTOAEP2048SHA256 => "RSA_DECRYPT_OAEP_2048_SHA256",
            PublicKeyAlgorithmEnum::RSADECRYPTOAEP3072SHA256 => "RSA_DECRYPT_OAEP_3072_SHA256",
            PublicKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA256 => "RSA_DECRYPT_OAEP_4096_SHA256",
            PublicKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA512 => "RSA_DECRYPT_OAEP_4096_SHA512",
            PublicKeyAlgorithmEnum::RSADECRYPTOAEP2048SHA1 => "RSA_DECRYPT_OAEP_2048_SHA1",
            PublicKeyAlgorithmEnum::RSADECRYPTOAEP3072SHA1 => "RSA_DECRYPT_OAEP_3072_SHA1",
            PublicKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA1 => "RSA_DECRYPT_OAEP_4096_SHA1",
            PublicKeyAlgorithmEnum::ECSIGNP256SHA256 => "EC_SIGN_P256_SHA256",
            PublicKeyAlgorithmEnum::ECSIGNP384SHA384 => "EC_SIGN_P384_SHA384",
            PublicKeyAlgorithmEnum::ECSIGNSECP256K1SHA256 => "EC_SIGN_SECP256K1_SHA256",
            PublicKeyAlgorithmEnum::HMACSHA256 => "HMAC_SHA256",
            PublicKeyAlgorithmEnum::HMACSHA1 => "HMAC_SHA1",
            PublicKeyAlgorithmEnum::HMACSHA384 => "HMAC_SHA384",
            PublicKeyAlgorithmEnum::HMACSHA512 => "HMAC_SHA512",
            PublicKeyAlgorithmEnum::HMACSHA224 => "HMAC_SHA224",
            PublicKeyAlgorithmEnum::EXTERNALSYMMETRICENCRYPTION => "EXTERNAL_SYMMETRIC_ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for PublicKeyAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => Ok(PublicKeyAlgorithmEnum::CRYPTOKEYVERSIONALGORITHMUNSPECIFIED),
           "GOOGLE_SYMMETRIC_ENCRYPTION" => Ok(PublicKeyAlgorithmEnum::GOOGLESYMMETRICENCRYPTION),
           "AES_128_GCM" => Ok(PublicKeyAlgorithmEnum::AES128GCM),
           "AES_256_GCM" => Ok(PublicKeyAlgorithmEnum::AES256GCM),
           "AES_128_CBC" => Ok(PublicKeyAlgorithmEnum::AES128CBC),
           "AES_256_CBC" => Ok(PublicKeyAlgorithmEnum::AES256CBC),
           "AES_128_CTR" => Ok(PublicKeyAlgorithmEnum::AES128CTR),
           "AES_256_CTR" => Ok(PublicKeyAlgorithmEnum::AES256CTR),
           "RSA_SIGN_PSS_2048_SHA256" => Ok(PublicKeyAlgorithmEnum::RSASIGNPSS2048SHA256),
           "RSA_SIGN_PSS_3072_SHA256" => Ok(PublicKeyAlgorithmEnum::RSASIGNPSS3072SHA256),
           "RSA_SIGN_PSS_4096_SHA256" => Ok(PublicKeyAlgorithmEnum::RSASIGNPSS4096SHA256),
           "RSA_SIGN_PSS_4096_SHA512" => Ok(PublicKeyAlgorithmEnum::RSASIGNPSS4096SHA512),
           "RSA_SIGN_PKCS1_2048_SHA256" => Ok(PublicKeyAlgorithmEnum::RSASIGNPKCS12048SHA256),
           "RSA_SIGN_PKCS1_3072_SHA256" => Ok(PublicKeyAlgorithmEnum::RSASIGNPKCS13072SHA256),
           "RSA_SIGN_PKCS1_4096_SHA256" => Ok(PublicKeyAlgorithmEnum::RSASIGNPKCS14096SHA256),
           "RSA_SIGN_PKCS1_4096_SHA512" => Ok(PublicKeyAlgorithmEnum::RSASIGNPKCS14096SHA512),
           "RSA_SIGN_RAW_PKCS1_2048" => Ok(PublicKeyAlgorithmEnum::RSASIGNRAWPKCS12048),
           "RSA_SIGN_RAW_PKCS1_3072" => Ok(PublicKeyAlgorithmEnum::RSASIGNRAWPKCS13072),
           "RSA_SIGN_RAW_PKCS1_4096" => Ok(PublicKeyAlgorithmEnum::RSASIGNRAWPKCS14096),
           "RSA_DECRYPT_OAEP_2048_SHA256" => Ok(PublicKeyAlgorithmEnum::RSADECRYPTOAEP2048SHA256),
           "RSA_DECRYPT_OAEP_3072_SHA256" => Ok(PublicKeyAlgorithmEnum::RSADECRYPTOAEP3072SHA256),
           "RSA_DECRYPT_OAEP_4096_SHA256" => Ok(PublicKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA256),
           "RSA_DECRYPT_OAEP_4096_SHA512" => Ok(PublicKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA512),
           "RSA_DECRYPT_OAEP_2048_SHA1" => Ok(PublicKeyAlgorithmEnum::RSADECRYPTOAEP2048SHA1),
           "RSA_DECRYPT_OAEP_3072_SHA1" => Ok(PublicKeyAlgorithmEnum::RSADECRYPTOAEP3072SHA1),
           "RSA_DECRYPT_OAEP_4096_SHA1" => Ok(PublicKeyAlgorithmEnum::RSADECRYPTOAEP4096SHA1),
           "EC_SIGN_P256_SHA256" => Ok(PublicKeyAlgorithmEnum::ECSIGNP256SHA256),
           "EC_SIGN_P384_SHA384" => Ok(PublicKeyAlgorithmEnum::ECSIGNP384SHA384),
           "EC_SIGN_SECP256K1_SHA256" => Ok(PublicKeyAlgorithmEnum::ECSIGNSECP256K1SHA256),
           "HMAC_SHA256" => Ok(PublicKeyAlgorithmEnum::HMACSHA256),
           "HMAC_SHA1" => Ok(PublicKeyAlgorithmEnum::HMACSHA1),
           "HMAC_SHA384" => Ok(PublicKeyAlgorithmEnum::HMACSHA384),
           "HMAC_SHA512" => Ok(PublicKeyAlgorithmEnum::HMACSHA512),
           "HMAC_SHA224" => Ok(PublicKeyAlgorithmEnum::HMACSHA224),
           "EXTERNAL_SYMMETRIC_ENCRYPTION" => Ok(PublicKeyAlgorithmEnum::EXTERNALSYMMETRICENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublicKeyAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublicKeyProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ProtectionLevel of the CryptoKeyVersion public key.
pub enum PublicKeyProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for PublicKeyProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublicKeyProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            PublicKeyProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            PublicKeyProtectionLevelEnum::HSM => "HSM",
            PublicKeyProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            PublicKeyProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for PublicKeyProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(PublicKeyProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(PublicKeyProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(PublicKeyProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(PublicKeyProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(PublicKeyProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublicKeyProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RawDecryptResponseProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ProtectionLevel of the CryptoKeyVersion used in decryption.
pub enum RawDecryptResponseProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for RawDecryptResponseProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RawDecryptResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            RawDecryptResponseProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            RawDecryptResponseProtectionLevelEnum::HSM => "HSM",
            RawDecryptResponseProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            RawDecryptResponseProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for RawDecryptResponseProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(RawDecryptResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(RawDecryptResponseProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(RawDecryptResponseProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(RawDecryptResponseProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(RawDecryptResponseProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RawDecryptResponseProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RawEncryptResponseProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ProtectionLevel of the CryptoKeyVersion used in encryption.
pub enum RawEncryptResponseProtectionLevelEnum {
    

    /// Not specified.
    ///
    /// "PROTECTION_LEVEL_UNSPECIFIED"
    #[serde(rename="PROTECTION_LEVEL_UNSPECIFIED")]
    PROTECTIONLEVELUNSPECIFIED,
    

    /// Crypto operations are performed in software.
    ///
    /// "SOFTWARE"
    #[serde(rename="SOFTWARE")]
    SOFTWARE,
    

    /// Crypto operations are performed in a Hardware Security Module.
    ///
    /// "HSM"
    #[serde(rename="HSM")]
    HSM,
    

    /// Crypto operations are performed by an external key manager.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Crypto operations are performed in an EKM-over-VPC backend.
    ///
    /// "EXTERNAL_VPC"
    #[serde(rename="EXTERNAL_VPC")]
    EXTERNALVPC,
}

impl AsRef<str> for RawEncryptResponseProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RawEncryptResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED => "PROTECTION_LEVEL_UNSPECIFIED",
            RawEncryptResponseProtectionLevelEnum::SOFTWARE => "SOFTWARE",
            RawEncryptResponseProtectionLevelEnum::HSM => "HSM",
            RawEncryptResponseProtectionLevelEnum::EXTERNAL => "EXTERNAL",
            RawEncryptResponseProtectionLevelEnum::EXTERNALVPC => "EXTERNAL_VPC",
        }
    }
}

impl std::convert::TryFrom< &str> for RawEncryptResponseProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTECTION_LEVEL_UNSPECIFIED" => Ok(RawEncryptResponseProtectionLevelEnum::PROTECTIONLEVELUNSPECIFIED),
           "SOFTWARE" => Ok(RawEncryptResponseProtectionLevelEnum::SOFTWARE),
           "HSM" => Ok(RawEncryptResponseProtectionLevelEnum::HSM),
           "EXTERNAL" => Ok(RawEncryptResponseProtectionLevelEnum::EXTERNAL),
           "EXTERNAL_VPC" => Ok(RawEncryptResponseProtectionLevelEnum::EXTERNALVPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RawEncryptResponseProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The fields to include in the response.
pub enum ProjectViewEnum {
    

    /// Default view for each CryptoKeyVersion. Does not include the attestation field.
    ///
    /// "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED"
    #[serde(rename="CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED")]
    CRYPTOKEYVERSIONVIEWUNSPECIFIED,
    

    /// Provides all fields in each CryptoKeyVersion, including the attestation.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::CRYPTOKEYVERSIONVIEWUNSPECIFIED => "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED",
            ProjectViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::CRYPTOKEYVERSIONVIEWUNSPECIFIED),
           "FULL" => Ok(ProjectViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectVersionViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The fields of the primary version to include in the response.
pub enum ProjectVersionViewEnum {
    

    /// Default view for each CryptoKeyVersion. Does not include the attestation field.
    ///
    /// "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED"
    #[serde(rename="CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED")]
    CRYPTOKEYVERSIONVIEWUNSPECIFIED,
    

    /// Provides all fields in each CryptoKeyVersion, including the attestation.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectVersionViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectVersionViewEnum::CRYPTOKEYVERSIONVIEWUNSPECIFIED => "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED",
            ProjectVersionViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectVersionViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED" => Ok(ProjectVersionViewEnum::CRYPTOKEYVERSIONVIEWUNSPECIFIED),
           "FULL" => Ok(ProjectVersionViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectVersionViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


