use super::*;



// region AggregateBucketTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of a bucket signifies how the data aggregation is performed in the bucket.
pub enum AggregateBucketTypeEnum {
    
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    

    /// Denotes that bucketing by time is requested. When this is specified, the timeBucketDurationMillis field is used to determine how many buckets will be returned.
    ///
    /// "time"
    #[serde(rename="time")]
    Time,
    

    /// Denotes that bucketing by session is requested. When this is specified, only data that occurs within sessions that begin and end within the dataset time frame, is included in the results.
    ///
    /// "session"
    #[serde(rename="session")]
    Session,
    

    /// Denotes that bucketing by activity type is requested. When this is specified, there will be one bucket for each unique activity type that a user participated in, during the dataset time frame of interest.
    ///
    /// "activityType"
    #[serde(rename="activityType")]
    ActivityType,
    

    /// Denotes that bucketing by individual activity segment is requested. This will aggregate data by the time boundaries specified by each activity segment occurring within the dataset time frame of interest.
    ///
    /// "activitySegment"
    #[serde(rename="activitySegment")]
    ActivitySegment,
}

impl AsRef<str> for AggregateBucketTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AggregateBucketTypeEnum::Unknown => "unknown",
            AggregateBucketTypeEnum::Time => "time",
            AggregateBucketTypeEnum::Session => "session",
            AggregateBucketTypeEnum::ActivityType => "activityType",
            AggregateBucketTypeEnum::ActivitySegment => "activitySegment",
        }
    }
}

impl std::convert::TryFrom< &str> for AggregateBucketTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(AggregateBucketTypeEnum::Unknown),
           "time" => Ok(AggregateBucketTypeEnum::Time),
           "session" => Ok(AggregateBucketTypeEnum::Session),
           "activityType" => Ok(AggregateBucketTypeEnum::ActivityType),
           "activitySegment" => Ok(AggregateBucketTypeEnum::ActivitySegment),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AggregateBucketTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AggregateRequestFilteredDataQualityStandardEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// DO NOT POPULATE THIS FIELD. It is ignored.
pub enum AggregateRequestFilteredDataQualityStandardEnum {
    
    /// "dataQualityUnknown"
    #[serde(rename="dataQualityUnknown")]
    DataQualityUnknown,
    
    /// "dataQualityBloodPressureEsh2002"
    #[serde(rename="dataQualityBloodPressureEsh2002")]
    DataQualityBloodPressureEsh2002,
    
    /// "dataQualityBloodPressureEsh2010"
    #[serde(rename="dataQualityBloodPressureEsh2010")]
    DataQualityBloodPressureEsh2010,
    
    /// "dataQualityBloodPressureAami"
    #[serde(rename="dataQualityBloodPressureAami")]
    DataQualityBloodPressureAami,
    
    /// "dataQualityBloodPressureBhsAA"
    #[serde(rename="dataQualityBloodPressureBhsAA")]
    DataQualityBloodPressureBhsAA,
    
    /// "dataQualityBloodPressureBhsAB"
    #[serde(rename="dataQualityBloodPressureBhsAB")]
    DataQualityBloodPressureBhsAB,
    
    /// "dataQualityBloodPressureBhsBA"
    #[serde(rename="dataQualityBloodPressureBhsBA")]
    DataQualityBloodPressureBhsBA,
    
    /// "dataQualityBloodPressureBhsBB"
    #[serde(rename="dataQualityBloodPressureBhsBB")]
    DataQualityBloodPressureBhsBB,
    
    /// "dataQualityBloodGlucoseIso151972003"
    #[serde(rename="dataQualityBloodGlucoseIso151972003")]
    DataQualityBloodGlucoseIso151972003,
    
    /// "dataQualityBloodGlucoseIso151972013"
    #[serde(rename="dataQualityBloodGlucoseIso151972013")]
    DataQualityBloodGlucoseIso151972013,
}

