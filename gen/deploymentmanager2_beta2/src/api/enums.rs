use super::*;



// region DeploymentCreatePolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets the policy to use for creating new resources.
pub enum DeploymentCreatePolicyEnum {
    
    /// "ACQUIRE"
    #[serde(rename="ACQUIRE")]
    ACQUIRE,
    
    /// "CREATE_OR_ACQUIRE"
    #[serde(rename="CREATE_OR_ACQUIRE")]
    CREATEORACQUIRE,
}

impl AsRef<str> for DeploymentCreatePolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeploymentCreatePolicyEnum::ACQUIRE => "ACQUIRE",
            DeploymentCreatePolicyEnum::CREATEORACQUIRE => "CREATE_OR_ACQUIRE",
        }
    }
}

impl std::convert::TryFrom< &str> for DeploymentCreatePolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACQUIRE" => Ok(DeploymentCreatePolicyEnum::ACQUIRE),
           "CREATE_OR_ACQUIRE" => Ok(DeploymentCreatePolicyEnum::CREATEORACQUIRE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeploymentCreatePolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for DeploymentCreatePolicyEnum {
    fn default() -> DeploymentCreatePolicyEnum {
        DeploymentCreatePolicyEnum::CREATEORACQUIRE
    }
}

// endregion


// region DeploymentDeletePolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets the policy to use for deleting resources.
pub enum DeploymentDeletePolicyEnum {
    
    /// "ABANDON"
    #[serde(rename="ABANDON")]
    ABANDON,
    
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
}

impl AsRef<str> for DeploymentDeletePolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeploymentDeletePolicyEnum::ABANDON => "ABANDON",
            DeploymentDeletePolicyEnum::DELETE => "DELETE",
        }
    }
}

impl std::convert::TryFrom< &str> for DeploymentDeletePolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ABANDON" => Ok(DeploymentDeletePolicyEnum::ABANDON),
           "DELETE" => Ok(DeploymentDeletePolicyEnum::DELETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeploymentDeletePolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for DeploymentDeletePolicyEnum {
    fn default() -> DeploymentDeletePolicyEnum {
        DeploymentDeletePolicyEnum::DELETE
    }
}

// endregion


// region DeploymentUpdatePolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets the policy to use for updating resources.
pub enum DeploymentUpdatePolicyEnum {
    
    /// "PATCH"
    #[serde(rename="PATCH")]
    PATCH,
    
    /// "UPDATE"
    #[serde(rename="UPDATE")]
    UPDATE,
}

impl AsRef<str> for DeploymentUpdatePolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeploymentUpdatePolicyEnum::PATCH => "PATCH",
            DeploymentUpdatePolicyEnum::UPDATE => "UPDATE",
        }
    }
}

impl std::convert::TryFrom< &str> for DeploymentUpdatePolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PATCH" => Ok(DeploymentUpdatePolicyEnum::PATCH),
           "UPDATE" => Ok(DeploymentUpdatePolicyEnum::UPDATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeploymentUpdatePolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for DeploymentUpdatePolicyEnum {
    fn default() -> DeploymentUpdatePolicyEnum {
        DeploymentUpdatePolicyEnum::PATCH
    }
}

// endregion


