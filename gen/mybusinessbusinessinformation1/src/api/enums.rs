use super::*;



// region AttributeValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of value that this attribute contains. This should be used to determine how to interpret the value.
pub enum AttributeValueTypeEnum {
    

    /// Not specified.
    ///
    /// "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED"
    #[serde(rename="ATTRIBUTE_VALUE_TYPE_UNSPECIFIED")]
    ATTRIBUTEVALUETYPEUNSPECIFIED,
    

    /// The values for this attribute are boolean values.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// The attribute has a predetermined list of available values that can be used. Metadata for this attribute will list these values.
    ///
    /// "ENUM"
    #[serde(rename="ENUM")]
    ENUM,
    

    /// The values for this attribute are URLs.
    ///
    /// "URL"
    #[serde(rename="URL")]
    URL,
    

    /// The attribute value is an enum with multiple possible values that can be explicitly set or unset.
    ///
    /// "REPEATED_ENUM"
    #[serde(rename="REPEATED_ENUM")]
    REPEATEDENUM,
}

impl AsRef<str> for AttributeValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AttributeValueTypeEnum::ATTRIBUTEVALUETYPEUNSPECIFIED => "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED",
            AttributeValueTypeEnum::BOOL => "BOOL",
            AttributeValueTypeEnum::ENUM => "ENUM",
            AttributeValueTypeEnum::URL => "URL",
            AttributeValueTypeEnum::REPEATEDENUM => "REPEATED_ENUM",
        }
    }
}

impl std::convert::TryFrom< &str> for AttributeValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED" => Ok(AttributeValueTypeEnum::ATTRIBUTEVALUETYPEUNSPECIFIED),
           "BOOL" => Ok(AttributeValueTypeEnum::BOOL),
           "ENUM" => Ok(AttributeValueTypeEnum::ENUM),
           "URL" => Ok(AttributeValueTypeEnum::URL),
           "REPEATED_ENUM" => Ok(AttributeValueTypeEnum::REPEATEDENUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AttributeValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AttributeMetadataValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The value type for the attribute. Values set and retrieved should be expected to be of this type.
pub enum AttributeMetadataValueTypeEnum {
    

    /// Not specified.
    ///
    /// "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED"
    #[serde(rename="ATTRIBUTE_VALUE_TYPE_UNSPECIFIED")]
    ATTRIBUTEVALUETYPEUNSPECIFIED,
    

    /// The values for this attribute are boolean values.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// The attribute has a predetermined list of available values that can be used. Metadata for this attribute will list these values.
    ///
    /// "ENUM"
    #[serde(rename="ENUM")]
    ENUM,
    

    /// The values for this attribute are URLs.
    ///
    /// "URL"
    #[serde(rename="URL")]
    URL,
    

    /// The attribute value is an enum with multiple possible values that can be explicitly set or unset.
    ///
    /// "REPEATED_ENUM"
    #[serde(rename="REPEATED_ENUM")]
    REPEATEDENUM,
}

impl AsRef<str> for AttributeMetadataValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AttributeMetadataValueTypeEnum::ATTRIBUTEVALUETYPEUNSPECIFIED => "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED",
            AttributeMetadataValueTypeEnum::BOOL => "BOOL",
            AttributeMetadataValueTypeEnum::ENUM => "ENUM",
            AttributeMetadataValueTypeEnum::URL => "URL",
            AttributeMetadataValueTypeEnum::REPEATEDENUM => "REPEATED_ENUM",
        }
    }
}

