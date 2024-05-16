use super::*;



// region AwsKinesiStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. An output-only field that indicates the state of the Kinesis ingestion source.
pub enum AwsKinesiStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Ingestion is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Permission denied encountered while consuming data from Kinesis. This can happen if: - The provided `aws_role_arn` does not exist or does not have the appropriate permissions attached. - The provided `aws_role_arn` is not set up properly for Identity Federation using `gcp_service_account`. - The Pub/Sub SA is not granted the `iam.serviceAccounts.getOpenIdToken` permission on `gcp_service_account`.
    ///
    /// "KINESIS_PERMISSION_DENIED"
    #[serde(rename="KINESIS_PERMISSION_DENIED")]
    KINESISPERMISSIONDENIED,
    

    /// Permission denied encountered while publishing to the topic. This can happen if the Pub/Sub SA has not been granted the [appropriate publish permissions](https://cloud.google.com/pubsub/docs/access-control#pubsub.publisher)
    ///
    /// "PUBLISH_PERMISSION_DENIED"
    #[serde(rename="PUBLISH_PERMISSION_DENIED")]
    PUBLISHPERMISSIONDENIED,
    

    /// The Kinesis stream does not exist.
    ///
    /// "STREAM_NOT_FOUND"
    #[serde(rename="STREAM_NOT_FOUND")]
    STREAMNOTFOUND,
    

    /// The Kinesis consumer does not exist.
    ///
    /// "CONSUMER_NOT_FOUND"
    #[serde(rename="CONSUMER_NOT_FOUND")]
    CONSUMERNOTFOUND,
}

impl AsRef<str> for AwsKinesiStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AwsKinesiStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AwsKinesiStateEnum::ACTIVE => "ACTIVE",
            AwsKinesiStateEnum::KINESISPERMISSIONDENIED => "KINESIS_PERMISSION_DENIED",
            AwsKinesiStateEnum::PUBLISHPERMISSIONDENIED => "PUBLISH_PERMISSION_DENIED",
            AwsKinesiStateEnum::STREAMNOTFOUND => "STREAM_NOT_FOUND",
            AwsKinesiStateEnum::CONSUMERNOTFOUND => "CONSUMER_NOT_FOUND",
        }
    }
}

impl std::convert::TryFrom< &str> for AwsKinesiStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AwsKinesiStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(AwsKinesiStateEnum::ACTIVE),
           "KINESIS_PERMISSION_DENIED" => Ok(AwsKinesiStateEnum::KINESISPERMISSIONDENIED),
           "PUBLISH_PERMISSION_DENIED" => Ok(AwsKinesiStateEnum::PUBLISHPERMISSIONDENIED),
           "STREAM_NOT_FOUND" => Ok(AwsKinesiStateEnum::STREAMNOTFOUND),
           "CONSUMER_NOT_FOUND" => Ok(AwsKinesiStateEnum::CONSUMERNOTFOUND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AwsKinesiStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BigQueryConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. An output-only field that indicates whether or not the subscription can receive messages.
pub enum BigQueryConfigStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The subscription can actively send messages to BigQuery
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Cannot write to the BigQuery table because of permission denied errors. This can happen if - Pub/Sub SA has not been granted the [appropriate BigQuery IAM permissions](https://cloud.google.com/pubsub/docs/create-subscription#assign_bigquery_service_account) - bigquery.googleapis.com API is not enabled for the project ([instructions](https://cloud.google.com/service-usage/docs/enable-disable))
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// Cannot write to the BigQuery table because it does not exist.
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
    

    /// Cannot write to the BigQuery table due to a schema mismatch.
    ///
    /// "SCHEMA_MISMATCH"
    #[serde(rename="SCHEMA_MISMATCH")]
    SCHEMAMISMATCH,
    

    /// Cannot write to the destination because enforce_in_transit is set to true and the destination locations are not in the allowed regions.
    ///
    /// "IN_TRANSIT_LOCATION_RESTRICTION"
    #[serde(rename="IN_TRANSIT_LOCATION_RESTRICTION")]
    INTRANSITLOCATIONRESTRICTION,
}

impl AsRef<str> for BigQueryConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BigQueryConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            BigQueryConfigStateEnum::ACTIVE => "ACTIVE",
            BigQueryConfigStateEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            BigQueryConfigStateEnum::NOTFOUND => "NOT_FOUND",
            BigQueryConfigStateEnum::SCHEMAMISMATCH => "SCHEMA_MISMATCH",
            BigQueryConfigStateEnum::INTRANSITLOCATIONRESTRICTION => "IN_TRANSIT_LOCATION_RESTRICTION",
        }
    }
}

