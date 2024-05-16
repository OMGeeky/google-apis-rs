use super::*;



// region CustomClasStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The CustomClass lifecycle state. This field is not used.
pub enum CustomClasStateEnum {
    

    /// Unspecified state. This is only used/useful for distinguishing unset values.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The normal and active state.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// This CustomClass has been deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for CustomClasStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomClasStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CustomClasStateEnum::ACTIVE => "ACTIVE",
            CustomClasStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomClasStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CustomClasStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(CustomClasStateEnum::ACTIVE),
           "DELETED" => Ok(CustomClasStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomClasStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PhraseSetStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The CustomClass lifecycle state. This field is not used.
pub enum PhraseSetStateEnum {
    

    /// Unspecified state. This is only used/useful for distinguishing unset values.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The normal and active state.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// This CustomClass has been deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for PhraseSetStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PhraseSetStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            PhraseSetStateEnum::ACTIVE => "ACTIVE",
            PhraseSetStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for PhraseSetStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(PhraseSetStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(PhraseSetStateEnum::ACTIVE),
           "DELETED" => Ok(PhraseSetStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PhraseSetStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RecognitionConfigEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Encoding of audio data sent in all `RecognitionAudio` messages. This field is optional for `FLAC` and `WAV` audio files and required for all other audio formats. For details, see AudioEncoding.
pub enum RecognitionConfigEncodingEnum {
    

    /// Not specified.
    ///
    /// "ENCODING_UNSPECIFIED"
    #[serde(rename="ENCODING_UNSPECIFIED")]
    ENCODINGUNSPECIFIED,
    

    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    ///
    /// "LINEAR16"
    #[serde(rename="LINEAR16")]
    LINEAR16,
    

    /// `FLAC` (Free Lossless Audio Codec) is the recommended encoding because it is lossless--therefore recognition is not compromised--and requires only about half the bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and 24-bit samples, however, not all fields in `STREAMINFO` are supported.
    ///
    /// "FLAC"
    #[serde(rename="FLAC")]
    FLAC,
    

    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    ///
    /// "MULAW"
    #[serde(rename="MULAW")]
    MULAW,
    

    /// Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000.
    ///
    /// "AMR"
    #[serde(rename="AMR")]
    AMR,
    

    /// Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000.
    ///
    /// "AMR_WB"
    #[serde(rename="AMR_WB")]
    AMRWB,
    

    /// Opus encoded audio frames in Ogg container ([OggOpus](https://wiki.xiph.org/OggOpus)). `sample_rate_hertz` must be one of 8000, 12000, 16000, 24000, or 48000.
    ///
    /// "OGG_OPUS"
    #[serde(rename="OGG_OPUS")]
    OGGOPUS,
    

    /// Although the use of lossy encodings is not recommended, if a very low bitrate encoding is required, `OGG_OPUS` is highly preferred over Speex encoding. The [Speex](https://speex.org/) encoding supported by Cloud Speech API has a header byte in each block, as in MIME type `audio/x-speex-with-header-byte`. It is a variant of the RTP Speex encoding defined in [RFC 5574](https://tools.ietf.org/html/rfc5574). The stream is a sequence of blocks, one block per RTP packet. Each block starts with a byte containing the length of the block, in bytes, followed by one or more frames of Speex data, padded to an integral number of bytes (octets) as specified in RFC 5574. In other words, each RTP header is replaced with a single byte containing the block length. Only Speex wideband is supported. `sample_rate_hertz` must be 16000.
    ///
    /// "SPEEX_WITH_HEADER_BYTE"
    #[serde(rename="SPEEX_WITH_HEADER_BYTE")]
    SPEEXWITHHEADERBYTE,
    

    /// MP3 audio. MP3 encoding is a Beta feature and only available in v1p1beta1. Support all standard MP3 bitrates (which range from 32-320 kbps). When using this encoding, `sample_rate_hertz` has to match the sample rate of the file being used.
    ///
    /// "MP3"
    #[serde(rename="MP3")]
    MP3,
    

    /// Opus encoded audio frames in WebM container ([WebM](https://www.webmproject.org/docs/container/)). `sample_rate_hertz` must be one of 8000, 12000, 16000, 24000, or 48000.
    ///
    /// "WEBM_OPUS"
    #[serde(rename="WEBM_OPUS")]
    WEBMOPUS,
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
            RecognitionConfigEncodingEnum::OGGOPUS => "OGG_OPUS",
            RecognitionConfigEncodingEnum::SPEEXWITHHEADERBYTE => "SPEEX_WITH_HEADER_BYTE",
            RecognitionConfigEncodingEnum::MP3 => "MP3",
            RecognitionConfigEncodingEnum::WEBMOPUS => "WEBM_OPUS",
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
           "OGG_OPUS" => Ok(RecognitionConfigEncodingEnum::OGGOPUS),
           "SPEEX_WITH_HEADER_BYTE" => Ok(RecognitionConfigEncodingEnum::SPEEXWITHHEADERBYTE),
           "MP3" => Ok(RecognitionConfigEncodingEnum::MP3),
           "WEBM_OPUS" => Ok(RecognitionConfigEncodingEnum::WEBMOPUS),
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


// region RecognitionMetadataInteractionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The use case most closely describing the audio content to be recognized.
pub enum RecognitionMetadataInteractionTypeEnum {
    

    /// Use case is either unknown or is something other than one of the other values below.
    ///
    /// "INTERACTION_TYPE_UNSPECIFIED"
    #[serde(rename="INTERACTION_TYPE_UNSPECIFIED")]
    INTERACTIONTYPEUNSPECIFIED,
    

    /// Multiple people in a conversation or discussion. For example in a meeting with two or more people actively participating. Typically all the primary people speaking would be in the same room (if not, see PHONE_CALL)
    ///
    /// "DISCUSSION"
    #[serde(rename="DISCUSSION")]
    DISCUSSION,
    

    /// One or more persons lecturing or presenting to others, mostly uninterrupted.
    ///
    /// "PRESENTATION"
    #[serde(rename="PRESENTATION")]
    PRESENTATION,
    

    /// A phone-call or video-conference in which two or more people, who are not in the same room, are actively participating.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// A recorded message intended for another person to listen to.
    ///
    /// "VOICEMAIL"
    #[serde(rename="VOICEMAIL")]
    VOICEMAIL,
    

    /// Professionally produced audio (eg. TV Show, Podcast).
    ///
    /// "PROFESSIONALLY_PRODUCED"
    #[serde(rename="PROFESSIONALLY_PRODUCED")]
    PROFESSIONALLYPRODUCED,
    

    /// Transcribe spoken questions and queries into text.
    ///
    /// "VOICE_SEARCH"
    #[serde(rename="VOICE_SEARCH")]
    VOICESEARCH,
    

    /// Transcribe voice commands, such as for controlling a device.
    ///
    /// "VOICE_COMMAND"
    #[serde(rename="VOICE_COMMAND")]
    VOICECOMMAND,
    

    /// Transcribe speech to text to create a written document, such as a text-message, email or report.
    ///
    /// "DICTATION"
    #[serde(rename="DICTATION")]
    DICTATION,
}

impl AsRef<str> for RecognitionMetadataInteractionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RecognitionMetadataInteractionTypeEnum::INTERACTIONTYPEUNSPECIFIED => "INTERACTION_TYPE_UNSPECIFIED",
            RecognitionMetadataInteractionTypeEnum::DISCUSSION => "DISCUSSION",
            RecognitionMetadataInteractionTypeEnum::PRESENTATION => "PRESENTATION",
            RecognitionMetadataInteractionTypeEnum::PHONECALL => "PHONE_CALL",
            RecognitionMetadataInteractionTypeEnum::VOICEMAIL => "VOICEMAIL",
            RecognitionMetadataInteractionTypeEnum::PROFESSIONALLYPRODUCED => "PROFESSIONALLY_PRODUCED",
            RecognitionMetadataInteractionTypeEnum::VOICESEARCH => "VOICE_SEARCH",
            RecognitionMetadataInteractionTypeEnum::VOICECOMMAND => "VOICE_COMMAND",
            RecognitionMetadataInteractionTypeEnum::DICTATION => "DICTATION",
        }
    }
}

impl std::convert::TryFrom< &str> for RecognitionMetadataInteractionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTERACTION_TYPE_UNSPECIFIED" => Ok(RecognitionMetadataInteractionTypeEnum::INTERACTIONTYPEUNSPECIFIED),
           "DISCUSSION" => Ok(RecognitionMetadataInteractionTypeEnum::DISCUSSION),
           "PRESENTATION" => Ok(RecognitionMetadataInteractionTypeEnum::PRESENTATION),
           "PHONE_CALL" => Ok(RecognitionMetadataInteractionTypeEnum::PHONECALL),
           "VOICEMAIL" => Ok(RecognitionMetadataInteractionTypeEnum::VOICEMAIL),
           "PROFESSIONALLY_PRODUCED" => Ok(RecognitionMetadataInteractionTypeEnum::PROFESSIONALLYPRODUCED),
           "VOICE_SEARCH" => Ok(RecognitionMetadataInteractionTypeEnum::VOICESEARCH),
           "VOICE_COMMAND" => Ok(RecognitionMetadataInteractionTypeEnum::VOICECOMMAND),
           "DICTATION" => Ok(RecognitionMetadataInteractionTypeEnum::DICTATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RecognitionMetadataInteractionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RecognitionMetadataMicrophoneDistanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The audio type that most closely describes the audio being recognized.
pub enum RecognitionMetadataMicrophoneDistanceEnum {
    

    /// Audio type is not known.
    ///
    /// "MICROPHONE_DISTANCE_UNSPECIFIED"
    #[serde(rename="MICROPHONE_DISTANCE_UNSPECIFIED")]
    MICROPHONEDISTANCEUNSPECIFIED,
    

    /// The audio was captured from a closely placed microphone. Eg. phone, dictaphone, or handheld microphone. Generally if there speaker is within 1 meter of the microphone.
    ///
    /// "NEARFIELD"
    #[serde(rename="NEARFIELD")]
    NEARFIELD,
    

    /// The speaker if within 3 meters of the microphone.
    ///
    /// "MIDFIELD"
    #[serde(rename="MIDFIELD")]
    MIDFIELD,
    

    /// The speaker is more than 3 meters away from the microphone.
    ///
    /// "FARFIELD"
    #[serde(rename="FARFIELD")]
    FARFIELD,
}

impl AsRef<str> for RecognitionMetadataMicrophoneDistanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RecognitionMetadataMicrophoneDistanceEnum::MICROPHONEDISTANCEUNSPECIFIED => "MICROPHONE_DISTANCE_UNSPECIFIED",
            RecognitionMetadataMicrophoneDistanceEnum::NEARFIELD => "NEARFIELD",
            RecognitionMetadataMicrophoneDistanceEnum::MIDFIELD => "MIDFIELD",
            RecognitionMetadataMicrophoneDistanceEnum::FARFIELD => "FARFIELD",
        }
    }
}

impl std::convert::TryFrom< &str> for RecognitionMetadataMicrophoneDistanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MICROPHONE_DISTANCE_UNSPECIFIED" => Ok(RecognitionMetadataMicrophoneDistanceEnum::MICROPHONEDISTANCEUNSPECIFIED),
           "NEARFIELD" => Ok(RecognitionMetadataMicrophoneDistanceEnum::NEARFIELD),
           "MIDFIELD" => Ok(RecognitionMetadataMicrophoneDistanceEnum::MIDFIELD),
           "FARFIELD" => Ok(RecognitionMetadataMicrophoneDistanceEnum::FARFIELD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RecognitionMetadataMicrophoneDistanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RecognitionMetadataOriginalMediaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The original media the speech was recorded on.
pub enum RecognitionMetadataOriginalMediaTypeEnum {
    

    /// Unknown original media type.
    ///
    /// "ORIGINAL_MEDIA_TYPE_UNSPECIFIED"
    #[serde(rename="ORIGINAL_MEDIA_TYPE_UNSPECIFIED")]
    ORIGINALMEDIATYPEUNSPECIFIED,
    

    /// The speech data is an audio recording.
    ///
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
    

    /// The speech data originally recorded on a video.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
}

impl AsRef<str> for RecognitionMetadataOriginalMediaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RecognitionMetadataOriginalMediaTypeEnum::ORIGINALMEDIATYPEUNSPECIFIED => "ORIGINAL_MEDIA_TYPE_UNSPECIFIED",
            RecognitionMetadataOriginalMediaTypeEnum::AUDIO => "AUDIO",
            RecognitionMetadataOriginalMediaTypeEnum::VIDEO => "VIDEO",
        }
    }
}

