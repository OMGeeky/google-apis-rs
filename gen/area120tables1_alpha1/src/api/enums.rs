use super::*;



// region CreateRowRequestViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Column key to use for values in the row. Defaults to user entered name.
pub enum CreateRowRequestViewEnum {
    

    /// Defaults to user entered text.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// Uses internally generated column id to identify values.
    ///
    /// "COLUMN_ID_VIEW"
    #[serde(rename="COLUMN_ID_VIEW")]
    COLUMNIDVIEW,
}

impl AsRef<str> for CreateRowRequestViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateRowRequestViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            CreateRowRequestViewEnum::COLUMNIDVIEW => "COLUMN_ID_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateRowRequestViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(CreateRowRequestViewEnum::VIEWUNSPECIFIED),
           "COLUMN_ID_VIEW" => Ok(CreateRowRequestViewEnum::COLUMNIDVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateRowRequestViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UpdateRowRequestViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Column key to use for values in the row. Defaults to user entered name.
pub enum UpdateRowRequestViewEnum {
    

    /// Defaults to user entered text.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// Uses internally generated column id to identify values.
    ///
    /// "COLUMN_ID_VIEW"
    #[serde(rename="COLUMN_ID_VIEW")]
    COLUMNIDVIEW,
}

impl AsRef<str> for UpdateRowRequestViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UpdateRowRequestViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            UpdateRowRequestViewEnum::COLUMNIDVIEW => "COLUMN_ID_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for UpdateRowRequestViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(UpdateRowRequestViewEnum::VIEWUNSPECIFIED),
           "COLUMN_ID_VIEW" => Ok(UpdateRowRequestViewEnum::COLUMNIDVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UpdateRowRequestViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TableViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Column key to use for values in the row. Defaults to user entered name.
pub enum TableViewEnum {
    

    /// Defaults to user entered text.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// Uses internally generated column id to identify values.
    ///
    /// "COLUMN_ID_VIEW"
    #[serde(rename="COLUMN_ID_VIEW")]
    COLUMNIDVIEW,
}

impl AsRef<str> for TableViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TableViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            TableViewEnum::COLUMNIDVIEW => "COLUMN_ID_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for TableViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(TableViewEnum::VIEWUNSPECIFIED),
           "COLUMN_ID_VIEW" => Ok(TableViewEnum::COLUMNIDVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TableViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