impl std::convert::TryFrom< &str> for BigQueryConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(BigQueryConfigStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(BigQueryConfigStateEnum::ACTIVE),
           "PERMISSION_DENIED" => Ok(BigQueryConfigStateEnum::PERMISSIONDENIED),
           "NOT_FOUND" => Ok(BigQueryConfigStateEnum::NOTFOUND),
           "SCHEMA_MISMATCH" => Ok(BigQueryConfigStateEnum::SCHEMAMISMATCH),
           "IN_TRANSIT_LOCATION_RESTRICTION" => Ok(BigQueryConfigStateEnum::INTRANSITLOCATIONRESTRICTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BigQueryConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudStorageConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. An output-only field that indicates whether or not the subscription can receive messages.
pub enum CloudStorageConfigStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The subscription can actively send messages to Cloud Storage.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Cannot write to the Cloud Storage bucket because of permission denied errors.
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// Cannot write to the Cloud Storage bucket because it does not exist.
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
    

    /// Cannot write to the destination because enforce_in_transit is set to true and the destination locations are not in the allowed regions.
    ///
    /// "IN_TRANSIT_LOCATION_RESTRICTION"
    #[serde(rename="IN_TRANSIT_LOCATION_RESTRICTION")]
    INTRANSITLOCATIONRESTRICTION,
}

impl AsRef<str> for CloudStorageConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudStorageConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CloudStorageConfigStateEnum::ACTIVE => "ACTIVE",
            CloudStorageConfigStateEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            CloudStorageConfigStateEnum::NOTFOUND => "NOT_FOUND",
            CloudStorageConfigStateEnum::INTRANSITLOCATIONRESTRICTION => "IN_TRANSIT_LOCATION_RESTRICTION",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudStorageConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CloudStorageConfigStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(CloudStorageConfigStateEnum::ACTIVE),
           "PERMISSION_DENIED" => Ok(CloudStorageConfigStateEnum::PERMISSIONDENIED),
           "NOT_FOUND" => Ok(CloudStorageConfigStateEnum::NOTFOUND),
           "IN_TRANSIT_LOCATION_RESTRICTION" => Ok(CloudStorageConfigStateEnum::INTRANSITLOCATIONRESTRICTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudStorageConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SchemaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the schema definition.
pub enum SchemaTypeEnum {
    

    /// Default value. This value is unused.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// A Protocol Buffer schema definition.
    ///
    /// "PROTOCOL_BUFFER"
    #[serde(rename="PROTOCOL_BUFFER")]
    PROTOCOLBUFFER,
    

    /// An Avro schema definition.
    ///
    /// "AVRO"
    #[serde(rename="AVRO")]
    AVRO,
}

impl AsRef<str> for SchemaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SchemaTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            SchemaTypeEnum::PROTOCOLBUFFER => "PROTOCOL_BUFFER",
            SchemaTypeEnum::AVRO => "AVRO",
        }
    }
}

impl std::convert::TryFrom< &str> for SchemaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(SchemaTypeEnum::TYPEUNSPECIFIED),
           "PROTOCOL_BUFFER" => Ok(SchemaTypeEnum::PROTOCOLBUFFER),
           "AVRO" => Ok(SchemaTypeEnum::AVRO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SchemaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SchemaSettingEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The encoding of messages validated against `schema`.
pub enum SchemaSettingEncodingEnum {
    

    /// Unspecified
    ///
    /// "ENCODING_UNSPECIFIED"
    #[serde(rename="ENCODING_UNSPECIFIED")]
    ENCODINGUNSPECIFIED,
    

    /// JSON encoding
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
    

    /// Binary encoding, as defined by the schema type. For some schema types, binary encoding may not be available.
    ///
    /// "BINARY"
    #[serde(rename="BINARY")]
    BINARY,
}

impl AsRef<str> for SchemaSettingEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SchemaSettingEncodingEnum::ENCODINGUNSPECIFIED => "ENCODING_UNSPECIFIED",
            SchemaSettingEncodingEnum::JSON => "JSON",
            SchemaSettingEncodingEnum::BINARY => "BINARY",
        }
    }
}