impl std::convert::TryFrom< &str> for RecognitionMetadataOriginalMediaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORIGINAL_MEDIA_TYPE_UNSPECIFIED" => Ok(RecognitionMetadataOriginalMediaTypeEnum::ORIGINALMEDIATYPEUNSPECIFIED),
           "AUDIO" => Ok(RecognitionMetadataOriginalMediaTypeEnum::AUDIO),
           "VIDEO" => Ok(RecognitionMetadataOriginalMediaTypeEnum::VIDEO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RecognitionMetadataOriginalMediaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RecognitionMetadataRecordingDeviceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of device the speech was recorded with.
pub enum RecognitionMetadataRecordingDeviceTypeEnum {
    

    /// The recording device is unknown.
    ///
    /// "RECORDING_DEVICE_TYPE_UNSPECIFIED"
    #[serde(rename="RECORDING_DEVICE_TYPE_UNSPECIFIED")]
    RECORDINGDEVICETYPEUNSPECIFIED,
    

    /// Speech was recorded on a smartphone.
    ///
    /// "SMARTPHONE"
    #[serde(rename="SMARTPHONE")]
    SMARTPHONE,
    

    /// Speech was recorded using a personal computer or tablet.
    ///
    /// "PC"
    #[serde(rename="PC")]
    PC,
    

    /// Speech was recorded over a phone line.
    ///
    /// "PHONE_LINE"
    #[serde(rename="PHONE_LINE")]
    PHONELINE,
    

    /// Speech was recorded in a vehicle.
    ///
    /// "VEHICLE"
    #[serde(rename="VEHICLE")]
    VEHICLE,
    

    /// Speech was recorded outdoors.
    ///
    /// "OTHER_OUTDOOR_DEVICE"
    #[serde(rename="OTHER_OUTDOOR_DEVICE")]
    OTHEROUTDOORDEVICE,
    

    /// Speech was recorded indoors.
    ///
    /// "OTHER_INDOOR_DEVICE"
    #[serde(rename="OTHER_INDOOR_DEVICE")]
    OTHERINDOORDEVICE,
}

impl AsRef<str> for RecognitionMetadataRecordingDeviceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RecognitionMetadataRecordingDeviceTypeEnum::RECORDINGDEVICETYPEUNSPECIFIED => "RECORDING_DEVICE_TYPE_UNSPECIFIED",
            RecognitionMetadataRecordingDeviceTypeEnum::SMARTPHONE => "SMARTPHONE",
            RecognitionMetadataRecordingDeviceTypeEnum::PC => "PC",
            RecognitionMetadataRecordingDeviceTypeEnum::PHONELINE => "PHONE_LINE",
            RecognitionMetadataRecordingDeviceTypeEnum::VEHICLE => "VEHICLE",
            RecognitionMetadataRecordingDeviceTypeEnum::OTHEROUTDOORDEVICE => "OTHER_OUTDOOR_DEVICE",
            RecognitionMetadataRecordingDeviceTypeEnum::OTHERINDOORDEVICE => "OTHER_INDOOR_DEVICE",
        }
    }
}

impl std::convert::TryFrom< &str> for RecognitionMetadataRecordingDeviceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECORDING_DEVICE_TYPE_UNSPECIFIED" => Ok(RecognitionMetadataRecordingDeviceTypeEnum::RECORDINGDEVICETYPEUNSPECIFIED),
           "SMARTPHONE" => Ok(RecognitionMetadataRecordingDeviceTypeEnum::SMARTPHONE),
           "PC" => Ok(RecognitionMetadataRecordingDeviceTypeEnum::PC),
           "PHONE_LINE" => Ok(RecognitionMetadataRecordingDeviceTypeEnum::PHONELINE),
           "VEHICLE" => Ok(RecognitionMetadataRecordingDeviceTypeEnum::VEHICLE),
           "OTHER_OUTDOOR_DEVICE" => Ok(RecognitionMetadataRecordingDeviceTypeEnum::OTHEROUTDOORDEVICE),
           "OTHER_INDOOR_DEVICE" => Ok(RecognitionMetadataRecordingDeviceTypeEnum::OTHERINDOORDEVICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RecognitionMetadataRecordingDeviceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


