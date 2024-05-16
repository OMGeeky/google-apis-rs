use super::*;



// region AccessibilityMobilityAccessibleElevatorExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobility accessible elevator exception.
pub enum AccessibilityMobilityAccessibleElevatorExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for AccessibilityMobilityAccessibleElevatorExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccessibilityMobilityAccessibleElevatorExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            AccessibilityMobilityAccessibleElevatorExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            AccessibilityMobilityAccessibleElevatorExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            AccessibilityMobilityAccessibleElevatorExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for AccessibilityMobilityAccessibleElevatorExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(AccessibilityMobilityAccessibleElevatorExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(AccessibilityMobilityAccessibleElevatorExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(AccessibilityMobilityAccessibleElevatorExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(AccessibilityMobilityAccessibleElevatorExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccessibilityMobilityAccessibleElevatorExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccessibilityMobilityAccessibleExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobility accessible exception.
pub enum AccessibilityMobilityAccessibleExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for AccessibilityMobilityAccessibleExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccessibilityMobilityAccessibleExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            AccessibilityMobilityAccessibleExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            AccessibilityMobilityAccessibleExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            AccessibilityMobilityAccessibleExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for AccessibilityMobilityAccessibleExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(AccessibilityMobilityAccessibleExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(AccessibilityMobilityAccessibleExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(AccessibilityMobilityAccessibleExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(AccessibilityMobilityAccessibleExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccessibilityMobilityAccessibleExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccessibilityMobilityAccessibleParkingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobility accessible parking exception.
pub enum AccessibilityMobilityAccessibleParkingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for AccessibilityMobilityAccessibleParkingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccessibilityMobilityAccessibleParkingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            AccessibilityMobilityAccessibleParkingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            AccessibilityMobilityAccessibleParkingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            AccessibilityMobilityAccessibleParkingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for AccessibilityMobilityAccessibleParkingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(AccessibilityMobilityAccessibleParkingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(AccessibilityMobilityAccessibleParkingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(AccessibilityMobilityAccessibleParkingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(AccessibilityMobilityAccessibleParkingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccessibilityMobilityAccessibleParkingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccessibilityMobilityAccessiblePoolExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobility accessible pool exception.
pub enum AccessibilityMobilityAccessiblePoolExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for AccessibilityMobilityAccessiblePoolExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccessibilityMobilityAccessiblePoolExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            AccessibilityMobilityAccessiblePoolExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            AccessibilityMobilityAccessiblePoolExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            AccessibilityMobilityAccessiblePoolExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for AccessibilityMobilityAccessiblePoolExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(AccessibilityMobilityAccessiblePoolExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(AccessibilityMobilityAccessiblePoolExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(AccessibilityMobilityAccessiblePoolExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(AccessibilityMobilityAccessiblePoolExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccessibilityMobilityAccessiblePoolExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityBeachAccessExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Beach access exception.
pub enum ActivityBeachAccessExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityBeachAccessExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityBeachAccessExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityBeachAccessExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityBeachAccessExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityBeachAccessExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityBeachAccessExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityBeachAccessExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityBeachAccessExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityBeachAccessExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityBeachAccessExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityBeachAccessExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityBeachFrontExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Beach front exception.
pub enum ActivityBeachFrontExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityBeachFrontExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityBeachFrontExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityBeachFrontExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityBeachFrontExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityBeachFrontExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityBeachFrontExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityBeachFrontExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityBeachFrontExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityBeachFrontExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityBeachFrontExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityBeachFrontExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityBicycleRentalExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Bicycle rental exception.
pub enum ActivityBicycleRentalExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityBicycleRentalExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityBicycleRentalExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityBicycleRentalExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityBicycleRentalExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityBicycleRentalExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityBicycleRentalExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityBicycleRentalExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityBicycleRentalExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityBicycleRentalExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityBicycleRentalExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityBicycleRentalExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityBoutiqueStoresExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Boutique stores exception.
pub enum ActivityBoutiqueStoresExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityBoutiqueStoresExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityBoutiqueStoresExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityBoutiqueStoresExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityBoutiqueStoresExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityBoutiqueStoresExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityBoutiqueStoresExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityBoutiqueStoresExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityBoutiqueStoresExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityBoutiqueStoresExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityBoutiqueStoresExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityBoutiqueStoresExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityCasinoExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Casino exception.
pub enum ActivityCasinoExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityCasinoExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityCasinoExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityCasinoExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityCasinoExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityCasinoExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityCasinoExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityCasinoExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityCasinoExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityCasinoExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityCasinoExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityCasinoExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityFreeBicycleRentalExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free bicycle rental exception.
pub enum ActivityFreeBicycleRentalExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityFreeBicycleRentalExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityFreeBicycleRentalExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityFreeBicycleRentalExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityFreeBicycleRentalExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityFreeBicycleRentalExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityFreeBicycleRentalExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityFreeBicycleRentalExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityFreeBicycleRentalExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityFreeBicycleRentalExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityFreeBicycleRentalExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityFreeBicycleRentalExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityFreeWatercraftRentalExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free Watercraft rental exception.
pub enum ActivityFreeWatercraftRentalExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityFreeWatercraftRentalExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityFreeWatercraftRentalExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityFreeWatercraftRentalExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityFreeWatercraftRentalExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityFreeWatercraftRentalExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityFreeWatercraftRentalExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityFreeWatercraftRentalExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityFreeWatercraftRentalExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityFreeWatercraftRentalExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityFreeWatercraftRentalExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityFreeWatercraftRentalExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityGameRoomExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Game room exception.
pub enum ActivityGameRoomExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityGameRoomExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityGameRoomExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityGameRoomExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityGameRoomExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityGameRoomExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityGameRoomExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityGameRoomExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityGameRoomExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityGameRoomExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityGameRoomExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityGameRoomExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityGolfExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Golf exception.
pub enum ActivityGolfExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityGolfExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityGolfExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityGolfExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityGolfExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityGolfExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityGolfExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityGolfExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityGolfExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityGolfExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityGolfExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityGolfExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityHorsebackRidingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Horseback riding exception.
pub enum ActivityHorsebackRidingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityHorsebackRidingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityHorsebackRidingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityHorsebackRidingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityHorsebackRidingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityHorsebackRidingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityHorsebackRidingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityHorsebackRidingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityHorsebackRidingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityHorsebackRidingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityHorsebackRidingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityHorsebackRidingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityNightclubExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Nightclub exception.
pub enum ActivityNightclubExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityNightclubExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityNightclubExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityNightclubExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityNightclubExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityNightclubExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityNightclubExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityNightclubExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityNightclubExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityNightclubExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityNightclubExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityNightclubExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityPrivateBeachExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Private beach exception.
pub enum ActivityPrivateBeachExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityPrivateBeachExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityPrivateBeachExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityPrivateBeachExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityPrivateBeachExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityPrivateBeachExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityPrivateBeachExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityPrivateBeachExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityPrivateBeachExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityPrivateBeachExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityPrivateBeachExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityPrivateBeachExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityScubaExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Scuba exception.
pub enum ActivityScubaExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityScubaExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityScubaExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityScubaExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityScubaExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityScubaExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityScubaExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityScubaExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityScubaExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityScubaExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityScubaExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityScubaExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivitySnorkelingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Snorkeling exception.
pub enum ActivitySnorkelingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivitySnorkelingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivitySnorkelingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivitySnorkelingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivitySnorkelingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivitySnorkelingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivitySnorkelingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivitySnorkelingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivitySnorkelingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivitySnorkelingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivitySnorkelingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivitySnorkelingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityTennisExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tennis exception.
pub enum ActivityTennisExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityTennisExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityTennisExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityTennisExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityTennisExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityTennisExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityTennisExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityTennisExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityTennisExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityTennisExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityTennisExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityTennisExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityWaterSkiingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Water skiing exception.
pub enum ActivityWaterSkiingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityWaterSkiingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityWaterSkiingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityWaterSkiingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityWaterSkiingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityWaterSkiingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityWaterSkiingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityWaterSkiingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityWaterSkiingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityWaterSkiingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityWaterSkiingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityWaterSkiingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityWatercraftRentalExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Watercraft rental exception.
pub enum ActivityWatercraftRentalExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ActivityWatercraftRentalExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityWatercraftRentalExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ActivityWatercraftRentalExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ActivityWatercraftRentalExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ActivityWatercraftRentalExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityWatercraftRentalExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ActivityWatercraftRentalExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ActivityWatercraftRentalExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ActivityWatercraftRentalExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ActivityWatercraftRentalExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityWatercraftRentalExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BusinesBusinessCenterExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Business center exception.
pub enum BusinesBusinessCenterExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for BusinesBusinessCenterExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BusinesBusinessCenterExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            BusinesBusinessCenterExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            BusinesBusinessCenterExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            BusinesBusinessCenterExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for BusinesBusinessCenterExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(BusinesBusinessCenterExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(BusinesBusinessCenterExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(BusinesBusinessCenterExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(BusinesBusinessCenterExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BusinesBusinessCenterExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BusinesMeetingRoomsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Meeting rooms count exception.
pub enum BusinesMeetingRoomsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for BusinesMeetingRoomsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BusinesMeetingRoomsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            BusinesMeetingRoomsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            BusinesMeetingRoomsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            BusinesMeetingRoomsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for BusinesMeetingRoomsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(BusinesMeetingRoomsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(BusinesMeetingRoomsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(BusinesMeetingRoomsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(BusinesMeetingRoomsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BusinesMeetingRoomsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BusinesMeetingRoomsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Meeting rooms exception.
pub enum BusinesMeetingRoomsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for BusinesMeetingRoomsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BusinesMeetingRoomsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            BusinesMeetingRoomsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            BusinesMeetingRoomsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            BusinesMeetingRoomsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for BusinesMeetingRoomsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(BusinesMeetingRoomsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(BusinesMeetingRoomsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(BusinesMeetingRoomsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(BusinesMeetingRoomsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BusinesMeetingRoomsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectivityFreeWifiExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free wifi exception.
pub enum ConnectivityFreeWifiExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ConnectivityFreeWifiExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectivityFreeWifiExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ConnectivityFreeWifiExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ConnectivityFreeWifiExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ConnectivityFreeWifiExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectivityFreeWifiExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ConnectivityFreeWifiExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ConnectivityFreeWifiExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ConnectivityFreeWifiExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ConnectivityFreeWifiExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectivityFreeWifiExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectivityPublicAreaWifiAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Public area wifi available exception.
pub enum ConnectivityPublicAreaWifiAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ConnectivityPublicAreaWifiAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectivityPublicAreaWifiAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ConnectivityPublicAreaWifiAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ConnectivityPublicAreaWifiAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ConnectivityPublicAreaWifiAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectivityPublicAreaWifiAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ConnectivityPublicAreaWifiAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ConnectivityPublicAreaWifiAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ConnectivityPublicAreaWifiAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ConnectivityPublicAreaWifiAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectivityPublicAreaWifiAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectivityPublicInternetTerminalExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Public internet terminal exception.
pub enum ConnectivityPublicInternetTerminalExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ConnectivityPublicInternetTerminalExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectivityPublicInternetTerminalExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ConnectivityPublicInternetTerminalExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ConnectivityPublicInternetTerminalExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ConnectivityPublicInternetTerminalExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectivityPublicInternetTerminalExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ConnectivityPublicInternetTerminalExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ConnectivityPublicInternetTerminalExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ConnectivityPublicInternetTerminalExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ConnectivityPublicInternetTerminalExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectivityPublicInternetTerminalExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectivityWifiAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Wifi available exception.
pub enum ConnectivityWifiAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ConnectivityWifiAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectivityWifiAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ConnectivityWifiAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ConnectivityWifiAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ConnectivityWifiAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectivityWifiAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ConnectivityWifiAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ConnectivityWifiAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ConnectivityWifiAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ConnectivityWifiAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectivityWifiAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EcoCertificationAwardedExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Awarded exception.
pub enum EcoCertificationAwardedExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EcoCertificationAwardedExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EcoCertificationAwardedExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EcoCertificationAwardedExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EcoCertificationAwardedExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EcoCertificationAwardedExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EcoCertificationAwardedExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EcoCertificationAwardedExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EcoCertificationAwardedExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EcoCertificationAwardedExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EcoCertificationAwardedExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EcoCertificationAwardedExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EcoCertificationEcoCertificateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The eco certificate.
pub enum EcoCertificationEcoCertificateEnum {
    

    /// Default EcoCertificate. Do not use.
    ///
    /// "ECO_CERTIFICATE_UNSPECIFIED"
    #[serde(rename="ECO_CERTIFICATE_UNSPECIFIED")]
    ECOCERTIFICATEUNSPECIFIED,
    

    /// ISO14001.
    ///
    /// "ISO14001"
    #[serde(rename="ISO14001")]
    ISO14001,
    

    /// ISO50001.
    ///
    /// "ISO50001"
    #[serde(rename="ISO50001")]
    ISO50001,
    

    /// Asian Ecotourism Standard for Accommodations (AESA).
    ///
    /// "ASIAN_ECOTOURISM"
    #[serde(rename="ASIAN_ECOTOURISM")]
    ASIANECOTOURISM,
    

    /// Biosphere Responsible Tourism Standard.
    ///
    /// "BIOSPHERE_RESPOSNIBLE_TOURISM"
    #[serde(rename="BIOSPHERE_RESPOSNIBLE_TOURISM")]
    BIOSPHERERESPOSNIBLETOURISM,
    

    /// Bureau Veritas.
    ///
    /// "BUREAU_VERITAS"
    #[serde(rename="BUREAU_VERITAS")]
    BUREAUVERITAS,
    

    /// Control Union.
    ///
    /// "CONTROL_UNION"
    #[serde(rename="CONTROL_UNION")]
    CONTROLUNION,
    

    /// EarthCheck.
    ///
    /// "EARTHCHECK"
    #[serde(rename="EARTHCHECK")]
    EARTHCHECK,
    

    /// Eco-Certification Malta Standard.
    ///
    /// "ECO_CERTIFICATION_MALTA"
    #[serde(rename="ECO_CERTIFICATION_MALTA")]
    ECOCERTIFICATIONMALTA,
    

    /// Ecotourism Australia's ECO Certification Standard.
    ///
    /// "ECOTOURISM_AUSTRALIAS_ECO"
    #[serde(rename="ECOTOURISM_AUSTRALIAS_ECO")]
    ECOTOURISMAUSTRALIASECO,
    

    /// GREAT Green Deal Certification.
    ///
    /// "GREAT_GREEN_DEAL"
    #[serde(rename="GREAT_GREEN_DEAL")]
    GREATGREENDEAL,
    

    /// Green Globe.
    ///
    /// "GREEN_GLOBE"
    #[serde(rename="GREEN_GLOBE")]
    GREENGLOBE,
    

    /// Green Growth 2050 Standard.
    ///
    /// "GREEN_GROWTH2050"
    #[serde(rename="GREEN_GROWTH2050")]
    GREENGROWTH2050,
    

    /// Green Key.
    ///
    /// "GREEN_KEY"
    #[serde(rename="GREEN_KEY")]
    GREENKEY,
    

    /// Geen Key Eco Rating.
    ///
    /// "GREEN_KEY_ECO_RATING"
    #[serde(rename="GREEN_KEY_ECO_RATING")]
    GREENKEYECORATING,
    

    /// Green Seal.
    ///
    /// "GREEN_SEAL"
    #[serde(rename="GREEN_SEAL")]
    GREENSEAL,
    

    /// Green Star Hotel Standard.
    ///
    /// "GREEN_STAR"
    #[serde(rename="GREEN_STAR")]
    GREENSTAR,
    

    /// Green Tourism Active Standard.
    ///
    /// "GREEN_TOURISM_ACTIVE"
    #[serde(rename="GREEN_TOURISM_ACTIVE")]
    GREENTOURISMACTIVE,
    

    /// Hilton LightStay.
    ///
    /// "HILTON_LIGHTSTAY"
    #[serde(rename="HILTON_LIGHTSTAY")]
    HILTONLIGHTSTAY,
    

    /// Hostelling International's Quality and Sustainability Standard.
    ///
    /// "HOSTELLING_INTERNATIONALS_QUALITY_AND_SUSTAINABILITY"
    #[serde(rename="HOSTELLING_INTERNATIONALS_QUALITY_AND_SUSTAINABILITY")]
    HOSTELLINGINTERNATIONALSQUALITYANDSUSTAINABILITY,
    

    /// Hoteles más Verdes (AHT) Standard.
    ///
    /// "HOTELES_MAS_VERDES"
    #[serde(rename="HOTELES_MAS_VERDES")]
    HOTELESMASVERDES,
    

    /// Nordic Swan Ecolabel.
    ///
    /// "NORDIC_SWAN_ECOLABEL"
    #[serde(rename="NORDIC_SWAN_ECOLABEL")]
    NORDICSWANECOLABEL,
    

    /// Preferred by Nature Sustainable Tourism Standard for Accommodation.
    ///
    /// "PREFERRED_BY_NATURE_SUSTAINABLE_TOURISM"
    #[serde(rename="PREFERRED_BY_NATURE_SUSTAINABLE_TOURISM")]
    PREFERREDBYNATURESUSTAINABLETOURISM,
    

    /// Sustainable Travel Ireland – GSTC Industry Criteria.
    ///
    /// "SUSTAINABLE_TRAVEL_IRELAND"
    #[serde(rename="SUSTAINABLE_TRAVEL_IRELAND")]
    SUSTAINABLETRAVELIRELAND,
    

    /// TOFTigers Initiative's Pug Standard.
    ///
    /// "TOF_TIGERS_INITITIVES_PUG"
    #[serde(rename="TOF_TIGERS_INITITIVES_PUG")]
    TOFTIGERSINITITIVESPUG,
    

    /// Travelife Standard for Hotels & Accommodations.
    ///
    /// "TRAVELIFE"
    #[serde(rename="TRAVELIFE")]
    TRAVELIFE,
    

    /// United Certification Systems Limited.
    ///
    /// "UNITED_CERTIFICATION_SYSTEMS_LIMITED"
    #[serde(rename="UNITED_CERTIFICATION_SYSTEMS_LIMITED")]
    UNITEDCERTIFICATIONSYSTEMSLIMITED,
    

    /// Vireo Srl.
    ///
    /// "VIREO_SRL"
    #[serde(rename="VIREO_SRL")]
    VIREOSRL,
}

impl AsRef<str> for EcoCertificationEcoCertificateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EcoCertificationEcoCertificateEnum::ECOCERTIFICATEUNSPECIFIED => "ECO_CERTIFICATE_UNSPECIFIED",
            EcoCertificationEcoCertificateEnum::ISO14001 => "ISO14001",
            EcoCertificationEcoCertificateEnum::ISO50001 => "ISO50001",
            EcoCertificationEcoCertificateEnum::ASIANECOTOURISM => "ASIAN_ECOTOURISM",
            EcoCertificationEcoCertificateEnum::BIOSPHERERESPOSNIBLETOURISM => "BIOSPHERE_RESPOSNIBLE_TOURISM",
            EcoCertificationEcoCertificateEnum::BUREAUVERITAS => "BUREAU_VERITAS",
            EcoCertificationEcoCertificateEnum::CONTROLUNION => "CONTROL_UNION",
            EcoCertificationEcoCertificateEnum::EARTHCHECK => "EARTHCHECK",
            EcoCertificationEcoCertificateEnum::ECOCERTIFICATIONMALTA => "ECO_CERTIFICATION_MALTA",
            EcoCertificationEcoCertificateEnum::ECOTOURISMAUSTRALIASECO => "ECOTOURISM_AUSTRALIAS_ECO",
            EcoCertificationEcoCertificateEnum::GREATGREENDEAL => "GREAT_GREEN_DEAL",
            EcoCertificationEcoCertificateEnum::GREENGLOBE => "GREEN_GLOBE",
            EcoCertificationEcoCertificateEnum::GREENGROWTH2050 => "GREEN_GROWTH2050",
            EcoCertificationEcoCertificateEnum::GREENKEY => "GREEN_KEY",
            EcoCertificationEcoCertificateEnum::GREENKEYECORATING => "GREEN_KEY_ECO_RATING",
            EcoCertificationEcoCertificateEnum::GREENSEAL => "GREEN_SEAL",
            EcoCertificationEcoCertificateEnum::GREENSTAR => "GREEN_STAR",
            EcoCertificationEcoCertificateEnum::GREENTOURISMACTIVE => "GREEN_TOURISM_ACTIVE",
            EcoCertificationEcoCertificateEnum::HILTONLIGHTSTAY => "HILTON_LIGHTSTAY",
            EcoCertificationEcoCertificateEnum::HOSTELLINGINTERNATIONALSQUALITYANDSUSTAINABILITY => "HOSTELLING_INTERNATIONALS_QUALITY_AND_SUSTAINABILITY",
            EcoCertificationEcoCertificateEnum::HOTELESMASVERDES => "HOTELES_MAS_VERDES",
            EcoCertificationEcoCertificateEnum::NORDICSWANECOLABEL => "NORDIC_SWAN_ECOLABEL",
            EcoCertificationEcoCertificateEnum::PREFERREDBYNATURESUSTAINABLETOURISM => "PREFERRED_BY_NATURE_SUSTAINABLE_TOURISM",
            EcoCertificationEcoCertificateEnum::SUSTAINABLETRAVELIRELAND => "SUSTAINABLE_TRAVEL_IRELAND",
            EcoCertificationEcoCertificateEnum::TOFTIGERSINITITIVESPUG => "TOF_TIGERS_INITITIVES_PUG",
            EcoCertificationEcoCertificateEnum::TRAVELIFE => "TRAVELIFE",
            EcoCertificationEcoCertificateEnum::UNITEDCERTIFICATIONSYSTEMSLIMITED => "UNITED_CERTIFICATION_SYSTEMS_LIMITED",
            EcoCertificationEcoCertificateEnum::VIREOSRL => "VIREO_SRL",
        }
    }
}

impl std::convert::TryFrom< &str> for EcoCertificationEcoCertificateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ECO_CERTIFICATE_UNSPECIFIED" => Ok(EcoCertificationEcoCertificateEnum::ECOCERTIFICATEUNSPECIFIED),
           "ISO14001" => Ok(EcoCertificationEcoCertificateEnum::ISO14001),
           "ISO50001" => Ok(EcoCertificationEcoCertificateEnum::ISO50001),
           "ASIAN_ECOTOURISM" => Ok(EcoCertificationEcoCertificateEnum::ASIANECOTOURISM),
           "BIOSPHERE_RESPOSNIBLE_TOURISM" => Ok(EcoCertificationEcoCertificateEnum::BIOSPHERERESPOSNIBLETOURISM),
           "BUREAU_VERITAS" => Ok(EcoCertificationEcoCertificateEnum::BUREAUVERITAS),
           "CONTROL_UNION" => Ok(EcoCertificationEcoCertificateEnum::CONTROLUNION),
           "EARTHCHECK" => Ok(EcoCertificationEcoCertificateEnum::EARTHCHECK),
           "ECO_CERTIFICATION_MALTA" => Ok(EcoCertificationEcoCertificateEnum::ECOCERTIFICATIONMALTA),
           "ECOTOURISM_AUSTRALIAS_ECO" => Ok(EcoCertificationEcoCertificateEnum::ECOTOURISMAUSTRALIASECO),
           "GREAT_GREEN_DEAL" => Ok(EcoCertificationEcoCertificateEnum::GREATGREENDEAL),
           "GREEN_GLOBE" => Ok(EcoCertificationEcoCertificateEnum::GREENGLOBE),
           "GREEN_GROWTH2050" => Ok(EcoCertificationEcoCertificateEnum::GREENGROWTH2050),
           "GREEN_KEY" => Ok(EcoCertificationEcoCertificateEnum::GREENKEY),
           "GREEN_KEY_ECO_RATING" => Ok(EcoCertificationEcoCertificateEnum::GREENKEYECORATING),
           "GREEN_SEAL" => Ok(EcoCertificationEcoCertificateEnum::GREENSEAL),
           "GREEN_STAR" => Ok(EcoCertificationEcoCertificateEnum::GREENSTAR),
           "GREEN_TOURISM_ACTIVE" => Ok(EcoCertificationEcoCertificateEnum::GREENTOURISMACTIVE),
           "HILTON_LIGHTSTAY" => Ok(EcoCertificationEcoCertificateEnum::HILTONLIGHTSTAY),
           "HOSTELLING_INTERNATIONALS_QUALITY_AND_SUSTAINABILITY" => Ok(EcoCertificationEcoCertificateEnum::HOSTELLINGINTERNATIONALSQUALITYANDSUSTAINABILITY),
           "HOTELES_MAS_VERDES" => Ok(EcoCertificationEcoCertificateEnum::HOTELESMASVERDES),
           "NORDIC_SWAN_ECOLABEL" => Ok(EcoCertificationEcoCertificateEnum::NORDICSWANECOLABEL),
           "PREFERRED_BY_NATURE_SUSTAINABLE_TOURISM" => Ok(EcoCertificationEcoCertificateEnum::PREFERREDBYNATURESUSTAINABLETOURISM),
           "SUSTAINABLE_TRAVEL_IRELAND" => Ok(EcoCertificationEcoCertificateEnum::SUSTAINABLETRAVELIRELAND),
           "TOF_TIGERS_INITITIVES_PUG" => Ok(EcoCertificationEcoCertificateEnum::TOFTIGERSINITITIVESPUG),
           "TRAVELIFE" => Ok(EcoCertificationEcoCertificateEnum::TRAVELIFE),
           "UNITED_CERTIFICATION_SYSTEMS_LIMITED" => Ok(EcoCertificationEcoCertificateEnum::UNITEDCERTIFICATIONSYSTEMSLIMITED),
           "VIREO_SRL" => Ok(EcoCertificationEcoCertificateEnum::VIREOSRL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EcoCertificationEcoCertificateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Carbon free energy sources exception.
pub enum EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnergyEfficiencyEnergyConservationProgramExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Energy conservation program exception.
pub enum EnergyEfficiencyEnergyConservationProgramExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnergyEfficiencyEnergyConservationProgramExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnergyEfficiencyEnergyConservationProgramExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnergyEfficiencyEnergyConservationProgramExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnergyEfficiencyEnergyConservationProgramExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnergyEfficiencyEnergyConservationProgramExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnergyEfficiencyEnergyConservationProgramExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnergyEfficiencyEnergyConservationProgramExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnergyEfficiencyEnergyConservationProgramExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnergyEfficiencyEnergyConservationProgramExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnergyEfficiencyEnergyConservationProgramExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnergyEfficiencyEnergyConservationProgramExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Energy efficient heating and cooling systems exception.
pub enum EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnergyEfficiencyEnergyEfficientLightingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Energy efficient lighting exception.
pub enum EnergyEfficiencyEnergyEfficientLightingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnergyEfficiencyEnergyEfficientLightingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnergyEfficiencyEnergyEfficientLightingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnergyEfficiencyEnergyEfficientLightingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnergyEfficiencyEnergyEfficientLightingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnergyEfficiencyEnergyEfficientLightingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnergyEfficiencyEnergyEfficientLightingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnergyEfficiencyEnergyEfficientLightingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnergyEfficiencyEnergyEfficientLightingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnergyEfficiencyEnergyEfficientLightingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnergyEfficiencyEnergyEfficientLightingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnergyEfficiencyEnergyEfficientLightingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnergyEfficiencyEnergySavingThermostatsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Energy saving thermostats exception.
pub enum EnergyEfficiencyEnergySavingThermostatsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnergyEfficiencyEnergySavingThermostatsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnergyEfficiencyEnergySavingThermostatsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnergyEfficiencyEnergySavingThermostatsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnergyEfficiencyEnergySavingThermostatsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnergyEfficiencyEnergySavingThermostatsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnergyEfficiencyEnergySavingThermostatsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnergyEfficiencyEnergySavingThermostatsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnergyEfficiencyEnergySavingThermostatsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnergyEfficiencyEnergySavingThermostatsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnergyEfficiencyEnergySavingThermostatsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnergyEfficiencyEnergySavingThermostatsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnergyEfficiencyGreenBuildingDesignExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Green building design exception.
pub enum EnergyEfficiencyGreenBuildingDesignExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnergyEfficiencyGreenBuildingDesignExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnergyEfficiencyGreenBuildingDesignExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnergyEfficiencyGreenBuildingDesignExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnergyEfficiencyGreenBuildingDesignExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnergyEfficiencyGreenBuildingDesignExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnergyEfficiencyGreenBuildingDesignExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnergyEfficiencyGreenBuildingDesignExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnergyEfficiencyGreenBuildingDesignExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnergyEfficiencyGreenBuildingDesignExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnergyEfficiencyGreenBuildingDesignExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnergyEfficiencyGreenBuildingDesignExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Independent organization audits energy use exception.
pub enum EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Commercial grade disinfectant cleaning exception.
pub enum EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Common areas enhanced cleaning exception.
pub enum EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Employees trained cleaning procedures exception.
pub enum EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Employees trained thorough hand washing exception.
pub enum EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Employees wear protective equipment exception.
pub enum EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Guest rooms enhanced cleaning exception.
pub enum EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FamilyBabysittingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Babysitting exception.
pub enum FamilyBabysittingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FamilyBabysittingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FamilyBabysittingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FamilyBabysittingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FamilyBabysittingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FamilyBabysittingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FamilyBabysittingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FamilyBabysittingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FamilyBabysittingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FamilyBabysittingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FamilyBabysittingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FamilyBabysittingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FamilyKidsActivitiesExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Kids activities exception.
pub enum FamilyKidsActivitiesExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FamilyKidsActivitiesExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FamilyKidsActivitiesExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FamilyKidsActivitiesExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FamilyKidsActivitiesExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FamilyKidsActivitiesExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FamilyKidsActivitiesExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FamilyKidsActivitiesExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FamilyKidsActivitiesExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FamilyKidsActivitiesExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FamilyKidsActivitiesExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FamilyKidsActivitiesExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FamilyKidsClubExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Kids club exception.
pub enum FamilyKidsClubExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FamilyKidsClubExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FamilyKidsClubExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FamilyKidsClubExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FamilyKidsClubExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FamilyKidsClubExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FamilyKidsClubExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FamilyKidsClubExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FamilyKidsClubExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FamilyKidsClubExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FamilyKidsClubExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FamilyKidsClubExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FamilyKidsFriendlyExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Kids friendly exception.
pub enum FamilyKidsFriendlyExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FamilyKidsFriendlyExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FamilyKidsFriendlyExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FamilyKidsFriendlyExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FamilyKidsFriendlyExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FamilyKidsFriendlyExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FamilyKidsFriendlyExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FamilyKidsFriendlyExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FamilyKidsFriendlyExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FamilyKidsFriendlyExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FamilyKidsFriendlyExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FamilyKidsFriendlyExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkBarExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Bar exception.
pub enum FoodAndDrinkBarExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkBarExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkBarExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkBarExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkBarExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkBarExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkBarExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkBarExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkBarExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkBarExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkBarExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkBarExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkBreakfastAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Breakfast available exception.
pub enum FoodAndDrinkBreakfastAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkBreakfastAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkBreakfastAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkBreakfastAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkBreakfastAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkBreakfastAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkBreakfastAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkBreakfastAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkBreakfastAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkBreakfastAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkBreakfastAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkBreakfastAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkBreakfastBuffetExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Breakfast buffet exception.
pub enum FoodAndDrinkBreakfastBuffetExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkBreakfastBuffetExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkBreakfastBuffetExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkBreakfastBuffetExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkBreakfastBuffetExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkBreakfastBuffetExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkBreakfastBuffetExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkBreakfastBuffetExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkBreakfastBuffetExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkBreakfastBuffetExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkBreakfastBuffetExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkBreakfastBuffetExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkBuffetExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Buffet exception.
pub enum FoodAndDrinkBuffetExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkBuffetExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkBuffetExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkBuffetExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkBuffetExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkBuffetExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkBuffetExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkBuffetExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkBuffetExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkBuffetExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkBuffetExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkBuffetExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkDinnerBuffetExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Dinner buffet exception.
pub enum FoodAndDrinkDinnerBuffetExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkDinnerBuffetExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkDinnerBuffetExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkDinnerBuffetExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkDinnerBuffetExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkDinnerBuffetExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkDinnerBuffetExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkDinnerBuffetExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkDinnerBuffetExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkDinnerBuffetExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkDinnerBuffetExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkDinnerBuffetExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkFreeBreakfastExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free breakfast exception.
pub enum FoodAndDrinkFreeBreakfastExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkFreeBreakfastExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkFreeBreakfastExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkFreeBreakfastExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkFreeBreakfastExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkFreeBreakfastExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkFreeBreakfastExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkFreeBreakfastExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkFreeBreakfastExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkFreeBreakfastExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkFreeBreakfastExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkFreeBreakfastExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkRestaurantExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restaurant exception.
pub enum FoodAndDrinkRestaurantExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkRestaurantExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkRestaurantExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkRestaurantExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkRestaurantExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkRestaurantExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkRestaurantExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkRestaurantExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkRestaurantExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkRestaurantExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkRestaurantExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkRestaurantExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkRestaurantsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restaurants count exception.
pub enum FoodAndDrinkRestaurantsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkRestaurantsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkRestaurantsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkRestaurantsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkRestaurantsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkRestaurantsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkRestaurantsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkRestaurantsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkRestaurantsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkRestaurantsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkRestaurantsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkRestaurantsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkRoomServiceExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Room service exception.
pub enum FoodAndDrinkRoomServiceExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkRoomServiceExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkRoomServiceExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkRoomServiceExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkRoomServiceExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkRoomServiceExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkRoomServiceExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkRoomServiceExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkRoomServiceExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkRoomServiceExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkRoomServiceExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkRoomServiceExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkTableServiceExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Table service exception.
pub enum FoodAndDrinkTableServiceExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkTableServiceExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkTableServiceExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkTableServiceExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkTableServiceExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkTableServiceExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkTableServiceExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkTableServiceExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkTableServiceExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkTableServiceExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkTableServiceExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkTableServiceExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// 24hr room service exception.
pub enum FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FoodAndDrinkVendingMachineExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Vending machine exception.
pub enum FoodAndDrinkVendingMachineExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for FoodAndDrinkVendingMachineExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FoodAndDrinkVendingMachineExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            FoodAndDrinkVendingMachineExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            FoodAndDrinkVendingMachineExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            FoodAndDrinkVendingMachineExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for FoodAndDrinkVendingMachineExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(FoodAndDrinkVendingMachineExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(FoodAndDrinkVendingMachineExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(FoodAndDrinkVendingMachineExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(FoodAndDrinkVendingMachineExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FoodAndDrinkVendingMachineExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuestUnitFeatureBungalowOrVillaExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Bungalow or villa exception.
pub enum GuestUnitFeatureBungalowOrVillaExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for GuestUnitFeatureBungalowOrVillaExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuestUnitFeatureBungalowOrVillaExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            GuestUnitFeatureBungalowOrVillaExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            GuestUnitFeatureBungalowOrVillaExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            GuestUnitFeatureBungalowOrVillaExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for GuestUnitFeatureBungalowOrVillaExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(GuestUnitFeatureBungalowOrVillaExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(GuestUnitFeatureBungalowOrVillaExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(GuestUnitFeatureBungalowOrVillaExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(GuestUnitFeatureBungalowOrVillaExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuestUnitFeatureBungalowOrVillaExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuestUnitFeatureConnectingUnitAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Connecting unit available exception.
pub enum GuestUnitFeatureConnectingUnitAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for GuestUnitFeatureConnectingUnitAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuestUnitFeatureConnectingUnitAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            GuestUnitFeatureConnectingUnitAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            GuestUnitFeatureConnectingUnitAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            GuestUnitFeatureConnectingUnitAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for GuestUnitFeatureConnectingUnitAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(GuestUnitFeatureConnectingUnitAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(GuestUnitFeatureConnectingUnitAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(GuestUnitFeatureConnectingUnitAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(GuestUnitFeatureConnectingUnitAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuestUnitFeatureConnectingUnitAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuestUnitFeatureExecutiveFloorExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Executive floor exception.
pub enum GuestUnitFeatureExecutiveFloorExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for GuestUnitFeatureExecutiveFloorExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuestUnitFeatureExecutiveFloorExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            GuestUnitFeatureExecutiveFloorExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            GuestUnitFeatureExecutiveFloorExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            GuestUnitFeatureExecutiveFloorExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for GuestUnitFeatureExecutiveFloorExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(GuestUnitFeatureExecutiveFloorExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(GuestUnitFeatureExecutiveFloorExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(GuestUnitFeatureExecutiveFloorExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(GuestUnitFeatureExecutiveFloorExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuestUnitFeatureExecutiveFloorExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Max adult occupants count exception.
pub enum GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuestUnitFeatureMaxChildOccupantsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Max child occupants count exception.
pub enum GuestUnitFeatureMaxChildOccupantsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for GuestUnitFeatureMaxChildOccupantsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuestUnitFeatureMaxChildOccupantsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            GuestUnitFeatureMaxChildOccupantsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            GuestUnitFeatureMaxChildOccupantsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            GuestUnitFeatureMaxChildOccupantsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for GuestUnitFeatureMaxChildOccupantsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(GuestUnitFeatureMaxChildOccupantsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(GuestUnitFeatureMaxChildOccupantsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(GuestUnitFeatureMaxChildOccupantsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(GuestUnitFeatureMaxChildOccupantsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuestUnitFeatureMaxChildOccupantsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuestUnitFeatureMaxOccupantsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Max occupants count exception.
pub enum GuestUnitFeatureMaxOccupantsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for GuestUnitFeatureMaxOccupantsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuestUnitFeatureMaxOccupantsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            GuestUnitFeatureMaxOccupantsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            GuestUnitFeatureMaxOccupantsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            GuestUnitFeatureMaxOccupantsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for GuestUnitFeatureMaxOccupantsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(GuestUnitFeatureMaxOccupantsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(GuestUnitFeatureMaxOccupantsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(GuestUnitFeatureMaxOccupantsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(GuestUnitFeatureMaxOccupantsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuestUnitFeatureMaxOccupantsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuestUnitFeaturePrivateHomeExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Private home exception.
pub enum GuestUnitFeaturePrivateHomeExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for GuestUnitFeaturePrivateHomeExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuestUnitFeaturePrivateHomeExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            GuestUnitFeaturePrivateHomeExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            GuestUnitFeaturePrivateHomeExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            GuestUnitFeaturePrivateHomeExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for GuestUnitFeaturePrivateHomeExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(GuestUnitFeaturePrivateHomeExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(GuestUnitFeaturePrivateHomeExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(GuestUnitFeaturePrivateHomeExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(GuestUnitFeaturePrivateHomeExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuestUnitFeaturePrivateHomeExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuestUnitFeatureSuiteExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Suite exception.
pub enum GuestUnitFeatureSuiteExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for GuestUnitFeatureSuiteExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuestUnitFeatureSuiteExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            GuestUnitFeatureSuiteExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            GuestUnitFeatureSuiteExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            GuestUnitFeatureSuiteExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for GuestUnitFeatureSuiteExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(GuestUnitFeatureSuiteExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(GuestUnitFeatureSuiteExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(GuestUnitFeatureSuiteExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(GuestUnitFeatureSuiteExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuestUnitFeatureSuiteExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuestUnitFeatureTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tier. Classification of the unit based on available features/amenities. A non-standard tier is only permitted if at least one other unit type falls under the standard tier.
pub enum GuestUnitFeatureTierEnum {
    

    /// Default tier. Equivalent to STANDARD. Prefer using STANDARD directly.
    ///
    /// "UNIT_TIER_UNSPECIFIED"
    #[serde(rename="UNIT_TIER_UNSPECIFIED")]
    UNITTIERUNSPECIFIED,
    

    /// Standard unit. The predominant and most basic guestroom type available at the hotel. All other guestroom types include the features/amenities of this room, as well as additional features/amenities.
    ///
    /// "STANDARD_UNIT"
    #[serde(rename="STANDARD_UNIT")]
    STANDARDUNIT,
    

    /// Deluxe unit. A guestroom type that builds on the features of the standard guestroom by offering additional amenities and/or more space, and/or views. The room rate is higher than that of the standard room type. Also known as Superior. Only allowed if another unit type is a standard tier.
    ///
    /// "DELUXE_UNIT"
    #[serde(rename="DELUXE_UNIT")]
    DELUXEUNIT,
}

impl AsRef<str> for GuestUnitFeatureTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuestUnitFeatureTierEnum::UNITTIERUNSPECIFIED => "UNIT_TIER_UNSPECIFIED",
            GuestUnitFeatureTierEnum::STANDARDUNIT => "STANDARD_UNIT",
            GuestUnitFeatureTierEnum::DELUXEUNIT => "DELUXE_UNIT",
        }
    }
}

impl std::convert::TryFrom< &str> for GuestUnitFeatureTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNIT_TIER_UNSPECIFIED" => Ok(GuestUnitFeatureTierEnum::UNITTIERUNSPECIFIED),
           "STANDARD_UNIT" => Ok(GuestUnitFeatureTierEnum::STANDARDUNIT),
           "DELUXE_UNIT" => Ok(GuestUnitFeatureTierEnum::DELUXEUNIT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuestUnitFeatureTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuestUnitFeatureTierExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tier exception.
pub enum GuestUnitFeatureTierExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for GuestUnitFeatureTierExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuestUnitFeatureTierExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            GuestUnitFeatureTierExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            GuestUnitFeatureTierExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            GuestUnitFeatureTierExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for GuestUnitFeatureTierExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(GuestUnitFeatureTierExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(GuestUnitFeatureTierExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(GuestUnitFeatureTierExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(GuestUnitFeatureTierExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuestUnitFeatureTierExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HousekeepingDailyHousekeepingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Daily housekeeping exception.
pub enum HousekeepingDailyHousekeepingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for HousekeepingDailyHousekeepingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HousekeepingDailyHousekeepingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            HousekeepingDailyHousekeepingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            HousekeepingDailyHousekeepingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            HousekeepingDailyHousekeepingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for HousekeepingDailyHousekeepingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(HousekeepingDailyHousekeepingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(HousekeepingDailyHousekeepingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(HousekeepingDailyHousekeepingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(HousekeepingDailyHousekeepingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HousekeepingDailyHousekeepingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HousekeepingHousekeepingAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Housekeeping available exception.
pub enum HousekeepingHousekeepingAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for HousekeepingHousekeepingAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HousekeepingHousekeepingAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            HousekeepingHousekeepingAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            HousekeepingHousekeepingAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            HousekeepingHousekeepingAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for HousekeepingHousekeepingAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(HousekeepingHousekeepingAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(HousekeepingHousekeepingAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(HousekeepingHousekeepingAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(HousekeepingHousekeepingAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HousekeepingHousekeepingAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HousekeepingTurndownServiceExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Turndown service exception.
pub enum HousekeepingTurndownServiceExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for HousekeepingTurndownServiceExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HousekeepingTurndownServiceExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            HousekeepingTurndownServiceExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            HousekeepingTurndownServiceExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            HousekeepingTurndownServiceExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for HousekeepingTurndownServiceExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(HousekeepingTurndownServiceExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(HousekeepingTurndownServiceExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(HousekeepingTurndownServiceExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(HousekeepingTurndownServiceExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HousekeepingTurndownServiceExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Dining areas additional sanitation exception.
pub enum IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IncreasedFoodSafetyDisposableFlatwareExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Disposable flatware exception.
pub enum IncreasedFoodSafetyDisposableFlatwareExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for IncreasedFoodSafetyDisposableFlatwareExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IncreasedFoodSafetyDisposableFlatwareExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            IncreasedFoodSafetyDisposableFlatwareExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            IncreasedFoodSafetyDisposableFlatwareExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            IncreasedFoodSafetyDisposableFlatwareExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for IncreasedFoodSafetyDisposableFlatwareExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(IncreasedFoodSafetyDisposableFlatwareExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(IncreasedFoodSafetyDisposableFlatwareExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(IncreasedFoodSafetyDisposableFlatwareExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(IncreasedFoodSafetyDisposableFlatwareExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IncreasedFoodSafetyDisposableFlatwareExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Food preparation and serving additional safety exception.
pub enum IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Individual packaged meals exception.
pub enum IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IncreasedFoodSafetySingleUseFoodMenusExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Single use food menus exception.
pub enum IncreasedFoodSafetySingleUseFoodMenusExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for IncreasedFoodSafetySingleUseFoodMenusExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IncreasedFoodSafetySingleUseFoodMenusExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            IncreasedFoodSafetySingleUseFoodMenusExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            IncreasedFoodSafetySingleUseFoodMenusExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            IncreasedFoodSafetySingleUseFoodMenusExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for IncreasedFoodSafetySingleUseFoodMenusExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(IncreasedFoodSafetySingleUseFoodMenusExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(IncreasedFoodSafetySingleUseFoodMenusExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(IncreasedFoodSafetySingleUseFoodMenusExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(IncreasedFoodSafetySingleUseFoodMenusExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IncreasedFoodSafetySingleUseFoodMenusExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LanguageSpokenSpokenExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Spoken exception.
pub enum LanguageSpokenSpokenExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LanguageSpokenSpokenExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LanguageSpokenSpokenExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LanguageSpokenSpokenExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LanguageSpokenSpokenExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LanguageSpokenSpokenExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LanguageSpokenSpokenExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LanguageSpokenSpokenExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LanguageSpokenSpokenExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LanguageSpokenSpokenExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LanguageSpokenSpokenExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LanguageSpokenSpokenExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaAccessibilityAdaCompliantUnitExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// ADA compliant unit exception.
pub enum LivingAreaAccessibilityAdaCompliantUnitExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaAccessibilityAdaCompliantUnitExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaAccessibilityAdaCompliantUnitExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaAccessibilityAdaCompliantUnitExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaAccessibilityAdaCompliantUnitExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaAccessibilityAdaCompliantUnitExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaAccessibilityAdaCompliantUnitExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaAccessibilityAdaCompliantUnitExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaAccessibilityAdaCompliantUnitExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaAccessibilityAdaCompliantUnitExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaAccessibilityAdaCompliantUnitExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaAccessibilityAdaCompliantUnitExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Hearing-accessible doorbell exception.
pub enum LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Hearing-accessible fire alarm exception.
pub enum LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Hearing-accessible unit exception.
pub enum LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobility-accessible bathtub exception.
pub enum LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobility-accessible shower exception.
pub enum LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobility-accessible toilet exception.
pub enum LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobility-accessible unit exception.
pub enum LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingCoffeeMakerExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Coffee maker exception.
pub enum LivingAreaEatingCoffeeMakerExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingCoffeeMakerExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingCoffeeMakerExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingCoffeeMakerExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingCoffeeMakerExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingCoffeeMakerExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingCoffeeMakerExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingCoffeeMakerExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingCoffeeMakerExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingCoffeeMakerExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingCoffeeMakerExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingCoffeeMakerExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingCookwareExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Cookware exception.
pub enum LivingAreaEatingCookwareExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingCookwareExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingCookwareExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingCookwareExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingCookwareExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingCookwareExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingCookwareExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingCookwareExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingCookwareExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingCookwareExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingCookwareExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingCookwareExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingDishwasherExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Dishwasher exception.
pub enum LivingAreaEatingDishwasherExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingDishwasherExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingDishwasherExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingDishwasherExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingDishwasherExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingDishwasherExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingDishwasherExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingDishwasherExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingDishwasherExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingDishwasherExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingDishwasherExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingDishwasherExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingIndoorGrillExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indoor grill exception.
pub enum LivingAreaEatingIndoorGrillExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingIndoorGrillExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingIndoorGrillExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingIndoorGrillExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingIndoorGrillExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingIndoorGrillExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingIndoorGrillExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingIndoorGrillExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingIndoorGrillExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingIndoorGrillExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingIndoorGrillExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingIndoorGrillExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingKettleExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Kettle exception.
pub enum LivingAreaEatingKettleExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingKettleExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingKettleExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingKettleExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingKettleExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingKettleExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingKettleExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingKettleExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingKettleExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingKettleExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingKettleExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingKettleExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingKitchenAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Kitchen available exception.
pub enum LivingAreaEatingKitchenAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingKitchenAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingKitchenAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingKitchenAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingKitchenAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingKitchenAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingKitchenAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingKitchenAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingKitchenAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingKitchenAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingKitchenAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingKitchenAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingMicrowaveExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Microwave exception.
pub enum LivingAreaEatingMicrowaveExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingMicrowaveExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingMicrowaveExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingMicrowaveExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingMicrowaveExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingMicrowaveExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingMicrowaveExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingMicrowaveExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingMicrowaveExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingMicrowaveExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingMicrowaveExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingMicrowaveExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingMinibarExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Minibar exception.
pub enum LivingAreaEatingMinibarExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingMinibarExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingMinibarExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingMinibarExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingMinibarExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingMinibarExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingMinibarExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingMinibarExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingMinibarExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingMinibarExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingMinibarExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingMinibarExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingOutdoorGrillExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Outdoor grill exception.
pub enum LivingAreaEatingOutdoorGrillExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingOutdoorGrillExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingOutdoorGrillExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingOutdoorGrillExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingOutdoorGrillExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingOutdoorGrillExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingOutdoorGrillExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingOutdoorGrillExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingOutdoorGrillExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingOutdoorGrillExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingOutdoorGrillExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingOutdoorGrillExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingOvenExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Oven exception.
pub enum LivingAreaEatingOvenExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingOvenExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingOvenExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingOvenExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingOvenExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingOvenExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingOvenExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingOvenExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingOvenExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingOvenExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingOvenExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingOvenExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingRefrigeratorExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Refrigerator exception.
pub enum LivingAreaEatingRefrigeratorExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingRefrigeratorExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingRefrigeratorExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingRefrigeratorExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingRefrigeratorExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingRefrigeratorExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingRefrigeratorExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingRefrigeratorExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingRefrigeratorExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingRefrigeratorExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingRefrigeratorExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingRefrigeratorExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingSinkExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sink exception.
pub enum LivingAreaEatingSinkExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingSinkExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingSinkExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingSinkExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingSinkExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingSinkExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingSinkExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingSinkExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingSinkExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingSinkExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingSinkExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingSinkExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingSnackbarExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Snackbar exception.
pub enum LivingAreaEatingSnackbarExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingSnackbarExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingSnackbarExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingSnackbarExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingSnackbarExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingSnackbarExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingSnackbarExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingSnackbarExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingSnackbarExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingSnackbarExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingSnackbarExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingSnackbarExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingStoveExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Stove exception.
pub enum LivingAreaEatingStoveExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingStoveExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingStoveExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingStoveExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingStoveExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingStoveExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingStoveExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingStoveExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingStoveExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingStoveExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingStoveExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingStoveExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingTeaStationExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tea station exception.
pub enum LivingAreaEatingTeaStationExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingTeaStationExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingTeaStationExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingTeaStationExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingTeaStationExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingTeaStationExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingTeaStationExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingTeaStationExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingTeaStationExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingTeaStationExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingTeaStationExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingTeaStationExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaEatingToasterExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Toaster exception.
pub enum LivingAreaEatingToasterExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaEatingToasterExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaEatingToasterExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaEatingToasterExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaEatingToasterExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaEatingToasterExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaEatingToasterExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaEatingToasterExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaEatingToasterExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaEatingToasterExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaEatingToasterExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaEatingToasterExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureAirConditioningExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Air conditioning exception.
pub enum LivingAreaFeatureAirConditioningExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureAirConditioningExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureAirConditioningExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureAirConditioningExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureAirConditioningExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureAirConditioningExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureAirConditioningExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureAirConditioningExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureAirConditioningExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureAirConditioningExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureAirConditioningExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureAirConditioningExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureBathtubExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Bathtub exception.
pub enum LivingAreaFeatureBathtubExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureBathtubExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureBathtubExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureBathtubExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureBathtubExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureBathtubExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureBathtubExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureBathtubExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureBathtubExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureBathtubExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureBathtubExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureBathtubExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureBidetExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Bidet exception.
pub enum LivingAreaFeatureBidetExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureBidetExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureBidetExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureBidetExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureBidetExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureBidetExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureBidetExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureBidetExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureBidetExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureBidetExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureBidetExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureBidetExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureDryerExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Dryer exception.
pub enum LivingAreaFeatureDryerExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureDryerExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureDryerExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureDryerExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureDryerExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureDryerExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureDryerExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureDryerExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureDryerExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureDryerExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureDryerExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureDryerExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureElectronicRoomKeyExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Electronic room key exception.
pub enum LivingAreaFeatureElectronicRoomKeyExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureElectronicRoomKeyExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureElectronicRoomKeyExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureElectronicRoomKeyExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureElectronicRoomKeyExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureElectronicRoomKeyExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureElectronicRoomKeyExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureElectronicRoomKeyExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureElectronicRoomKeyExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureElectronicRoomKeyExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureElectronicRoomKeyExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureElectronicRoomKeyExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureFireplaceExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Fireplace exception.
pub enum LivingAreaFeatureFireplaceExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureFireplaceExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureFireplaceExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureFireplaceExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureFireplaceExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureFireplaceExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureFireplaceExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureFireplaceExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureFireplaceExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureFireplaceExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureFireplaceExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureFireplaceExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureHairdryerExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Hairdryer exception.
pub enum LivingAreaFeatureHairdryerExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureHairdryerExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureHairdryerExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureHairdryerExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureHairdryerExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureHairdryerExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureHairdryerExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureHairdryerExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureHairdryerExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureHairdryerExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureHairdryerExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureHairdryerExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureHeatingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Heating exception.
pub enum LivingAreaFeatureHeatingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureHeatingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureHeatingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureHeatingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureHeatingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureHeatingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureHeatingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureHeatingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureHeatingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureHeatingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureHeatingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureHeatingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureInunitSafeExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// In-unit safe exception.
pub enum LivingAreaFeatureInunitSafeExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureInunitSafeExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureInunitSafeExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureInunitSafeExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureInunitSafeExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureInunitSafeExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureInunitSafeExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureInunitSafeExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureInunitSafeExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureInunitSafeExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureInunitSafeExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureInunitSafeExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureInunitWifiAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// In-unit Wifi available exception.
pub enum LivingAreaFeatureInunitWifiAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureInunitWifiAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureInunitWifiAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureInunitWifiAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureInunitWifiAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureInunitWifiAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureInunitWifiAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureInunitWifiAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureInunitWifiAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureInunitWifiAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureInunitWifiAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureInunitWifiAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureIroningEquipmentExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Ironing equipment exception.
pub enum LivingAreaFeatureIroningEquipmentExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureIroningEquipmentExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureIroningEquipmentExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureIroningEquipmentExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureIroningEquipmentExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureIroningEquipmentExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureIroningEquipmentExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureIroningEquipmentExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureIroningEquipmentExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureIroningEquipmentExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureIroningEquipmentExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureIroningEquipmentExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeaturePayPerViewMoviesExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Pay per view movies exception.
pub enum LivingAreaFeaturePayPerViewMoviesExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeaturePayPerViewMoviesExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeaturePayPerViewMoviesExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeaturePayPerViewMoviesExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeaturePayPerViewMoviesExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeaturePayPerViewMoviesExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeaturePayPerViewMoviesExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeaturePayPerViewMoviesExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeaturePayPerViewMoviesExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeaturePayPerViewMoviesExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeaturePayPerViewMoviesExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeaturePayPerViewMoviesExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeaturePrivateBathroomExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Private bathroom exception.
pub enum LivingAreaFeaturePrivateBathroomExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeaturePrivateBathroomExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeaturePrivateBathroomExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeaturePrivateBathroomExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeaturePrivateBathroomExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeaturePrivateBathroomExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeaturePrivateBathroomExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeaturePrivateBathroomExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeaturePrivateBathroomExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeaturePrivateBathroomExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeaturePrivateBathroomExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeaturePrivateBathroomExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureShowerExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Shower exception.
pub enum LivingAreaFeatureShowerExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureShowerExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureShowerExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureShowerExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureShowerExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureShowerExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureShowerExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureShowerExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureShowerExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureShowerExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureShowerExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureShowerExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureToiletExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Toilet exception.
pub enum LivingAreaFeatureToiletExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureToiletExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureToiletExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureToiletExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureToiletExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureToiletExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureToiletExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureToiletExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureToiletExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureToiletExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureToiletExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureToiletExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureTvCastingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// TV exception.
pub enum LivingAreaFeatureTvCastingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureTvCastingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureTvCastingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureTvCastingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureTvCastingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureTvCastingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureTvCastingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureTvCastingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureTvCastingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureTvCastingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureTvCastingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureTvCastingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureTvExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// TV exception.
pub enum LivingAreaFeatureTvExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureTvExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureTvExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureTvExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureTvExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureTvExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureTvExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureTvExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureTvExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureTvExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureTvExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureTvExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureTvStreamingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// TV streaming exception.
pub enum LivingAreaFeatureTvStreamingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureTvStreamingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureTvStreamingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureTvStreamingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureTvStreamingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureTvStreamingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureTvStreamingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureTvStreamingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureTvStreamingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureTvStreamingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureTvStreamingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureTvStreamingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureUniversalPowerAdaptersExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Universal power adapters exception.
pub enum LivingAreaFeatureUniversalPowerAdaptersExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureUniversalPowerAdaptersExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureUniversalPowerAdaptersExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureUniversalPowerAdaptersExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureUniversalPowerAdaptersExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureUniversalPowerAdaptersExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureUniversalPowerAdaptersExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureUniversalPowerAdaptersExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureUniversalPowerAdaptersExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureUniversalPowerAdaptersExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureUniversalPowerAdaptersExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureUniversalPowerAdaptersExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaFeatureWasherExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Washer exception.
pub enum LivingAreaFeatureWasherExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaFeatureWasherExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaFeatureWasherExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaFeatureWasherExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaFeatureWasherExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaFeatureWasherExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaFeatureWasherExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaFeatureWasherExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaFeatureWasherExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaFeatureWasherExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaFeatureWasherExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaFeatureWasherExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaLayoutBalconyExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Balcony exception.
pub enum LivingAreaLayoutBalconyExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaLayoutBalconyExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaLayoutBalconyExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaLayoutBalconyExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaLayoutBalconyExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaLayoutBalconyExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaLayoutBalconyExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaLayoutBalconyExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaLayoutBalconyExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaLayoutBalconyExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaLayoutBalconyExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaLayoutBalconyExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaLayoutLivingAreaSqMetersExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Living area sq meters exception.
pub enum LivingAreaLayoutLivingAreaSqMetersExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaLayoutLivingAreaSqMetersExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaLayoutLivingAreaSqMetersExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaLayoutLivingAreaSqMetersExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaLayoutLivingAreaSqMetersExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaLayoutLivingAreaSqMetersExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaLayoutLivingAreaSqMetersExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaLayoutLivingAreaSqMetersExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaLayoutLivingAreaSqMetersExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaLayoutLivingAreaSqMetersExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaLayoutLivingAreaSqMetersExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaLayoutLivingAreaSqMetersExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaLayoutLoftExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Loft exception.
pub enum LivingAreaLayoutLoftExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaLayoutLoftExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaLayoutLoftExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaLayoutLoftExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaLayoutLoftExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaLayoutLoftExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaLayoutLoftExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaLayoutLoftExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaLayoutLoftExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaLayoutLoftExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaLayoutLoftExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaLayoutLoftExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaLayoutNonSmokingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Non smoking exception.
pub enum LivingAreaLayoutNonSmokingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaLayoutNonSmokingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaLayoutNonSmokingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaLayoutNonSmokingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaLayoutNonSmokingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaLayoutNonSmokingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaLayoutNonSmokingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaLayoutNonSmokingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaLayoutNonSmokingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaLayoutNonSmokingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaLayoutNonSmokingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaLayoutNonSmokingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaLayoutPatioExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Patio exception.
pub enum LivingAreaLayoutPatioExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaLayoutPatioExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaLayoutPatioExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaLayoutPatioExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaLayoutPatioExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaLayoutPatioExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaLayoutPatioExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaLayoutPatioExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaLayoutPatioExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaLayoutPatioExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaLayoutPatioExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaLayoutPatioExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaLayoutStairsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Stairs exception.
pub enum LivingAreaLayoutStairsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaLayoutStairsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaLayoutStairsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaLayoutStairsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaLayoutStairsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaLayoutStairsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaLayoutStairsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaLayoutStairsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaLayoutStairsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaLayoutStairsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaLayoutStairsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaLayoutStairsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingBedsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Beds count exception.
pub enum LivingAreaSleepingBedsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingBedsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingBedsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingBedsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingBedsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingBedsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingBedsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingBedsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingBunkBedsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Bunk beds count exception.
pub enum LivingAreaSleepingBunkBedsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingBunkBedsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingBunkBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingBunkBedsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingBunkBedsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingBunkBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingBunkBedsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingBunkBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingBunkBedsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingBunkBedsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingBunkBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingBunkBedsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingCribsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Cribs count exception.
pub enum LivingAreaSleepingCribsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingCribsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingCribsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingCribsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingCribsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingCribsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingCribsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingCribsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingCribsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingCribsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingCribsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingCribsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingDoubleBedsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Double beds count exception.
pub enum LivingAreaSleepingDoubleBedsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingDoubleBedsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingDoubleBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingDoubleBedsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingDoubleBedsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingDoubleBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingDoubleBedsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingDoubleBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingDoubleBedsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingDoubleBedsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingDoubleBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingDoubleBedsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingFeatherPillowsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Feather pillows exception.
pub enum LivingAreaSleepingFeatherPillowsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingFeatherPillowsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingFeatherPillowsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingFeatherPillowsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingFeatherPillowsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingFeatherPillowsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingFeatherPillowsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingFeatherPillowsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingFeatherPillowsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingFeatherPillowsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingFeatherPillowsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingFeatherPillowsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingHypoallergenicBeddingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Hypoallergenic bedding exception.
pub enum LivingAreaSleepingHypoallergenicBeddingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingHypoallergenicBeddingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingHypoallergenicBeddingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingHypoallergenicBeddingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingHypoallergenicBeddingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingHypoallergenicBeddingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingHypoallergenicBeddingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingHypoallergenicBeddingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingHypoallergenicBeddingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingHypoallergenicBeddingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingHypoallergenicBeddingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingHypoallergenicBeddingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingKingBedsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// King beds count exception.
pub enum LivingAreaSleepingKingBedsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingKingBedsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingKingBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingKingBedsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingKingBedsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingKingBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingKingBedsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingKingBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingKingBedsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingKingBedsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingKingBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingKingBedsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingMemoryFoamPillowsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Memory foam pillows exception.
pub enum LivingAreaSleepingMemoryFoamPillowsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingMemoryFoamPillowsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingMemoryFoamPillowsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingMemoryFoamPillowsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingMemoryFoamPillowsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingMemoryFoamPillowsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingMemoryFoamPillowsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingMemoryFoamPillowsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingMemoryFoamPillowsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingMemoryFoamPillowsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingMemoryFoamPillowsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingMemoryFoamPillowsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingOtherBedsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Other beds count exception.
pub enum LivingAreaSleepingOtherBedsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingOtherBedsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingOtherBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingOtherBedsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingOtherBedsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingOtherBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingOtherBedsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingOtherBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingOtherBedsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingOtherBedsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingOtherBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingOtherBedsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingQueenBedsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Queen beds count exception.
pub enum LivingAreaSleepingQueenBedsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingQueenBedsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingQueenBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingQueenBedsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingQueenBedsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingQueenBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingQueenBedsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingQueenBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingQueenBedsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingQueenBedsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingQueenBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingQueenBedsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingRollAwayBedsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Roll away beds count exception.
pub enum LivingAreaSleepingRollAwayBedsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingRollAwayBedsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingRollAwayBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingRollAwayBedsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingRollAwayBedsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingRollAwayBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingRollAwayBedsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingRollAwayBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingRollAwayBedsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingRollAwayBedsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingRollAwayBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingRollAwayBedsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Single or twin beds count exception.
pub enum LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingSofaBedsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sofa beds count exception.
pub enum LivingAreaSleepingSofaBedsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingSofaBedsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingSofaBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingSofaBedsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingSofaBedsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingSofaBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingSofaBedsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingSofaBedsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingSofaBedsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingSofaBedsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingSofaBedsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingSofaBedsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LivingAreaSleepingSyntheticPillowsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Synthetic pillows exception.
pub enum LivingAreaSleepingSyntheticPillowsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for LivingAreaSleepingSyntheticPillowsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LivingAreaSleepingSyntheticPillowsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            LivingAreaSleepingSyntheticPillowsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            LivingAreaSleepingSyntheticPillowsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            LivingAreaSleepingSyntheticPillowsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for LivingAreaSleepingSyntheticPillowsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(LivingAreaSleepingSyntheticPillowsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(LivingAreaSleepingSyntheticPillowsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(LivingAreaSleepingSyntheticPillowsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(LivingAreaSleepingSyntheticPillowsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LivingAreaSleepingSyntheticPillowsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MinimizedContactContactlessCheckinCheckoutExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Contactless check-in check-out exception.
pub enum MinimizedContactContactlessCheckinCheckoutExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for MinimizedContactContactlessCheckinCheckoutExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MinimizedContactContactlessCheckinCheckoutExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            MinimizedContactContactlessCheckinCheckoutExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            MinimizedContactContactlessCheckinCheckoutExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            MinimizedContactContactlessCheckinCheckoutExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for MinimizedContactContactlessCheckinCheckoutExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(MinimizedContactContactlessCheckinCheckoutExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(MinimizedContactContactlessCheckinCheckoutExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(MinimizedContactContactlessCheckinCheckoutExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(MinimizedContactContactlessCheckinCheckoutExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MinimizedContactContactlessCheckinCheckoutExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MinimizedContactDigitalGuestRoomKeysExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Digital guest room keys exception.
pub enum MinimizedContactDigitalGuestRoomKeysExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for MinimizedContactDigitalGuestRoomKeysExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MinimizedContactDigitalGuestRoomKeysExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            MinimizedContactDigitalGuestRoomKeysExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            MinimizedContactDigitalGuestRoomKeysExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            MinimizedContactDigitalGuestRoomKeysExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for MinimizedContactDigitalGuestRoomKeysExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(MinimizedContactDigitalGuestRoomKeysExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(MinimizedContactDigitalGuestRoomKeysExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(MinimizedContactDigitalGuestRoomKeysExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(MinimizedContactDigitalGuestRoomKeysExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MinimizedContactDigitalGuestRoomKeysExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Housekeeping scheduled request only exception.
pub enum MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// No high touch items common areas exception.
pub enum MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// No high touch items guest rooms exception.
pub enum MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MinimizedContactPlasticKeycardsDisinfectedExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Plastic keycards disinfected exception.
pub enum MinimizedContactPlasticKeycardsDisinfectedExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for MinimizedContactPlasticKeycardsDisinfectedExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MinimizedContactPlasticKeycardsDisinfectedExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            MinimizedContactPlasticKeycardsDisinfectedExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            MinimizedContactPlasticKeycardsDisinfectedExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            MinimizedContactPlasticKeycardsDisinfectedExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for MinimizedContactPlasticKeycardsDisinfectedExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(MinimizedContactPlasticKeycardsDisinfectedExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(MinimizedContactPlasticKeycardsDisinfectedExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(MinimizedContactPlasticKeycardsDisinfectedExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(MinimizedContactPlasticKeycardsDisinfectedExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MinimizedContactPlasticKeycardsDisinfectedExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MinimizedContactRoomBookingsBufferExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Room bookings buffer exception.
pub enum MinimizedContactRoomBookingsBufferExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for MinimizedContactRoomBookingsBufferExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MinimizedContactRoomBookingsBufferExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            MinimizedContactRoomBookingsBufferExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            MinimizedContactRoomBookingsBufferExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            MinimizedContactRoomBookingsBufferExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for MinimizedContactRoomBookingsBufferExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(MinimizedContactRoomBookingsBufferExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(MinimizedContactRoomBookingsBufferExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(MinimizedContactRoomBookingsBufferExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(MinimizedContactRoomBookingsBufferExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MinimizedContactRoomBookingsBufferExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParkingElectricCarChargingStationsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Electric car charging stations exception.
pub enum ParkingElectricCarChargingStationsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ParkingElectricCarChargingStationsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParkingElectricCarChargingStationsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ParkingElectricCarChargingStationsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ParkingElectricCarChargingStationsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ParkingElectricCarChargingStationsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ParkingElectricCarChargingStationsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ParkingElectricCarChargingStationsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ParkingElectricCarChargingStationsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ParkingElectricCarChargingStationsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ParkingElectricCarChargingStationsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParkingElectricCarChargingStationsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParkingFreeParkingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free parking exception.
pub enum ParkingFreeParkingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ParkingFreeParkingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParkingFreeParkingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ParkingFreeParkingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ParkingFreeParkingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ParkingFreeParkingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ParkingFreeParkingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ParkingFreeParkingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ParkingFreeParkingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ParkingFreeParkingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ParkingFreeParkingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParkingFreeParkingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParkingFreeSelfParkingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free self parking exception.
pub enum ParkingFreeSelfParkingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ParkingFreeSelfParkingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParkingFreeSelfParkingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ParkingFreeSelfParkingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ParkingFreeSelfParkingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ParkingFreeSelfParkingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ParkingFreeSelfParkingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ParkingFreeSelfParkingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ParkingFreeSelfParkingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ParkingFreeSelfParkingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ParkingFreeSelfParkingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParkingFreeSelfParkingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParkingFreeValetParkingExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free valet parking exception.
pub enum ParkingFreeValetParkingExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ParkingFreeValetParkingExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParkingFreeValetParkingExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ParkingFreeValetParkingExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ParkingFreeValetParkingExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ParkingFreeValetParkingExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ParkingFreeValetParkingExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ParkingFreeValetParkingExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ParkingFreeValetParkingExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ParkingFreeValetParkingExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ParkingFreeValetParkingExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParkingFreeValetParkingExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParkingParkingAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Parking available exception.
pub enum ParkingParkingAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ParkingParkingAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParkingParkingAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ParkingParkingAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ParkingParkingAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ParkingParkingAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ParkingParkingAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ParkingParkingAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ParkingParkingAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ParkingParkingAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ParkingParkingAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParkingParkingAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParkingSelfParkingAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Self parking available exception.
pub enum ParkingSelfParkingAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ParkingSelfParkingAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParkingSelfParkingAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ParkingSelfParkingAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ParkingSelfParkingAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ParkingSelfParkingAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ParkingSelfParkingAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ParkingSelfParkingAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ParkingSelfParkingAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ParkingSelfParkingAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ParkingSelfParkingAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParkingSelfParkingAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParkingValetParkingAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Valet parking available exception.
pub enum ParkingValetParkingAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ParkingValetParkingAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParkingValetParkingAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ParkingValetParkingAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ParkingValetParkingAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ParkingValetParkingAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ParkingValetParkingAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ParkingValetParkingAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ParkingValetParkingAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ParkingValetParkingAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ParkingValetParkingAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParkingValetParkingAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PaymentOptionCashExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Cash exception.
pub enum PaymentOptionCashExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PaymentOptionCashExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PaymentOptionCashExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PaymentOptionCashExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PaymentOptionCashExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PaymentOptionCashExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PaymentOptionCashExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PaymentOptionCashExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PaymentOptionCashExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PaymentOptionCashExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PaymentOptionCashExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PaymentOptionCashExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PaymentOptionChequeExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Cheque exception.
pub enum PaymentOptionChequeExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PaymentOptionChequeExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PaymentOptionChequeExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PaymentOptionChequeExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PaymentOptionChequeExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PaymentOptionChequeExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PaymentOptionChequeExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PaymentOptionChequeExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PaymentOptionChequeExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PaymentOptionChequeExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PaymentOptionChequeExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PaymentOptionChequeExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PaymentOptionCreditCardExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Credit card exception.
pub enum PaymentOptionCreditCardExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PaymentOptionCreditCardExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PaymentOptionCreditCardExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PaymentOptionCreditCardExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PaymentOptionCreditCardExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PaymentOptionCreditCardExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PaymentOptionCreditCardExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PaymentOptionCreditCardExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PaymentOptionCreditCardExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PaymentOptionCreditCardExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PaymentOptionCreditCardExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PaymentOptionCreditCardExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PaymentOptionDebitCardExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Debit card exception.
pub enum PaymentOptionDebitCardExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PaymentOptionDebitCardExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PaymentOptionDebitCardExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PaymentOptionDebitCardExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PaymentOptionDebitCardExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PaymentOptionDebitCardExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PaymentOptionDebitCardExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PaymentOptionDebitCardExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PaymentOptionDebitCardExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PaymentOptionDebitCardExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PaymentOptionDebitCardExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PaymentOptionDebitCardExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PaymentOptionMobileNfcExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mobile nfc exception.
pub enum PaymentOptionMobileNfcExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PaymentOptionMobileNfcExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PaymentOptionMobileNfcExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PaymentOptionMobileNfcExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PaymentOptionMobileNfcExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PaymentOptionMobileNfcExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PaymentOptionMobileNfcExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PaymentOptionMobileNfcExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PaymentOptionMobileNfcExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PaymentOptionMobileNfcExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PaymentOptionMobileNfcExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PaymentOptionMobileNfcExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Common areas offer sanitizing items exception.
pub enum PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonalProtectionFaceMaskRequiredExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Face mask required exception.
pub enum PersonalProtectionFaceMaskRequiredExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PersonalProtectionFaceMaskRequiredExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonalProtectionFaceMaskRequiredExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PersonalProtectionFaceMaskRequiredExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PersonalProtectionFaceMaskRequiredExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PersonalProtectionFaceMaskRequiredExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonalProtectionFaceMaskRequiredExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PersonalProtectionFaceMaskRequiredExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PersonalProtectionFaceMaskRequiredExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PersonalProtectionFaceMaskRequiredExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PersonalProtectionFaceMaskRequiredExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonalProtectionFaceMaskRequiredExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Guest room hygiene kits available exception.
pub enum PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonalProtectionProtectiveEquipmentAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Protective equipment available exception.
pub enum PersonalProtectionProtectiveEquipmentAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PersonalProtectionProtectiveEquipmentAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonalProtectionProtectiveEquipmentAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PersonalProtectionProtectiveEquipmentAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PersonalProtectionProtectiveEquipmentAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PersonalProtectionProtectiveEquipmentAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonalProtectionProtectiveEquipmentAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PersonalProtectionProtectiveEquipmentAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PersonalProtectionProtectiveEquipmentAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PersonalProtectionProtectiveEquipmentAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PersonalProtectionProtectiveEquipmentAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonalProtectionProtectiveEquipmentAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PetCatsAllowedExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Cats allowed exception.
pub enum PetCatsAllowedExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PetCatsAllowedExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PetCatsAllowedExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PetCatsAllowedExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PetCatsAllowedExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PetCatsAllowedExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PetCatsAllowedExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PetCatsAllowedExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PetCatsAllowedExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PetCatsAllowedExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PetCatsAllowedExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PetCatsAllowedExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PetDogsAllowedExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Dogs allowed exception.
pub enum PetDogsAllowedExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PetDogsAllowedExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PetDogsAllowedExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PetDogsAllowedExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PetDogsAllowedExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PetDogsAllowedExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PetDogsAllowedExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PetDogsAllowedExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PetDogsAllowedExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PetDogsAllowedExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PetDogsAllowedExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PetDogsAllowedExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PetPetsAllowedExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Pets allowed exception.
pub enum PetPetsAllowedExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PetPetsAllowedExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PetPetsAllowedExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PetPetsAllowedExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PetPetsAllowedExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PetPetsAllowedExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PetPetsAllowedExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PetPetsAllowedExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PetPetsAllowedExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PetPetsAllowedExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PetPetsAllowedExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PetPetsAllowedExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PetPetsAllowedFreeExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Pets allowed free exception.
pub enum PetPetsAllowedFreeExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PetPetsAllowedFreeExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PetPetsAllowedFreeExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PetPetsAllowedFreeExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PetPetsAllowedFreeExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PetPetsAllowedFreeExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PetPetsAllowedFreeExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PetPetsAllowedFreeExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PetPetsAllowedFreeExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PetPetsAllowedFreeExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PetPetsAllowedFreeExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PetPetsAllowedFreeExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Common areas physical distancing arranged exception.
pub enum PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PhysicalDistancingPhysicalDistancingRequiredExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Physical distancing required exception.
pub enum PhysicalDistancingPhysicalDistancingRequiredExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PhysicalDistancingPhysicalDistancingRequiredExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PhysicalDistancingPhysicalDistancingRequiredExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PhysicalDistancingPhysicalDistancingRequiredExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PhysicalDistancingPhysicalDistancingRequiredExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PhysicalDistancingPhysicalDistancingRequiredExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PhysicalDistancingPhysicalDistancingRequiredExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PhysicalDistancingPhysicalDistancingRequiredExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PhysicalDistancingPhysicalDistancingRequiredExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PhysicalDistancingPhysicalDistancingRequiredExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PhysicalDistancingPhysicalDistancingRequiredExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PhysicalDistancingPhysicalDistancingRequiredExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PhysicalDistancingSafetyDividersExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Safety dividers exception.
pub enum PhysicalDistancingSafetyDividersExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PhysicalDistancingSafetyDividersExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PhysicalDistancingSafetyDividersExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PhysicalDistancingSafetyDividersExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PhysicalDistancingSafetyDividersExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PhysicalDistancingSafetyDividersExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PhysicalDistancingSafetyDividersExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PhysicalDistancingSafetyDividersExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PhysicalDistancingSafetyDividersExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PhysicalDistancingSafetyDividersExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PhysicalDistancingSafetyDividersExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PhysicalDistancingSafetyDividersExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Shared areas limited occupancy exception.
pub enum PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Wellness areas have private spaces exception.
pub enum PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyAllInclusiveAvailableExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All inclusive available exception.
pub enum PolicyAllInclusiveAvailableExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PolicyAllInclusiveAvailableExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyAllInclusiveAvailableExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PolicyAllInclusiveAvailableExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PolicyAllInclusiveAvailableExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PolicyAllInclusiveAvailableExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyAllInclusiveAvailableExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PolicyAllInclusiveAvailableExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PolicyAllInclusiveAvailableExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PolicyAllInclusiveAvailableExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PolicyAllInclusiveAvailableExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyAllInclusiveAvailableExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyAllInclusiveOnlyExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All inclusive only exception.
pub enum PolicyAllInclusiveOnlyExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PolicyAllInclusiveOnlyExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyAllInclusiveOnlyExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PolicyAllInclusiveOnlyExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PolicyAllInclusiveOnlyExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PolicyAllInclusiveOnlyExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyAllInclusiveOnlyExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PolicyAllInclusiveOnlyExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PolicyAllInclusiveOnlyExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PolicyAllInclusiveOnlyExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PolicyAllInclusiveOnlyExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyAllInclusiveOnlyExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyCheckinTimeExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Check-in time exception.
pub enum PolicyCheckinTimeExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PolicyCheckinTimeExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyCheckinTimeExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PolicyCheckinTimeExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PolicyCheckinTimeExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PolicyCheckinTimeExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyCheckinTimeExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PolicyCheckinTimeExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PolicyCheckinTimeExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PolicyCheckinTimeExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PolicyCheckinTimeExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyCheckinTimeExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyCheckoutTimeExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Check-out time exception.
pub enum PolicyCheckoutTimeExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PolicyCheckoutTimeExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyCheckoutTimeExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PolicyCheckoutTimeExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PolicyCheckoutTimeExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PolicyCheckoutTimeExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyCheckoutTimeExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PolicyCheckoutTimeExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PolicyCheckoutTimeExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PolicyCheckoutTimeExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PolicyCheckoutTimeExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyCheckoutTimeExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyKidsStayFreeExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Kids stay free exception.
pub enum PolicyKidsStayFreeExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PolicyKidsStayFreeExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyKidsStayFreeExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PolicyKidsStayFreeExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PolicyKidsStayFreeExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PolicyKidsStayFreeExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyKidsStayFreeExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PolicyKidsStayFreeExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PolicyKidsStayFreeExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PolicyKidsStayFreeExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PolicyKidsStayFreeExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyKidsStayFreeExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyMaxChildAgeExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Max child age exception.
pub enum PolicyMaxChildAgeExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PolicyMaxChildAgeExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyMaxChildAgeExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PolicyMaxChildAgeExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PolicyMaxChildAgeExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PolicyMaxChildAgeExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyMaxChildAgeExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PolicyMaxChildAgeExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PolicyMaxChildAgeExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PolicyMaxChildAgeExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PolicyMaxChildAgeExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyMaxChildAgeExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyMaxKidsStayFreeCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Max kids stay free count exception.
pub enum PolicyMaxKidsStayFreeCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PolicyMaxKidsStayFreeCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyMaxKidsStayFreeCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PolicyMaxKidsStayFreeCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PolicyMaxKidsStayFreeCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PolicyMaxKidsStayFreeCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyMaxKidsStayFreeCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PolicyMaxKidsStayFreeCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PolicyMaxKidsStayFreeCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PolicyMaxKidsStayFreeCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PolicyMaxKidsStayFreeCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyMaxKidsStayFreeCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicySmokeFreePropertyExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Smoke free property exception.
pub enum PolicySmokeFreePropertyExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PolicySmokeFreePropertyExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicySmokeFreePropertyExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PolicySmokeFreePropertyExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PolicySmokeFreePropertyExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PolicySmokeFreePropertyExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicySmokeFreePropertyExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PolicySmokeFreePropertyExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PolicySmokeFreePropertyExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PolicySmokeFreePropertyExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PolicySmokeFreePropertyExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicySmokeFreePropertyExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolAdultPoolExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Adult pool exception.
pub enum PoolAdultPoolExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolAdultPoolExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolAdultPoolExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolAdultPoolExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolAdultPoolExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolAdultPoolExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolAdultPoolExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolAdultPoolExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolAdultPoolExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolAdultPoolExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolAdultPoolExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolAdultPoolExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolHotTubExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Hot tub exception.
pub enum PoolHotTubExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolHotTubExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolHotTubExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolHotTubExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolHotTubExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolHotTubExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolHotTubExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolHotTubExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolHotTubExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolHotTubExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolHotTubExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolHotTubExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolIndoorPoolExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indoor pool exception.
pub enum PoolIndoorPoolExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolIndoorPoolExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolIndoorPoolExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolIndoorPoolExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolIndoorPoolExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolIndoorPoolExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolIndoorPoolExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolIndoorPoolExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolIndoorPoolExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolIndoorPoolExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolIndoorPoolExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolIndoorPoolExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolIndoorPoolsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indoor pools count exception.
pub enum PoolIndoorPoolsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolIndoorPoolsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolIndoorPoolsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolIndoorPoolsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolIndoorPoolsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolIndoorPoolsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolIndoorPoolsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolIndoorPoolsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolIndoorPoolsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolIndoorPoolsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolIndoorPoolsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolIndoorPoolsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolLazyRiverExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Lazy river exception.
pub enum PoolLazyRiverExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolLazyRiverExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolLazyRiverExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolLazyRiverExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolLazyRiverExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolLazyRiverExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolLazyRiverExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolLazyRiverExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolLazyRiverExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolLazyRiverExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolLazyRiverExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolLazyRiverExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolLifeguardExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Lifeguard exception.
pub enum PoolLifeguardExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolLifeguardExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolLifeguardExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolLifeguardExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolLifeguardExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolLifeguardExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolLifeguardExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolLifeguardExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolLifeguardExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolLifeguardExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolLifeguardExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolLifeguardExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolOutdoorPoolExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Outdoor pool exception.
pub enum PoolOutdoorPoolExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolOutdoorPoolExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolOutdoorPoolExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolOutdoorPoolExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolOutdoorPoolExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolOutdoorPoolExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolOutdoorPoolExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolOutdoorPoolExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolOutdoorPoolExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolOutdoorPoolExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolOutdoorPoolExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolOutdoorPoolExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolOutdoorPoolsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Outdoor pools count exception.
pub enum PoolOutdoorPoolsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolOutdoorPoolsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolOutdoorPoolsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolOutdoorPoolsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolOutdoorPoolsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolOutdoorPoolsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolOutdoorPoolsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolOutdoorPoolsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolOutdoorPoolsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolOutdoorPoolsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolOutdoorPoolsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolOutdoorPoolsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolPoolExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Pool exception.
pub enum PoolPoolExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolPoolExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolPoolExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolPoolExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolPoolExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolPoolExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolPoolExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolPoolExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolPoolExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolPoolExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolPoolExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolPoolExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolPoolsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Pools count exception.
pub enum PoolPoolsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolPoolsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolPoolsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolPoolsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolPoolsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolPoolsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolPoolsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolPoolsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolPoolsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolPoolsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolPoolsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolPoolsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolWadingPoolExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Wading pool exception.
pub enum PoolWadingPoolExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolWadingPoolExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolWadingPoolExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolWadingPoolExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolWadingPoolExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolWadingPoolExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolWadingPoolExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolWadingPoolExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolWadingPoolExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolWadingPoolExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolWadingPoolExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolWadingPoolExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolWaterParkExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Water park exception.
pub enum PoolWaterParkExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolWaterParkExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolWaterParkExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolWaterParkExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolWaterParkExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolWaterParkExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolWaterParkExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolWaterParkExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolWaterParkExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolWaterParkExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolWaterParkExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolWaterParkExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolWaterslideExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Waterslide exception.
pub enum PoolWaterslideExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolWaterslideExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolWaterslideExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolWaterslideExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolWaterslideExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolWaterslideExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolWaterslideExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolWaterslideExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolWaterslideExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolWaterslideExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolWaterslideExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolWaterslideExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PoolWavePoolExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Wave pool exception.
pub enum PoolWavePoolExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PoolWavePoolExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PoolWavePoolExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PoolWavePoolExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PoolWavePoolExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PoolWavePoolExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PoolWavePoolExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PoolWavePoolExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PoolWavePoolExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PoolWavePoolExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PoolWavePoolExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PoolWavePoolExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PropertyBuiltYearExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Built year exception.
pub enum PropertyBuiltYearExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PropertyBuiltYearExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PropertyBuiltYearExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PropertyBuiltYearExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PropertyBuiltYearExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PropertyBuiltYearExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PropertyBuiltYearExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PropertyBuiltYearExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PropertyBuiltYearExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PropertyBuiltYearExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PropertyBuiltYearExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PropertyBuiltYearExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PropertyFloorsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Floors count exception.
pub enum PropertyFloorsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PropertyFloorsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PropertyFloorsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PropertyFloorsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PropertyFloorsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PropertyFloorsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PropertyFloorsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PropertyFloorsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PropertyFloorsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PropertyFloorsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PropertyFloorsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PropertyFloorsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PropertyLastRenovatedYearExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Last renovated year exception.
pub enum PropertyLastRenovatedYearExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PropertyLastRenovatedYearExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PropertyLastRenovatedYearExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PropertyLastRenovatedYearExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PropertyLastRenovatedYearExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PropertyLastRenovatedYearExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PropertyLastRenovatedYearExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PropertyLastRenovatedYearExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PropertyLastRenovatedYearExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PropertyLastRenovatedYearExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PropertyLastRenovatedYearExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PropertyLastRenovatedYearExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PropertyRoomsCountExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rooms count exception.
pub enum PropertyRoomsCountExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for PropertyRoomsCountExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PropertyRoomsCountExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            PropertyRoomsCountExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            PropertyRoomsCountExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            PropertyRoomsCountExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for PropertyRoomsCountExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(PropertyRoomsCountExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(PropertyRoomsCountExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(PropertyRoomsCountExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(PropertyRoomsCountExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PropertyRoomsCountExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceBaggageStorageExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Baggage storage exception.
pub enum ServiceBaggageStorageExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceBaggageStorageExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceBaggageStorageExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceBaggageStorageExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceBaggageStorageExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceBaggageStorageExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceBaggageStorageExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceBaggageStorageExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceBaggageStorageExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceBaggageStorageExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceBaggageStorageExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceBaggageStorageExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceConciergeExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Concierge exception.
pub enum ServiceConciergeExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceConciergeExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceConciergeExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceConciergeExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceConciergeExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceConciergeExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceConciergeExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceConciergeExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceConciergeExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceConciergeExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceConciergeExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceConciergeExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceConvenienceStoreExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Convenience store exception.
pub enum ServiceConvenienceStoreExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceConvenienceStoreExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceConvenienceStoreExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceConvenienceStoreExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceConvenienceStoreExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceConvenienceStoreExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceConvenienceStoreExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceConvenienceStoreExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceConvenienceStoreExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceConvenienceStoreExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceConvenienceStoreExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceConvenienceStoreExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceCurrencyExchangeExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Currency exchange exception.
pub enum ServiceCurrencyExchangeExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceCurrencyExchangeExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceCurrencyExchangeExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceCurrencyExchangeExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceCurrencyExchangeExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceCurrencyExchangeExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceCurrencyExchangeExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceCurrencyExchangeExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceCurrencyExchangeExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceCurrencyExchangeExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceCurrencyExchangeExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceCurrencyExchangeExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceElevatorExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Elevator exception.
pub enum ServiceElevatorExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceElevatorExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceElevatorExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceElevatorExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceElevatorExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceElevatorExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceElevatorExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceElevatorExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceElevatorExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceElevatorExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceElevatorExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceElevatorExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceFrontDeskExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Front desk exception.
pub enum ServiceFrontDeskExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceFrontDeskExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceFrontDeskExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceFrontDeskExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceFrontDeskExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceFrontDeskExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceFrontDeskExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceFrontDeskExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceFrontDeskExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceFrontDeskExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceFrontDeskExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceFrontDeskExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceFullServiceLaundryExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Full service laundry exception.
pub enum ServiceFullServiceLaundryExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceFullServiceLaundryExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceFullServiceLaundryExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceFullServiceLaundryExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceFullServiceLaundryExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceFullServiceLaundryExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceFullServiceLaundryExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceFullServiceLaundryExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceFullServiceLaundryExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceFullServiceLaundryExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceFullServiceLaundryExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceFullServiceLaundryExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceGiftShopExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Gift shop exception.
pub enum ServiceGiftShopExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceGiftShopExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceGiftShopExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceGiftShopExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceGiftShopExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceGiftShopExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceGiftShopExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceGiftShopExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceGiftShopExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceGiftShopExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceGiftShopExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceGiftShopExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceSelfServiceLaundryExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Self service laundry exception.
pub enum ServiceSelfServiceLaundryExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceSelfServiceLaundryExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceSelfServiceLaundryExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceSelfServiceLaundryExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceSelfServiceLaundryExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceSelfServiceLaundryExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceSelfServiceLaundryExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceSelfServiceLaundryExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceSelfServiceLaundryExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceSelfServiceLaundryExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceSelfServiceLaundryExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceSelfServiceLaundryExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceSocialHourExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Social hour exception.
pub enum ServiceSocialHourExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceSocialHourExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceSocialHourExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceSocialHourExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceSocialHourExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceSocialHourExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceSocialHourExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceSocialHourExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceSocialHourExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceSocialHourExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceSocialHourExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceSocialHourExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceTwentyFourHourFrontDeskExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// 24hr front desk exception.
pub enum ServiceTwentyFourHourFrontDeskExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceTwentyFourHourFrontDeskExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceTwentyFourHourFrontDeskExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceTwentyFourHourFrontDeskExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceTwentyFourHourFrontDeskExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceTwentyFourHourFrontDeskExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceTwentyFourHourFrontDeskExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceTwentyFourHourFrontDeskExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceTwentyFourHourFrontDeskExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceTwentyFourHourFrontDeskExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceTwentyFourHourFrontDeskExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceTwentyFourHourFrontDeskExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceWakeUpCallsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Wake up calls exception.
pub enum ServiceWakeUpCallsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ServiceWakeUpCallsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceWakeUpCallsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ServiceWakeUpCallsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ServiceWakeUpCallsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ServiceWakeUpCallsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceWakeUpCallsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ServiceWakeUpCallsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ServiceWakeUpCallsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ServiceWakeUpCallsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ServiceWakeUpCallsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceWakeUpCallsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainabilityCertificationBreeamCertificationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// BREEAM certification.
pub enum SustainabilityCertificationBreeamCertificationEnum {
    

    /// Default BreeamCertification. Do not use.
    ///
    /// "BREEAM_CERTIFICATION_UNSPECIFIED"
    #[serde(rename="BREEAM_CERTIFICATION_UNSPECIFIED")]
    BREEAMCERTIFICATIONUNSPECIFIED,
    

    /// Not certified.
    ///
    /// "NO_BREEAM_CERTIFICATION"
    #[serde(rename="NO_BREEAM_CERTIFICATION")]
    NOBREEAMCERTIFICATION,
    

    /// BREEAM Pass.
    ///
    /// "BREEAM_PASS"
    #[serde(rename="BREEAM_PASS")]
    BREEAMPASS,
    

    /// BREEAM Good.
    ///
    /// "BREEAM_GOOD"
    #[serde(rename="BREEAM_GOOD")]
    BREEAMGOOD,
    

    /// BREEAM Very Good.
    ///
    /// "BREEAM_VERY_GOOD"
    #[serde(rename="BREEAM_VERY_GOOD")]
    BREEAMVERYGOOD,
    

    /// BREEAM Excellent.
    ///
    /// "BREEAM_EXCELLENT"
    #[serde(rename="BREEAM_EXCELLENT")]
    BREEAMEXCELLENT,
    

    /// BREEAM Outstanding.
    ///
    /// "BREEAM_OUTSTANDING"
    #[serde(rename="BREEAM_OUTSTANDING")]
    BREEAMOUTSTANDING,
}

impl AsRef<str> for SustainabilityCertificationBreeamCertificationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainabilityCertificationBreeamCertificationEnum::BREEAMCERTIFICATIONUNSPECIFIED => "BREEAM_CERTIFICATION_UNSPECIFIED",
            SustainabilityCertificationBreeamCertificationEnum::NOBREEAMCERTIFICATION => "NO_BREEAM_CERTIFICATION",
            SustainabilityCertificationBreeamCertificationEnum::BREEAMPASS => "BREEAM_PASS",
            SustainabilityCertificationBreeamCertificationEnum::BREEAMGOOD => "BREEAM_GOOD",
            SustainabilityCertificationBreeamCertificationEnum::BREEAMVERYGOOD => "BREEAM_VERY_GOOD",
            SustainabilityCertificationBreeamCertificationEnum::BREEAMEXCELLENT => "BREEAM_EXCELLENT",
            SustainabilityCertificationBreeamCertificationEnum::BREEAMOUTSTANDING => "BREEAM_OUTSTANDING",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainabilityCertificationBreeamCertificationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BREEAM_CERTIFICATION_UNSPECIFIED" => Ok(SustainabilityCertificationBreeamCertificationEnum::BREEAMCERTIFICATIONUNSPECIFIED),
           "NO_BREEAM_CERTIFICATION" => Ok(SustainabilityCertificationBreeamCertificationEnum::NOBREEAMCERTIFICATION),
           "BREEAM_PASS" => Ok(SustainabilityCertificationBreeamCertificationEnum::BREEAMPASS),
           "BREEAM_GOOD" => Ok(SustainabilityCertificationBreeamCertificationEnum::BREEAMGOOD),
           "BREEAM_VERY_GOOD" => Ok(SustainabilityCertificationBreeamCertificationEnum::BREEAMVERYGOOD),
           "BREEAM_EXCELLENT" => Ok(SustainabilityCertificationBreeamCertificationEnum::BREEAMEXCELLENT),
           "BREEAM_OUTSTANDING" => Ok(SustainabilityCertificationBreeamCertificationEnum::BREEAMOUTSTANDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainabilityCertificationBreeamCertificationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainabilityCertificationBreeamCertificationExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// BREEAM certification exception.
pub enum SustainabilityCertificationBreeamCertificationExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for SustainabilityCertificationBreeamCertificationExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainabilityCertificationBreeamCertificationExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            SustainabilityCertificationBreeamCertificationExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            SustainabilityCertificationBreeamCertificationExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            SustainabilityCertificationBreeamCertificationExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainabilityCertificationBreeamCertificationExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(SustainabilityCertificationBreeamCertificationExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(SustainabilityCertificationBreeamCertificationExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(SustainabilityCertificationBreeamCertificationExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(SustainabilityCertificationBreeamCertificationExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainabilityCertificationBreeamCertificationExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainabilityCertificationLeedCertificationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// LEED certification.
pub enum SustainabilityCertificationLeedCertificationEnum {
    

    /// Default LeedCertification. Do not use.
    ///
    /// "LEED_CERTIFICATION_UNSPECIFIED"
    #[serde(rename="LEED_CERTIFICATION_UNSPECIFIED")]
    LEEDCERTIFICATIONUNSPECIFIED,
    

    /// Not certified.
    ///
    /// "NO_LEED_CERTIFICATION"
    #[serde(rename="NO_LEED_CERTIFICATION")]
    NOLEEDCERTIFICATION,
    

    /// LEED Certified.
    ///
    /// "LEED_CERTIFIED"
    #[serde(rename="LEED_CERTIFIED")]
    LEEDCERTIFIED,
    

    /// LEED Silver.
    ///
    /// "LEED_SILVER"
    #[serde(rename="LEED_SILVER")]
    LEEDSILVER,
    

    /// LEED Gold.
    ///
    /// "LEED_GOLD"
    #[serde(rename="LEED_GOLD")]
    LEEDGOLD,
    

    /// LEED Platinum.
    ///
    /// "LEED_PLATINUM"
    #[serde(rename="LEED_PLATINUM")]
    LEEDPLATINUM,
}

impl AsRef<str> for SustainabilityCertificationLeedCertificationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainabilityCertificationLeedCertificationEnum::LEEDCERTIFICATIONUNSPECIFIED => "LEED_CERTIFICATION_UNSPECIFIED",
            SustainabilityCertificationLeedCertificationEnum::NOLEEDCERTIFICATION => "NO_LEED_CERTIFICATION",
            SustainabilityCertificationLeedCertificationEnum::LEEDCERTIFIED => "LEED_CERTIFIED",
            SustainabilityCertificationLeedCertificationEnum::LEEDSILVER => "LEED_SILVER",
            SustainabilityCertificationLeedCertificationEnum::LEEDGOLD => "LEED_GOLD",
            SustainabilityCertificationLeedCertificationEnum::LEEDPLATINUM => "LEED_PLATINUM",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainabilityCertificationLeedCertificationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LEED_CERTIFICATION_UNSPECIFIED" => Ok(SustainabilityCertificationLeedCertificationEnum::LEEDCERTIFICATIONUNSPECIFIED),
           "NO_LEED_CERTIFICATION" => Ok(SustainabilityCertificationLeedCertificationEnum::NOLEEDCERTIFICATION),
           "LEED_CERTIFIED" => Ok(SustainabilityCertificationLeedCertificationEnum::LEEDCERTIFIED),
           "LEED_SILVER" => Ok(SustainabilityCertificationLeedCertificationEnum::LEEDSILVER),
           "LEED_GOLD" => Ok(SustainabilityCertificationLeedCertificationEnum::LEEDGOLD),
           "LEED_PLATINUM" => Ok(SustainabilityCertificationLeedCertificationEnum::LEEDPLATINUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainabilityCertificationLeedCertificationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainabilityCertificationLeedCertificationExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// LEED certification exception.
pub enum SustainabilityCertificationLeedCertificationExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for SustainabilityCertificationLeedCertificationExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainabilityCertificationLeedCertificationExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            SustainabilityCertificationLeedCertificationExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            SustainabilityCertificationLeedCertificationExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            SustainabilityCertificationLeedCertificationExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainabilityCertificationLeedCertificationExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(SustainabilityCertificationLeedCertificationExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(SustainabilityCertificationLeedCertificationExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(SustainabilityCertificationLeedCertificationExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(SustainabilityCertificationLeedCertificationExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainabilityCertificationLeedCertificationExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainableSourcingEcoFriendlyToiletriesExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Eco friendly toiletries exception.
pub enum SustainableSourcingEcoFriendlyToiletriesExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for SustainableSourcingEcoFriendlyToiletriesExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainableSourcingEcoFriendlyToiletriesExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            SustainableSourcingEcoFriendlyToiletriesExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            SustainableSourcingEcoFriendlyToiletriesExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            SustainableSourcingEcoFriendlyToiletriesExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainableSourcingEcoFriendlyToiletriesExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(SustainableSourcingEcoFriendlyToiletriesExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(SustainableSourcingEcoFriendlyToiletriesExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(SustainableSourcingEcoFriendlyToiletriesExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(SustainableSourcingEcoFriendlyToiletriesExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainableSourcingEcoFriendlyToiletriesExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Locally sourced food and beverages exception.
pub enum SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainableSourcingOrganicCageFreeEggsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Organic cage free eggs exception.
pub enum SustainableSourcingOrganicCageFreeEggsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for SustainableSourcingOrganicCageFreeEggsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainableSourcingOrganicCageFreeEggsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            SustainableSourcingOrganicCageFreeEggsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            SustainableSourcingOrganicCageFreeEggsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            SustainableSourcingOrganicCageFreeEggsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainableSourcingOrganicCageFreeEggsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(SustainableSourcingOrganicCageFreeEggsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(SustainableSourcingOrganicCageFreeEggsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(SustainableSourcingOrganicCageFreeEggsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(SustainableSourcingOrganicCageFreeEggsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainableSourcingOrganicCageFreeEggsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainableSourcingOrganicFoodAndBeveragesExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Organic food and beverages exception.
pub enum SustainableSourcingOrganicFoodAndBeveragesExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for SustainableSourcingOrganicFoodAndBeveragesExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainableSourcingOrganicFoodAndBeveragesExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            SustainableSourcingOrganicFoodAndBeveragesExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            SustainableSourcingOrganicFoodAndBeveragesExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            SustainableSourcingOrganicFoodAndBeveragesExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainableSourcingOrganicFoodAndBeveragesExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(SustainableSourcingOrganicFoodAndBeveragesExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(SustainableSourcingOrganicFoodAndBeveragesExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(SustainableSourcingOrganicFoodAndBeveragesExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(SustainableSourcingOrganicFoodAndBeveragesExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainableSourcingOrganicFoodAndBeveragesExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainableSourcingResponsiblePurchasingPolicyExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Responsible purchasing policy exception.
pub enum SustainableSourcingResponsiblePurchasingPolicyExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for SustainableSourcingResponsiblePurchasingPolicyExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainableSourcingResponsiblePurchasingPolicyExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            SustainableSourcingResponsiblePurchasingPolicyExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            SustainableSourcingResponsiblePurchasingPolicyExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            SustainableSourcingResponsiblePurchasingPolicyExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainableSourcingResponsiblePurchasingPolicyExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(SustainableSourcingResponsiblePurchasingPolicyExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(SustainableSourcingResponsiblePurchasingPolicyExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(SustainableSourcingResponsiblePurchasingPolicyExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(SustainableSourcingResponsiblePurchasingPolicyExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainableSourcingResponsiblePurchasingPolicyExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainableSourcingResponsiblySourcesSeafoodExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Responsibly sources seafood exception.
pub enum SustainableSourcingResponsiblySourcesSeafoodExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for SustainableSourcingResponsiblySourcesSeafoodExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainableSourcingResponsiblySourcesSeafoodExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            SustainableSourcingResponsiblySourcesSeafoodExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            SustainableSourcingResponsiblySourcesSeafoodExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            SustainableSourcingResponsiblySourcesSeafoodExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainableSourcingResponsiblySourcesSeafoodExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(SustainableSourcingResponsiblySourcesSeafoodExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(SustainableSourcingResponsiblySourcesSeafoodExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(SustainableSourcingResponsiblySourcesSeafoodExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(SustainableSourcingResponsiblySourcesSeafoodExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainableSourcingResponsiblySourcesSeafoodExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainableSourcingVeganMealsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Vegan meals exception.
pub enum SustainableSourcingVeganMealsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for SustainableSourcingVeganMealsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainableSourcingVeganMealsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            SustainableSourcingVeganMealsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            SustainableSourcingVeganMealsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            SustainableSourcingVeganMealsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainableSourcingVeganMealsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(SustainableSourcingVeganMealsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(SustainableSourcingVeganMealsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(SustainableSourcingVeganMealsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(SustainableSourcingVeganMealsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainableSourcingVeganMealsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SustainableSourcingVegetarianMealsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Vegetarian meals exception.
pub enum SustainableSourcingVegetarianMealsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for SustainableSourcingVegetarianMealsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SustainableSourcingVegetarianMealsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            SustainableSourcingVegetarianMealsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            SustainableSourcingVegetarianMealsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            SustainableSourcingVegetarianMealsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for SustainableSourcingVegetarianMealsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(SustainableSourcingVegetarianMealsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(SustainableSourcingVegetarianMealsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(SustainableSourcingVegetarianMealsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(SustainableSourcingVegetarianMealsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SustainableSourcingVegetarianMealsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransportationAirportShuttleExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Airport shuttle exception.
pub enum TransportationAirportShuttleExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for TransportationAirportShuttleExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransportationAirportShuttleExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            TransportationAirportShuttleExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            TransportationAirportShuttleExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            TransportationAirportShuttleExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for TransportationAirportShuttleExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(TransportationAirportShuttleExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(TransportationAirportShuttleExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(TransportationAirportShuttleExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(TransportationAirportShuttleExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransportationAirportShuttleExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransportationCarRentalOnPropertyExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Car rental on property exception.
pub enum TransportationCarRentalOnPropertyExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for TransportationCarRentalOnPropertyExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransportationCarRentalOnPropertyExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            TransportationCarRentalOnPropertyExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            TransportationCarRentalOnPropertyExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            TransportationCarRentalOnPropertyExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for TransportationCarRentalOnPropertyExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(TransportationCarRentalOnPropertyExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(TransportationCarRentalOnPropertyExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(TransportationCarRentalOnPropertyExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(TransportationCarRentalOnPropertyExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransportationCarRentalOnPropertyExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransportationFreeAirportShuttleExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free airport shuttle exception.
pub enum TransportationFreeAirportShuttleExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for TransportationFreeAirportShuttleExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransportationFreeAirportShuttleExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            TransportationFreeAirportShuttleExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            TransportationFreeAirportShuttleExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            TransportationFreeAirportShuttleExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for TransportationFreeAirportShuttleExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(TransportationFreeAirportShuttleExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(TransportationFreeAirportShuttleExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(TransportationFreeAirportShuttleExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(TransportationFreeAirportShuttleExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransportationFreeAirportShuttleExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransportationFreePrivateCarServiceExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free private car service exception.
pub enum TransportationFreePrivateCarServiceExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for TransportationFreePrivateCarServiceExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransportationFreePrivateCarServiceExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            TransportationFreePrivateCarServiceExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            TransportationFreePrivateCarServiceExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            TransportationFreePrivateCarServiceExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for TransportationFreePrivateCarServiceExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(TransportationFreePrivateCarServiceExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(TransportationFreePrivateCarServiceExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(TransportationFreePrivateCarServiceExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(TransportationFreePrivateCarServiceExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransportationFreePrivateCarServiceExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransportationLocalShuttleExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Local shuttle exception.
pub enum TransportationLocalShuttleExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for TransportationLocalShuttleExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransportationLocalShuttleExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            TransportationLocalShuttleExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            TransportationLocalShuttleExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            TransportationLocalShuttleExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for TransportationLocalShuttleExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(TransportationLocalShuttleExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(TransportationLocalShuttleExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(TransportationLocalShuttleExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(TransportationLocalShuttleExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransportationLocalShuttleExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransportationPrivateCarServiceExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Private car service exception.
pub enum TransportationPrivateCarServiceExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for TransportationPrivateCarServiceExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransportationPrivateCarServiceExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            TransportationPrivateCarServiceExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            TransportationPrivateCarServiceExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            TransportationPrivateCarServiceExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for TransportationPrivateCarServiceExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(TransportationPrivateCarServiceExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(TransportationPrivateCarServiceExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(TransportationPrivateCarServiceExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(TransportationPrivateCarServiceExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransportationPrivateCarServiceExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransportationTransferExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Transfer exception.
pub enum TransportationTransferExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for TransportationTransferExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransportationTransferExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            TransportationTransferExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            TransportationTransferExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            TransportationTransferExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for TransportationTransferExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(TransportationTransferExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(TransportationTransferExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(TransportationTransferExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(TransportationTransferExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransportationTransferExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ViewsFromUnitBeachViewExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Beach view exception.
pub enum ViewsFromUnitBeachViewExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ViewsFromUnitBeachViewExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ViewsFromUnitBeachViewExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ViewsFromUnitBeachViewExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ViewsFromUnitBeachViewExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ViewsFromUnitBeachViewExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ViewsFromUnitBeachViewExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ViewsFromUnitBeachViewExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ViewsFromUnitBeachViewExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ViewsFromUnitBeachViewExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ViewsFromUnitBeachViewExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ViewsFromUnitBeachViewExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ViewsFromUnitCityViewExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// City view exception.
pub enum ViewsFromUnitCityViewExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ViewsFromUnitCityViewExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ViewsFromUnitCityViewExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ViewsFromUnitCityViewExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ViewsFromUnitCityViewExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ViewsFromUnitCityViewExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ViewsFromUnitCityViewExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ViewsFromUnitCityViewExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ViewsFromUnitCityViewExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ViewsFromUnitCityViewExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ViewsFromUnitCityViewExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ViewsFromUnitCityViewExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ViewsFromUnitGardenViewExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Garden view exception.
pub enum ViewsFromUnitGardenViewExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ViewsFromUnitGardenViewExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ViewsFromUnitGardenViewExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ViewsFromUnitGardenViewExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ViewsFromUnitGardenViewExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ViewsFromUnitGardenViewExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ViewsFromUnitGardenViewExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ViewsFromUnitGardenViewExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ViewsFromUnitGardenViewExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ViewsFromUnitGardenViewExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ViewsFromUnitGardenViewExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ViewsFromUnitGardenViewExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ViewsFromUnitLakeViewExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Lake view exception.
pub enum ViewsFromUnitLakeViewExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ViewsFromUnitLakeViewExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ViewsFromUnitLakeViewExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ViewsFromUnitLakeViewExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ViewsFromUnitLakeViewExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ViewsFromUnitLakeViewExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ViewsFromUnitLakeViewExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ViewsFromUnitLakeViewExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ViewsFromUnitLakeViewExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ViewsFromUnitLakeViewExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ViewsFromUnitLakeViewExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ViewsFromUnitLakeViewExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ViewsFromUnitLandmarkViewExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Landmark view exception.
pub enum ViewsFromUnitLandmarkViewExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ViewsFromUnitLandmarkViewExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ViewsFromUnitLandmarkViewExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ViewsFromUnitLandmarkViewExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ViewsFromUnitLandmarkViewExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ViewsFromUnitLandmarkViewExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ViewsFromUnitLandmarkViewExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ViewsFromUnitLandmarkViewExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ViewsFromUnitLandmarkViewExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ViewsFromUnitLandmarkViewExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ViewsFromUnitLandmarkViewExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ViewsFromUnitLandmarkViewExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ViewsFromUnitOceanViewExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Ocean view exception.
pub enum ViewsFromUnitOceanViewExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ViewsFromUnitOceanViewExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ViewsFromUnitOceanViewExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ViewsFromUnitOceanViewExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ViewsFromUnitOceanViewExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ViewsFromUnitOceanViewExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ViewsFromUnitOceanViewExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ViewsFromUnitOceanViewExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ViewsFromUnitOceanViewExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ViewsFromUnitOceanViewExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ViewsFromUnitOceanViewExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ViewsFromUnitOceanViewExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ViewsFromUnitPoolViewExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Pool view exception.
pub enum ViewsFromUnitPoolViewExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ViewsFromUnitPoolViewExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ViewsFromUnitPoolViewExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ViewsFromUnitPoolViewExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ViewsFromUnitPoolViewExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ViewsFromUnitPoolViewExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ViewsFromUnitPoolViewExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ViewsFromUnitPoolViewExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ViewsFromUnitPoolViewExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ViewsFromUnitPoolViewExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ViewsFromUnitPoolViewExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ViewsFromUnitPoolViewExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ViewsFromUnitValleyViewExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Valley view exception.
pub enum ViewsFromUnitValleyViewExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for ViewsFromUnitValleyViewExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ViewsFromUnitValleyViewExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            ViewsFromUnitValleyViewExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            ViewsFromUnitValleyViewExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            ViewsFromUnitValleyViewExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for ViewsFromUnitValleyViewExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(ViewsFromUnitValleyViewExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(ViewsFromUnitValleyViewExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(ViewsFromUnitValleyViewExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(ViewsFromUnitValleyViewExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ViewsFromUnitValleyViewExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionCompostableFoodContainersAndCutleryExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Compostable food containers and cutlery exception.
pub enum WasteReductionCompostableFoodContainersAndCutleryExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionCompostableFoodContainersAndCutleryExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionCompostableFoodContainersAndCutleryExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionCompostableFoodContainersAndCutleryExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionCompostableFoodContainersAndCutleryExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionCompostableFoodContainersAndCutleryExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionCompostableFoodContainersAndCutleryExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionCompostableFoodContainersAndCutleryExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionCompostableFoodContainersAndCutleryExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionCompostableFoodContainersAndCutleryExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionCompostableFoodContainersAndCutleryExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionCompostableFoodContainersAndCutleryExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionCompostsExcessFoodExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Composts excess food exception.
pub enum WasteReductionCompostsExcessFoodExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionCompostsExcessFoodExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionCompostsExcessFoodExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionCompostsExcessFoodExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionCompostsExcessFoodExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionCompostsExcessFoodExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionCompostsExcessFoodExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionCompostsExcessFoodExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionCompostsExcessFoodExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionCompostsExcessFoodExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionCompostsExcessFoodExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionCompostsExcessFoodExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionDonatesExcessFoodExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Donates excess food exception.
pub enum WasteReductionDonatesExcessFoodExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionDonatesExcessFoodExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionDonatesExcessFoodExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionDonatesExcessFoodExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionDonatesExcessFoodExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionDonatesExcessFoodExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionDonatesExcessFoodExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionDonatesExcessFoodExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionDonatesExcessFoodExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionDonatesExcessFoodExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionDonatesExcessFoodExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionDonatesExcessFoodExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionFoodWasteReductionProgramExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Food waste reduction program exception.
pub enum WasteReductionFoodWasteReductionProgramExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionFoodWasteReductionProgramExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionFoodWasteReductionProgramExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionFoodWasteReductionProgramExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionFoodWasteReductionProgramExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionFoodWasteReductionProgramExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionFoodWasteReductionProgramExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionFoodWasteReductionProgramExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionFoodWasteReductionProgramExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionFoodWasteReductionProgramExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionFoodWasteReductionProgramExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionFoodWasteReductionProgramExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionNoSingleUsePlasticStrawsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// No single use plastic straws exception.
pub enum WasteReductionNoSingleUsePlasticStrawsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionNoSingleUsePlasticStrawsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionNoSingleUsePlasticStrawsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionNoSingleUsePlasticStrawsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionNoSingleUsePlasticStrawsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionNoSingleUsePlasticStrawsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionNoSingleUsePlasticStrawsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionNoSingleUsePlasticStrawsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionNoSingleUsePlasticStrawsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionNoSingleUsePlasticStrawsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionNoSingleUsePlasticStrawsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionNoSingleUsePlasticStrawsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// No single use plastic water bottles exception.
pub enum WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionNoStyrofoamFoodContainersExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// No styrofoam food containers exception.
pub enum WasteReductionNoStyrofoamFoodContainersExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionNoStyrofoamFoodContainersExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionNoStyrofoamFoodContainersExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionNoStyrofoamFoodContainersExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionNoStyrofoamFoodContainersExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionNoStyrofoamFoodContainersExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionNoStyrofoamFoodContainersExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionNoStyrofoamFoodContainersExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionNoStyrofoamFoodContainersExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionNoStyrofoamFoodContainersExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionNoStyrofoamFoodContainersExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionNoStyrofoamFoodContainersExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionRecyclingProgramExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Recycling program exception.
pub enum WasteReductionRecyclingProgramExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionRecyclingProgramExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionRecyclingProgramExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionRecyclingProgramExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionRecyclingProgramExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionRecyclingProgramExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionRecyclingProgramExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionRecyclingProgramExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionRecyclingProgramExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionRecyclingProgramExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionRecyclingProgramExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionRecyclingProgramExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionRefillableToiletryContainersExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Refillable toiletry containers exception.
pub enum WasteReductionRefillableToiletryContainersExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionRefillableToiletryContainersExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionRefillableToiletryContainersExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionRefillableToiletryContainersExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionRefillableToiletryContainersExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionRefillableToiletryContainersExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionRefillableToiletryContainersExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionRefillableToiletryContainersExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionRefillableToiletryContainersExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionRefillableToiletryContainersExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionRefillableToiletryContainersExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionRefillableToiletryContainersExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionSafelyDisposesBatteriesExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Safely disposes batteries exception.
pub enum WasteReductionSafelyDisposesBatteriesExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionSafelyDisposesBatteriesExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionSafelyDisposesBatteriesExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionSafelyDisposesBatteriesExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionSafelyDisposesBatteriesExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionSafelyDisposesBatteriesExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionSafelyDisposesBatteriesExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionSafelyDisposesBatteriesExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionSafelyDisposesBatteriesExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionSafelyDisposesBatteriesExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionSafelyDisposesBatteriesExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionSafelyDisposesBatteriesExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionSafelyDisposesElectronicsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Safely disposes electronics exception.
pub enum WasteReductionSafelyDisposesElectronicsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionSafelyDisposesElectronicsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionSafelyDisposesElectronicsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionSafelyDisposesElectronicsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionSafelyDisposesElectronicsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionSafelyDisposesElectronicsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionSafelyDisposesElectronicsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionSafelyDisposesElectronicsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionSafelyDisposesElectronicsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionSafelyDisposesElectronicsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionSafelyDisposesElectronicsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionSafelyDisposesElectronicsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionSafelyDisposesLightbulbsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Safely disposes lightbulbs exception.
pub enum WasteReductionSafelyDisposesLightbulbsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionSafelyDisposesLightbulbsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionSafelyDisposesLightbulbsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionSafelyDisposesLightbulbsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionSafelyDisposesLightbulbsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionSafelyDisposesLightbulbsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionSafelyDisposesLightbulbsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionSafelyDisposesLightbulbsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionSafelyDisposesLightbulbsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionSafelyDisposesLightbulbsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionSafelyDisposesLightbulbsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionSafelyDisposesLightbulbsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Safely handles hazardous substances exception.
pub enum WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionSoapDonationProgramExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Soap donation program exception.
pub enum WasteReductionSoapDonationProgramExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionSoapDonationProgramExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionSoapDonationProgramExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionSoapDonationProgramExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionSoapDonationProgramExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionSoapDonationProgramExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionSoapDonationProgramExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionSoapDonationProgramExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionSoapDonationProgramExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionSoapDonationProgramExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionSoapDonationProgramExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionSoapDonationProgramExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionToiletryDonationProgramExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Toiletry donation program exception.
pub enum WasteReductionToiletryDonationProgramExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionToiletryDonationProgramExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionToiletryDonationProgramExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionToiletryDonationProgramExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionToiletryDonationProgramExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionToiletryDonationProgramExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionToiletryDonationProgramExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionToiletryDonationProgramExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionToiletryDonationProgramExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionToiletryDonationProgramExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionToiletryDonationProgramExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionToiletryDonationProgramExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WasteReductionWaterBottleFillingStationsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Water bottle filling stations exception.
pub enum WasteReductionWaterBottleFillingStationsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WasteReductionWaterBottleFillingStationsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WasteReductionWaterBottleFillingStationsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WasteReductionWaterBottleFillingStationsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WasteReductionWaterBottleFillingStationsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WasteReductionWaterBottleFillingStationsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WasteReductionWaterBottleFillingStationsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WasteReductionWaterBottleFillingStationsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WasteReductionWaterBottleFillingStationsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WasteReductionWaterBottleFillingStationsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WasteReductionWaterBottleFillingStationsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WasteReductionWaterBottleFillingStationsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Independent organization audits water use exception.
pub enum WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WaterConservationLinenReuseProgramExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Linen reuse program exception.
pub enum WaterConservationLinenReuseProgramExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WaterConservationLinenReuseProgramExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WaterConservationLinenReuseProgramExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WaterConservationLinenReuseProgramExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WaterConservationLinenReuseProgramExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WaterConservationLinenReuseProgramExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WaterConservationLinenReuseProgramExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WaterConservationLinenReuseProgramExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WaterConservationLinenReuseProgramExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WaterConservationLinenReuseProgramExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WaterConservationLinenReuseProgramExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WaterConservationLinenReuseProgramExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WaterConservationTowelReuseProgramExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Towel reuse program exception.
pub enum WaterConservationTowelReuseProgramExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WaterConservationTowelReuseProgramExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WaterConservationTowelReuseProgramExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WaterConservationTowelReuseProgramExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WaterConservationTowelReuseProgramExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WaterConservationTowelReuseProgramExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WaterConservationTowelReuseProgramExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WaterConservationTowelReuseProgramExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WaterConservationTowelReuseProgramExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WaterConservationTowelReuseProgramExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WaterConservationTowelReuseProgramExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WaterConservationTowelReuseProgramExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WaterConservationWaterSavingShowersExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Water saving showers exception.
pub enum WaterConservationWaterSavingShowersExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WaterConservationWaterSavingShowersExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WaterConservationWaterSavingShowersExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WaterConservationWaterSavingShowersExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WaterConservationWaterSavingShowersExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WaterConservationWaterSavingShowersExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WaterConservationWaterSavingShowersExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WaterConservationWaterSavingShowersExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WaterConservationWaterSavingShowersExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WaterConservationWaterSavingShowersExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WaterConservationWaterSavingShowersExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WaterConservationWaterSavingShowersExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WaterConservationWaterSavingSinksExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Water saving sinks exception.
pub enum WaterConservationWaterSavingSinksExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WaterConservationWaterSavingSinksExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WaterConservationWaterSavingSinksExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WaterConservationWaterSavingSinksExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WaterConservationWaterSavingSinksExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WaterConservationWaterSavingSinksExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WaterConservationWaterSavingSinksExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WaterConservationWaterSavingSinksExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WaterConservationWaterSavingSinksExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WaterConservationWaterSavingSinksExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WaterConservationWaterSavingSinksExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WaterConservationWaterSavingSinksExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WaterConservationWaterSavingToiletsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Water saving toilets exception.
pub enum WaterConservationWaterSavingToiletsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WaterConservationWaterSavingToiletsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WaterConservationWaterSavingToiletsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WaterConservationWaterSavingToiletsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WaterConservationWaterSavingToiletsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WaterConservationWaterSavingToiletsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WaterConservationWaterSavingToiletsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WaterConservationWaterSavingToiletsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WaterConservationWaterSavingToiletsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WaterConservationWaterSavingToiletsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WaterConservationWaterSavingToiletsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WaterConservationWaterSavingToiletsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesDoctorOnCallExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Doctor on call exception.
pub enum WellnesDoctorOnCallExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesDoctorOnCallExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesDoctorOnCallExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesDoctorOnCallExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesDoctorOnCallExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesDoctorOnCallExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesDoctorOnCallExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesDoctorOnCallExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesDoctorOnCallExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesDoctorOnCallExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesDoctorOnCallExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesDoctorOnCallExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesEllipticalMachineExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Elliptical machine exception.
pub enum WellnesEllipticalMachineExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesEllipticalMachineExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesEllipticalMachineExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesEllipticalMachineExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesEllipticalMachineExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesEllipticalMachineExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesEllipticalMachineExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesEllipticalMachineExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesEllipticalMachineExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesEllipticalMachineExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesEllipticalMachineExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesEllipticalMachineExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesFitnessCenterExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Fitness center exception.
pub enum WellnesFitnessCenterExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesFitnessCenterExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesFitnessCenterExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesFitnessCenterExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesFitnessCenterExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesFitnessCenterExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesFitnessCenterExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesFitnessCenterExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesFitnessCenterExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesFitnessCenterExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesFitnessCenterExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesFitnessCenterExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesFreeFitnessCenterExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free fitness center exception.
pub enum WellnesFreeFitnessCenterExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesFreeFitnessCenterExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesFreeFitnessCenterExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesFreeFitnessCenterExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesFreeFitnessCenterExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesFreeFitnessCenterExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesFreeFitnessCenterExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesFreeFitnessCenterExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesFreeFitnessCenterExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesFreeFitnessCenterExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesFreeFitnessCenterExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesFreeFitnessCenterExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesFreeWeightsExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Free weights exception.
pub enum WellnesFreeWeightsExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesFreeWeightsExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesFreeWeightsExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesFreeWeightsExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesFreeWeightsExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesFreeWeightsExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesFreeWeightsExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesFreeWeightsExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesFreeWeightsExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesFreeWeightsExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesFreeWeightsExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesFreeWeightsExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesMassageExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Massage exception.
pub enum WellnesMassageExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesMassageExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesMassageExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesMassageExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesMassageExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesMassageExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesMassageExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesMassageExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesMassageExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesMassageExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesMassageExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesMassageExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesSalonExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Salon exception.
pub enum WellnesSalonExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesSalonExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesSalonExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesSalonExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesSalonExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesSalonExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesSalonExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesSalonExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesSalonExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesSalonExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesSalonExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesSalonExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesSaunaExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sauna exception.
pub enum WellnesSaunaExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesSaunaExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesSaunaExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesSaunaExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesSaunaExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesSaunaExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesSaunaExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesSaunaExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesSaunaExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesSaunaExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesSaunaExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesSaunaExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesSpaExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Spa exception.
pub enum WellnesSpaExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesSpaExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesSpaExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesSpaExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesSpaExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesSpaExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesSpaExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesSpaExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesSpaExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesSpaExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesSpaExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesSpaExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesTreadmillExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Treadmill exception.
pub enum WellnesTreadmillExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesTreadmillExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesTreadmillExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesTreadmillExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesTreadmillExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesTreadmillExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesTreadmillExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesTreadmillExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesTreadmillExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesTreadmillExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesTreadmillExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesTreadmillExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WellnesWeightMachineExceptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Weight machine exception.
pub enum WellnesWeightMachineExceptionEnum {
    

    /// Default unspecified exception. Use this only if a more specific exception does not match.
    ///
    /// "EXCEPTION_UNSPECIFIED"
    #[serde(rename="EXCEPTION_UNSPECIFIED")]
    EXCEPTIONUNSPECIFIED,
    

    /// Amenity or service is unavailable due to ongoing work orders.
    ///
    /// "UNDER_CONSTRUCTION"
    #[serde(rename="UNDER_CONSTRUCTION")]
    UNDERCONSTRUCTION,
    

    /// Amenity or service availability is seasonal.
    ///
    /// "DEPENDENT_ON_SEASON"
    #[serde(rename="DEPENDENT_ON_SEASON")]
    DEPENDENTONSEASON,
    

    /// Amenity or service availability depends on the day of the week.
    ///
    /// "DEPENDENT_ON_DAY_OF_WEEK"
    #[serde(rename="DEPENDENT_ON_DAY_OF_WEEK")]
    DEPENDENTONDAYOFWEEK,
}

impl AsRef<str> for WellnesWeightMachineExceptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WellnesWeightMachineExceptionEnum::EXCEPTIONUNSPECIFIED => "EXCEPTION_UNSPECIFIED",
            WellnesWeightMachineExceptionEnum::UNDERCONSTRUCTION => "UNDER_CONSTRUCTION",
            WellnesWeightMachineExceptionEnum::DEPENDENTONSEASON => "DEPENDENT_ON_SEASON",
            WellnesWeightMachineExceptionEnum::DEPENDENTONDAYOFWEEK => "DEPENDENT_ON_DAY_OF_WEEK",
        }
    }
}

impl std::convert::TryFrom< &str> for WellnesWeightMachineExceptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCEPTION_UNSPECIFIED" => Ok(WellnesWeightMachineExceptionEnum::EXCEPTIONUNSPECIFIED),
           "UNDER_CONSTRUCTION" => Ok(WellnesWeightMachineExceptionEnum::UNDERCONSTRUCTION),
           "DEPENDENT_ON_SEASON" => Ok(WellnesWeightMachineExceptionEnum::DEPENDENTONSEASON),
           "DEPENDENT_ON_DAY_OF_WEEK" => Ok(WellnesWeightMachineExceptionEnum::DEPENDENTONDAYOFWEEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WellnesWeightMachineExceptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


