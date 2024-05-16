use super::*;



// region RemoteConfigConditionTagColorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional.
The display (tag) color of this condition. This serves as part of a tag
(in the future, we may add tag text as well as tag color, but that is not
yet implemented in the UI).
This value has no affect on the semantics of the delivered config and it
is ignored by the backend, except for passing it through write/read
requests.
Not having this value or having the "CONDITION_DISPLAY_COLOR_UNSPECIFIED"
value (0) have the same meaning:  Let the UI choose any valid color when
displaying the condition.
pub enum RemoteConfigConditionTagColorEnum {
    
    /// "CONDITION_DISPLAY_COLOR_UNSPECIFIED"
    #[serde(rename="CONDITION_DISPLAY_COLOR_UNSPECIFIED")]
    CONDITIONDISPLAYCOLORUNSPECIFIED,
    

    /// Blue
    ///
    /// "BLUE"
    #[serde(rename="BLUE")]
    BLUE,
    

    /// Brown
    ///
    /// "BROWN"
    #[serde(rename="BROWN")]
    BROWN,
    

    /// Cyan
    ///
    /// "CYAN"
    #[serde(rename="CYAN")]
    CYAN,
    

    /// aka "Red Orange"
    ///
    /// "DEEP_ORANGE"
    #[serde(rename="DEEP_ORANGE")]
    DEEPORANGE,
    

    /// Green
    ///
    /// "GREEN"
    #[serde(rename="GREEN")]
    GREEN,
    

    /// Indigo
*
    ///
    /// "INDIGO"
    #[serde(rename="INDIGO")]
    INDIGO,
    

    /// Lime - Approved deviation from Material color palette
    ///
    /// "LIME"
    #[serde(rename="LIME")]
    LIME,
    

    /// Orange
    ///
    /// "ORANGE"
    #[serde(rename="ORANGE")]
    ORANGE,
    

    /// Pink
    ///
    /// "PINK"
    #[serde(rename="PINK")]
    PINK,
    

    /// Purple
    ///
    /// "PURPLE"
    #[serde(rename="PURPLE")]
    PURPLE,
    

    /// Teal
    ///
    /// "TEAL"
    #[serde(rename="TEAL")]
    TEAL,
}

impl AsRef<str> for RemoteConfigConditionTagColorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RemoteConfigConditionTagColorEnum::CONDITIONDISPLAYCOLORUNSPECIFIED => "CONDITION_DISPLAY_COLOR_UNSPECIFIED",
            RemoteConfigConditionTagColorEnum::BLUE => "BLUE",
            RemoteConfigConditionTagColorEnum::BROWN => "BROWN",
            RemoteConfigConditionTagColorEnum::CYAN => "CYAN",
            RemoteConfigConditionTagColorEnum::DEEPORANGE => "DEEP_ORANGE",
            RemoteConfigConditionTagColorEnum::GREEN => "GREEN",
            RemoteConfigConditionTagColorEnum::INDIGO => "INDIGO",
            RemoteConfigConditionTagColorEnum::LIME => "LIME",
            RemoteConfigConditionTagColorEnum::ORANGE => "ORANGE",
            RemoteConfigConditionTagColorEnum::PINK => "PINK",
            RemoteConfigConditionTagColorEnum::PURPLE => "PURPLE",
            RemoteConfigConditionTagColorEnum::TEAL => "TEAL",
        }
    }
}

impl std::convert::TryFrom< &str> for RemoteConfigConditionTagColorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONDITION_DISPLAY_COLOR_UNSPECIFIED" => Ok(RemoteConfigConditionTagColorEnum::CONDITIONDISPLAYCOLORUNSPECIFIED),
           "BLUE" => Ok(RemoteConfigConditionTagColorEnum::BLUE),
           "BROWN" => Ok(RemoteConfigConditionTagColorEnum::BROWN),
           "CYAN" => Ok(RemoteConfigConditionTagColorEnum::CYAN),
           "DEEP_ORANGE" => Ok(RemoteConfigConditionTagColorEnum::DEEPORANGE),
           "GREEN" => Ok(RemoteConfigConditionTagColorEnum::GREEN),
           "INDIGO" => Ok(RemoteConfigConditionTagColorEnum::INDIGO),
           "LIME" => Ok(RemoteConfigConditionTagColorEnum::LIME),
           "ORANGE" => Ok(RemoteConfigConditionTagColorEnum::ORANGE),
           "PINK" => Ok(RemoteConfigConditionTagColorEnum::PINK),
           "PURPLE" => Ok(RemoteConfigConditionTagColorEnum::PURPLE),
           "TEAL" => Ok(RemoteConfigConditionTagColorEnum::TEAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RemoteConfigConditionTagColorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


