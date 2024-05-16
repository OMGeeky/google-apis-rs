use super::*;



// region BackfillJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Backfill job state.
pub enum BackfillJobStateEnum {
    

    /// Default value.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Backfill job was never started for the stream object (stream has backfill strategy defined as manual or object was explicitly excluded from automatic backfill).
    ///
    /// "NOT_STARTED"
    #[serde(rename="NOT_STARTED")]
    NOTSTARTED,
    

    /// Backfill job will start pending available resources.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Backfill job is running.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Backfill job stopped (next job run will start from beginning).
    ///
    /// "STOPPED"
    #[serde(rename="STOPPED")]
    STOPPED,
    

    /// Backfill job failed (due to an error).
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Backfill completed successfully.
    ///
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// Backfill job failed since the table structure is currently unsupported for backfill.
    ///
    /// "UNSUPPORTED"
    #[serde(rename="UNSUPPORTED")]
    UNSUPPORTED,
}

impl AsRef<str> for BackfillJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackfillJobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            BackfillJobStateEnum::NOTSTARTED => "NOT_STARTED",
            BackfillJobStateEnum::PENDING => "PENDING",
            BackfillJobStateEnum::ACTIVE => "ACTIVE",
            BackfillJobStateEnum::STOPPED => "STOPPED",
            BackfillJobStateEnum::FAILED => "FAILED",
            BackfillJobStateEnum::COMPLETED => "COMPLETED",
            BackfillJobStateEnum::UNSUPPORTED => "UNSUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for BackfillJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(BackfillJobStateEnum::STATEUNSPECIFIED),
           "NOT_STARTED" => Ok(BackfillJobStateEnum::NOTSTARTED),
           "PENDING" => Ok(BackfillJobStateEnum::PENDING),
           "ACTIVE" => Ok(BackfillJobStateEnum::ACTIVE),
           "STOPPED" => Ok(BackfillJobStateEnum::STOPPED),
           "FAILED" => Ok(BackfillJobStateEnum::FAILED),
           "COMPLETED" => Ok(BackfillJobStateEnum::COMPLETED),
           "UNSUPPORTED" => Ok(BackfillJobStateEnum::UNSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackfillJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackfillJobTriggerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Backfill job's triggering reason.
pub enum BackfillJobTriggerEnum {
    

    /// Default value.
    ///
    /// "TRIGGER_UNSPECIFIED"
    #[serde(rename="TRIGGER_UNSPECIFIED")]
    TRIGGERUNSPECIFIED,
    

    /// Object backfill job was triggered automatically according to the stream's backfill strategy.
    ///
    /// "AUTOMATIC"
    #[serde(rename="AUTOMATIC")]
    AUTOMATIC,
    

    /// Object backfill job was triggered manually using the dedicated API.
    ///
    /// "MANUAL"
    #[serde(rename="MANUAL")]
    MANUAL,
}

impl AsRef<str> for BackfillJobTriggerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackfillJobTriggerEnum::TRIGGERUNSPECIFIED => "TRIGGER_UNSPECIFIED",
            BackfillJobTriggerEnum::AUTOMATIC => "AUTOMATIC",
            BackfillJobTriggerEnum::MANUAL => "MANUAL",
        }
    }
}

impl std::convert::TryFrom< &str> for BackfillJobTriggerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRIGGER_UNSPECIFIED" => Ok(BackfillJobTriggerEnum::TRIGGERUNSPECIFIED),
           "AUTOMATIC" => Ok(BackfillJobTriggerEnum::AUTOMATIC),
           "MANUAL" => Ok(BackfillJobTriggerEnum::MANUAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackfillJobTriggerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JsonFileFormatCompressionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Compression of the loaded JSON file.
pub enum JsonFileFormatCompressionEnum {
    

    /// Unspecified json file compression.
    ///
    /// "JSON_COMPRESSION_UNSPECIFIED"
    #[serde(rename="JSON_COMPRESSION_UNSPECIFIED")]
    JSONCOMPRESSIONUNSPECIFIED,
    

    /// Do not compress JSON file.
    ///
    /// "NO_COMPRESSION"
    #[serde(rename="NO_COMPRESSION")]
    NOCOMPRESSION,
    

    /// Gzip compression.
    ///
    /// "GZIP"
    #[serde(rename="GZIP")]
    GZIP,
}

impl AsRef<str> for JsonFileFormatCompressionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JsonFileFormatCompressionEnum::JSONCOMPRESSIONUNSPECIFIED => "JSON_COMPRESSION_UNSPECIFIED",
            JsonFileFormatCompressionEnum::NOCOMPRESSION => "NO_COMPRESSION",
            JsonFileFormatCompressionEnum::GZIP => "GZIP",
        }
    }
}