impl AsRef<str> for AggregateRequestFilteredDataQualityStandardEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AggregateRequestFilteredDataQualityStandardEnum::DataQualityUnknown => "dataQualityUnknown",
            AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureEsh2002 => "dataQualityBloodPressureEsh2002",
            AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureEsh2010 => "dataQualityBloodPressureEsh2010",
            AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureAami => "dataQualityBloodPressureAami",
            AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureBhsAA => "dataQualityBloodPressureBhsAA",
            AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureBhsAB => "dataQualityBloodPressureBhsAB",
            AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureBhsBA => "dataQualityBloodPressureBhsBA",
            AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureBhsBB => "dataQualityBloodPressureBhsBB",
            AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodGlucoseIso151972003 => "dataQualityBloodGlucoseIso151972003",
            AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodGlucoseIso151972013 => "dataQualityBloodGlucoseIso151972013",
        }
    }
}

impl std::convert::TryFrom< &str> for AggregateRequestFilteredDataQualityStandardEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "dataQualityUnknown" => Ok(AggregateRequestFilteredDataQualityStandardEnum::DataQualityUnknown),
           "dataQualityBloodPressureEsh2002" => Ok(AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureEsh2002),
           "dataQualityBloodPressureEsh2010" => Ok(AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureEsh2010),
           "dataQualityBloodPressureAami" => Ok(AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureAami),
           "dataQualityBloodPressureBhsAA" => Ok(AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureBhsAA),
           "dataQualityBloodPressureBhsAB" => Ok(AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureBhsAB),
           "dataQualityBloodPressureBhsBA" => Ok(AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureBhsBA),
           "dataQualityBloodPressureBhsBB" => Ok(AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodPressureBhsBB),
           "dataQualityBloodGlucoseIso151972003" => Ok(AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodGlucoseIso151972003),
           "dataQualityBloodGlucoseIso151972013" => Ok(AggregateRequestFilteredDataQualityStandardEnum::DataQualityBloodGlucoseIso151972013),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AggregateRequestFilteredDataQualityStandardEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BucketByTimePeriodTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum BucketByTimePeriodTypeEnum {
    
    /// "day"
    #[serde(rename="day")]
    Day,
    
    /// "week"
    #[serde(rename="week")]
    Week,
    
    /// "month"
    #[serde(rename="month")]
    Month,
}

impl AsRef<str> for BucketByTimePeriodTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BucketByTimePeriodTypeEnum::Day => "day",
            BucketByTimePeriodTypeEnum::Week => "week",
            BucketByTimePeriodTypeEnum::Month => "month",
        }
    }
}

impl std::convert::TryFrom< &str> for BucketByTimePeriodTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "day" => Ok(BucketByTimePeriodTypeEnum::Day),
           "week" => Ok(BucketByTimePeriodTypeEnum::Week),
           "month" => Ok(BucketByTimePeriodTypeEnum::Month),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BucketByTimePeriodTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataSourceDataQualityStandardEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// DO NOT POPULATE THIS FIELD. It is never populated in responses from the platform, and is ignored in queries. It will be removed in a future version entirely.
pub enum DataSourceDataQualityStandardEnum {
    
    /// "dataQualityUnknown"
    #[serde(rename="dataQualityUnknown")]
    DataQualityUnknown,
    
    /// "dataQualityBloodPressureEsh2002"
    #[serde(rename="dataQualityBloodPressureEsh2002")]
    DataQualityBloodPressureEsh2002,
    
    /// "dataQualityBloodPressureEsh2010"
    #[serde(rename="dataQualityBloodPressureEsh2010")]
    DataQualityBloodPressureEsh2010,
    
    /// "dataQualityBloodPressureAami"
    #[serde(rename="dataQualityBloodPressureAami")]
    DataQualityBloodPressureAami,
    
    /// "dataQualityBloodPressureBhsAA"
    #[serde(rename="dataQualityBloodPressureBhsAA")]
    DataQualityBloodPressureBhsAA,
    
    /// "dataQualityBloodPressureBhsAB"
    #[serde(rename="dataQualityBloodPressureBhsAB")]
    DataQualityBloodPressureBhsAB,
    
    /// "dataQualityBloodPressureBhsBA"
    #[serde(rename="dataQualityBloodPressureBhsBA")]
    DataQualityBloodPressureBhsBA,
    
