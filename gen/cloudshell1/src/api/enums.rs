use super::*;



// region EnvironmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current execution state of this environment.
pub enum EnvironmentStateEnum {
    

    /// The environment's states is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The environment is not running and can't be connected to. Starting the environment will transition it to the PENDING state.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
    

    /// The environment is being started but is not yet ready to accept connections.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The environment is running and ready to accept connections. It will automatically transition back to DISABLED after a period of inactivity or if another environment is started.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The environment is being deleted and can't be connected to.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for EnvironmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnvironmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            EnvironmentStateEnum::SUSPENDED => "SUSPENDED",
            EnvironmentStateEnum::PENDING => "PENDING",
            EnvironmentStateEnum::RUNNING => "RUNNING",
            EnvironmentStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for EnvironmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(EnvironmentStateEnum::STATEUNSPECIFIED),
           "SUSPENDED" => Ok(EnvironmentStateEnum::SUSPENDED),
           "PENDING" => Ok(EnvironmentStateEnum::PENDING),
           "RUNNING" => Ok(EnvironmentStateEnum::RUNNING),
           "DELETING" => Ok(EnvironmentStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnvironmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


