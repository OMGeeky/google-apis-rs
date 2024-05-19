use super::*;



// region PlaceActionLinkPlaceActionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of place action that can be performed using this link.
pub enum PlaceActionLinkPlaceActionTypeEnum {
    

    /// Not specified.
    ///
    /// "PLACE_ACTION_TYPE_UNSPECIFIED"
    #[serde(rename="PLACE_ACTION_TYPE_UNSPECIFIED")]
    PLACEACTIONTYPEUNSPECIFIED,
    

    /// The action type is booking an appointment.
    ///
    /// "APPOINTMENT"
    #[serde(rename="APPOINTMENT")]
    APPOINTMENT,
    

    /// The action type is booking an online appointment.
    ///
    /// "ONLINE_APPOINTMENT"
    #[serde(rename="ONLINE_APPOINTMENT")]
    ONLINEAPPOINTMENT,
    

    /// The action type is making a dining reservation.
    ///
    /// "DINING_RESERVATION"
    #[serde(rename="DINING_RESERVATION")]
    DININGRESERVATION,
    

    /// The action type is ordering food for delivery and/or takeout.
    ///
    /// "FOOD_ORDERING"
    #[serde(rename="FOOD_ORDERING")]
    FOODORDERING,
    

    /// The action type is ordering food for delivery.
    ///
    /// "FOOD_DELIVERY"
    #[serde(rename="FOOD_DELIVERY")]
    FOODDELIVERY,
    

    /// The action type is ordering food for takeout.
    ///
    /// "FOOD_TAKEOUT"
    #[serde(rename="FOOD_TAKEOUT")]
    FOODTAKEOUT,
    

    /// The action type is shopping, that can be delivery and/or pickup.
    ///
    /// "SHOP_ONLINE"
    #[serde(rename="SHOP_ONLINE")]
    SHOPONLINE,
}

impl AsRef<str> for PlaceActionLinkPlaceActionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlaceActionLinkPlaceActionTypeEnum::PLACEACTIONTYPEUNSPECIFIED => "PLACE_ACTION_TYPE_UNSPECIFIED",
            PlaceActionLinkPlaceActionTypeEnum::APPOINTMENT => "APPOINTMENT",
            PlaceActionLinkPlaceActionTypeEnum::ONLINEAPPOINTMENT => "ONLINE_APPOINTMENT",
            PlaceActionLinkPlaceActionTypeEnum::DININGRESERVATION => "DINING_RESERVATION",
            PlaceActionLinkPlaceActionTypeEnum::FOODORDERING => "FOOD_ORDERING",
            PlaceActionLinkPlaceActionTypeEnum::FOODDELIVERY => "FOOD_DELIVERY",
            PlaceActionLinkPlaceActionTypeEnum::FOODTAKEOUT => "FOOD_TAKEOUT",
            PlaceActionLinkPlaceActionTypeEnum::SHOPONLINE => "SHOP_ONLINE",
        }
    }
}

impl std::convert::TryFrom< &str> for PlaceActionLinkPlaceActionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLACE_ACTION_TYPE_UNSPECIFIED" => Ok(PlaceActionLinkPlaceActionTypeEnum::PLACEACTIONTYPEUNSPECIFIED),
           "APPOINTMENT" => Ok(PlaceActionLinkPlaceActionTypeEnum::APPOINTMENT),
           "ONLINE_APPOINTMENT" => Ok(PlaceActionLinkPlaceActionTypeEnum::ONLINEAPPOINTMENT),
           "DINING_RESERVATION" => Ok(PlaceActionLinkPlaceActionTypeEnum::DININGRESERVATION),
           "FOOD_ORDERING" => Ok(PlaceActionLinkPlaceActionTypeEnum::FOODORDERING),
           "FOOD_DELIVERY" => Ok(PlaceActionLinkPlaceActionTypeEnum::FOODDELIVERY),
           "FOOD_TAKEOUT" => Ok(PlaceActionLinkPlaceActionTypeEnum::FOODTAKEOUT),
           "SHOP_ONLINE" => Ok(PlaceActionLinkPlaceActionTypeEnum::SHOPONLINE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlaceActionLinkPlaceActionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlaceActionLinkProviderTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the provider type.
pub enum PlaceActionLinkProviderTypeEnum {
    

    /// Not specified.
    ///
    /// "PROVIDER_TYPE_UNSPECIFIED"
    #[serde(rename="PROVIDER_TYPE_UNSPECIFIED")]
    PROVIDERTYPEUNSPECIFIED,
    

    /// A 1P provider such as a merchant, or an agency on behalf of a merchant.
    ///
    /// "MERCHANT"
    #[serde(rename="MERCHANT")]
    MERCHANT,
    

    /// A 3P aggregator, such as a `Reserve with Google` partner.
    ///
    /// "AGGREGATOR_3P"
    #[serde(rename="AGGREGATOR_3P")]
    AGGREGATOR3P,
}

impl AsRef<str> for PlaceActionLinkProviderTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlaceActionLinkProviderTypeEnum::PROVIDERTYPEUNSPECIFIED => "PROVIDER_TYPE_UNSPECIFIED",
            PlaceActionLinkProviderTypeEnum::MERCHANT => "MERCHANT",
            PlaceActionLinkProviderTypeEnum::AGGREGATOR3P => "AGGREGATOR_3P",
        }
    }
}

