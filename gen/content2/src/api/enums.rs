use super::*;



// region OrderreturnOrderByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Return the results in the specified order.
pub enum OrderreturnOrderByEnum {
    
    /// "RETURN_CREATION_TIME_DESC"
    #[serde(rename="RETURN_CREATION_TIME_DESC")]
    RETURNCREATIONTIMEDESC,
    
    /// "RETURN_CREATION_TIME_ASC"
    #[serde(rename="RETURN_CREATION_TIME_ASC")]
    RETURNCREATIONTIMEASC,
}

impl AsRef<str> for OrderreturnOrderByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderreturnOrderByEnum::RETURNCREATIONTIMEDESC => "RETURN_CREATION_TIME_DESC",
            OrderreturnOrderByEnum::RETURNCREATIONTIMEASC => "RETURN_CREATION_TIME_ASC",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderreturnOrderByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RETURN_CREATION_TIME_DESC" => Ok(OrderreturnOrderByEnum::RETURNCREATIONTIMEDESC),
           "RETURN_CREATION_TIME_ASC" => Ok(OrderreturnOrderByEnum::RETURNCREATIONTIMEASC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderreturnOrderByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderTemplateNameEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The name of the template to retrieve.
pub enum OrderTemplateNameEnum {
    
    /// "TEMPLATE1"
    #[serde(rename="TEMPLATE1")]
    TEMPLATE1,
    
    /// "TEMPLATE2"
    #[serde(rename="TEMPLATE2")]
    TEMPLATE2,
    
    /// "TEMPLATE1A"
    #[serde(rename="TEMPLATE1A")]
    TEMPLATE1A,
    
    /// "TEMPLATE1B"
    #[serde(rename="TEMPLATE1B")]
    TEMPLATE1B,
    
    /// "TEMPLATE3"
    #[serde(rename="TEMPLATE3")]
    TEMPLATE3,
}

impl AsRef<str> for OrderTemplateNameEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderTemplateNameEnum::TEMPLATE1 => "TEMPLATE1",
            OrderTemplateNameEnum::TEMPLATE2 => "TEMPLATE2",
            OrderTemplateNameEnum::TEMPLATE1A => "TEMPLATE1A",
            OrderTemplateNameEnum::TEMPLATE1B => "TEMPLATE1B",
            OrderTemplateNameEnum::TEMPLATE3 => "TEMPLATE3",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderTemplateNameEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEMPLATE1" => Ok(OrderTemplateNameEnum::TEMPLATE1),
           "TEMPLATE2" => Ok(OrderTemplateNameEnum::TEMPLATE2),
           "TEMPLATE1A" => Ok(OrderTemplateNameEnum::TEMPLATE1A),
           "TEMPLATE1B" => Ok(OrderTemplateNameEnum::TEMPLATE1B),
           "TEMPLATE3" => Ok(OrderTemplateNameEnum::TEMPLATE3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderTemplateNameEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderStatusesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Obtains orders that match any of the specified statuses. Please note that `active` is a shortcut for `pendingShipment` and `partiallyShipped`, and `completed` is a shortcut for `shipped`, `partiallyDelivered`, `delivered`, `partiallyReturned`, `returned`, and `canceled`.
pub enum OrderStatusesEnum {
    
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    
    /// "CANCELED"
    #[serde(rename="CANCELED")]
    CANCELED,
    
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    
    /// "PENDING_SHIPMENT"
    #[serde(rename="PENDING_SHIPMENT")]
    PENDINGSHIPMENT,
    
    /// "PARTIALLY_SHIPPED"
    #[serde(rename="PARTIALLY_SHIPPED")]
    PARTIALLYSHIPPED,
    
    /// "SHIPPED"
    #[serde(rename="SHIPPED")]
    SHIPPED,
    
    /// "PARTIALLY_DELIVERED"
    #[serde(rename="PARTIALLY_DELIVERED")]
    PARTIALLYDELIVERED,
    
    /// "DELIVERED"
    #[serde(rename="DELIVERED")]
    DELIVERED,
    
    /// "PARTIALLY_RETURNED"
    #[serde(rename="PARTIALLY_RETURNED")]
    PARTIALLYRETURNED,
    
    /// "RETURNED"
    #[serde(rename="RETURNED")]
    RETURNED,
}

impl AsRef<str> for OrderStatusesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderStatusesEnum::ACTIVE => "ACTIVE",
            OrderStatusesEnum::COMPLETED => "COMPLETED",
            OrderStatusesEnum::CANCELED => "CANCELED",
            OrderStatusesEnum::INPROGRESS => "IN_PROGRESS",
            OrderStatusesEnum::PENDINGSHIPMENT => "PENDING_SHIPMENT",
            OrderStatusesEnum::PARTIALLYSHIPPED => "PARTIALLY_SHIPPED",
            OrderStatusesEnum::SHIPPED => "SHIPPED",
            OrderStatusesEnum::PARTIALLYDELIVERED => "PARTIALLY_DELIVERED",
            OrderStatusesEnum::DELIVERED => "DELIVERED",
            OrderStatusesEnum::PARTIALLYRETURNED => "PARTIALLY_RETURNED",
            OrderStatusesEnum::RETURNED => "RETURNED",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderStatusesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTIVE" => Ok(OrderStatusesEnum::ACTIVE),
           "COMPLETED" => Ok(OrderStatusesEnum::COMPLETED),
           "CANCELED" => Ok(OrderStatusesEnum::CANCELED),
           "IN_PROGRESS" => Ok(OrderStatusesEnum::INPROGRESS),
           "PENDING_SHIPMENT" => Ok(OrderStatusesEnum::PENDINGSHIPMENT),
           "PARTIALLY_SHIPPED" => Ok(OrderStatusesEnum::PARTIALLYSHIPPED),
           "SHIPPED" => Ok(OrderStatusesEnum::SHIPPED),
           "PARTIALLY_DELIVERED" => Ok(OrderStatusesEnum::PARTIALLYDELIVERED),
           "DELIVERED" => Ok(OrderStatusesEnum::DELIVERED),
           "PARTIALLY_RETURNED" => Ok(OrderStatusesEnum::PARTIALLYRETURNED),
           "RETURNED" => Ok(OrderStatusesEnum::RETURNED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderStatusesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