    /// "dataQualityBloodPressureBhsBB"
    #[serde(rename="dataQualityBloodPressureBhsBB")]
    DataQualityBloodPressureBhsBB,
    
    /// "dataQualityBloodGlucoseIso151972003"
    #[serde(rename="dataQualityBloodGlucoseIso151972003")]
    DataQualityBloodGlucoseIso151972003,
    
    /// "dataQualityBloodGlucoseIso151972013"
    #[serde(rename="dataQualityBloodGlucoseIso151972013")]
    DataQualityBloodGlucoseIso151972013,
}

impl AsRef<str> for DataSourceDataQualityStandardEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataSourceDataQualityStandardEnum::DataQualityUnknown => "dataQualityUnknown",
            DataSourceDataQualityStandardEnum::DataQualityBloodPressureEsh2002 => "dataQualityBloodPressureEsh2002",
            DataSourceDataQualityStandardEnum::DataQualityBloodPressureEsh2010 => "dataQualityBloodPressureEsh2010",
            DataSourceDataQualityStandardEnum::DataQualityBloodPressureAami => "dataQualityBloodPressureAami",
            DataSourceDataQualityStandardEnum::DataQualityBloodPressureBhsAA => "dataQualityBloodPressureBhsAA",
            DataSourceDataQualityStandardEnum::DataQualityBloodPressureBhsAB => "dataQualityBloodPressureBhsAB",
            DataSourceDataQualityStandardEnum::DataQualityBloodPressureBhsBA => "dataQualityBloodPressureBhsBA",
            DataSourceDataQualityStandardEnum::DataQualityBloodPressureBhsBB => "dataQualityBloodPressureBhsBB",
            DataSourceDataQualityStandardEnum::DataQualityBloodGlucoseIso151972003 => "dataQualityBloodGlucoseIso151972003",
            DataSourceDataQualityStandardEnum::DataQualityBloodGlucoseIso151972013 => "dataQualityBloodGlucoseIso151972013",
        }
    }
}

impl std::convert::TryFrom< &str> for DataSourceDataQualityStandardEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "dataQualityUnknown" => Ok(DataSourceDataQualityStandardEnum::DataQualityUnknown),
           "dataQualityBloodPressureEsh2002" => Ok(DataSourceDataQualityStandardEnum::DataQualityBloodPressureEsh2002),
           "dataQualityBloodPressureEsh2010" => Ok(DataSourceDataQualityStandardEnum::DataQualityBloodPressureEsh2010),
           "dataQualityBloodPressureAami" => Ok(DataSourceDataQualityStandardEnum::DataQualityBloodPressureAami),
           "dataQualityBloodPressureBhsAA" => Ok(DataSourceDataQualityStandardEnum::DataQualityBloodPressureBhsAA),
           "dataQualityBloodPressureBhsAB" => Ok(DataSourceDataQualityStandardEnum::DataQualityBloodPressureBhsAB),
           "dataQualityBloodPressureBhsBA" => Ok(DataSourceDataQualityStandardEnum::DataQualityBloodPressureBhsBA),
           "dataQualityBloodPressureBhsBB" => Ok(DataSourceDataQualityStandardEnum::DataQualityBloodPressureBhsBB),
           "dataQualityBloodGlucoseIso151972003" => Ok(DataSourceDataQualityStandardEnum::DataQualityBloodGlucoseIso151972003),
           "dataQualityBloodGlucoseIso151972013" => Ok(DataSourceDataQualityStandardEnum::DataQualityBloodGlucoseIso151972013),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataSourceDataQualityStandardEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataSourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A constant describing the type of this data source. Indicates whether this data source produces raw or derived data.
pub enum DataSourceTypeEnum {
    
    /// "raw"
    #[serde(rename="raw")]
    Raw,
    
    /// "derived"
    #[serde(rename="derived")]
    Derived,
}

impl AsRef<str> for DataSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataSourceTypeEnum::Raw => "raw",
            DataSourceTypeEnum::Derived => "derived",
        }
    }
}

