use super::*;



// region DataOutputEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The selected format for the response. Default format is JSON.
pub enum DataOutputEnum {
    

    /// Returns the response in Google Charts Data Table format. This is useful in creating visualization using Google Charts.
    ///
    /// "dataTable"
    #[serde(rename="dataTable")]
    DataTable,
    

    /// Returns the response in standard JSON format.
    ///
    /// "json"
    #[serde(rename="json")]
    Json,
}

impl AsRef<str> for DataOutputEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataOutputEnum::DataTable => "dataTable",
            DataOutputEnum::Json => "json",
        }
    }
}

impl std::convert::TryFrom< &str> for DataOutputEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "dataTable" => Ok(DataOutputEnum::DataTable),
           "json" => Ok(DataOutputEnum::Json),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataOutputEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataSamplingLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired sampling level.
pub enum DataSamplingLevelEnum {
    

    /// Returns response with a sample size that balances speed and accuracy.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// Returns a fast response with a smaller sample size.
    ///
    /// "FASTER"
    #[serde(rename="FASTER")]
    FASTER,
    

    /// Returns a more accurate response using a large sample size, but this may result in the response being slower.
    ///
    /// "HIGHER_PRECISION"
    #[serde(rename="HIGHER_PRECISION")]
    HIGHERPRECISION,
}

impl AsRef<str> for DataSamplingLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataSamplingLevelEnum::DEFAULT => "DEFAULT",
            DataSamplingLevelEnum::FASTER => "FASTER",
            DataSamplingLevelEnum::HIGHERPRECISION => "HIGHER_PRECISION",
        }
    }
}

impl std::convert::TryFrom< &str> for DataSamplingLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(DataSamplingLevelEnum::DEFAULT),
           "FASTER" => Ok(DataSamplingLevelEnum::FASTER),
           "HIGHER_PRECISION" => Ok(DataSamplingLevelEnum::HIGHERPRECISION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataSamplingLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


