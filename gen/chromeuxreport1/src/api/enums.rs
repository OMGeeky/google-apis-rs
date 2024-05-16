use super::*;



// region HistoryKeyFormFactorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The form factor is the device class that all users used to access the site for this record. If the form factor is unspecified, then aggregated data over all form factors will be returned.
pub enum HistoryKeyFormFactorEnum {
    

    /// The default value, representing all device classes.
    ///
    /// "ALL_FORM_FACTORS"
    #[serde(rename="ALL_FORM_FACTORS")]
    ALLFORMFACTORS,
    

    /// The device class representing a "mobile"/"phone" sized client.
    ///
    /// "PHONE"
    #[serde(rename="PHONE")]
    PHONE,
    

    /// The device class representing a "desktop"/"laptop" type full size client.
    ///
    /// "DESKTOP"
    #[serde(rename="DESKTOP")]
    DESKTOP,
    

    /// The device class representing a "tablet" type client.
    ///
    /// "TABLET"
    #[serde(rename="TABLET")]
    TABLET,
}

impl AsRef<str> for HistoryKeyFormFactorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HistoryKeyFormFactorEnum::ALLFORMFACTORS => "ALL_FORM_FACTORS",
            HistoryKeyFormFactorEnum::PHONE => "PHONE",
            HistoryKeyFormFactorEnum::DESKTOP => "DESKTOP",
            HistoryKeyFormFactorEnum::TABLET => "TABLET",
        }
    }
}

impl std::convert::TryFrom< &str> for HistoryKeyFormFactorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_FORM_FACTORS" => Ok(HistoryKeyFormFactorEnum::ALLFORMFACTORS),
           "PHONE" => Ok(HistoryKeyFormFactorEnum::PHONE),
           "DESKTOP" => Ok(HistoryKeyFormFactorEnum::DESKTOP),
           "TABLET" => Ok(HistoryKeyFormFactorEnum::TABLET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HistoryKeyFormFactorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KeyFormFactorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The form factor is the device class that all users used to access the site for this record. If the form factor is unspecified, then aggregated data over all form factors will be returned.
pub enum KeyFormFactorEnum {
    

    /// The default value, representing all device classes.
    ///
    /// "ALL_FORM_FACTORS"
    #[serde(rename="ALL_FORM_FACTORS")]
    ALLFORMFACTORS,
    

    /// The device class representing a "mobile"/"phone" sized client.
    ///
    /// "PHONE"
    #[serde(rename="PHONE")]
    PHONE,
    

    /// The device class representing a "desktop"/"laptop" type full size client.
    ///
    /// "DESKTOP"
    #[serde(rename="DESKTOP")]
    DESKTOP,
    

    /// The device class representing a "tablet" type client.
    ///
    /// "TABLET"
    #[serde(rename="TABLET")]
    TABLET,
}

impl AsRef<str> for KeyFormFactorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KeyFormFactorEnum::ALLFORMFACTORS => "ALL_FORM_FACTORS",
            KeyFormFactorEnum::PHONE => "PHONE",
            KeyFormFactorEnum::DESKTOP => "DESKTOP",
            KeyFormFactorEnum::TABLET => "TABLET",
        }
    }
}

impl std::convert::TryFrom< &str> for KeyFormFactorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_FORM_FACTORS" => Ok(KeyFormFactorEnum::ALLFORMFACTORS),
           "PHONE" => Ok(KeyFormFactorEnum::PHONE),
           "DESKTOP" => Ok(KeyFormFactorEnum::DESKTOP),
           "TABLET" => Ok(KeyFormFactorEnum::TABLET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KeyFormFactorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryHistoryRequestFormFactorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The form factor is a query dimension that specifies the device class that the record's data should belong to. Note: If no form factor is specified, then a special record with aggregated data over all form factors will be returned.
pub enum QueryHistoryRequestFormFactorEnum {
    

    /// The default value, representing all device classes.
    ///
    /// "ALL_FORM_FACTORS"
    #[serde(rename="ALL_FORM_FACTORS")]
    ALLFORMFACTORS,
    

    /// The device class representing a "mobile"/"phone" sized client.
    ///
    /// "PHONE"
    #[serde(rename="PHONE")]
    PHONE,
    

    /// The device class representing a "desktop"/"laptop" type full size client.
    ///
    /// "DESKTOP"
    #[serde(rename="DESKTOP")]
    DESKTOP,
    

    /// The device class representing a "tablet" type client.
    ///
    /// "TABLET"
    #[serde(rename="TABLET")]
    TABLET,
}

impl AsRef<str> for QueryHistoryRequestFormFactorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryHistoryRequestFormFactorEnum::ALLFORMFACTORS => "ALL_FORM_FACTORS",
            QueryHistoryRequestFormFactorEnum::PHONE => "PHONE",
            QueryHistoryRequestFormFactorEnum::DESKTOP => "DESKTOP",
            QueryHistoryRequestFormFactorEnum::TABLET => "TABLET",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryHistoryRequestFormFactorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_FORM_FACTORS" => Ok(QueryHistoryRequestFormFactorEnum::ALLFORMFACTORS),
           "PHONE" => Ok(QueryHistoryRequestFormFactorEnum::PHONE),
           "DESKTOP" => Ok(QueryHistoryRequestFormFactorEnum::DESKTOP),
           "TABLET" => Ok(QueryHistoryRequestFormFactorEnum::TABLET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryHistoryRequestFormFactorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryRequestFormFactorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The form factor is a query dimension that specifies the device class that the record's data should belong to. Note: If no form factor is specified, then a special record with aggregated data over all form factors will be returned.
pub enum QueryRequestFormFactorEnum {
    

    /// The default value, representing all device classes.
    ///
    /// "ALL_FORM_FACTORS"
    #[serde(rename="ALL_FORM_FACTORS")]
    ALLFORMFACTORS,
    

    /// The device class representing a "mobile"/"phone" sized client.
    ///
    /// "PHONE"
    #[serde(rename="PHONE")]
    PHONE,
    

    /// The device class representing a "desktop"/"laptop" type full size client.
    ///
    /// "DESKTOP"
    #[serde(rename="DESKTOP")]
    DESKTOP,
    

    /// The device class representing a "tablet" type client.
    ///
    /// "TABLET"
    #[serde(rename="TABLET")]
    TABLET,
}

impl AsRef<str> for QueryRequestFormFactorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryRequestFormFactorEnum::ALLFORMFACTORS => "ALL_FORM_FACTORS",
            QueryRequestFormFactorEnum::PHONE => "PHONE",
            QueryRequestFormFactorEnum::DESKTOP => "DESKTOP",
            QueryRequestFormFactorEnum::TABLET => "TABLET",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryRequestFormFactorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_FORM_FACTORS" => Ok(QueryRequestFormFactorEnum::ALLFORMFACTORS),
           "PHONE" => Ok(QueryRequestFormFactorEnum::PHONE),
           "DESKTOP" => Ok(QueryRequestFormFactorEnum::DESKTOP),
           "TABLET" => Ok(QueryRequestFormFactorEnum::TABLET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryRequestFormFactorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