impl std::convert::TryFrom< &str> for JsonFileFormatCompressionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JSON_COMPRESSION_UNSPECIFIED" => Ok(JsonFileFormatCompressionEnum::JSONCOMPRESSIONUNSPECIFIED),
           "NO_COMPRESSION" => Ok(JsonFileFormatCompressionEnum::NOCOMPRESSION),
           "GZIP" => Ok(JsonFileFormatCompressionEnum::GZIP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JsonFileFormatCompressionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JsonFileFormatSchemaFileFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The schema file format along JSON data files.
pub enum JsonFileFormatSchemaFileFormatEnum {
    

    /// Unspecified schema file format.
    ///
    /// "SCHEMA_FILE_FORMAT_UNSPECIFIED"
    #[serde(rename="SCHEMA_FILE_FORMAT_UNSPECIFIED")]
    SCHEMAFILEFORMATUNSPECIFIED,
    

    /// Do not attach schema file.
    ///
    /// "NO_SCHEMA_FILE"
    #[serde(rename="NO_SCHEMA_FILE")]
    NOSCHEMAFILE,
    

    /// Avro schema format.
    ///
    /// "AVRO_SCHEMA_FILE"
    #[serde(rename="AVRO_SCHEMA_FILE")]
    AVROSCHEMAFILE,
}

impl AsRef<str> for JsonFileFormatSchemaFileFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JsonFileFormatSchemaFileFormatEnum::SCHEMAFILEFORMATUNSPECIFIED => "SCHEMA_FILE_FORMAT_UNSPECIFIED",
            JsonFileFormatSchemaFileFormatEnum::NOSCHEMAFILE => "NO_SCHEMA_FILE",
            JsonFileFormatSchemaFileFormatEnum::AVROSCHEMAFILE => "AVRO_SCHEMA_FILE",
        }
    }
}

impl std::convert::TryFrom< &str> for JsonFileFormatSchemaFileFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCHEMA_FILE_FORMAT_UNSPECIFIED" => Ok(JsonFileFormatSchemaFileFormatEnum::SCHEMAFILEFORMATUNSPECIFIED),
           "NO_SCHEMA_FILE" => Ok(JsonFileFormatSchemaFileFormatEnum::NOSCHEMAFILE),
           "AVRO_SCHEMA_FILE" => Ok(JsonFileFormatSchemaFileFormatEnum::AVROSCHEMAFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JsonFileFormatSchemaFileFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PrivateConnectionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the Private Connection.
pub enum PrivateConnectionStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The private connection is in creation state - creating resources.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The private connection has been created with all of its resources.
    ///
    /// "CREATED"
    #[serde(rename="CREATED")]
    CREATED,
    

    /// The private connection creation has failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The private connection is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Delete request has failed, resource is in invalid state.
    ///
    /// "FAILED_TO_DELETE"
    #[serde(rename="FAILED_TO_DELETE")]
    FAILEDTODELETE,
}

impl AsRef<str> for PrivateConnectionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PrivateConnectionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            PrivateConnectionStateEnum::CREATING => "CREATING",
            PrivateConnectionStateEnum::CREATED => "CREATED",
            PrivateConnectionStateEnum::FAILED => "FAILED",
            PrivateConnectionStateEnum::DELETING => "DELETING",
            PrivateConnectionStateEnum::FAILEDTODELETE => "FAILED_TO_DELETE",
        }
    }
}

impl std::convert::TryFrom< &str> for PrivateConnectionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(PrivateConnectionStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(PrivateConnectionStateEnum::CREATING),
           "CREATED" => Ok(PrivateConnectionStateEnum::CREATED),
           "FAILED" => Ok(PrivateConnectionStateEnum::FAILED),
           "DELETING" => Ok(PrivateConnectionStateEnum::DELETING),
           "FAILED_TO_DELETE" => Ok(PrivateConnectionStateEnum::FAILEDTODELETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PrivateConnectionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StreamStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the stream.
pub enum StreamStateEnum {
    

    /// Unspecified stream state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The stream has been created but has not yet started streaming data.
    ///
    /// "NOT_STARTED"
    #[serde(rename="NOT_STARTED")]
    NOTSTARTED,
    

    /// The stream is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The stream is paused.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// The stream is in maintenance mode. Updates are rejected on the resource in this state.
    ///
    /// "MAINTENANCE"
    #[serde(rename="MAINTENANCE")]
    MAINTENANCE,
    

    /// The stream is experiencing an error that is preventing data from being streamed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The stream has experienced a terminal failure.
    ///
    /// "FAILED_PERMANENTLY"
    #[serde(rename="FAILED_PERMANENTLY")]
    FAILEDPERMANENTLY,
    

    /// The stream is starting, but not yet running.
    ///
    /// "STARTING"
    #[serde(rename="STARTING")]
    STARTING,
    

    /// The Stream is no longer reading new events, but still writing events in the buffer.
    ///
    /// "DRAINING"
    #[serde(rename="DRAINING")]
    DRAINING,
}

impl AsRef<str> for StreamStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StreamStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            StreamStateEnum::NOTSTARTED => "NOT_STARTED",
            StreamStateEnum::RUNNING => "RUNNING",
            StreamStateEnum::PAUSED => "PAUSED",
            StreamStateEnum::MAINTENANCE => "MAINTENANCE",
            StreamStateEnum::FAILED => "FAILED",
            StreamStateEnum::FAILEDPERMANENTLY => "FAILED_PERMANENTLY",
            StreamStateEnum::STARTING => "STARTING",
            StreamStateEnum::DRAINING => "DRAINING",
        }
    }
}

impl std::convert::TryFrom< &str> for StreamStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(StreamStateEnum::STATEUNSPECIFIED),
           "NOT_STARTED" => Ok(StreamStateEnum::NOTSTARTED),
           "RUNNING" => Ok(StreamStateEnum::RUNNING),
           "PAUSED" => Ok(StreamStateEnum::PAUSED),
           "MAINTENANCE" => Ok(StreamStateEnum::MAINTENANCE),
           "FAILED" => Ok(StreamStateEnum::FAILED),
           "FAILED_PERMANENTLY" => Ok(StreamStateEnum::FAILEDPERMANENTLY),
           "STARTING" => Ok(StreamStateEnum::STARTING),
           "DRAINING" => Ok(StreamStateEnum::DRAINING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StreamStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


