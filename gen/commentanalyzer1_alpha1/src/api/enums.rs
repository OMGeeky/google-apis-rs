use super::*;



// region AttributeParameterScoreTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// What type of scores to return. If unset, defaults to probability scores.
pub enum AttributeParameterScoreTypeEnum {
    

    /// Unspecified. Defaults to PROBABILITY scores if available, and otherwise
RAW. Every model has a RAW score.
    ///
    /// "SCORE_TYPE_UNSPECIFIED"
    #[serde(rename="SCORE_TYPE_UNSPECIFIED")]
    SCORETYPEUNSPECIFIED,
    

    /// Probability scores are in the range [0, 1] and indicate level of confidence
in the attribute label.
    ///
    /// "PROBABILITY"
    #[serde(rename="PROBABILITY")]
    PROBABILITY,
    

    /// Standard deviation scores are in the range (-inf, +inf).
    ///
    /// "STD_DEV_SCORE"
    #[serde(rename="STD_DEV_SCORE")]
    STDDEVSCORE,
    

    /// Percentile scores are in the range [0, 1] and indicate the percentile of
the raw score, normalized with a test dataset. This is not generally
recommended, as the normalization is dependent on the dataset used, which
may not match other usecases.
    ///
    /// "PERCENTILE"
    #[serde(rename="PERCENTILE")]
    PERCENTILE,
    

    /// Raw scores are the raw values from the model, and may take any value. This
is primarily for debugging/testing, and not generally recommended.
    ///
    /// "RAW"
    #[serde(rename="RAW")]
    RAW,
}

impl AsRef<str> for AttributeParameterScoreTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AttributeParameterScoreTypeEnum::SCORETYPEUNSPECIFIED => "SCORE_TYPE_UNSPECIFIED",
            AttributeParameterScoreTypeEnum::PROBABILITY => "PROBABILITY",
            AttributeParameterScoreTypeEnum::STDDEVSCORE => "STD_DEV_SCORE",
            AttributeParameterScoreTypeEnum::PERCENTILE => "PERCENTILE",
            AttributeParameterScoreTypeEnum::RAW => "RAW",
        }
    }
}

impl std::convert::TryFrom< &str> for AttributeParameterScoreTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCORE_TYPE_UNSPECIFIED" => Ok(AttributeParameterScoreTypeEnum::SCORETYPEUNSPECIFIED),
           "PROBABILITY" => Ok(AttributeParameterScoreTypeEnum::PROBABILITY),
           "STD_DEV_SCORE" => Ok(AttributeParameterScoreTypeEnum::STDDEVSCORE),
           "PERCENTILE" => Ok(AttributeParameterScoreTypeEnum::PERCENTILE),
           "RAW" => Ok(AttributeParameterScoreTypeEnum::RAW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AttributeParameterScoreTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ScoreTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the above value.
pub enum ScoreTypeEnum {
    

    /// Unspecified. Defaults to PROBABILITY scores if available, and otherwise
RAW. Every model has a RAW score.
    ///
    /// "SCORE_TYPE_UNSPECIFIED"
    #[serde(rename="SCORE_TYPE_UNSPECIFIED")]
    SCORETYPEUNSPECIFIED,
    

    /// Probability scores are in the range [0, 1] and indicate level of confidence
in the attribute label.
    ///
    /// "PROBABILITY"
    #[serde(rename="PROBABILITY")]
    PROBABILITY,
    

    /// Standard deviation scores are in the range (-inf, +inf).
    ///
    /// "STD_DEV_SCORE"
    #[serde(rename="STD_DEV_SCORE")]
    STDDEVSCORE,
    

    /// Percentile scores are in the range [0, 1] and indicate the percentile of
the raw score, normalized with a test dataset. This is not generally
recommended, as the normalization is dependent on the dataset used, which
may not match other usecases.
    ///
    /// "PERCENTILE"
    #[serde(rename="PERCENTILE")]
    PERCENTILE,
    

    /// Raw scores are the raw values from the model, and may take any value. This
is primarily for debugging/testing, and not generally recommended.
    ///
    /// "RAW"
    #[serde(rename="RAW")]
    RAW,
}

impl AsRef<str> for ScoreTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScoreTypeEnum::SCORETYPEUNSPECIFIED => "SCORE_TYPE_UNSPECIFIED",
            ScoreTypeEnum::PROBABILITY => "PROBABILITY",
            ScoreTypeEnum::STDDEVSCORE => "STD_DEV_SCORE",
            ScoreTypeEnum::PERCENTILE => "PERCENTILE",
            ScoreTypeEnum::RAW => "RAW",
        }
    }
}

impl std::convert::TryFrom< &str> for ScoreTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCORE_TYPE_UNSPECIFIED" => Ok(ScoreTypeEnum::SCORETYPEUNSPECIFIED),
           "PROBABILITY" => Ok(ScoreTypeEnum::PROBABILITY),
           "STD_DEV_SCORE" => Ok(ScoreTypeEnum::STDDEVSCORE),
           "PERCENTILE" => Ok(ScoreTypeEnum::PERCENTILE),
           "RAW" => Ok(ScoreTypeEnum::RAW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScoreTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TextEntryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the text field.
pub enum TextEntryTypeEnum {
    

    /// The content type is not specified. Text will be interpreted as plain text
by default.
    ///
    /// "TEXT_TYPE_UNSPECIFIED"
    #[serde(rename="TEXT_TYPE_UNSPECIFIED")]
    TEXTTYPEUNSPECIFIED,
    

    /// Plain text.
    ///
    /// "PLAIN_TEXT"
    #[serde(rename="PLAIN_TEXT")]
    PLAINTEXT,
    

    /// HTML.
    ///
    /// "HTML"
    #[serde(rename="HTML")]
    HTML,
}

impl AsRef<str> for TextEntryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TextEntryTypeEnum::TEXTTYPEUNSPECIFIED => "TEXT_TYPE_UNSPECIFIED",
            TextEntryTypeEnum::PLAINTEXT => "PLAIN_TEXT",
            TextEntryTypeEnum::HTML => "HTML",
        }
    }
}

impl std::convert::TryFrom< &str> for TextEntryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEXT_TYPE_UNSPECIFIED" => Ok(TextEntryTypeEnum::TEXTTYPEUNSPECIFIED),
           "PLAIN_TEXT" => Ok(TextEntryTypeEnum::PLAINTEXT),
           "HTML" => Ok(TextEntryTypeEnum::HTML),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TextEntryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