impl std::convert::TryFrom< &str> for DataSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "raw" => Ok(DataSourceTypeEnum::Raw),
           "derived" => Ok(DataSourceTypeEnum::Derived),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataSourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataTypeFieldFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The different supported formats for each field in a data type.
pub enum DataTypeFieldFormatEnum {
    
    /// "integer"
    #[serde(rename="integer")]
    Integer,
    
    /// "floatPoint"
    #[serde(rename="floatPoint")]
    FloatPoint,
    
    /// "string"
    #[serde(rename="string")]
    String,
    
    /// "map"
    #[serde(rename="map")]
    Map,
    
    /// "integerList"
    #[serde(rename="integerList")]
    IntegerList,
    
    /// "floatList"
    #[serde(rename="floatList")]
    FloatList,
    
    /// "blob"
    #[serde(rename="blob")]
    Blob,
}

impl AsRef<str> for DataTypeFieldFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataTypeFieldFormatEnum::Integer => "integer",
            DataTypeFieldFormatEnum::FloatPoint => "floatPoint",
            DataTypeFieldFormatEnum::String => "string",
            DataTypeFieldFormatEnum::Map => "map",
            DataTypeFieldFormatEnum::IntegerList => "integerList",
            DataTypeFieldFormatEnum::FloatList => "floatList",
            DataTypeFieldFormatEnum::Blob => "blob",
        }
    }
}

impl std::convert::TryFrom< &str> for DataTypeFieldFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "integer" => Ok(DataTypeFieldFormatEnum::Integer),
           "floatPoint" => Ok(DataTypeFieldFormatEnum::FloatPoint),
           "string" => Ok(DataTypeFieldFormatEnum::String),
           "map" => Ok(DataTypeFieldFormatEnum::Map),
           "integerList" => Ok(DataTypeFieldFormatEnum::IntegerList),
           "floatList" => Ok(DataTypeFieldFormatEnum::FloatList),
           "blob" => Ok(DataTypeFieldFormatEnum::Blob),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataTypeFieldFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A constant representing the type of the device.
pub enum DeviceTypeEnum {
    

    /// Device type is not known.
    ///
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    

    /// An Android phone.
    ///
    /// "phone"
    #[serde(rename="phone")]
    Phone,
    

    /// An Android tablet.
    ///
    /// "tablet"
    #[serde(rename="tablet")]
    Tablet,
    

    /// A watch or other wrist-mounted band.
    ///
    /// "watch"
    #[serde(rename="watch")]
    Watch,
    

    /// A chest strap.
    ///
    /// "chestStrap"
    #[serde(rename="chestStrap")]
    ChestStrap,
    

    /// A scale.
    ///
    /// "scale"
    #[serde(rename="scale")]
    Scale,
    

    /// Glass or other head-mounted device.
    ///
    /// "headMounted"
    #[serde(rename="headMounted")]
    HeadMounted,
    

    /// A smart display e.g. Nest device.
    ///
    /// "smartDisplay"
    #[serde(rename="smartDisplay")]
    SmartDisplay,
}

impl AsRef<str> for DeviceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceTypeEnum::Unknown => "unknown",
            DeviceTypeEnum::Phone => "phone",
            DeviceTypeEnum::Tablet => "tablet",
            DeviceTypeEnum::Watch => "watch",
            DeviceTypeEnum::ChestStrap => "chestStrap",
            DeviceTypeEnum::Scale => "scale",
            DeviceTypeEnum::HeadMounted => "headMounted",
            DeviceTypeEnum::SmartDisplay => "smartDisplay",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(DeviceTypeEnum::Unknown),
           "phone" => Ok(DeviceTypeEnum::Phone),
           "tablet" => Ok(DeviceTypeEnum::Tablet),
           "watch" => Ok(DeviceTypeEnum::Watch),
           "chestStrap" => Ok(DeviceTypeEnum::ChestStrap),
           "scale" => Ok(DeviceTypeEnum::Scale),
           "headMounted" => Ok(DeviceTypeEnum::HeadMounted),
           "smartDisplay" => Ok(DeviceTypeEnum::SmartDisplay),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