impl std::convert::TryFrom< &str> for PlaceActionLinkProviderTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROVIDER_TYPE_UNSPECIFIED" => Ok(PlaceActionLinkProviderTypeEnum::PROVIDERTYPEUNSPECIFIED),
           "MERCHANT" => Ok(PlaceActionLinkProviderTypeEnum::MERCHANT),
           "AGGREGATOR_3P" => Ok(PlaceActionLinkProviderTypeEnum::AGGREGATOR3P),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlaceActionLinkProviderTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlaceActionTypeMetadataPlaceActionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The place action type.
pub enum PlaceActionTypeMetadataPlaceActionTypeEnum {
    

    /// Not specified.
    ///
    /// "PLACE_ACTION_TYPE_UNSPECIFIED"
    #[serde(rename="PLACE_ACTION_TYPE_UNSPECIFIED")]
    PLACEACTIONTYPEUNSPECIFIED,
    

    /// The action type is booking an appointment.
    ///
    /// "APPOINTMENT"
    #[serde(rename="APPOINTMENT")]
    APPOINTMENT,
    

    /// The action type is booking an online appointment.
    ///
    /// "ONLINE_APPOINTMENT"
    #[serde(rename="ONLINE_APPOINTMENT")]
    ONLINEAPPOINTMENT,
    

    /// The action type is making a dining reservation.
    ///
    /// "DINING_RESERVATION"
    #[serde(rename="DINING_RESERVATION")]
    DININGRESERVATION,
    

    /// The action type is ordering food for delivery and/or takeout.
    ///
    /// "FOOD_ORDERING"
    #[serde(rename="FOOD_ORDERING")]
    FOODORDERING,
    

    /// The action type is ordering food for delivery.
    ///
    /// "FOOD_DELIVERY"
    #[serde(rename="FOOD_DELIVERY")]
    FOODDELIVERY,
    

    /// The action type is ordering food for takeout.
    ///
    /// "FOOD_TAKEOUT"
    #[serde(rename="FOOD_TAKEOUT")]
    FOODTAKEOUT,
    

    /// The action type is shopping, that can be delivery and/or pickup.
    ///
    /// "SHOP_ONLINE"
    #[serde(rename="SHOP_ONLINE")]
    SHOPONLINE,
}

impl AsRef<str> for PlaceActionTypeMetadataPlaceActionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlaceActionTypeMetadataPlaceActionTypeEnum::PLACEACTIONTYPEUNSPECIFIED => "PLACE_ACTION_TYPE_UNSPECIFIED",
            PlaceActionTypeMetadataPlaceActionTypeEnum::APPOINTMENT => "APPOINTMENT",
            PlaceActionTypeMetadataPlaceActionTypeEnum::ONLINEAPPOINTMENT => "ONLINE_APPOINTMENT",
            PlaceActionTypeMetadataPlaceActionTypeEnum::DININGRESERVATION => "DINING_RESERVATION",
            PlaceActionTypeMetadataPlaceActionTypeEnum::FOODORDERING => "FOOD_ORDERING",
            PlaceActionTypeMetadataPlaceActionTypeEnum::FOODDELIVERY => "FOOD_DELIVERY",
            PlaceActionTypeMetadataPlaceActionTypeEnum::FOODTAKEOUT => "FOOD_TAKEOUT",
            PlaceActionTypeMetadataPlaceActionTypeEnum::SHOPONLINE => "SHOP_ONLINE",
        }
    }
}

impl std::convert::TryFrom< &str> for PlaceActionTypeMetadataPlaceActionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLACE_ACTION_TYPE_UNSPECIFIED" => Ok(PlaceActionTypeMetadataPlaceActionTypeEnum::PLACEACTIONTYPEUNSPECIFIED),
           "APPOINTMENT" => Ok(PlaceActionTypeMetadataPlaceActionTypeEnum::APPOINTMENT),
           "ONLINE_APPOINTMENT" => Ok(PlaceActionTypeMetadataPlaceActionTypeEnum::ONLINEAPPOINTMENT),
           "DINING_RESERVATION" => Ok(PlaceActionTypeMetadataPlaceActionTypeEnum::DININGRESERVATION),
           "FOOD_ORDERING" => Ok(PlaceActionTypeMetadataPlaceActionTypeEnum::FOODORDERING),
           "FOOD_DELIVERY" => Ok(PlaceActionTypeMetadataPlaceActionTypeEnum::FOODDELIVERY),
           "FOOD_TAKEOUT" => Ok(PlaceActionTypeMetadataPlaceActionTypeEnum::FOODTAKEOUT),
           "SHOP_ONLINE" => Ok(PlaceActionTypeMetadataPlaceActionTypeEnum::SHOPONLINE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlaceActionTypeMetadataPlaceActionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


