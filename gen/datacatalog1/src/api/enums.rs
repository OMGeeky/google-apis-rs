use super::*;



// region GoogleCloudDatacatalogV1BigQueryConnectionSpecConnectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the BigQuery connection.
pub enum GoogleCloudDatacatalogV1BigQueryConnectionSpecConnectionTypeEnum {
    

    /// Unspecified type.
    ///
    /// "CONNECTION_TYPE_UNSPECIFIED"
    #[serde(rename="CONNECTION_TYPE_UNSPECIFIED")]
    CONNECTIONTYPEUNSPECIFIED,
    

    /// Cloud SQL connection.
    ///
    /// "CLOUD_SQL"
    #[serde(rename="CLOUD_SQL")]
    CLOUDSQL,
}

impl AsRef<str> for GoogleCloudDatacatalogV1BigQueryConnectionSpecConnectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1BigQueryConnectionSpecConnectionTypeEnum::CONNECTIONTYPEUNSPECIFIED => "CONNECTION_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1BigQueryConnectionSpecConnectionTypeEnum::CLOUDSQL => "CLOUD_SQL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1BigQueryConnectionSpecConnectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1BigQueryConnectionSpecConnectionTypeEnum::CONNECTIONTYPEUNSPECIFIED),
           "CLOUD_SQL" => Ok(GoogleCloudDatacatalogV1BigQueryConnectionSpecConnectionTypeEnum::CLOUDSQL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1BigQueryConnectionSpecConnectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The table source type.
pub enum GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum {
    

    /// Default unknown type.
    ///
    /// "TABLE_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="TABLE_SOURCE_TYPE_UNSPECIFIED")]
    TABLESOURCETYPEUNSPECIFIED,
    

    /// Table view.
    ///
    /// "BIGQUERY_VIEW"
    #[serde(rename="BIGQUERY_VIEW")]
    BIGQUERYVIEW,
    

    /// BigQuery native table.
    ///
    /// "BIGQUERY_TABLE"
    #[serde(rename="BIGQUERY_TABLE")]
    BIGQUERYTABLE,
    

    /// BigQuery materialized view.
    ///
    /// "BIGQUERY_MATERIALIZED_VIEW"
    #[serde(rename="BIGQUERY_MATERIALIZED_VIEW")]
    BIGQUERYMATERIALIZEDVIEW,
}

impl AsRef<str> for GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum::TABLESOURCETYPEUNSPECIFIED => "TABLE_SOURCE_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYVIEW => "BIGQUERY_VIEW",
            GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYTABLE => "BIGQUERY_TABLE",
            GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYMATERIALIZEDVIEW => "BIGQUERY_MATERIALIZED_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TABLE_SOURCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum::TABLESOURCETYPEUNSPECIFIED),
           "BIGQUERY_VIEW" => Ok(GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYVIEW),
           "BIGQUERY_TABLE" => Ok(GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYTABLE),
           "BIGQUERY_MATERIALIZED_VIEW" => Ok(GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum::BIGQUERYMATERIALIZEDVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1BigQueryTableSpecTableSourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the Cloud SQL database.
pub enum GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum {
    

    /// Unspecified database type.
    ///
    /// "DATABASE_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_TYPE_UNSPECIFIED")]
    DATABASETYPEUNSPECIFIED,
    

    /// Cloud SQL for PostgreSQL.
    ///
    /// "POSTGRES"
    #[serde(rename="POSTGRES")]
    POSTGRES,
    

    /// Cloud SQL for MySQL.
    ///
    /// "MYSQL"
    #[serde(rename="MYSQL")]
    MYSQL,
}

impl AsRef<str> for GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum::DATABASETYPEUNSPECIFIED => "DATABASE_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum::POSTGRES => "POSTGRES",
            GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum::MYSQL => "MYSQL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum::DATABASETYPEUNSPECIFIED),
           "POSTGRES" => Ok(GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum::POSTGRES),
           "MYSQL" => Ok(GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum::MYSQL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1CloudSqlBigQueryConnectionSpecTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Most important inclusion of this column.
pub enum GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum {
    

    /// Unspecified.
    ///
    /// "INDEXING_TYPE_UNSPECIFIED"
    #[serde(rename="INDEXING_TYPE_UNSPECIFIED")]
    INDEXINGTYPEUNSPECIFIED,
    

    /// Column not a part of an index.
    ///
    /// "INDEXING_TYPE_NONE"
    #[serde(rename="INDEXING_TYPE_NONE")]
    INDEXINGTYPENONE,
    

    /// Column Part of non unique index.
    ///
    /// "INDEXING_TYPE_NON_UNIQUE"
    #[serde(rename="INDEXING_TYPE_NON_UNIQUE")]
    INDEXINGTYPENONUNIQUE,
    

    /// Column part of unique index.
    ///
    /// "INDEXING_TYPE_UNIQUE"
    #[serde(rename="INDEXING_TYPE_UNIQUE")]
    INDEXINGTYPEUNIQUE,
    

    /// Column part of the primary key.
    ///
    /// "INDEXING_TYPE_PRIMARY_KEY"
    #[serde(rename="INDEXING_TYPE_PRIMARY_KEY")]
    INDEXINGTYPEPRIMARYKEY,
}

impl AsRef<str> for GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum::INDEXINGTYPEUNSPECIFIED => "INDEXING_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum::INDEXINGTYPENONE => "INDEXING_TYPE_NONE",
            GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum::INDEXINGTYPENONUNIQUE => "INDEXING_TYPE_NON_UNIQUE",
            GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum::INDEXINGTYPEUNIQUE => "INDEXING_TYPE_UNIQUE",
            GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum::INDEXINGTYPEPRIMARYKEY => "INDEXING_TYPE_PRIMARY_KEY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INDEXING_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum::INDEXINGTYPEUNSPECIFIED),
           "INDEXING_TYPE_NONE" => Ok(GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum::INDEXINGTYPENONE),
           "INDEXING_TYPE_NON_UNIQUE" => Ok(GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum::INDEXINGTYPENONUNIQUE),
           "INDEXING_TYPE_UNIQUE" => Ok(GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum::INDEXINGTYPEUNIQUE),
           "INDEXING_TYPE_PRIMARY_KEY" => Ok(GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum::INDEXINGTYPEPRIMARYKEY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1ColumnSchemaHighestIndexingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Looker specific column type of this column.
pub enum GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum {
    

    /// Unspecified.
    ///
    /// "LOOKER_COLUMN_TYPE_UNSPECIFIED"
    #[serde(rename="LOOKER_COLUMN_TYPE_UNSPECIFIED")]
    LOOKERCOLUMNTYPEUNSPECIFIED,
    

    /// Dimension.
    ///
    /// "DIMENSION"
    #[serde(rename="DIMENSION")]
    DIMENSION,
    

    /// Dimension group - parent for Dimension.
    ///
    /// "DIMENSION_GROUP"
    #[serde(rename="DIMENSION_GROUP")]
    DIMENSIONGROUP,
    

    /// Filter.
    ///
    /// "FILTER"
    #[serde(rename="FILTER")]
    FILTER,
    

    /// Measure.
    ///
    /// "MEASURE"
    #[serde(rename="MEASURE")]
    MEASURE,
    

    /// Parameter.
    ///
    /// "PARAMETER"
    #[serde(rename="PARAMETER")]
    PARAMETER,
}

impl AsRef<str> for GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::LOOKERCOLUMNTYPEUNSPECIFIED => "LOOKER_COLUMN_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::DIMENSION => "DIMENSION",
            GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::DIMENSIONGROUP => "DIMENSION_GROUP",
            GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::FILTER => "FILTER",
            GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::MEASURE => "MEASURE",
            GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::PARAMETER => "PARAMETER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOOKER_COLUMN_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::LOOKERCOLUMNTYPEUNSPECIFIED),
           "DIMENSION" => Ok(GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::DIMENSION),
           "DIMENSION_GROUP" => Ok(GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::DIMENSIONGROUP),
           "FILTER" => Ok(GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::FILTER),
           "MEASURE" => Ok(GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::MEASURE),
           "PARAMETER" => Ok(GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum::PARAMETER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1ColumnSchemaLookerColumnSpecTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1DataSourceServiceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Service that physically stores the data.
pub enum GoogleCloudDatacatalogV1DataSourceServiceEnum {
    

    /// Default unknown service.
    ///
    /// "SERVICE_UNSPECIFIED"
    #[serde(rename="SERVICE_UNSPECIFIED")]
    SERVICEUNSPECIFIED,
    

    /// Google Cloud Storage service.
    ///
    /// "CLOUD_STORAGE"
    #[serde(rename="CLOUD_STORAGE")]
    CLOUDSTORAGE,
    

    /// BigQuery service.
    ///
    /// "BIGQUERY"
    #[serde(rename="BIGQUERY")]
    BIGQUERY,
}

impl AsRef<str> for GoogleCloudDatacatalogV1DataSourceServiceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1DataSourceServiceEnum::SERVICEUNSPECIFIED => "SERVICE_UNSPECIFIED",
            GoogleCloudDatacatalogV1DataSourceServiceEnum::CLOUDSTORAGE => "CLOUD_STORAGE",
            GoogleCloudDatacatalogV1DataSourceServiceEnum::BIGQUERY => "BIGQUERY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1DataSourceServiceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1DataSourceServiceEnum::SERVICEUNSPECIFIED),
           "CLOUD_STORAGE" => Ok(GoogleCloudDatacatalogV1DataSourceServiceEnum::CLOUDSTORAGE),
           "BIGQUERY" => Ok(GoogleCloudDatacatalogV1DataSourceServiceEnum::BIGQUERY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1DataSourceServiceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this table.
pub enum GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum {
    

    /// Default unknown table type.
    ///
    /// "TABLE_TYPE_UNSPECIFIED"
    #[serde(rename="TABLE_TYPE_UNSPECIFIED")]
    TABLETYPEUNSPECIFIED,
    

    /// Native table.
    ///
    /// "NATIVE"
    #[serde(rename="NATIVE")]
    NATIVE,
    

    /// External table.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
}

impl AsRef<str> for GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum::TABLETYPEUNSPECIFIED => "TABLE_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum::NATIVE => "NATIVE",
            GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum::EXTERNAL => "EXTERNAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TABLE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum::TABLETYPEUNSPECIFIED),
           "NATIVE" => Ok(GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum::NATIVE),
           "EXTERNAL" => Ok(GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum::EXTERNAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1DatabaseTableSpecTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this view.
pub enum GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum {
    

    /// Default unknown view type.
    ///
    /// "VIEW_TYPE_UNSPECIFIED"
    #[serde(rename="VIEW_TYPE_UNSPECIFIED")]
    VIEWTYPEUNSPECIFIED,
    

    /// Standard view.
    ///
    /// "STANDARD_VIEW"
    #[serde(rename="STANDARD_VIEW")]
    STANDARDVIEW,
    

    /// Materialized view.
    ///
    /// "MATERIALIZED_VIEW"
    #[serde(rename="MATERIALIZED_VIEW")]
    MATERIALIZEDVIEW,
}

impl AsRef<str> for GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum::VIEWTYPEUNSPECIFIED => "VIEW_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum::STANDARDVIEW => "STANDARD_VIEW",
            GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum::MATERIALIZEDVIEW => "MATERIALIZED_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum::VIEWTYPEUNSPECIFIED),
           "STANDARD_VIEW" => Ok(GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum::STANDARDVIEW),
           "MATERIALIZED_VIEW" => Ok(GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum::MATERIALIZEDVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1DatabaseTableSpecDatabaseViewSpecViewTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Service in which the external table is registered.
pub enum GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum {
    

    /// Default unknown system.
    ///
    /// "INTEGRATED_SYSTEM_UNSPECIFIED"
    #[serde(rename="INTEGRATED_SYSTEM_UNSPECIFIED")]
    INTEGRATEDSYSTEMUNSPECIFIED,
    

    /// BigQuery.
    ///
    /// "BIGQUERY"
    #[serde(rename="BIGQUERY")]
    BIGQUERY,
    

    /// Cloud Pub/Sub.
    ///
    /// "CLOUD_PUBSUB"
    #[serde(rename="CLOUD_PUBSUB")]
    CLOUDPUBSUB,
    

    /// Dataproc Metastore.
    ///
    /// "DATAPROC_METASTORE"
    #[serde(rename="DATAPROC_METASTORE")]
    DATAPROCMETASTORE,
    

    /// Dataplex.
    ///
    /// "DATAPLEX"
    #[serde(rename="DATAPLEX")]
    DATAPLEX,
    

    /// Cloud Spanner
    ///
    /// "CLOUD_SPANNER"
    #[serde(rename="CLOUD_SPANNER")]
    CLOUDSPANNER,
    

    /// Cloud Bigtable
    ///
    /// "CLOUD_BIGTABLE"
    #[serde(rename="CLOUD_BIGTABLE")]
    CLOUDBIGTABLE,
    

    /// Cloud Sql
    ///
    /// "CLOUD_SQL"
    #[serde(rename="CLOUD_SQL")]
    CLOUDSQL,
    

    /// Looker
    ///
    /// "LOOKER"
    #[serde(rename="LOOKER")]
    LOOKER,
    

    /// Vertex AI
    ///
    /// "VERTEX_AI"
    #[serde(rename="VERTEX_AI")]
    VERTEXAI,
}

impl AsRef<str> for GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::INTEGRATEDSYSTEMUNSPECIFIED => "INTEGRATED_SYSTEM_UNSPECIFIED",
            GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::BIGQUERY => "BIGQUERY",
            GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::CLOUDPUBSUB => "CLOUD_PUBSUB",
            GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::DATAPROCMETASTORE => "DATAPROC_METASTORE",
            GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::DATAPLEX => "DATAPLEX",
            GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::CLOUDSPANNER => "CLOUD_SPANNER",
            GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::CLOUDBIGTABLE => "CLOUD_BIGTABLE",
            GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::CLOUDSQL => "CLOUD_SQL",
            GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::LOOKER => "LOOKER",
            GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::VERTEXAI => "VERTEX_AI",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTEGRATED_SYSTEM_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::INTEGRATEDSYSTEMUNSPECIFIED),
           "BIGQUERY" => Ok(GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::BIGQUERY),
           "CLOUD_PUBSUB" => Ok(GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::CLOUDPUBSUB),
           "DATAPROC_METASTORE" => Ok(GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::DATAPROCMETASTORE),
           "DATAPLEX" => Ok(GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::DATAPLEX),
           "CLOUD_SPANNER" => Ok(GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::CLOUDSPANNER),
           "CLOUD_BIGTABLE" => Ok(GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::CLOUDBIGTABLE),
           "CLOUD_SQL" => Ok(GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::CLOUDSQL),
           "LOOKER" => Ok(GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::LOOKER),
           "VERTEX_AI" => Ok(GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum::VERTEXAI),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1DataplexExternalTableSystemEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1EntryIntegratedSystemEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Indicates the entry's source system that Data Catalog integrates with, such as BigQuery, Pub/Sub, or Dataproc Metastore.
pub enum GoogleCloudDatacatalogV1EntryIntegratedSystemEnum {
    

    /// Default unknown system.
    ///
    /// "INTEGRATED_SYSTEM_UNSPECIFIED"
    #[serde(rename="INTEGRATED_SYSTEM_UNSPECIFIED")]
    INTEGRATEDSYSTEMUNSPECIFIED,
    

    /// BigQuery.
    ///
    /// "BIGQUERY"
    #[serde(rename="BIGQUERY")]
    BIGQUERY,
    

    /// Cloud Pub/Sub.
    ///
    /// "CLOUD_PUBSUB"
    #[serde(rename="CLOUD_PUBSUB")]
    CLOUDPUBSUB,
    

    /// Dataproc Metastore.
    ///
    /// "DATAPROC_METASTORE"
    #[serde(rename="DATAPROC_METASTORE")]
    DATAPROCMETASTORE,
    

    /// Dataplex.
    ///
    /// "DATAPLEX"
    #[serde(rename="DATAPLEX")]
    DATAPLEX,
    

    /// Cloud Spanner
    ///
    /// "CLOUD_SPANNER"
    #[serde(rename="CLOUD_SPANNER")]
    CLOUDSPANNER,
    

    /// Cloud Bigtable
    ///
    /// "CLOUD_BIGTABLE"
    #[serde(rename="CLOUD_BIGTABLE")]
    CLOUDBIGTABLE,
    

    /// Cloud Sql
    ///
    /// "CLOUD_SQL"
    #[serde(rename="CLOUD_SQL")]
    CLOUDSQL,
    

    /// Looker
    ///
    /// "LOOKER"
    #[serde(rename="LOOKER")]
    LOOKER,
    

    /// Vertex AI
    ///
    /// "VERTEX_AI"
    #[serde(rename="VERTEX_AI")]
    VERTEXAI,
}

impl AsRef<str> for GoogleCloudDatacatalogV1EntryIntegratedSystemEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::INTEGRATEDSYSTEMUNSPECIFIED => "INTEGRATED_SYSTEM_UNSPECIFIED",
            GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::BIGQUERY => "BIGQUERY",
            GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::CLOUDPUBSUB => "CLOUD_PUBSUB",
            GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::DATAPROCMETASTORE => "DATAPROC_METASTORE",
            GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::DATAPLEX => "DATAPLEX",
            GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::CLOUDSPANNER => "CLOUD_SPANNER",
            GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::CLOUDBIGTABLE => "CLOUD_BIGTABLE",
            GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::CLOUDSQL => "CLOUD_SQL",
            GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::LOOKER => "LOOKER",
            GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::VERTEXAI => "VERTEX_AI",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1EntryIntegratedSystemEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTEGRATED_SYSTEM_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::INTEGRATEDSYSTEMUNSPECIFIED),
           "BIGQUERY" => Ok(GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::BIGQUERY),
           "CLOUD_PUBSUB" => Ok(GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::CLOUDPUBSUB),
           "DATAPROC_METASTORE" => Ok(GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::DATAPROCMETASTORE),
           "DATAPLEX" => Ok(GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::DATAPLEX),
           "CLOUD_SPANNER" => Ok(GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::CLOUDSPANNER),
           "CLOUD_BIGTABLE" => Ok(GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::CLOUDBIGTABLE),
           "CLOUD_SQL" => Ok(GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::CLOUDSQL),
           "LOOKER" => Ok(GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::LOOKER),
           "VERTEX_AI" => Ok(GoogleCloudDatacatalogV1EntryIntegratedSystemEnum::VERTEXAI),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1EntryIntegratedSystemEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1EntryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the entry. For details, see [`EntryType`](#entrytype).
pub enum GoogleCloudDatacatalogV1EntryTypeEnum {
    

    /// Default unknown type.
    ///
    /// "ENTRY_TYPE_UNSPECIFIED"
    #[serde(rename="ENTRY_TYPE_UNSPECIFIED")]
    ENTRYTYPEUNSPECIFIED,
    

    /// The entry type that has a GoogleSQL schema, including logical views.
    ///
    /// "TABLE"
    #[serde(rename="TABLE")]
    TABLE,
    

    /// The type of models. For more information, see [Supported models in BigQuery ML](/bigquery/docs/bqml-introduction#supported_models).
    ///
    /// "MODEL"
    #[serde(rename="MODEL")]
    MODEL,
    

    /// An entry type for streaming entries. For example, a Pub/Sub topic.
    ///
    /// "DATA_STREAM"
    #[serde(rename="DATA_STREAM")]
    DATASTREAM,
    

    /// An entry type for a set of files or objects. For example, a Cloud Storage fileset.
    ///
    /// "FILESET"
    #[serde(rename="FILESET")]
    FILESET,
    

    /// A group of servers that work together. For example, a Kafka cluster.
    ///
    /// "CLUSTER"
    #[serde(rename="CLUSTER")]
    CLUSTER,
    

    /// A database.
    ///
    /// "DATABASE"
    #[serde(rename="DATABASE")]
    DATABASE,
    

    /// Connection to a data source. For example, a BigQuery connection.
    ///
    /// "DATA_SOURCE_CONNECTION"
    #[serde(rename="DATA_SOURCE_CONNECTION")]
    DATASOURCECONNECTION,
    

    /// Routine, for example, a BigQuery routine.
    ///
    /// "ROUTINE"
    #[serde(rename="ROUTINE")]
    ROUTINE,
    

    /// A Dataplex lake.
    ///
    /// "LAKE"
    #[serde(rename="LAKE")]
    LAKE,
    

    /// A Dataplex zone.
    ///
    /// "ZONE"
    #[serde(rename="ZONE")]
    ZONE,
    

    /// A service, for example, a Dataproc Metastore service.
    ///
    /// "SERVICE"
    #[serde(rename="SERVICE")]
    SERVICE,
    

    /// Schema within a relational database.
    ///
    /// "DATABASE_SCHEMA"
    #[serde(rename="DATABASE_SCHEMA")]
    DATABASESCHEMA,
    

    /// A Dashboard, for example from Looker.
    ///
    /// "DASHBOARD"
    #[serde(rename="DASHBOARD")]
    DASHBOARD,
    

    /// A Looker Explore. For more information, see [Looker Explore API] (https://developers.looker.com/api/explorer/4.0/methods/LookmlModel/lookml_model_explore).
    ///
    /// "EXPLORE"
    #[serde(rename="EXPLORE")]
    EXPLORE,
    

    /// A Looker Look. For more information, see [Looker Look API] (https://developers.looker.com/api/explorer/4.0/methods/Look).
    ///
    /// "LOOK"
    #[serde(rename="LOOK")]
    LOOK,
    

    /// Feature Online Store resource in Vertex AI Feature Store.
    ///
    /// "FEATURE_ONLINE_STORE"
    #[serde(rename="FEATURE_ONLINE_STORE")]
    FEATUREONLINESTORE,
    

    /// Feature View resource in Vertex AI Feature Store.
    ///
    /// "FEATURE_VIEW"
    #[serde(rename="FEATURE_VIEW")]
    FEATUREVIEW,
    

    /// Feature Group resource in Vertex AI Feature Store.
    ///
    /// "FEATURE_GROUP"
    #[serde(rename="FEATURE_GROUP")]
    FEATUREGROUP,
}

impl AsRef<str> for GoogleCloudDatacatalogV1EntryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1EntryTypeEnum::ENTRYTYPEUNSPECIFIED => "ENTRY_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1EntryTypeEnum::TABLE => "TABLE",
            GoogleCloudDatacatalogV1EntryTypeEnum::MODEL => "MODEL",
            GoogleCloudDatacatalogV1EntryTypeEnum::DATASTREAM => "DATA_STREAM",
            GoogleCloudDatacatalogV1EntryTypeEnum::FILESET => "FILESET",
            GoogleCloudDatacatalogV1EntryTypeEnum::CLUSTER => "CLUSTER",
            GoogleCloudDatacatalogV1EntryTypeEnum::DATABASE => "DATABASE",
            GoogleCloudDatacatalogV1EntryTypeEnum::DATASOURCECONNECTION => "DATA_SOURCE_CONNECTION",
            GoogleCloudDatacatalogV1EntryTypeEnum::ROUTINE => "ROUTINE",
            GoogleCloudDatacatalogV1EntryTypeEnum::LAKE => "LAKE",
            GoogleCloudDatacatalogV1EntryTypeEnum::ZONE => "ZONE",
            GoogleCloudDatacatalogV1EntryTypeEnum::SERVICE => "SERVICE",
            GoogleCloudDatacatalogV1EntryTypeEnum::DATABASESCHEMA => "DATABASE_SCHEMA",
            GoogleCloudDatacatalogV1EntryTypeEnum::DASHBOARD => "DASHBOARD",
            GoogleCloudDatacatalogV1EntryTypeEnum::EXPLORE => "EXPLORE",
            GoogleCloudDatacatalogV1EntryTypeEnum::LOOK => "LOOK",
            GoogleCloudDatacatalogV1EntryTypeEnum::FEATUREONLINESTORE => "FEATURE_ONLINE_STORE",
            GoogleCloudDatacatalogV1EntryTypeEnum::FEATUREVIEW => "FEATURE_VIEW",
            GoogleCloudDatacatalogV1EntryTypeEnum::FEATUREGROUP => "FEATURE_GROUP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1EntryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTRY_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::ENTRYTYPEUNSPECIFIED),
           "TABLE" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::TABLE),
           "MODEL" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::MODEL),
           "DATA_STREAM" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::DATASTREAM),
           "FILESET" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::FILESET),
           "CLUSTER" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::CLUSTER),
           "DATABASE" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::DATABASE),
           "DATA_SOURCE_CONNECTION" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::DATASOURCECONNECTION),
           "ROUTINE" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::ROUTINE),
           "LAKE" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::LAKE),
           "ZONE" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::ZONE),
           "SERVICE" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::SERVICE),
           "DATABASE_SCHEMA" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::DATABASESCHEMA),
           "DASHBOARD" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::DASHBOARD),
           "EXPLORE" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::EXPLORE),
           "LOOK" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::LOOK),
           "FEATURE_ONLINE_STORE" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::FEATUREONLINESTORE),
           "FEATURE_VIEW" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::FEATUREVIEW),
           "FEATURE_GROUP" => Ok(GoogleCloudDatacatalogV1EntryTypeEnum::FEATUREGROUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1EntryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Type of underelaying storage for the FeatureOnlineStore.
pub enum GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum {
    

    /// Should not be used.
    ///
    /// "STORAGE_TYPE_UNSPECIFIED"
    #[serde(rename="STORAGE_TYPE_UNSPECIFIED")]
    STORAGETYPEUNSPECIFIED,
    

    /// Underlsying storgae is Bigtable.
    ///
    /// "BIGTABLE"
    #[serde(rename="BIGTABLE")]
    BIGTABLE,
    

    /// Underlaying is optimized online server (Lightning).
    ///
    /// "OPTIMIZED"
    #[serde(rename="OPTIMIZED")]
    OPTIMIZED,
}

impl AsRef<str> for GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum::STORAGETYPEUNSPECIFIED => "STORAGE_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum::BIGTABLE => "BIGTABLE",
            GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum::OPTIMIZED => "OPTIMIZED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STORAGE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum::STORAGETYPEUNSPECIFIED),
           "BIGTABLE" => Ok(GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum::BIGTABLE),
           "OPTIMIZED" => Ok(GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum::OPTIMIZED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1FeatureOnlineStoreSpecStorageTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Primitive types, such as string, boolean, etc.
pub enum GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum {
    

    /// The default invalid value for a type.
    ///
    /// "PRIMITIVE_TYPE_UNSPECIFIED"
    #[serde(rename="PRIMITIVE_TYPE_UNSPECIFIED")]
    PRIMITIVETYPEUNSPECIFIED,
    

    /// A double precision number.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// An UTF-8 string.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// A boolean value.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// A timestamp.
    ///
    /// "TIMESTAMP"
    #[serde(rename="TIMESTAMP")]
    TIMESTAMP,
    

    /// A Richtext description.
    ///
    /// "RICHTEXT"
    #[serde(rename="RICHTEXT")]
    RICHTEXT,
}

impl AsRef<str> for GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::PRIMITIVETYPEUNSPECIFIED => "PRIMITIVE_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::DOUBLE => "DOUBLE",
            GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::STRING => "STRING",
            GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::BOOL => "BOOL",
            GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::TIMESTAMP => "TIMESTAMP",
            GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::RICHTEXT => "RICHTEXT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIMITIVE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::PRIMITIVETYPEUNSPECIFIED),
           "DOUBLE" => Ok(GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::DOUBLE),
           "STRING" => Ok(GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::STRING),
           "BOOL" => Ok(GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::BOOL),
           "TIMESTAMP" => Ok(GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::TIMESTAMP),
           "RICHTEXT" => Ok(GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum::RICHTEXT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1FieldTypePrimitiveTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the routine.
pub enum GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum {
    

    /// Unspecified type.
    ///
    /// "ROUTINE_TYPE_UNSPECIFIED"
    #[serde(rename="ROUTINE_TYPE_UNSPECIFIED")]
    ROUTINETYPEUNSPECIFIED,
    

    /// Non-builtin permanent scalar function.
    ///
    /// "SCALAR_FUNCTION"
    #[serde(rename="SCALAR_FUNCTION")]
    SCALARFUNCTION,
    

    /// Stored procedure.
    ///
    /// "PROCEDURE"
    #[serde(rename="PROCEDURE")]
    PROCEDURE,
}

impl AsRef<str> for GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum::ROUTINETYPEUNSPECIFIED => "ROUTINE_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum::SCALARFUNCTION => "SCALAR_FUNCTION",
            GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum::PROCEDURE => "PROCEDURE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROUTINE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum::ROUTINETYPEUNSPECIFIED),
           "SCALAR_FUNCTION" => Ok(GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum::SCALARFUNCTION),
           "PROCEDURE" => Ok(GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum::PROCEDURE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1RoutineSpecRoutineTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether the argument is input or output.
pub enum GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum {
    

    /// Unspecified mode.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// The argument is input-only.
    ///
    /// "IN"
    #[serde(rename="IN")]
    IN,
    

    /// The argument is output-only.
    ///
    /// "OUT"
    #[serde(rename="OUT")]
    OUT,
    

    /// The argument is both an input and an output.
    ///
    /// "INOUT"
    #[serde(rename="INOUT")]
    INOUT,
}

impl AsRef<str> for GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum::IN => "IN",
            GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum::OUT => "OUT",
            GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum::INOUT => "INOUT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum::MODEUNSPECIFIED),
           "IN" => Ok(GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum::IN),
           "OUT" => Ok(GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum::OUT),
           "INOUT" => Ok(GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum::INOUT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1RoutineSpecArgumentModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The source system that Data Catalog automatically integrates with, such as BigQuery, Cloud Pub/Sub, or Dataproc Metastore.
pub enum GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum {
    

    /// Default unknown system.
    ///
    /// "INTEGRATED_SYSTEM_UNSPECIFIED"
    #[serde(rename="INTEGRATED_SYSTEM_UNSPECIFIED")]
    INTEGRATEDSYSTEMUNSPECIFIED,
    

    /// BigQuery.
    ///
    /// "BIGQUERY"
    #[serde(rename="BIGQUERY")]
    BIGQUERY,
    

    /// Cloud Pub/Sub.
    ///
    /// "CLOUD_PUBSUB"
    #[serde(rename="CLOUD_PUBSUB")]
    CLOUDPUBSUB,
    

    /// Dataproc Metastore.
    ///
    /// "DATAPROC_METASTORE"
    #[serde(rename="DATAPROC_METASTORE")]
    DATAPROCMETASTORE,
    

    /// Dataplex.
    ///
    /// "DATAPLEX"
    #[serde(rename="DATAPLEX")]
    DATAPLEX,
    

    /// Cloud Spanner
    ///
    /// "CLOUD_SPANNER"
    #[serde(rename="CLOUD_SPANNER")]
    CLOUDSPANNER,
    

    /// Cloud Bigtable
    ///
    /// "CLOUD_BIGTABLE"
    #[serde(rename="CLOUD_BIGTABLE")]
    CLOUDBIGTABLE,
    

    /// Cloud Sql
    ///
    /// "CLOUD_SQL"
    #[serde(rename="CLOUD_SQL")]
    CLOUDSQL,
    

    /// Looker
    ///
    /// "LOOKER"
    #[serde(rename="LOOKER")]
    LOOKER,
    

    /// Vertex AI
    ///
    /// "VERTEX_AI"
    #[serde(rename="VERTEX_AI")]
    VERTEXAI,
}

impl AsRef<str> for GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::INTEGRATEDSYSTEMUNSPECIFIED => "INTEGRATED_SYSTEM_UNSPECIFIED",
            GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::BIGQUERY => "BIGQUERY",
            GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::CLOUDPUBSUB => "CLOUD_PUBSUB",
            GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::DATAPROCMETASTORE => "DATAPROC_METASTORE",
            GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::DATAPLEX => "DATAPLEX",
            GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::CLOUDSPANNER => "CLOUD_SPANNER",
            GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::CLOUDBIGTABLE => "CLOUD_BIGTABLE",
            GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::CLOUDSQL => "CLOUD_SQL",
            GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::LOOKER => "LOOKER",
            GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::VERTEXAI => "VERTEX_AI",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTEGRATED_SYSTEM_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::INTEGRATEDSYSTEMUNSPECIFIED),
           "BIGQUERY" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::BIGQUERY),
           "CLOUD_PUBSUB" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::CLOUDPUBSUB),
           "DATAPROC_METASTORE" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::DATAPROCMETASTORE),
           "DATAPLEX" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::DATAPLEX),
           "CLOUD_SPANNER" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::CLOUDSPANNER),
           "CLOUD_BIGTABLE" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::CLOUDBIGTABLE),
           "CLOUD_SQL" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::CLOUDSQL),
           "LOOKER" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::LOOKER),
           "VERTEX_AI" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum::VERTEXAI),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1SearchCatalogResultIntegratedSystemEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the search result. You can use this field to determine which get method to call to fetch the full resource.