impl std::convert::TryFrom< &str> for SchemaSettingEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCODING_UNSPECIFIED" => Ok(SchemaSettingEncodingEnum::ENCODINGUNSPECIFIED),
           "JSON" => Ok(SchemaSettingEncodingEnum::JSON),
           "BINARY" => Ok(SchemaSettingEncodingEnum::BINARY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SchemaSettingEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SubscriptionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. An output-only field indicating whether or not the subscription can receive messages.
pub enum SubscriptionStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The subscription can actively receive messages
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The subscription cannot receive messages because of an error with the resource to which it pushes messages. See the more detailed error state in the corresponding configuration.
    ///
    /// "RESOURCE_ERROR"
    #[serde(rename="RESOURCE_ERROR")]
    RESOURCEERROR,
}

impl AsRef<str> for SubscriptionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SubscriptionStateEnum::ACTIVE => "ACTIVE",
            SubscriptionStateEnum::RESOURCEERROR => "RESOURCE_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for SubscriptionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SubscriptionStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(SubscriptionStateEnum::ACTIVE),
           "RESOURCE_ERROR" => Ok(SubscriptionStateEnum::RESOURCEERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TopicStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. An output-only field indicating the state of the topic.
pub enum TopicStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The topic does not have any persistent errors.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Ingestion from the data source has encountered a permanent error. See the more detailed error state in the corresponding ingestion source configuration.
    ///
    /// "INGESTION_RESOURCE_ERROR"
    #[serde(rename="INGESTION_RESOURCE_ERROR")]
    INGESTIONRESOURCEERROR,
}

impl AsRef<str> for TopicStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TopicStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            TopicStateEnum::ACTIVE => "ACTIVE",
            TopicStateEnum::INGESTIONRESOURCEERROR => "INGESTION_RESOURCE_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for TopicStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(TopicStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(TopicStateEnum::ACTIVE),
           "INGESTION_RESOURCE_ERROR" => Ok(TopicStateEnum::INGESTIONRESOURCEERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TopicStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ValidateMessageRequestEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The encoding expected for messages
pub enum ValidateMessageRequestEncodingEnum {
    

    /// Unspecified
    ///
    /// "ENCODING_UNSPECIFIED"
    #[serde(rename="ENCODING_UNSPECIFIED")]
    ENCODINGUNSPECIFIED,
    

    /// JSON encoding
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
    

    /// Binary encoding, as defined by the schema type. For some schema types, binary encoding may not be available.
    ///
    /// "BINARY"
    #[serde(rename="BINARY")]
    BINARY,
}

impl AsRef<str> for ValidateMessageRequestEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ValidateMessageRequestEncodingEnum::ENCODINGUNSPECIFIED => "ENCODING_UNSPECIFIED",
            ValidateMessageRequestEncodingEnum::JSON => "JSON",
            ValidateMessageRequestEncodingEnum::BINARY => "BINARY",
        }
    }
}

impl std::convert::TryFrom< &str> for ValidateMessageRequestEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCODING_UNSPECIFIED" => Ok(ValidateMessageRequestEncodingEnum::ENCODINGUNSPECIFIED),
           "JSON" => Ok(ValidateMessageRequestEncodingEnum::JSON),
           "BINARY" => Ok(ValidateMessageRequestEncodingEnum::BINARY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ValidateMessageRequestEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The set of Schema fields to return in the response. If not set, returns Schemas with `name` and `type`, but not `definition`. Set to `FULL` to retrieve all fields.
pub enum ProjectViewEnum {
    

    /// The default / unset value. The API will default to the BASIC view.
    ///
    /// "SCHEMA_VIEW_UNSPECIFIED"
    #[serde(rename="SCHEMA_VIEW_UNSPECIFIED")]
    SCHEMAVIEWUNSPECIFIED,
    

    /// Include the name and type of the schema, but not the definition.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Include all Schema object fields.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::SCHEMAVIEWUNSPECIFIED => "SCHEMA_VIEW_UNSPECIFIED",
            ProjectViewEnum::BASIC => "BASIC",
            ProjectViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCHEMA_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::SCHEMAVIEWUNSPECIFIED),
           "BASIC" => Ok(ProjectViewEnum::BASIC),
           "FULL" => Ok(ProjectViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


