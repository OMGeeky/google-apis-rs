use super::*;



// region AudioConfigAudioEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The format of the audio byte stream.
pub enum AudioConfigAudioEncodingEnum {
    

    /// Not specified. Will return result google.rpc.Code.INVALID_ARGUMENT.
    ///
    /// "AUDIO_ENCODING_UNSPECIFIED"
    #[serde(rename="AUDIO_ENCODING_UNSPECIFIED")]
    AUDIOENCODINGUNSPECIFIED,
    

    /// Uncompressed 16-bit signed little-endian samples (Linear PCM). Audio content returned as LINEAR16 also contains a WAV header.
    ///
    /// "LINEAR16"
    #[serde(rename="LINEAR16")]
    LINEAR16,
    

    /// MP3 audio at 32kbps.
    ///
    /// "MP3"
    #[serde(rename="MP3")]
    MP3,
    

    /// Opus encoded audio wrapped in an ogg container. The result will be a file which can be played natively on Android, and in browsers (at least Chrome and Firefox). The quality of the encoding is considerably higher than MP3 while using approximately the same bitrate.
    ///
    /// "OGG_OPUS"
    #[serde(rename="OGG_OPUS")]
    OGGOPUS,
    

    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law. Audio content returned as MULAW also contains a WAV header.
    ///
    /// "MULAW"
    #[serde(rename="MULAW")]
    MULAW,
    

    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/A-law. Audio content returned as ALAW also contains a WAV header.
    ///
    /// "ALAW"
    #[serde(rename="ALAW")]
    ALAW,
}

impl AsRef<str> for AudioConfigAudioEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AudioConfigAudioEncodingEnum::AUDIOENCODINGUNSPECIFIED => "AUDIO_ENCODING_UNSPECIFIED",
            AudioConfigAudioEncodingEnum::LINEAR16 => "LINEAR16",
            AudioConfigAudioEncodingEnum::MP3 => "MP3",
            AudioConfigAudioEncodingEnum::OGGOPUS => "OGG_OPUS",
            AudioConfigAudioEncodingEnum::MULAW => "MULAW",
            AudioConfigAudioEncodingEnum::ALAW => "ALAW",
        }
    }
}

