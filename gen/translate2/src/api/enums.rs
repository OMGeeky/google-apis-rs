use super::*;



// region TranslationFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The format of the source text, in either HTML (default) or plain-text. A
value of "html" indicates HTML and a value of "text" indicates plain-text.
pub enum TranslationFormatEnum {
    

    /// Specifies the input is in HTML
    ///
    /// "html"
    #[serde(rename="html")]
    Html,
    

    /// Specifies the input is in plain textual format
    ///
    /// "text"
    #[serde(rename="text")]
    Text,
}

impl AsRef<str> for TranslationFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TranslationFormatEnum::Html => "html",
            TranslationFormatEnum::Text => "text",
        }
    }
}

impl std::convert::TryFrom< &str> for TranslationFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "html" => Ok(TranslationFormatEnum::Html),
           "text" => Ok(TranslationFormatEnum::Text),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TranslationFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