impl std::convert::TryFrom< &str> for AttributeMetadataValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTRIBUTE_VALUE_TYPE_UNSPECIFIED" => Ok(AttributeMetadataValueTypeEnum::ATTRIBUTEVALUETYPEUNSPECIFIED),
           "BOOL" => Ok(AttributeMetadataValueTypeEnum::BOOL),
           "ENUM" => Ok(AttributeMetadataValueTypeEnum::ENUM),
           "URL" => Ok(AttributeMetadataValueTypeEnum::URL),
           "REPEATED_ENUM" => Ok(AttributeMetadataValueTypeEnum::REPEATEDENUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AttributeMetadataValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OpenInfoStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates whether or not the Location is currently open for business. All locations are open by default, unless updated to be closed.
pub enum OpenInfoStatusEnum {
    

    /// Not specified.
    ///
    /// "OPEN_FOR_BUSINESS_UNSPECIFIED"
    #[serde(rename="OPEN_FOR_BUSINESS_UNSPECIFIED")]
    OPENFORBUSINESSUNSPECIFIED,
    

    /// Indicates that the location is open.
    ///
    /// "OPEN"
    #[serde(rename="OPEN")]
    OPEN,
    

    /// Indicates that the location has been permanently closed.
    ///
    /// "CLOSED_PERMANENTLY"
    #[serde(rename="CLOSED_PERMANENTLY")]
    CLOSEDPERMANENTLY,
    

    /// Indicates that the location has been temporarily closed.
    ///
    /// "CLOSED_TEMPORARILY"
    #[serde(rename="CLOSED_TEMPORARILY")]
    CLOSEDTEMPORARILY,
}

impl AsRef<str> for OpenInfoStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OpenInfoStatusEnum::OPENFORBUSINESSUNSPECIFIED => "OPEN_FOR_BUSINESS_UNSPECIFIED",
            OpenInfoStatusEnum::OPEN => "OPEN",
            OpenInfoStatusEnum::CLOSEDPERMANENTLY => "CLOSED_PERMANENTLY",
            OpenInfoStatusEnum::CLOSEDTEMPORARILY => "CLOSED_TEMPORARILY",
        }
    }
}

impl std::convert::TryFrom< &str> for OpenInfoStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPEN_FOR_BUSINESS_UNSPECIFIED" => Ok(OpenInfoStatusEnum::OPENFORBUSINESSUNSPECIFIED),
           "OPEN" => Ok(OpenInfoStatusEnum::OPEN),
           "CLOSED_PERMANENTLY" => Ok(OpenInfoStatusEnum::CLOSEDPERMANENTLY),
           "CLOSED_TEMPORARILY" => Ok(OpenInfoStatusEnum::CLOSEDTEMPORARILY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OpenInfoStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RelevantLocationRelationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the relationship.
pub enum RelevantLocationRelationTypeEnum {
    

    /// Type unspecified.
    ///
    /// "RELATION_TYPE_UNSPECIFIED"
    #[serde(rename="RELATION_TYPE_UNSPECIFIED")]
    RELATIONTYPEUNSPECIFIED,
    

    /// This represents a relation between 2 locations which share one physical area, same brand/upper management/organization, but with different key attributes like store hours or phone numbers. For example, Costco Pharmacy is a department in Costco Wholesale.
    ///
    /// "DEPARTMENT_OF"
    #[serde(rename="DEPARTMENT_OF")]
    DEPARTMENTOF,
    

    /// This represents the cases where 2 locations are co-located in the same physical location, but from different companies (e.g. Starbucks in a Safeway, shops in a mall).
    ///
    /// "INDEPENDENT_ESTABLISHMENT_IN"
    #[serde(rename="INDEPENDENT_ESTABLISHMENT_IN")]
    INDEPENDENTESTABLISHMENTIN,
}

impl AsRef<str> for RelevantLocationRelationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RelevantLocationRelationTypeEnum::RELATIONTYPEUNSPECIFIED => "RELATION_TYPE_UNSPECIFIED",
            RelevantLocationRelationTypeEnum::DEPARTMENTOF => "DEPARTMENT_OF",
            RelevantLocationRelationTypeEnum::INDEPENDENTESTABLISHMENTIN => "INDEPENDENT_ESTABLISHMENT_IN",
        }
    }
}

