use super::*;



// region VariableStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the variable. The variable state indicates the outcome of the `variables().watch` call and is visible through the `get` and `list` calls.
pub enum VariableStateEnum {
    

    /// Default variable state.
    ///
    /// "VARIABLE_STATE_UNSPECIFIED"
    #[serde(rename="VARIABLE_STATE_UNSPECIFIED")]
    VARIABLESTATEUNSPECIFIED,
    

    /// The variable was updated, while `variables().watch` was executing.
    ///
    /// "UPDATED"
    #[serde(rename="UPDATED")]
    UPDATED,
    

    /// The variable was deleted, while `variables().watch` was executing.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for VariableStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VariableStateEnum::VARIABLESTATEUNSPECIFIED => "VARIABLE_STATE_UNSPECIFIED",
            VariableStateEnum::UPDATED => "UPDATED",
            VariableStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for VariableStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VARIABLE_STATE_UNSPECIFIED" => Ok(VariableStateEnum::VARIABLESTATEUNSPECIFIED),
           "UPDATED" => Ok(VariableStateEnum::UPDATED),
           "DELETED" => Ok(VariableStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VariableStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


