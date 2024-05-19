use super::*;



// region RecognitionConfigEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// *Required* Encoding of audio data sent in all `RecognitionAudio` messages.
pub enum RecognitionConfigEncodingEnum {
    

    /// Not specified. Will return result google.rpc.Code.INVALID_ARGUMENT.
    ///
    /// "ENCODING_UNSPECIFIED"
    #[serde(rename="ENCODING_UNSPECIFIED")]
    ENCODINGUNSPECIFIED,
    

    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
This is the only encoding that may be used by `AsyncRecognize`.
    ///
    /// "LINEAR16"
    #[serde(rename="LINEAR16")]
    LINEAR16,
    

    /// This is the recommended encoding for `SyncRecognize` and
`StreamingRecognize` because it uses lossless compression; therefore
recognition accuracy is not compromised by a lossy codec.

The stream FLAC (Free Lossless Audio Codec) encoding is specified at:
http://flac.sourceforge.net/documentation.html.
16-bit and 24-bit samples are supported.
Not all fields in STREAMINFO are supported.
    ///
    /// "FLAC"
    #[serde(rename="FLAC")]
    FLAC,
    

    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    ///
    /// "MULAW"
    #[serde(rename="MULAW")]
    MULAW,
    

    /// Adaptive Multi-Rate Narrowband codec. `sample_rate` must be 8000 Hz.
    ///
    /// "AMR"
    #[serde(rename="AMR")]
    AMR,
    

    /// Adaptive Multi-Rate Wideband codec. `sample_rate` must be 16000 Hz.
    ///
    /// "AMR_WB"
    #[serde(rename="AMR_WB")]
    AMRWB,
}

impl AsRef<str> for RecognitionConfigEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RecognitionConfigEncodingEnum::ENCODINGUNSPECIFIED => "ENCODING_UNSPECIFIED",
            RecognitionConfigEncodingEnum::LINEAR16 => "LINEAR16",
            RecognitionConfigEncodingEnum::FLAC => "FLAC",
            RecognitionConfigEncodingEnum::MULAW => "MULAW",
            RecognitionConfigEncodingEnum::AMR => "AMR",
            RecognitionConfigEncodingEnum::AMRWB => "AMR_WB",
        }
    }
}

impl std::convert::TryFrom< &str> for RecognitionConfigEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCODING_UNSPECIFIED" => Ok(RecognitionConfigEncodingEnum::ENCODINGUNSPECIFIED),
           "LINEAR16" => Ok(RecognitionConfigEncodingEnum::LINEAR16),
           "FLAC" => Ok(RecognitionConfigEncodingEnum::FLAC),
           "MULAW" => Ok(RecognitionConfigEncodingEnum::MULAW),
           "AMR" => Ok(RecognitionConfigEncodingEnum::AMR),
           "AMR_WB" => Ok(RecognitionConfigEncodingEnum::AMRWB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RecognitionConfigEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