pub enum GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum {
    

    /// Default unknown type.
    ///
    /// "SEARCH_RESULT_TYPE_UNSPECIFIED"
    #[serde(rename="SEARCH_RESULT_TYPE_UNSPECIFIED")]
    SEARCHRESULTTYPEUNSPECIFIED,
    

    /// An Entry.
    ///
    /// "ENTRY"
    #[serde(rename="ENTRY")]
    ENTRY,
    

    /// A TagTemplate.
    ///
    /// "TAG_TEMPLATE"
    #[serde(rename="TAG_TEMPLATE")]
    TAGTEMPLATE,
    

    /// An EntryGroup.
    ///
    /// "ENTRY_GROUP"
    #[serde(rename="ENTRY_GROUP")]
    ENTRYGROUP,
}

impl AsRef<str> for GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum::SEARCHRESULTTYPEUNSPECIFIED => "SEARCH_RESULT_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum::ENTRY => "ENTRY",
            GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum::TAGTEMPLATE => "TAG_TEMPLATE",
            GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum::ENTRYGROUP => "ENTRY_GROUP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEARCH_RESULT_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum::SEARCHRESULTTYPEUNSPECIFIED),
           "ENTRY" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum::ENTRY),
           "TAG_TEMPLATE" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum::TAGTEMPLATE),
           "ENTRY_GROUP" => Ok(GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum::ENTRYGROUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1SearchCatalogResultSearchResultTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1SerializedTaxonomyActivatedPolicyTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of policy types that are activated per taxonomy.
pub enum GoogleCloudDatacatalogV1SerializedTaxonomyActivatedPolicyTypesEnum {
    

    /// Unspecified policy type.
    ///
    /// "POLICY_TYPE_UNSPECIFIED"
    #[serde(rename="POLICY_TYPE_UNSPECIFIED")]
    POLICYTYPEUNSPECIFIED,
    

    /// Fine-grained access control policy that enables access control on tagged sub-resources.
    ///
    /// "FINE_GRAINED_ACCESS_CONTROL"
    #[serde(rename="FINE_GRAINED_ACCESS_CONTROL")]
    FINEGRAINEDACCESSCONTROL,
}

impl AsRef<str> for GoogleCloudDatacatalogV1SerializedTaxonomyActivatedPolicyTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1SerializedTaxonomyActivatedPolicyTypesEnum::POLICYTYPEUNSPECIFIED => "POLICY_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1SerializedTaxonomyActivatedPolicyTypesEnum::FINEGRAINEDACCESSCONTROL => "FINE_GRAINED_ACCESS_CONTROL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1SerializedTaxonomyActivatedPolicyTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POLICY_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1SerializedTaxonomyActivatedPolicyTypesEnum::POLICYTYPEUNSPECIFIED),
           "FINE_GRAINED_ACCESS_CONTROL" => Ok(GoogleCloudDatacatalogV1SerializedTaxonomyActivatedPolicyTypesEnum::FINEGRAINEDACCESSCONTROL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1SerializedTaxonomyActivatedPolicyTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1TagTemplateDataplexTransferStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Transfer status of the TagTemplate
pub enum GoogleCloudDatacatalogV1TagTemplateDataplexTransferStatusEnum {
    

    /// Default value. TagTemplate and its tags are only visible and editable in DataCatalog.
    ///
    /// "DATAPLEX_TRANSFER_STATUS_UNSPECIFIED"
    #[serde(rename="DATAPLEX_TRANSFER_STATUS_UNSPECIFIED")]
    DATAPLEXTRANSFERSTATUSUNSPECIFIED,
    

    /// TagTemplate and its tags are auto-copied to Dataplex service. Visible in both services. Editable in DataCatalog, read-only in Dataplex.
    ///
    /// "MIGRATED"
    #[serde(rename="MIGRATED")]
    MIGRATED,
}

impl AsRef<str> for GoogleCloudDatacatalogV1TagTemplateDataplexTransferStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1TagTemplateDataplexTransferStatusEnum::DATAPLEXTRANSFERSTATUSUNSPECIFIED => "DATAPLEX_TRANSFER_STATUS_UNSPECIFIED",
            GoogleCloudDatacatalogV1TagTemplateDataplexTransferStatusEnum::MIGRATED => "MIGRATED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1TagTemplateDataplexTransferStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATAPLEX_TRANSFER_STATUS_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1TagTemplateDataplexTransferStatusEnum::DATAPLEXTRANSFERSTATUSUNSPECIFIED),
           "MIGRATED" => Ok(GoogleCloudDatacatalogV1TagTemplateDataplexTransferStatusEnum::MIGRATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1TagTemplateDataplexTransferStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1TaxonomyActivatedPolicyTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A list of policy types that are activated for this taxonomy. If not set, defaults to an empty list.
pub enum GoogleCloudDatacatalogV1TaxonomyActivatedPolicyTypesEnum {
    

    /// Unspecified policy type.
    ///
    /// "POLICY_TYPE_UNSPECIFIED"
    #[serde(rename="POLICY_TYPE_UNSPECIFIED")]
    POLICYTYPEUNSPECIFIED,
    

    /// Fine-grained access control policy that enables access control on tagged sub-resources.
    ///
    /// "FINE_GRAINED_ACCESS_CONTROL"
    #[serde(rename="FINE_GRAINED_ACCESS_CONTROL")]
    FINEGRAINEDACCESSCONTROL,
}

impl AsRef<str> for GoogleCloudDatacatalogV1TaxonomyActivatedPolicyTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1TaxonomyActivatedPolicyTypesEnum::POLICYTYPEUNSPECIFIED => "POLICY_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1TaxonomyActivatedPolicyTypesEnum::FINEGRAINEDACCESSCONTROL => "FINE_GRAINED_ACCESS_CONTROL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1TaxonomyActivatedPolicyTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POLICY_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1TaxonomyActivatedPolicyTypesEnum::POLICYTYPEUNSPECIFIED),
           "FINE_GRAINED_ACCESS_CONTROL" => Ok(GoogleCloudDatacatalogV1TaxonomyActivatedPolicyTypesEnum::FINEGRAINEDACCESSCONTROL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1TaxonomyActivatedPolicyTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1TaxonomyServiceNameEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Google Cloud service name.
pub enum GoogleCloudDatacatalogV1TaxonomyServiceNameEnum {
    

    /// Default value
    ///
    /// "MANAGING_SYSTEM_UNSPECIFIED"
    #[serde(rename="MANAGING_SYSTEM_UNSPECIFIED")]
    MANAGINGSYSTEMUNSPECIFIED,
    

    /// Dataplex.
    ///
    /// "MANAGING_SYSTEM_DATAPLEX"
    #[serde(rename="MANAGING_SYSTEM_DATAPLEX")]
    MANAGINGSYSTEMDATAPLEX,
    

    /// Other
    ///
    /// "MANAGING_SYSTEM_OTHER"
    #[serde(rename="MANAGING_SYSTEM_OTHER")]
    MANAGINGSYSTEMOTHER,
}

impl AsRef<str> for GoogleCloudDatacatalogV1TaxonomyServiceNameEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1TaxonomyServiceNameEnum::MANAGINGSYSTEMUNSPECIFIED => "MANAGING_SYSTEM_UNSPECIFIED",
            GoogleCloudDatacatalogV1TaxonomyServiceNameEnum::MANAGINGSYSTEMDATAPLEX => "MANAGING_SYSTEM_DATAPLEX",
            GoogleCloudDatacatalogV1TaxonomyServiceNameEnum::MANAGINGSYSTEMOTHER => "MANAGING_SYSTEM_OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1TaxonomyServiceNameEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGING_SYSTEM_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1TaxonomyServiceNameEnum::MANAGINGSYSTEMUNSPECIFIED),
           "MANAGING_SYSTEM_DATAPLEX" => Ok(GoogleCloudDatacatalogV1TaxonomyServiceNameEnum::MANAGINGSYSTEMDATAPLEX),
           "MANAGING_SYSTEM_OTHER" => Ok(GoogleCloudDatacatalogV1TaxonomyServiceNameEnum::MANAGINGSYSTEMOTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1TaxonomyServiceNameEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the dataset.
pub enum GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum {
    

    /// Should not be used.
    ///
    /// "DATA_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_TYPE_UNSPECIFIED")]
    DATATYPEUNSPECIFIED,
    

    /// Structured data dataset.
    ///
    /// "TABLE"
    #[serde(rename="TABLE")]
    TABLE,
    

    /// Image dataset which supports ImageClassification, ImageObjectDetection and ImageSegmentation problems.
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// Document dataset which supports TextClassification, TextExtraction and TextSentiment problems.
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// Video dataset which supports VideoClassification, VideoObjectTracking and VideoActionRecognition problems.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// Conversation dataset which supports conversation problems.
    ///
    /// "CONVERSATION"
    #[serde(rename="CONVERSATION")]
    CONVERSATION,
    

    /// TimeSeries dataset.
    ///
    /// "TIME_SERIES"
    #[serde(rename="TIME_SERIES")]
    TIMESERIES,
    

    /// Document dataset which supports DocumentAnnotation problems.
    ///
    /// "DOCUMENT"
    #[serde(rename="DOCUMENT")]
    DOCUMENT,
    

    /// TextToSpeech dataset which supports TextToSpeech problems.
    ///
    /// "TEXT_TO_SPEECH"
    #[serde(rename="TEXT_TO_SPEECH")]
    TEXTTOSPEECH,
    

    /// Translation dataset which supports Translation problems.
    ///
    /// "TRANSLATION"
    #[serde(rename="TRANSLATION")]
    TRANSLATION,
    

    /// Store Vision dataset which is used for HITL integration.
    ///
    /// "STORE_VISION"
    #[serde(rename="STORE_VISION")]
    STOREVISION,
    

    /// Enterprise Knowledge Graph dataset which is used for HITL labeling integration.
    ///
    /// "ENTERPRISE_KNOWLEDGE_GRAPH"
    #[serde(rename="ENTERPRISE_KNOWLEDGE_GRAPH")]
    ENTERPRISEKNOWLEDGEGRAPH,
    

    /// Text prompt dataset which supports Large Language Models.
    ///
    /// "TEXT_PROMPT"
    #[serde(rename="TEXT_PROMPT")]
    TEXTPROMPT,
}

impl AsRef<str> for GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::DATATYPEUNSPECIFIED => "DATA_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TABLE => "TABLE",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::IMAGE => "IMAGE",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TEXT => "TEXT",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::VIDEO => "VIDEO",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::CONVERSATION => "CONVERSATION",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TIMESERIES => "TIME_SERIES",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::DOCUMENT => "DOCUMENT",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TEXTTOSPEECH => "TEXT_TO_SPEECH",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TRANSLATION => "TRANSLATION",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::STOREVISION => "STORE_VISION",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::ENTERPRISEKNOWLEDGEGRAPH => "ENTERPRISE_KNOWLEDGE_GRAPH",
            GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TEXTPROMPT => "TEXT_PROMPT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::DATATYPEUNSPECIFIED),
           "TABLE" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TABLE),
           "IMAGE" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::IMAGE),
           "TEXT" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TEXT),
           "VIDEO" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::VIDEO),
           "CONVERSATION" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::CONVERSATION),
           "TIME_SERIES" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TIMESERIES),
           "DOCUMENT" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::DOCUMENT),
           "TEXT_TO_SPEECH" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TEXTTOSPEECH),
           "TRANSLATION" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TRANSLATION),
           "STORE_VISION" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::STOREVISION),
           "ENTERPRISE_KNOWLEDGE_GRAPH" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::ENTERPRISEKNOWLEDGEGRAPH),
           "TEXT_PROMPT" => Ok(GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum::TEXTPROMPT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1VertexDatasetSpecDataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the model source.
pub enum GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum {
    

    /// Should not be used.
    ///
    /// "MODEL_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="MODEL_SOURCE_TYPE_UNSPECIFIED")]
    MODELSOURCETYPEUNSPECIFIED,
    

    /// The Model is uploaded by automl training pipeline.
    ///
    /// "AUTOML"
    #[serde(rename="AUTOML")]
    AUTOML,
    

    /// The Model is uploaded by user or custom training pipeline.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
    

    /// The Model is registered and sync'ed from BigQuery ML.
    ///
    /// "BQML"
    #[serde(rename="BQML")]
    BQML,
    

    /// The Model is saved or tuned from Model Garden.
    ///
    /// "MODEL_GARDEN"
    #[serde(rename="MODEL_GARDEN")]
    MODELGARDEN,
}

impl AsRef<str> for GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum::MODELSOURCETYPEUNSPECIFIED => "MODEL_SOURCE_TYPE_UNSPECIFIED",
            GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum::AUTOML => "AUTOML",
            GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum::CUSTOM => "CUSTOM",
            GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum::BQML => "BQML",
            GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum::MODELGARDEN => "MODEL_GARDEN",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODEL_SOURCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum::MODELSOURCETYPEUNSPECIFIED),
           "AUTOML" => Ok(GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum::AUTOML),
           "CUSTOM" => Ok(GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum::CUSTOM),
           "BQML" => Ok(GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum::BQML),
           "MODEL_GARDEN" => Ok(GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum::MODELGARDEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatacatalogV1VertexModelSourceInfoSourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


