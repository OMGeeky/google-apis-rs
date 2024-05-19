use super::*;



// region GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum {
    
    /// "CHARACTER_GROUP_UNSPECIFIED"
    #[serde(rename="CHARACTER_GROUP_UNSPECIFIED")]
    CHARACTERGROUPUNSPECIFIED,
    

    /// 0-9
    ///
    /// "NUMERIC"
    #[serde(rename="NUMERIC")]
    NUMERIC,
    

    /// A-Z
    ///
    /// "ALPHA_UPPER_CASE"
    #[serde(rename="ALPHA_UPPER_CASE")]
    ALPHAUPPERCASE,
    

    /// a-z
    ///
    /// "ALPHA_LOWER_CASE"
    #[serde(rename="ALPHA_LOWER_CASE")]
    ALPHALOWERCASE,
    

    /// US Punctuation, one of !"#$%&'()*+,-./:;<=>?@[\]^_`{|}~
    ///
    /// "PUNCTUATION"
    #[serde(rename="PUNCTUATION")]
    PUNCTUATION,
    

    /// Whitespace character, one of [ \t\n\x0B\f\r]
    ///
    /// "WHITESPACE"
    #[serde(rename="WHITESPACE")]
    WHITESPACE,
}

impl AsRef<str> for GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::CHARACTERGROUPUNSPECIFIED => "CHARACTER_GROUP_UNSPECIFIED",
            GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::NUMERIC => "NUMERIC",
            GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::ALPHAUPPERCASE => "ALPHA_UPPER_CASE",
            GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::ALPHALOWERCASE => "ALPHA_LOWER_CASE",
            GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::PUNCTUATION => "PUNCTUATION",
            GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::WHITESPACE => "WHITESPACE",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHARACTER_GROUP_UNSPECIFIED" => Ok(GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::CHARACTERGROUPUNSPECIFIED),
           "NUMERIC" => Ok(GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::NUMERIC),
           "ALPHA_UPPER_CASE" => Ok(GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::ALPHAUPPERCASE),
           "ALPHA_LOWER_CASE" => Ok(GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::ALPHALOWERCASE),
           "PUNCTUATION" => Ok(GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::PUNCTUATION),
           "WHITESPACE" => Ok(GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum::WHITESPACE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2beta1CharsToIgnoreCommonCharactersToIgnoreEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2beta1FindingLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Estimate of how likely it is that the info_type is correct.
pub enum GooglePrivacyDlpV2beta1FindingLikelihoodEnum {
    

    /// Default value; information with all likelihoods is included.
    ///
    /// "LIKELIHOOD_UNSPECIFIED"
    #[serde(rename="LIKELIHOOD_UNSPECIFIED")]
    LIKELIHOODUNSPECIFIED,
    

    /// Few matching elements.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// Some matching elements.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// Many matching elements.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for GooglePrivacyDlpV2beta1FindingLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2beta1FindingLikelihoodEnum::LIKELIHOODUNSPECIFIED => "LIKELIHOOD_UNSPECIFIED",
            GooglePrivacyDlpV2beta1FindingLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            GooglePrivacyDlpV2beta1FindingLikelihoodEnum::UNLIKELY => "UNLIKELY",
            GooglePrivacyDlpV2beta1FindingLikelihoodEnum::POSSIBLE => "POSSIBLE",
            GooglePrivacyDlpV2beta1FindingLikelihoodEnum::LIKELY => "LIKELY",
            GooglePrivacyDlpV2beta1FindingLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2beta1FindingLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIKELIHOOD_UNSPECIFIED" => Ok(GooglePrivacyDlpV2beta1FindingLikelihoodEnum::LIKELIHOODUNSPECIFIED),
           "VERY_UNLIKELY" => Ok(GooglePrivacyDlpV2beta1FindingLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(GooglePrivacyDlpV2beta1FindingLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(GooglePrivacyDlpV2beta1FindingLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(GooglePrivacyDlpV2beta1FindingLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(GooglePrivacyDlpV2beta1FindingLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2beta1FindingLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2beta1ExpressionLogicalOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator to apply to the result of conditions. Default and currently
only supported value is `AND`.
pub enum GooglePrivacyDlpV2beta1ExpressionLogicalOperatorEnum {
    
    /// "LOGICAL_OPERATOR_UNSPECIFIED"
    #[serde(rename="LOGICAL_OPERATOR_UNSPECIFIED")]
    LOGICALOPERATORUNSPECIFIED,
    
    /// "AND"
    #[serde(rename="AND")]
    AND,
}

impl AsRef<str> for GooglePrivacyDlpV2beta1ExpressionLogicalOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2beta1ExpressionLogicalOperatorEnum::LOGICALOPERATORUNSPECIFIED => "LOGICAL_OPERATOR_UNSPECIFIED",
            GooglePrivacyDlpV2beta1ExpressionLogicalOperatorEnum::AND => "AND",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2beta1ExpressionLogicalOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOGICAL_OPERATOR_UNSPECIFIED" => Ok(GooglePrivacyDlpV2beta1ExpressionLogicalOperatorEnum::LOGICALOPERATORUNSPECIFIED),
           "AND" => Ok(GooglePrivacyDlpV2beta1ExpressionLogicalOperatorEnum::AND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2beta1ExpressionLogicalOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum {
    
    /// "TIME_PART_UNSPECIFIED"
    #[serde(rename="TIME_PART_UNSPECIFIED")]
    TIMEPARTUNSPECIFIED,
    

    /// [000-9999]
    ///
    /// "YEAR"
    #[serde(rename="YEAR")]
    YEAR,
    

    /// [1-12]
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// [1-31]
    ///
    /// "DAY_OF_MONTH"
    #[serde(rename="DAY_OF_MONTH")]
    DAYOFMONTH,
    

    /// [1-7]
    ///
    /// "DAY_OF_WEEK"
    #[serde(rename="DAY_OF_WEEK")]
    DAYOFWEEK,
    

    /// [1-52]
    ///
    /// "WEEK_OF_YEAR"
    #[serde(rename="WEEK_OF_YEAR")]
    WEEKOFYEAR,
    

    /// [0-24]
    ///
    /// "HOUR_OF_DAY"
    #[serde(rename="HOUR_OF_DAY")]
    HOUROFDAY,
}

impl AsRef<str> for GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::TIMEPARTUNSPECIFIED => "TIME_PART_UNSPECIFIED",
            GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::YEAR => "YEAR",
            GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::MONTH => "MONTH",
            GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::DAYOFMONTH => "DAY_OF_MONTH",
            GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::DAYOFWEEK => "DAY_OF_WEEK",
            GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::WEEKOFYEAR => "WEEK_OF_YEAR",
            GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::HOUROFDAY => "HOUR_OF_DAY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_PART_UNSPECIFIED" => Ok(GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::TIMEPARTUNSPECIFIED),
           "YEAR" => Ok(GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::YEAR),
           "MONTH" => Ok(GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::MONTH),
           "DAY_OF_MONTH" => Ok(GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::DAYOFMONTH),
           "DAY_OF_WEEK" => Ok(GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::DAYOFWEEK),
           "WEEK_OF_YEAR" => Ok(GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::WEEKOFYEAR),
           "HOUR_OF_DAY" => Ok(GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum::HOUROFDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2beta1TimePartConfigPartToExtractEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Only returns findings equal or above this threshold.
pub enum GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum {
    

    /// Default value; information with all likelihoods is included.
    ///
    /// "LIKELIHOOD_UNSPECIFIED"
    #[serde(rename="LIKELIHOOD_UNSPECIFIED")]
    LIKELIHOODUNSPECIFIED,
    

    /// Few matching elements.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// Some matching elements.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// Many matching elements.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::LIKELIHOODUNSPECIFIED => "LIKELIHOOD_UNSPECIFIED",
            GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::UNLIKELY => "UNLIKELY",
            GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::POSSIBLE => "POSSIBLE",
            GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::LIKELY => "LIKELY",
            GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIKELIHOOD_UNSPECIFIED" => Ok(GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::LIKELIHOODUNSPECIFIED),
           "VERY_UNLIKELY" => Ok(GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2beta1InspectConfigMinLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2beta1SummaryResultCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum GooglePrivacyDlpV2beta1SummaryResultCodeEnum {
    
    /// "TRANSFORMATION_RESULT_CODE_UNSPECIFIED"
    #[serde(rename="TRANSFORMATION_RESULT_CODE_UNSPECIFIED")]
    TRANSFORMATIONRESULTCODEUNSPECIFIED,
    
    /// "SUCCESS"
    #[serde(rename="SUCCESS")]
    SUCCESS,
    
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for GooglePrivacyDlpV2beta1SummaryResultCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2beta1SummaryResultCodeEnum::TRANSFORMATIONRESULTCODEUNSPECIFIED => "TRANSFORMATION_RESULT_CODE_UNSPECIFIED",
            GooglePrivacyDlpV2beta1SummaryResultCodeEnum::SUCCESS => "SUCCESS",
            GooglePrivacyDlpV2beta1SummaryResultCodeEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2beta1SummaryResultCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFORMATION_RESULT_CODE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2beta1SummaryResultCodeEnum::TRANSFORMATIONRESULTCODEUNSPECIFIED),
           "SUCCESS" => Ok(GooglePrivacyDlpV2beta1SummaryResultCodeEnum::SUCCESS),
           "ERROR" => Ok(GooglePrivacyDlpV2beta1SummaryResultCodeEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2beta1SummaryResultCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
    
    /// "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED"
    #[serde(rename="FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED")]
    FFXCOMMONNATIVEALPHABETUNSPECIFIED,
    

    /// [0-9] (radix of 10)
    ///
    /// "NUMERIC"
    #[serde(rename="NUMERIC")]
    NUMERIC,
    

    /// [0-9A-F] (radix of 16)
    ///
    /// "HEXADECIMAL"
    #[serde(rename="HEXADECIMAL")]
    HEXADECIMAL,
    

    /// [0-9A-Z] (radix of 36)
    ///
    /// "UPPER_CASE_ALPHA_NUMERIC"
    #[serde(rename="UPPER_CASE_ALPHA_NUMERIC")]
    UPPERCASEALPHANUMERIC,
    

    /// [0-9A-Za-z] (radix of 62)
    ///
    /// "ALPHA_NUMERIC"
    #[serde(rename="ALPHA_NUMERIC")]
    ALPHANUMERIC,
}

impl AsRef<str> for GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum::FFXCOMMONNATIVEALPHABETUNSPECIFIED => "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED",
            GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum::NUMERIC => "NUMERIC",
            GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum::HEXADECIMAL => "HEXADECIMAL",
            GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum::UPPERCASEALPHANUMERIC => "UPPER_CASE_ALPHA_NUMERIC",
            GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum::ALPHANUMERIC => "ALPHA_NUMERIC",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED" => Ok(GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum::FFXCOMMONNATIVEALPHABETUNSPECIFIED),
           "NUMERIC" => Ok(GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum::NUMERIC),
           "HEXADECIMAL" => Ok(GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum::HEXADECIMAL),
           "UPPER_CASE_ALPHA_NUMERIC" => Ok(GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum::UPPERCASEALPHANUMERIC),
           "ALPHA_NUMERIC" => Ok(GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum::ALPHANUMERIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2beta1ConditionOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Operator used to compare the field or info type to the value. [required]
pub enum GooglePrivacyDlpV2beta1ConditionOperatorEnum {
    
    /// "RELATIONAL_OPERATOR_UNSPECIFIED"
    #[serde(rename="RELATIONAL_OPERATOR_UNSPECIFIED")]
    RELATIONALOPERATORUNSPECIFIED,
    

    /// Equal.
    ///
    /// "EQUAL_TO"
    #[serde(rename="EQUAL_TO")]
    EQUALTO,
    

    /// Not equal to.
    ///
    /// "NOT_EQUAL_TO"
    #[serde(rename="NOT_EQUAL_TO")]
    NOTEQUALTO,
    

    /// Greater than.
    ///
    /// "GREATER_THAN"
    #[serde(rename="GREATER_THAN")]
    GREATERTHAN,
    

    /// Less than.
    ///
    /// "LESS_THAN"
    #[serde(rename="LESS_THAN")]
    LESSTHAN,
    

    /// Greater than or equals.
    ///
    /// "GREATER_THAN_OR_EQUALS"
    #[serde(rename="GREATER_THAN_OR_EQUALS")]
    GREATERTHANOREQUALS,
    

    /// Less than or equals.
    ///
    /// "LESS_THAN_OR_EQUALS"
    #[serde(rename="LESS_THAN_OR_EQUALS")]
    LESSTHANOREQUALS,
    

    /// Exists
    ///
    /// "EXISTS"
    #[serde(rename="EXISTS")]
    EXISTS,
}

impl AsRef<str> for GooglePrivacyDlpV2beta1ConditionOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2beta1ConditionOperatorEnum::RELATIONALOPERATORUNSPECIFIED => "RELATIONAL_OPERATOR_UNSPECIFIED",
            GooglePrivacyDlpV2beta1ConditionOperatorEnum::EQUALTO => "EQUAL_TO",
            GooglePrivacyDlpV2beta1ConditionOperatorEnum::NOTEQUALTO => "NOT_EQUAL_TO",
            GooglePrivacyDlpV2beta1ConditionOperatorEnum::GREATERTHAN => "GREATER_THAN",
            GooglePrivacyDlpV2beta1ConditionOperatorEnum::LESSTHAN => "LESS_THAN",
            GooglePrivacyDlpV2beta1ConditionOperatorEnum::GREATERTHANOREQUALS => "GREATER_THAN_OR_EQUALS",
            GooglePrivacyDlpV2beta1ConditionOperatorEnum::LESSTHANOREQUALS => "LESS_THAN_OR_EQUALS",
            GooglePrivacyDlpV2beta1ConditionOperatorEnum::EXISTS => "EXISTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2beta1ConditionOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELATIONAL_OPERATOR_UNSPECIFIED" => Ok(GooglePrivacyDlpV2beta1ConditionOperatorEnum::RELATIONALOPERATORUNSPECIFIED),
           "EQUAL_TO" => Ok(GooglePrivacyDlpV2beta1ConditionOperatorEnum::EQUALTO),
           "NOT_EQUAL_TO" => Ok(GooglePrivacyDlpV2beta1ConditionOperatorEnum::NOTEQUALTO),
           "GREATER_THAN" => Ok(GooglePrivacyDlpV2beta1ConditionOperatorEnum::GREATERTHAN),
           "LESS_THAN" => Ok(GooglePrivacyDlpV2beta1ConditionOperatorEnum::LESSTHAN),
           "GREATER_THAN_OR_EQUALS" => Ok(GooglePrivacyDlpV2beta1ConditionOperatorEnum::GREATERTHANOREQUALS),
           "LESS_THAN_OR_EQUALS" => Ok(GooglePrivacyDlpV2beta1ConditionOperatorEnum::LESSTHANOREQUALS),
           "EXISTS" => Ok(GooglePrivacyDlpV2beta1ConditionOperatorEnum::EXISTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2beta1ConditionOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