impl std::convert::TryFrom< &str> for AudioConfigAudioEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUDIO_ENCODING_UNSPECIFIED" => Ok(AudioConfigAudioEncodingEnum::AUDIOENCODINGUNSPECIFIED),
           "LINEAR16" => Ok(AudioConfigAudioEncodingEnum::LINEAR16),
           "MP3" => Ok(AudioConfigAudioEncodingEnum::MP3),
           "OGG_OPUS" => Ok(AudioConfigAudioEncodingEnum::OGGOPUS),
           "MULAW" => Ok(AudioConfigAudioEncodingEnum::MULAW),
           "ALAW" => Ok(AudioConfigAudioEncodingEnum::ALAW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AudioConfigAudioEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomVoiceParamReportedUsageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Deprecated. The usage of the synthesized audio to be reported.
pub enum CustomVoiceParamReportedUsageEnum {
    

    /// Request with reported usage unspecified will be rejected.
    ///
    /// "REPORTED_USAGE_UNSPECIFIED"
    #[serde(rename="REPORTED_USAGE_UNSPECIFIED")]
    REPORTEDUSAGEUNSPECIFIED,
    

    /// For scenarios where the synthesized audio is not downloadable and can only be used once. For example, real-time request in IVR system.
    ///
    /// "REALTIME"
    #[serde(rename="REALTIME")]
    REALTIME,
    

    /// For scenarios where the synthesized audio is downloadable and can be reused. For example, the synthesized audio is downloaded, stored in customer service system and played repeatedly.
    ///
    /// "OFFLINE"
    #[serde(rename="OFFLINE")]
    OFFLINE,
}

impl AsRef<str> for CustomVoiceParamReportedUsageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomVoiceParamReportedUsageEnum::REPORTEDUSAGEUNSPECIFIED => "REPORTED_USAGE_UNSPECIFIED",
            CustomVoiceParamReportedUsageEnum::REALTIME => "REALTIME",
            CustomVoiceParamReportedUsageEnum::OFFLINE => "OFFLINE",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomVoiceParamReportedUsageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPORTED_USAGE_UNSPECIFIED" => Ok(CustomVoiceParamReportedUsageEnum::REPORTEDUSAGEUNSPECIFIED),
           "REALTIME" => Ok(CustomVoiceParamReportedUsageEnum::REALTIME),
           "OFFLINE" => Ok(CustomVoiceParamReportedUsageEnum::OFFLINE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomVoiceParamReportedUsageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VoiceSsmlGenderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The gender of this voice.
pub enum VoiceSsmlGenderEnum {
    

    /// An unspecified gender. In VoiceSelectionParams, this means that the client doesn't care which gender the selected voice will have. In the Voice field of ListVoicesResponse, this may mean that the voice doesn't fit any of the other categories in this enum, or that the gender of the voice isn't known.
    ///
    /// "SSML_VOICE_GENDER_UNSPECIFIED"
    #[serde(rename="SSML_VOICE_GENDER_UNSPECIFIED")]
    SSMLVOICEGENDERUNSPECIFIED,
    

    /// A male voice.
    ///
    /// "MALE"
    #[serde(rename="MALE")]
    MALE,
    

    /// A female voice.
    ///
    /// "FEMALE"
    #[serde(rename="FEMALE")]
    FEMALE,
    

    /// A gender-neutral voice. This voice is not yet supported.
    ///
    /// "NEUTRAL"
    #[serde(rename="NEUTRAL")]
    NEUTRAL,
}

impl AsRef<str> for VoiceSsmlGenderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VoiceSsmlGenderEnum::SSMLVOICEGENDERUNSPECIFIED => "SSML_VOICE_GENDER_UNSPECIFIED",
            VoiceSsmlGenderEnum::MALE => "MALE",
            VoiceSsmlGenderEnum::FEMALE => "FEMALE",
            VoiceSsmlGenderEnum::NEUTRAL => "NEUTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for VoiceSsmlGenderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSML_VOICE_GENDER_UNSPECIFIED" => Ok(VoiceSsmlGenderEnum::SSMLVOICEGENDERUNSPECIFIED),
           "MALE" => Ok(VoiceSsmlGenderEnum::MALE),
           "FEMALE" => Ok(VoiceSsmlGenderEnum::FEMALE),
           "NEUTRAL" => Ok(VoiceSsmlGenderEnum::NEUTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VoiceSsmlGenderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VoiceSelectionParamSsmlGenderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement; if a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request.
pub enum VoiceSelectionParamSsmlGenderEnum {
    

    /// An unspecified gender. In VoiceSelectionParams, this means that the client doesn't care which gender the selected voice will have. In the Voice field of ListVoicesResponse, this may mean that the voice doesn't fit any of the other categories in this enum, or that the gender of the voice isn't known.
    ///
    /// "SSML_VOICE_GENDER_UNSPECIFIED"
    #[serde(rename="SSML_VOICE_GENDER_UNSPECIFIED")]
    SSMLVOICEGENDERUNSPECIFIED,
    

    /// A male voice.
    ///
    /// "MALE"
    #[serde(rename="MALE")]
    MALE,
    

    /// A female voice.
    ///
    /// "FEMALE"
    #[serde(rename="FEMALE")]
    FEMALE,
    

    /// A gender-neutral voice. This voice is not yet supported.
    ///
    /// "NEUTRAL"
    #[serde(rename="NEUTRAL")]
    NEUTRAL,
}

impl AsRef<str> for VoiceSelectionParamSsmlGenderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERUNSPECIFIED => "SSML_VOICE_GENDER_UNSPECIFIED",
            VoiceSelectionParamSsmlGenderEnum::MALE => "MALE",
            VoiceSelectionParamSsmlGenderEnum::FEMALE => "FEMALE",
            VoiceSelectionParamSsmlGenderEnum::NEUTRAL => "NEUTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for VoiceSelectionParamSsmlGenderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSML_VOICE_GENDER_UNSPECIFIED" => Ok(VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERUNSPECIFIED),
           "MALE" => Ok(VoiceSelectionParamSsmlGenderEnum::MALE),
           "FEMALE" => Ok(VoiceSelectionParamSsmlGenderEnum::FEMALE),
           "NEUTRAL" => Ok(VoiceSelectionParamSsmlGenderEnum::NEUTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VoiceSelectionParamSsmlGenderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


