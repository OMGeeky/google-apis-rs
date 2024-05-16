use super::*;



// region BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All the digest functions supported by the remote cache. Remote cache may support multiple digest functions simultaneously.
pub enum BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum {
    

    /// It is an error for the server to return this value.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The SHA-256 digest function.
    ///
    /// "SHA256"
    #[serde(rename="SHA256")]
    SHA256,
    

    /// The SHA-1 digest function.
    ///
    /// "SHA1"
    #[serde(rename="SHA1")]
    SHA1,
    

    /// The MD5 digest function.
    ///
    /// "MD5"
    #[serde(rename="MD5")]
    MD5,
    

    /// The Microsoft "VSO-Hash" paged SHA256 digest function. See https://github.com/microsoft/BuildXL/blob/master/Documentation/Specs/PagedHash.md .
    ///
    /// "VSO"
    #[serde(rename="VSO")]
    VSO,
    

    /// The SHA-384 digest function.
    ///
    /// "SHA384"
    #[serde(rename="SHA384")]
    SHA384,
    

    /// The SHA-512 digest function.
    ///
    /// "SHA512"
    #[serde(rename="SHA512")]
    SHA512,
    

    /// Murmur3 128-bit digest function, x64 variant. Note that this is not a cryptographic hash function and its collision properties are not strongly guaranteed. See https://github.com/aappleby/smhasher/wiki/MurmurHash3 .
    ///
    /// "MURMUR3"
    #[serde(rename="MURMUR3")]
    MURMUR3,
}

impl AsRef<str> for BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::UNKNOWN => "UNKNOWN",
            BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::SHA256 => "SHA256",
            BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::SHA1 => "SHA1",
            BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::MD5 => "MD5",
            BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::VSO => "VSO",
            BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::SHA384 => "SHA384",
            BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::SHA512 => "SHA512",
            BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::MURMUR3 => "MURMUR3",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::UNKNOWN),
           "SHA256" => Ok(BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::SHA256),
           "SHA1" => Ok(BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::SHA1),
           "MD5" => Ok(BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::MD5),
           "VSO" => Ok(BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::VSO),
           "SHA384" => Ok(BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::SHA384),
           "SHA512" => Ok(BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::SHA512),
           "MURMUR3" => Ok(BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum::MURMUR3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildBazelRemoteExecutionV2CacheCapabilitySupportedCompressorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Compressors supported by the "compressed-blobs" bytestream resources. Servers MUST support identity/no-compression, even if it is not listed here. Note that this does not imply which if any compressors are supported by the server at the gRPC level.
pub enum BuildBazelRemoteExecutionV2CacheCapabilitySupportedCompressorEnum {
    

    /// No compression. Servers and clients MUST always support this, and do not need to advertise it.
    ///
    /// "IDENTITY"
    #[serde(rename="IDENTITY")]
    IDENTITY,
    

    /// Zstandard compression.
    ///
    /// "ZSTD"
    #[serde(rename="ZSTD")]
    ZSTD,
}

impl AsRef<str> for BuildBazelRemoteExecutionV2CacheCapabilitySupportedCompressorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildBazelRemoteExecutionV2CacheCapabilitySupportedCompressorEnum::IDENTITY => "IDENTITY",
            BuildBazelRemoteExecutionV2CacheCapabilitySupportedCompressorEnum::ZSTD => "ZSTD",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildBazelRemoteExecutionV2CacheCapabilitySupportedCompressorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IDENTITY" => Ok(BuildBazelRemoteExecutionV2CacheCapabilitySupportedCompressorEnum::IDENTITY),
           "ZSTD" => Ok(BuildBazelRemoteExecutionV2CacheCapabilitySupportedCompressorEnum::ZSTD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildBazelRemoteExecutionV2CacheCapabilitySupportedCompressorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether absolute symlink targets are supported.
pub enum BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum {
    

    /// Invalid value.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Server will return an `INVALID_ARGUMENT` on input symlinks with absolute targets. If an action tries to create an output symlink with an absolute target, a `FAILED_PRECONDITION` will be returned.
    ///
    /// "DISALLOWED"
    #[serde(rename="DISALLOWED")]
    DISALLOWED,
    

    /// Server will allow symlink targets to escape the input root tree, possibly resulting in non-hermetic builds.
    ///
    /// "ALLOWED"
    #[serde(rename="ALLOWED")]
    ALLOWED,
}

impl AsRef<str> for BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum::UNKNOWN => "UNKNOWN",
            BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum::DISALLOWED => "DISALLOWED",
            BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum::ALLOWED => "ALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum::UNKNOWN),
           "DISALLOWED" => Ok(BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum::DISALLOWED),
           "ALLOWED" => Ok(BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum::ALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Remote execution may only support a single digest function.
pub enum BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum {
    

    /// It is an error for the server to return this value.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The SHA-256 digest function.
    ///
    /// "SHA256"
    #[serde(rename="SHA256")]
    SHA256,
    

    /// The SHA-1 digest function.
    ///
    /// "SHA1"
    #[serde(rename="SHA1")]
    SHA1,
    

    /// The MD5 digest function.
    ///
    /// "MD5"
    #[serde(rename="MD5")]
    MD5,
    

    /// The Microsoft "VSO-Hash" paged SHA256 digest function. See https://github.com/microsoft/BuildXL/blob/master/Documentation/Specs/PagedHash.md .
    ///
    /// "VSO"
    #[serde(rename="VSO")]
    VSO,
    

    /// The SHA-384 digest function.
    ///
    /// "SHA384"
    #[serde(rename="SHA384")]
    SHA384,
    

    /// The SHA-512 digest function.
    ///
    /// "SHA512"
    #[serde(rename="SHA512")]
    SHA512,
    

    /// Murmur3 128-bit digest function, x64 variant. Note that this is not a cryptographic hash function and its collision properties are not strongly guaranteed. See https://github.com/aappleby/smhasher/wiki/MurmurHash3 .
    ///
    /// "MURMUR3"
    #[serde(rename="MURMUR3")]
    MURMUR3,
}

impl AsRef<str> for BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::UNKNOWN => "UNKNOWN",
            BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::SHA256 => "SHA256",
            BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::SHA1 => "SHA1",
            BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::MD5 => "MD5",
            BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::VSO => "VSO",
            BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::SHA384 => "SHA384",
            BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::SHA512 => "SHA512",
            BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::MURMUR3 => "MURMUR3",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::UNKNOWN),
           "SHA256" => Ok(BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::SHA256),
           "SHA1" => Ok(BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::SHA1),
           "MD5" => Ok(BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::MD5),
           "VSO" => Ok(BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::VSO),
           "SHA384" => Ok(BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::SHA384),
           "SHA512" => Ok(BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::SHA512),
           "MURMUR3" => Ok(BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum::MURMUR3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