impl std::convert::TryFrom< &str> for RelevantLocationRelationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELATION_TYPE_UNSPECIFIED" => Ok(RelevantLocationRelationTypeEnum::RELATIONTYPEUNSPECIFIED),
           "DEPARTMENT_OF" => Ok(RelevantLocationRelationTypeEnum::DEPARTMENTOF),
           "INDEPENDENT_ESTABLISHMENT_IN" => Ok(RelevantLocationRelationTypeEnum::INDEPENDENTESTABLISHMENTIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RelevantLocationRelationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceAreaBusinesBusinessTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates the type of the service area business.
pub enum ServiceAreaBusinesBusinessTypeEnum {
    

    /// Output only. Not specified.
    ///
    /// "BUSINESS_TYPE_UNSPECIFIED"
    #[serde(rename="BUSINESS_TYPE_UNSPECIFIED")]
    BUSINESSTYPEUNSPECIFIED,
    

    /// Offers service only in the surrounding area (not at the business address). If a business is being updated from a CUSTOMER_AND_BUSINESS_LOCATION to a CUSTOMER_LOCATION_ONLY, the location update must include field mask `storefront_address` and set the field to empty.
    ///
    /// "CUSTOMER_LOCATION_ONLY"
    #[serde(rename="CUSTOMER_LOCATION_ONLY")]
    CUSTOMERLOCATIONONLY,
    

    /// Offers service at the business address and the surrounding area.
    ///
    /// "CUSTOMER_AND_BUSINESS_LOCATION"
    #[serde(rename="CUSTOMER_AND_BUSINESS_LOCATION")]
    CUSTOMERANDBUSINESSLOCATION,
}

impl AsRef<str> for ServiceAreaBusinesBusinessTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceAreaBusinesBusinessTypeEnum::BUSINESSTYPEUNSPECIFIED => "BUSINESS_TYPE_UNSPECIFIED",
            ServiceAreaBusinesBusinessTypeEnum::CUSTOMERLOCATIONONLY => "CUSTOMER_LOCATION_ONLY",
            ServiceAreaBusinesBusinessTypeEnum::CUSTOMERANDBUSINESSLOCATION => "CUSTOMER_AND_BUSINESS_LOCATION",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceAreaBusinesBusinessTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUSINESS_TYPE_UNSPECIFIED" => Ok(ServiceAreaBusinesBusinessTypeEnum::BUSINESSTYPEUNSPECIFIED),
           "CUSTOMER_LOCATION_ONLY" => Ok(ServiceAreaBusinesBusinessTypeEnum::CUSTOMERLOCATIONONLY),
           "CUSTOMER_AND_BUSINESS_LOCATION" => Ok(ServiceAreaBusinesBusinessTypeEnum::CUSTOMERANDBUSINESSLOCATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceAreaBusinesBusinessTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TimePeriodCloseDayEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates the day of the week this period ends on.
pub enum TimePeriodCloseDayEnum {
    

    /// The day of the week is unspecified.
    ///
    /// "DAY_OF_WEEK_UNSPECIFIED"
    #[serde(rename="DAY_OF_WEEK_UNSPECIFIED")]
    DAYOFWEEKUNSPECIFIED,
    

    /// Monday
    ///
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    

    /// Tuesday
    ///
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    

    /// Wednesday
    ///
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    

    /// Thursday
    ///
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    

    /// Friday
    ///
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    

    /// Saturday
    ///
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    

    /// Sunday
    ///
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for TimePeriodCloseDayEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimePeriodCloseDayEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            TimePeriodCloseDayEnum::MONDAY => "MONDAY",
            TimePeriodCloseDayEnum::TUESDAY => "TUESDAY",
            TimePeriodCloseDayEnum::WEDNESDAY => "WEDNESDAY",
            TimePeriodCloseDayEnum::THURSDAY => "THURSDAY",
            TimePeriodCloseDayEnum::FRIDAY => "FRIDAY",
            TimePeriodCloseDayEnum::SATURDAY => "SATURDAY",
            TimePeriodCloseDayEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for TimePeriodCloseDayEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(TimePeriodCloseDayEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(TimePeriodCloseDayEnum::MONDAY),
           "TUESDAY" => Ok(TimePeriodCloseDayEnum::TUESDAY),
           "WEDNESDAY" => Ok(TimePeriodCloseDayEnum::WEDNESDAY),
           "THURSDAY" => Ok(TimePeriodCloseDayEnum::THURSDAY),
           "FRIDAY" => Ok(TimePeriodCloseDayEnum::FRIDAY),
           "SATURDAY" => Ok(TimePeriodCloseDayEnum::SATURDAY),
           "SUNDAY" => Ok(TimePeriodCloseDayEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimePeriodCloseDayEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TimePeriodOpenDayEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates the day of the week this period starts on.
pub enum TimePeriodOpenDayEnum {
    

    /// The day of the week is unspecified.
    ///
    /// "DAY_OF_WEEK_UNSPECIFIED"
    #[serde(rename="DAY_OF_WEEK_UNSPECIFIED")]
    DAYOFWEEKUNSPECIFIED,
    

    /// Monday
    ///
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    

    /// Tuesday
    ///
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    

    /// Wednesday
    ///
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    

    /// Thursday
    ///
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    

    /// Friday
    ///
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    

    /// Saturday
    ///
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    

    /// Sunday
    ///
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for TimePeriodOpenDayEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimePeriodOpenDayEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            TimePeriodOpenDayEnum::MONDAY => "MONDAY",
            TimePeriodOpenDayEnum::TUESDAY => "TUESDAY",
            TimePeriodOpenDayEnum::WEDNESDAY => "WEDNESDAY",
            TimePeriodOpenDayEnum::THURSDAY => "THURSDAY",
            TimePeriodOpenDayEnum::FRIDAY => "FRIDAY",
            TimePeriodOpenDayEnum::SATURDAY => "SATURDAY",
            TimePeriodOpenDayEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for TimePeriodOpenDayEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(TimePeriodOpenDayEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(TimePeriodOpenDayEnum::MONDAY),
           "TUESDAY" => Ok(TimePeriodOpenDayEnum::TUESDAY),
           "WEDNESDAY" => Ok(TimePeriodOpenDayEnum::WEDNESDAY),
           "THURSDAY" => Ok(TimePeriodOpenDayEnum::THURSDAY),
           "FRIDAY" => Ok(TimePeriodOpenDayEnum::FRIDAY),
           "SATURDAY" => Ok(TimePeriodOpenDayEnum::SATURDAY),
           "SUNDAY" => Ok(TimePeriodOpenDayEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimePeriodOpenDayEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CategoryViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Specifies which parts to the Category resource should be returned in the response.
pub enum CategoryViewEnum {
    

    /// Not specified, equivalent to CATEGORY_METADATA_ONLY.
    ///
    /// "CATEGORY_VIEW_UNSPECIFIED"
    #[serde(rename="CATEGORY_VIEW_UNSPECIFIED")]
    CATEGORYVIEWUNSPECIFIED,
    

    /// The server response will only include Category fields display_name, category_id and language_code. It omits any service type metadata related fields.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Returns all the fields in the response.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for CategoryViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CategoryViewEnum::CATEGORYVIEWUNSPECIFIED => "CATEGORY_VIEW_UNSPECIFIED",
            CategoryViewEnum::BASIC => "BASIC",
            CategoryViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for CategoryViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CATEGORY_VIEW_UNSPECIFIED" => Ok(CategoryViewEnum::CATEGORYVIEWUNSPECIFIED),
           "BASIC" => Ok(CategoryViewEnum::BASIC),
           "FULL" => Ok(CategoryViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CategoryViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


