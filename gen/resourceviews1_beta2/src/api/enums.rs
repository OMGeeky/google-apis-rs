use super::*;



// region ZoneViewFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The requested format of the return value. It can be URL or URL_PORT. A JSON object will be included in the response based on the format. The default format is NONE, which results in no JSON in the response.
pub enum ZoneViewFormatEnum {
    
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    
    /// "URL"
    #[serde(rename="URL")]
    URL,
    
    /// "URL_PORT"
    #[serde(rename="URL_PORT")]
    URLPORT,
}

impl AsRef<str> for ZoneViewFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ZoneViewFormatEnum::NONE => "NONE",
            ZoneViewFormatEnum::URL => "URL",
            ZoneViewFormatEnum::URLPORT => "URL_PORT",
        }
    }
}

impl std::convert::TryFrom< &str> for ZoneViewFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(ZoneViewFormatEnum::NONE),
           "URL" => Ok(ZoneViewFormatEnum::URL),
           "URL_PORT" => Ok(ZoneViewFormatEnum::URLPORT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ZoneViewFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ZoneViewListStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the instance to list. By default, it lists all instances.
pub enum ZoneViewListStateEnum {
    
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
}

impl AsRef<str> for ZoneViewListStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ZoneViewListStateEnum::ALL => "ALL",
            ZoneViewListStateEnum::RUNNING => "RUNNING",
        }
    }
}

impl std::convert::TryFrom< &str> for ZoneViewListStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL" => Ok(ZoneViewListStateEnum::ALL),
           "RUNNING" => Ok(ZoneViewListStateEnum::RUNNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ZoneViewListStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ZoneViewListStateEnum {
    fn default() -> ZoneViewListStateEnum {
        ZoneViewListStateEnum::ALL
    }
}

// endregion


