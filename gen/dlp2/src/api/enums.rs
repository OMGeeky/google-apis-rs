use super::*;



// region GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How to sample the data.
pub enum GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum {
    

    /// No sampling.
    ///
    /// "SAMPLE_METHOD_UNSPECIFIED"
    #[serde(rename="SAMPLE_METHOD_UNSPECIFIED")]
    SAMPLEMETHODUNSPECIFIED,
    

    /// Scan groups of rows in the order BigQuery provides (default). Multiple groups of rows may be scanned in parallel, so results may not appear in the same order the rows are read.
    ///
    /// "TOP"
    #[serde(rename="TOP")]
    TOP,
    

    /// Randomly pick groups of rows to scan.
    ///
    /// "RANDOM_START"
    #[serde(rename="RANDOM_START")]
    RANDOMSTART,
}

impl AsRef<str> for GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum::SAMPLEMETHODUNSPECIFIED => "SAMPLE_METHOD_UNSPECIFIED",
            GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum::TOP => "TOP",
            GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum::RANDOMSTART => "RANDOM_START",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SAMPLE_METHOD_UNSPECIFIED" => Ok(GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum::SAMPLEMETHODUNSPECIFIED),
           "TOP" => Ok(GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum::TOP),
           "RANDOM_START" => Ok(GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum::RANDOMSTART),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2BigQueryOptionSampleMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2BigQueryTableTypeTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A set of BigQuery table types.
pub enum GooglePrivacyDlpV2BigQueryTableTypeTypesEnum {
    

    /// Unused.
    ///
    /// "BIG_QUERY_TABLE_TYPE_UNSPECIFIED"
    #[serde(rename="BIG_QUERY_TABLE_TYPE_UNSPECIFIED")]
    BIGQUERYTABLETYPEUNSPECIFIED,
    

    /// A normal BigQuery table.
    ///
    /// "BIG_QUERY_TABLE_TYPE_TABLE"
    #[serde(rename="BIG_QUERY_TABLE_TYPE_TABLE")]
    BIGQUERYTABLETYPETABLE,
    

    /// A table that references data stored in Cloud Storage.
    ///
    /// "BIG_QUERY_TABLE_TYPE_EXTERNAL_BIG_LAKE"
    #[serde(rename="BIG_QUERY_TABLE_TYPE_EXTERNAL_BIG_LAKE")]
    BIGQUERYTABLETYPEEXTERNALBIGLAKE,
}

impl AsRef<str> for GooglePrivacyDlpV2BigQueryTableTypeTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2BigQueryTableTypeTypesEnum::BIGQUERYTABLETYPEUNSPECIFIED => "BIG_QUERY_TABLE_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2BigQueryTableTypeTypesEnum::BIGQUERYTABLETYPETABLE => "BIG_QUERY_TABLE_TYPE_TABLE",
            GooglePrivacyDlpV2BigQueryTableTypeTypesEnum::BIGQUERYTABLETYPEEXTERNALBIGLAKE => "BIG_QUERY_TABLE_TYPE_EXTERNAL_BIG_LAKE",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2BigQueryTableTypeTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BIG_QUERY_TABLE_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2BigQueryTableTypeTypesEnum::BIGQUERYTABLETYPEUNSPECIFIED),
           "BIG_QUERY_TABLE_TYPE_TABLE" => Ok(GooglePrivacyDlpV2BigQueryTableTypeTypesEnum::BIGQUERYTABLETYPETABLE),
           "BIG_QUERY_TABLE_TYPE_EXTERNAL_BIG_LAKE" => Ok(GooglePrivacyDlpV2BigQueryTableTypeTypesEnum::BIGQUERYTABLETYPEEXTERNALBIGLAKE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2BigQueryTableTypeTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ByteContentItemTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of data stored in the bytes string. Default will be TEXT_UTF8.
pub enum GooglePrivacyDlpV2ByteContentItemTypeEnum {
    

    /// Unused
    ///
    /// "BYTES_TYPE_UNSPECIFIED"
    #[serde(rename="BYTES_TYPE_UNSPECIFIED")]
    BYTESTYPEUNSPECIFIED,
    

    /// Any image type.
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// jpeg
    ///
    /// "IMAGE_JPEG"
    #[serde(rename="IMAGE_JPEG")]
    IMAGEJPEG,
    

    /// bmp
    ///
    /// "IMAGE_BMP"
    #[serde(rename="IMAGE_BMP")]
    IMAGEBMP,
    

    /// png
    ///
    /// "IMAGE_PNG"
    #[serde(rename="IMAGE_PNG")]
    IMAGEPNG,
    

    /// svg
    ///
    /// "IMAGE_SVG"
    #[serde(rename="IMAGE_SVG")]
    IMAGESVG,
    

    /// plain text
    ///
    /// "TEXT_UTF8"
    #[serde(rename="TEXT_UTF8")]
    TEXTUTF8,
    

    /// docx, docm, dotx, dotm
    ///
    /// "WORD_DOCUMENT"
    #[serde(rename="WORD_DOCUMENT")]
    WORDDOCUMENT,
    

    /// pdf
    ///
    /// "PDF"
    #[serde(rename="PDF")]
    PDF,
    

    /// pptx, pptm, potx, potm, pot
    ///
    /// "POWERPOINT_DOCUMENT"
    #[serde(rename="POWERPOINT_DOCUMENT")]
    POWERPOINTDOCUMENT,
    

    /// xlsx, xlsm, xltx, xltm
    ///
    /// "EXCEL_DOCUMENT"
    #[serde(rename="EXCEL_DOCUMENT")]
    EXCELDOCUMENT,
    

    /// avro
    ///
    /// "AVRO"
    #[serde(rename="AVRO")]
    AVRO,
    

    /// csv
    ///
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
    

    /// tsv
    ///
    /// "TSV"
    #[serde(rename="TSV")]
    TSV,
}

impl AsRef<str> for GooglePrivacyDlpV2ByteContentItemTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ByteContentItemTypeEnum::BYTESTYPEUNSPECIFIED => "BYTES_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::IMAGE => "IMAGE",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::IMAGEJPEG => "IMAGE_JPEG",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::IMAGEBMP => "IMAGE_BMP",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::IMAGEPNG => "IMAGE_PNG",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::IMAGESVG => "IMAGE_SVG",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::TEXTUTF8 => "TEXT_UTF8",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::WORDDOCUMENT => "WORD_DOCUMENT",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::PDF => "PDF",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::POWERPOINTDOCUMENT => "POWERPOINT_DOCUMENT",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::EXCELDOCUMENT => "EXCEL_DOCUMENT",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::AVRO => "AVRO",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::CSV => "CSV",
            GooglePrivacyDlpV2ByteContentItemTypeEnum::TSV => "TSV",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ByteContentItemTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BYTES_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::BYTESTYPEUNSPECIFIED),
           "IMAGE" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::IMAGE),
           "IMAGE_JPEG" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::IMAGEJPEG),
           "IMAGE_BMP" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::IMAGEBMP),
           "IMAGE_PNG" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::IMAGEPNG),
           "IMAGE_SVG" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::IMAGESVG),
           "TEXT_UTF8" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::TEXTUTF8),
           "WORD_DOCUMENT" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::WORDDOCUMENT),
           "PDF" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::PDF),
           "POWERPOINT_DOCUMENT" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::POWERPOINTDOCUMENT),
           "EXCEL_DOCUMENT" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::EXCELDOCUMENT),
           "AVRO" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::AVRO),
           "CSV" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::CSV),
           "TSV" => Ok(GooglePrivacyDlpV2ByteContentItemTypeEnum::TSV),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ByteContentItemTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Common characters to not transform when masking. Useful to avoid removing punctuation.
pub enum GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum {
    

    /// Unused.
    ///
    /// "COMMON_CHARS_TO_IGNORE_UNSPECIFIED"
    #[serde(rename="COMMON_CHARS_TO_IGNORE_UNSPECIFIED")]
    COMMONCHARSTOIGNOREUNSPECIFIED,
    

    /// 0-9
    ///
    /// "NUMERIC"
    #[serde(rename="NUMERIC")]
    NUMERIC,
    

    /// A-Z
    ///
    /// "ALPHA_UPPER_CASE"
    #[serde(rename="ALPHA_UPPER_CASE")]
    ALPHAUPPERCASE,
    

    /// a-z
    ///
    /// "ALPHA_LOWER_CASE"
    #[serde(rename="ALPHA_LOWER_CASE")]
    ALPHALOWERCASE,
    

    /// US Punctuation, one of !"#$%&'()*+,-./:;<=>?@[\]^_`{|}~
    ///
    /// "PUNCTUATION"
    #[serde(rename="PUNCTUATION")]
    PUNCTUATION,
    

    /// Whitespace character, one of [ \t\n\x0B\f\r]
    ///
    /// "WHITESPACE"
    #[serde(rename="WHITESPACE")]
    WHITESPACE,
}

impl AsRef<str> for GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::COMMONCHARSTOIGNOREUNSPECIFIED => "COMMON_CHARS_TO_IGNORE_UNSPECIFIED",
            GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::NUMERIC => "NUMERIC",
            GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::ALPHAUPPERCASE => "ALPHA_UPPER_CASE",
            GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::ALPHALOWERCASE => "ALPHA_LOWER_CASE",
            GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::PUNCTUATION => "PUNCTUATION",
            GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::WHITESPACE => "WHITESPACE",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMMON_CHARS_TO_IGNORE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::COMMONCHARSTOIGNOREUNSPECIFIED),
           "NUMERIC" => Ok(GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::NUMERIC),
           "ALPHA_UPPER_CASE" => Ok(GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::ALPHAUPPERCASE),
           "ALPHA_LOWER_CASE" => Ok(GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::ALPHALOWERCASE),
           "PUNCTUATION" => Ok(GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::PUNCTUATION),
           "WHITESPACE" => Ok(GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum::WHITESPACE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The database engine used by the Cloud SQL instance that this connection configures.
pub enum GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum {
    

    /// An engine that is not currently supported by SDP.
    ///
    /// "DATABASE_ENGINE_UNKNOWN"
    #[serde(rename="DATABASE_ENGINE_UNKNOWN")]
    DATABASEENGINEUNKNOWN,
    

    /// Cloud SQL for MySQL instance.
    ///
    /// "DATABASE_ENGINE_MYSQL"
    #[serde(rename="DATABASE_ENGINE_MYSQL")]
    DATABASEENGINEMYSQL,
    

    /// Cloud SQL for Postgres instance.
    ///
    /// "DATABASE_ENGINE_POSTGRES"
    #[serde(rename="DATABASE_ENGINE_POSTGRES")]
    DATABASEENGINEPOSTGRES,
}

impl AsRef<str> for GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum::DATABASEENGINEUNKNOWN => "DATABASE_ENGINE_UNKNOWN",
            GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum::DATABASEENGINEMYSQL => "DATABASE_ENGINE_MYSQL",
            GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum::DATABASEENGINEPOSTGRES => "DATABASE_ENGINE_POSTGRES",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENGINE_UNKNOWN" => Ok(GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum::DATABASEENGINEUNKNOWN),
           "DATABASE_ENGINE_MYSQL" => Ok(GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum::DATABASEENGINEMYSQL),
           "DATABASE_ENGINE_POSTGRES" => Ok(GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum::DATABASEENGINEPOSTGRES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2CloudSqlPropertyDatabaseEngineEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of file type groups to include in the scan. If empty, all files are scanned and available data format processors are applied. In addition, the binary content of the selected files is always scanned as well. Images are scanned only as binary if the specified region does not support image inspection and no file_types were specified. Image inspection is restricted to 'global', 'us', 'asia', and 'europe'.
pub enum GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum {
    

    /// Includes all files.
    ///
    /// "FILE_TYPE_UNSPECIFIED"
    #[serde(rename="FILE_TYPE_UNSPECIFIED")]
    FILETYPEUNSPECIFIED,
    

    /// Includes all file extensions not covered by another entry. Binary scanning attempts to convert the content of the file to utf_8 to scan the file. If you wish to avoid this fall back, specify one or more of the other file types in your storage scan.
    ///
    /// "BINARY_FILE"
    #[serde(rename="BINARY_FILE")]
    BINARYFILE,
    

    /// Included file extensions: asc,asp, aspx, brf, c, cc,cfm, cgi, cpp, csv, cxx, c++, cs, css, dart, dat, dot, eml,, epbub, ged, go, h, hh, hpp, hxx, h++, hs, html, htm, mkd, markdown, m, ml, mli, perl, pl, plist, pm, php, phtml, pht, properties, py, pyw, rb, rbw, rs, rss, rc, scala, sh, sql, swift, tex, shtml, shtm, xhtml, lhs, ics, ini, java, js, json, jsonl, kix, kml, ocaml, md, txt, text, tsv, vb, vcard, vcs, wml, xcodeproj, xml, xsl, xsd, yml, yaml.
    ///
    /// "TEXT_FILE"
    #[serde(rename="TEXT_FILE")]
    TEXTFILE,
    

    /// Included file extensions: bmp, gif, jpg, jpeg, jpe, png. Setting bytes_limit_per_file or bytes_limit_per_file_percent has no effect on image files. Image inspection is restricted to the `global`, `us`, `asia`, and `europe` regions.
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// Microsoft Word files larger than 30 MB will be scanned as binary files. Included file extensions: docx, dotx, docm, dotm. Setting `bytes_limit_per_file` or `bytes_limit_per_file_percent` has no effect on Word files.
    ///
    /// "WORD"
    #[serde(rename="WORD")]
    WORD,
    

    /// PDF files larger than 30 MB will be scanned as binary files. Included file extensions: pdf. Setting `bytes_limit_per_file` or `bytes_limit_per_file_percent` has no effect on PDF files.
    ///
    /// "PDF"
    #[serde(rename="PDF")]
    PDF,
    

    /// Included file extensions: avro
    ///
    /// "AVRO"
    #[serde(rename="AVRO")]
    AVRO,
    

    /// Included file extensions: csv
    ///
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
    

    /// Included file extensions: tsv
    ///
    /// "TSV"
    #[serde(rename="TSV")]
    TSV,
    

    /// Microsoft PowerPoint files larger than 30 MB will be scanned as binary files. Included file extensions: pptx, pptm, potx, potm, pot. Setting `bytes_limit_per_file` or `bytes_limit_per_file_percent` has no effect on PowerPoint files.
    ///
    /// "POWERPOINT"
    #[serde(rename="POWERPOINT")]
    POWERPOINT,
    

    /// Microsoft Excel files larger than 30 MB will be scanned as binary files. Included file extensions: xlsx, xlsm, xltx, xltm. Setting `bytes_limit_per_file` or `bytes_limit_per_file_percent` has no effect on Excel files.
    ///
    /// "EXCEL"
    #[serde(rename="EXCEL")]
    EXCEL,
}

impl AsRef<str> for GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::FILETYPEUNSPECIFIED => "FILE_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::BINARYFILE => "BINARY_FILE",
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::TEXTFILE => "TEXT_FILE",
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::IMAGE => "IMAGE",
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::WORD => "WORD",
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::PDF => "PDF",
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::AVRO => "AVRO",
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::CSV => "CSV",
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::TSV => "TSV",
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::POWERPOINT => "POWERPOINT",
            GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::EXCEL => "EXCEL",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILE_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::FILETYPEUNSPECIFIED),
           "BINARY_FILE" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::BINARYFILE),
           "TEXT_FILE" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::TEXTFILE),
           "IMAGE" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::IMAGE),
           "WORD" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::WORD),
           "PDF" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::PDF),
           "AVRO" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::AVRO),
           "CSV" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::CSV),
           "TSV" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::TSV),
           "POWERPOINT" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::POWERPOINT),
           "EXCEL" => Ok(GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum::EXCEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2CloudStorageOptionFileTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How to sample the data.
pub enum GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum {
    

    /// No sampling.
    ///
    /// "SAMPLE_METHOD_UNSPECIFIED"
    #[serde(rename="SAMPLE_METHOD_UNSPECIFIED")]
    SAMPLEMETHODUNSPECIFIED,
    

    /// Scan from the top (default).
    ///
    /// "TOP"
    #[serde(rename="TOP")]
    TOP,
    

    /// For each file larger than bytes_limit_per_file, randomly pick the offset to start scanning. The scanned bytes are contiguous.
    ///
    /// "RANDOM_START"
    #[serde(rename="RANDOM_START")]
    RANDOMSTART,
}

impl AsRef<str> for GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum::SAMPLEMETHODUNSPECIFIED => "SAMPLE_METHOD_UNSPECIFIED",
            GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum::TOP => "TOP",
            GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum::RANDOMSTART => "RANDOM_START",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SAMPLE_METHOD_UNSPECIFIED" => Ok(GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum::SAMPLEMETHODUNSPECIFIED),
           "TOP" => Ok(GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum::TOP),
           "RANDOM_START" => Ok(GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum::RANDOMSTART),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2CloudStorageOptionSampleMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The data type of a given column.
pub enum GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum {
    

    /// Invalid type.
    ///
    /// "COLUMN_DATA_TYPE_UNSPECIFIED"
    #[serde(rename="COLUMN_DATA_TYPE_UNSPECIFIED")]
    COLUMNDATATYPEUNSPECIFIED,
    

    /// Encoded as a string in decimal format.
    ///
    /// "TYPE_INT64"
    #[serde(rename="TYPE_INT64")]
    TYPEINT64,
    

    /// Encoded as a boolean "false" or "true".
    ///
    /// "TYPE_BOOL"
    #[serde(rename="TYPE_BOOL")]
    TYPEBOOL,
    

    /// Encoded as a number, or string "NaN", "Infinity" or "-Infinity".
    ///
    /// "TYPE_FLOAT64"
    #[serde(rename="TYPE_FLOAT64")]
    TYPEFLOAT64,
    

    /// Encoded as a string value.
    ///
    /// "TYPE_STRING"
    #[serde(rename="TYPE_STRING")]
    TYPESTRING,
    

    /// Encoded as a base64 string per RFC 4648, section 4.
    ///
    /// "TYPE_BYTES"
    #[serde(rename="TYPE_BYTES")]
    TYPEBYTES,
    

    /// Encoded as an RFC 3339 timestamp with mandatory "Z" time zone string: 1985-04-12T23:20:50.52Z
    ///
    /// "TYPE_TIMESTAMP"
    #[serde(rename="TYPE_TIMESTAMP")]
    TYPETIMESTAMP,
    

    /// Encoded as RFC 3339 full-date format string: 1985-04-12
    ///
    /// "TYPE_DATE"
    #[serde(rename="TYPE_DATE")]
    TYPEDATE,
    

    /// Encoded as RFC 3339 partial-time format string: 23:20:50.52
    ///
    /// "TYPE_TIME"
    #[serde(rename="TYPE_TIME")]
    TYPETIME,
    

    /// Encoded as RFC 3339 full-date "T" partial-time: 1985-04-12T23:20:50.52
    ///
    /// "TYPE_DATETIME"
    #[serde(rename="TYPE_DATETIME")]
    TYPEDATETIME,
    

    /// Encoded as WKT
    ///
    /// "TYPE_GEOGRAPHY"
    #[serde(rename="TYPE_GEOGRAPHY")]
    TYPEGEOGRAPHY,
    

    /// Encoded as a decimal string.
    ///
    /// "TYPE_NUMERIC"
    #[serde(rename="TYPE_NUMERIC")]
    TYPENUMERIC,
    

    /// Container of ordered fields, each with a type and field name.
    ///
    /// "TYPE_RECORD"
    #[serde(rename="TYPE_RECORD")]
    TYPERECORD,
    

    /// Decimal type.
    ///
    /// "TYPE_BIGNUMERIC"
    #[serde(rename="TYPE_BIGNUMERIC")]
    TYPEBIGNUMERIC,
    

    /// Json type.
    ///
    /// "TYPE_JSON"
    #[serde(rename="TYPE_JSON")]
    TYPEJSON,
    

    /// Interval type.
    ///
    /// "TYPE_INTERVAL"
    #[serde(rename="TYPE_INTERVAL")]
    TYPEINTERVAL,
    

    /// Range type.
    ///
    /// "TYPE_RANGE_DATE"
    #[serde(rename="TYPE_RANGE_DATE")]
    TYPERANGEDATE,
    

    /// Range type.
    ///
    /// "TYPE_RANGE_DATETIME"
    #[serde(rename="TYPE_RANGE_DATETIME")]
    TYPERANGEDATETIME,
    

    /// Range type.
    ///
    /// "TYPE_RANGE_TIMESTAMP"
    #[serde(rename="TYPE_RANGE_TIMESTAMP")]
    TYPERANGETIMESTAMP,
}

impl AsRef<str> for GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::COLUMNDATATYPEUNSPECIFIED => "COLUMN_DATA_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEINT64 => "TYPE_INT64",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEBOOL => "TYPE_BOOL",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEFLOAT64 => "TYPE_FLOAT64",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPESTRING => "TYPE_STRING",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEBYTES => "TYPE_BYTES",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPETIMESTAMP => "TYPE_TIMESTAMP",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEDATE => "TYPE_DATE",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPETIME => "TYPE_TIME",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEDATETIME => "TYPE_DATETIME",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEGEOGRAPHY => "TYPE_GEOGRAPHY",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPENUMERIC => "TYPE_NUMERIC",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPERECORD => "TYPE_RECORD",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEBIGNUMERIC => "TYPE_BIGNUMERIC",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEJSON => "TYPE_JSON",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEINTERVAL => "TYPE_INTERVAL",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPERANGEDATE => "TYPE_RANGE_DATE",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPERANGEDATETIME => "TYPE_RANGE_DATETIME",
            GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPERANGETIMESTAMP => "TYPE_RANGE_TIMESTAMP",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COLUMN_DATA_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::COLUMNDATATYPEUNSPECIFIED),
           "TYPE_INT64" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEINT64),
           "TYPE_BOOL" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEBOOL),
           "TYPE_FLOAT64" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEFLOAT64),
           "TYPE_STRING" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPESTRING),
           "TYPE_BYTES" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEBYTES),
           "TYPE_TIMESTAMP" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPETIMESTAMP),
           "TYPE_DATE" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEDATE),
           "TYPE_TIME" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPETIME),
           "TYPE_DATETIME" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEDATETIME),
           "TYPE_GEOGRAPHY" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEGEOGRAPHY),
           "TYPE_NUMERIC" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPENUMERIC),
           "TYPE_RECORD" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPERECORD),
           "TYPE_BIGNUMERIC" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEBIGNUMERIC),
           "TYPE_JSON" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEJSON),
           "TYPE_INTERVAL" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPEINTERVAL),
           "TYPE_RANGE_DATE" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPERANGEDATE),
           "TYPE_RANGE_DATETIME" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPERANGEDATETIME),
           "TYPE_RANGE_TIMESTAMP" => Ok(GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum::TYPERANGETIMESTAMP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ColumnDataProfileColumnTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Approximate percentage of entries being null in the column.
pub enum GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum {
    

    /// Unused.
    ///
    /// "NULL_PERCENTAGE_LEVEL_UNSPECIFIED"
    #[serde(rename="NULL_PERCENTAGE_LEVEL_UNSPECIFIED")]
    NULLPERCENTAGELEVELUNSPECIFIED,
    

    /// Very few null entries.
    ///
    /// "NULL_PERCENTAGE_VERY_LOW"
    #[serde(rename="NULL_PERCENTAGE_VERY_LOW")]
    NULLPERCENTAGEVERYLOW,
    

    /// Some null entries.
    ///
    /// "NULL_PERCENTAGE_LOW"
    #[serde(rename="NULL_PERCENTAGE_LOW")]
    NULLPERCENTAGELOW,
    

    /// A few null entries.
    ///
    /// "NULL_PERCENTAGE_MEDIUM"
    #[serde(rename="NULL_PERCENTAGE_MEDIUM")]
    NULLPERCENTAGEMEDIUM,
    

    /// A lot of null entries.
    ///
    /// "NULL_PERCENTAGE_HIGH"
    #[serde(rename="NULL_PERCENTAGE_HIGH")]
    NULLPERCENTAGEHIGH,
}

impl AsRef<str> for GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum::NULLPERCENTAGELEVELUNSPECIFIED => "NULL_PERCENTAGE_LEVEL_UNSPECIFIED",
            GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum::NULLPERCENTAGEVERYLOW => "NULL_PERCENTAGE_VERY_LOW",
            GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum::NULLPERCENTAGELOW => "NULL_PERCENTAGE_LOW",
            GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum::NULLPERCENTAGEMEDIUM => "NULL_PERCENTAGE_MEDIUM",
            GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum::NULLPERCENTAGEHIGH => "NULL_PERCENTAGE_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NULL_PERCENTAGE_LEVEL_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum::NULLPERCENTAGELEVELUNSPECIFIED),
           "NULL_PERCENTAGE_VERY_LOW" => Ok(GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum::NULLPERCENTAGEVERYLOW),
           "NULL_PERCENTAGE_LOW" => Ok(GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum::NULLPERCENTAGELOW),
           "NULL_PERCENTAGE_MEDIUM" => Ok(GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum::NULLPERCENTAGEMEDIUM),
           "NULL_PERCENTAGE_HIGH" => Ok(GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum::NULLPERCENTAGEHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ColumnDataProfileEstimatedNullPercentageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Approximate uniqueness of the column.
pub enum GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum {
    

    /// Some columns do not have estimated uniqueness. Possible reasons include having too few values.
    ///
    /// "UNIQUENESS_SCORE_LEVEL_UNSPECIFIED"
    #[serde(rename="UNIQUENESS_SCORE_LEVEL_UNSPECIFIED")]
    UNIQUENESSSCORELEVELUNSPECIFIED,
    

    /// Low uniqueness, possibly a boolean, enum or similiarly typed column.
    ///
    /// "UNIQUENESS_SCORE_LOW"
    #[serde(rename="UNIQUENESS_SCORE_LOW")]
    UNIQUENESSSCORELOW,
    

    /// Medium uniqueness.
    ///
    /// "UNIQUENESS_SCORE_MEDIUM"
    #[serde(rename="UNIQUENESS_SCORE_MEDIUM")]
    UNIQUENESSSCOREMEDIUM,
    

    /// High uniqueness, possibly a column of free text or unique identifiers.
    ///
    /// "UNIQUENESS_SCORE_HIGH"
    #[serde(rename="UNIQUENESS_SCORE_HIGH")]
    UNIQUENESSSCOREHIGH,
}

impl AsRef<str> for GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum::UNIQUENESSSCORELEVELUNSPECIFIED => "UNIQUENESS_SCORE_LEVEL_UNSPECIFIED",
            GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum::UNIQUENESSSCORELOW => "UNIQUENESS_SCORE_LOW",
            GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum::UNIQUENESSSCOREMEDIUM => "UNIQUENESS_SCORE_MEDIUM",
            GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum::UNIQUENESSSCOREHIGH => "UNIQUENESS_SCORE_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNIQUENESS_SCORE_LEVEL_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum::UNIQUENESSSCORELEVELUNSPECIFIED),
           "UNIQUENESS_SCORE_LOW" => Ok(GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum::UNIQUENESSSCORELOW),
           "UNIQUENESS_SCORE_MEDIUM" => Ok(GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum::UNIQUENESSSCOREMEDIUM),
           "UNIQUENESS_SCORE_HIGH" => Ok(GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum::UNIQUENESSSCOREHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ColumnDataProfileEstimatedUniquenessScoreEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ColumnDataProfilePolicyStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates if a policy tag has been applied to the column.
pub enum GooglePrivacyDlpV2ColumnDataProfilePolicyStateEnum {
    

    /// No policy tags.
    ///
    /// "COLUMN_POLICY_STATE_UNSPECIFIED"
    #[serde(rename="COLUMN_POLICY_STATE_UNSPECIFIED")]
    COLUMNPOLICYSTATEUNSPECIFIED,
    

    /// Column has policy tag applied.
    ///
    /// "COLUMN_POLICY_TAGGED"
    #[serde(rename="COLUMN_POLICY_TAGGED")]
    COLUMNPOLICYTAGGED,
}

impl AsRef<str> for GooglePrivacyDlpV2ColumnDataProfilePolicyStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ColumnDataProfilePolicyStateEnum::COLUMNPOLICYSTATEUNSPECIFIED => "COLUMN_POLICY_STATE_UNSPECIFIED",
            GooglePrivacyDlpV2ColumnDataProfilePolicyStateEnum::COLUMNPOLICYTAGGED => "COLUMN_POLICY_TAGGED",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ColumnDataProfilePolicyStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COLUMN_POLICY_STATE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ColumnDataProfilePolicyStateEnum::COLUMNPOLICYSTATEUNSPECIFIED),
           "COLUMN_POLICY_TAGGED" => Ok(GooglePrivacyDlpV2ColumnDataProfilePolicyStateEnum::COLUMNPOLICYTAGGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ColumnDataProfilePolicyStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ColumnDataProfileStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of a profile.
pub enum GooglePrivacyDlpV2ColumnDataProfileStateEnum {
    

    /// Unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The profile is currently running. Once a profile has finished it will transition to DONE.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The profile is no longer generating. If profile_status.status.code is 0, the profile succeeded, otherwise, it failed.
    ///
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
}

impl AsRef<str> for GooglePrivacyDlpV2ColumnDataProfileStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ColumnDataProfileStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GooglePrivacyDlpV2ColumnDataProfileStateEnum::RUNNING => "RUNNING",
            GooglePrivacyDlpV2ColumnDataProfileStateEnum::DONE => "DONE",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ColumnDataProfileStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ColumnDataProfileStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(GooglePrivacyDlpV2ColumnDataProfileStateEnum::RUNNING),
           "DONE" => Ok(GooglePrivacyDlpV2ColumnDataProfileStateEnum::DONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ColumnDataProfileStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ConditionOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Operator used to compare the field or infoType to the value.
pub enum GooglePrivacyDlpV2ConditionOperatorEnum {
    

    /// Unused
    ///
    /// "RELATIONAL_OPERATOR_UNSPECIFIED"
    #[serde(rename="RELATIONAL_OPERATOR_UNSPECIFIED")]
    RELATIONALOPERATORUNSPECIFIED,
    

    /// Equal. Attempts to match even with incompatible types.
    ///
    /// "EQUAL_TO"
    #[serde(rename="EQUAL_TO")]
    EQUALTO,
    

    /// Not equal to. Attempts to match even with incompatible types.
    ///
    /// "NOT_EQUAL_TO"
    #[serde(rename="NOT_EQUAL_TO")]
    NOTEQUALTO,
    

    /// Greater than.
    ///
    /// "GREATER_THAN"
    #[serde(rename="GREATER_THAN")]
    GREATERTHAN,
    

    /// Less than.
    ///
    /// "LESS_THAN"
    #[serde(rename="LESS_THAN")]
    LESSTHAN,
    

    /// Greater than or equals.
    ///
    /// "GREATER_THAN_OR_EQUALS"
    #[serde(rename="GREATER_THAN_OR_EQUALS")]
    GREATERTHANOREQUALS,
    

    /// Less than or equals.
    ///
    /// "LESS_THAN_OR_EQUALS"
    #[serde(rename="LESS_THAN_OR_EQUALS")]
    LESSTHANOREQUALS,
    

    /// Exists
    ///
    /// "EXISTS"
    #[serde(rename="EXISTS")]
    EXISTS,
}

impl AsRef<str> for GooglePrivacyDlpV2ConditionOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ConditionOperatorEnum::RELATIONALOPERATORUNSPECIFIED => "RELATIONAL_OPERATOR_UNSPECIFIED",
            GooglePrivacyDlpV2ConditionOperatorEnum::EQUALTO => "EQUAL_TO",
            GooglePrivacyDlpV2ConditionOperatorEnum::NOTEQUALTO => "NOT_EQUAL_TO",
            GooglePrivacyDlpV2ConditionOperatorEnum::GREATERTHAN => "GREATER_THAN",
            GooglePrivacyDlpV2ConditionOperatorEnum::LESSTHAN => "LESS_THAN",
            GooglePrivacyDlpV2ConditionOperatorEnum::GREATERTHANOREQUALS => "GREATER_THAN_OR_EQUALS",
            GooglePrivacyDlpV2ConditionOperatorEnum::LESSTHANOREQUALS => "LESS_THAN_OR_EQUALS",
            GooglePrivacyDlpV2ConditionOperatorEnum::EXISTS => "EXISTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ConditionOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELATIONAL_OPERATOR_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ConditionOperatorEnum::RELATIONALOPERATORUNSPECIFIED),
           "EQUAL_TO" => Ok(GooglePrivacyDlpV2ConditionOperatorEnum::EQUALTO),
           "NOT_EQUAL_TO" => Ok(GooglePrivacyDlpV2ConditionOperatorEnum::NOTEQUALTO),
           "GREATER_THAN" => Ok(GooglePrivacyDlpV2ConditionOperatorEnum::GREATERTHAN),
           "LESS_THAN" => Ok(GooglePrivacyDlpV2ConditionOperatorEnum::LESSTHAN),
           "GREATER_THAN_OR_EQUALS" => Ok(GooglePrivacyDlpV2ConditionOperatorEnum::GREATERTHANOREQUALS),
           "LESS_THAN_OR_EQUALS" => Ok(GooglePrivacyDlpV2ConditionOperatorEnum::LESSTHANOREQUALS),
           "EXISTS" => Ok(GooglePrivacyDlpV2ConditionOperatorEnum::EXISTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ConditionOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ConnectionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The connection's state in its lifecycle.
pub enum GooglePrivacyDlpV2ConnectionStateEnum {
    

    /// Unused
    ///
    /// "CONNECTION_STATE_UNSPECIFIED"
    #[serde(rename="CONNECTION_STATE_UNSPECIFIED")]
    CONNECTIONSTATEUNSPECIFIED,
    

    /// DLP automatically created this connection during an initial scan, and it is awaiting full configuration by a user.
    ///
    /// "MISSING_CREDENTIALS"
    #[serde(rename="MISSING_CREDENTIALS")]
    MISSINGCREDENTIALS,
    

    /// A configured connection that has not encountered any errors.
    ///
    /// "AVAILABLE"
    #[serde(rename="AVAILABLE")]
    AVAILABLE,
    

    /// A configured connection that encountered errors during its last use. It will not be used again until it is set to AVAILABLE. If the resolution requires external action, then a request to set the status to AVAILABLE will mark this connection for use. Otherwise, any changes to the connection properties will automatically mark it as AVAILABLE.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for GooglePrivacyDlpV2ConnectionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ConnectionStateEnum::CONNECTIONSTATEUNSPECIFIED => "CONNECTION_STATE_UNSPECIFIED",
            GooglePrivacyDlpV2ConnectionStateEnum::MISSINGCREDENTIALS => "MISSING_CREDENTIALS",
            GooglePrivacyDlpV2ConnectionStateEnum::AVAILABLE => "AVAILABLE",
            GooglePrivacyDlpV2ConnectionStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ConnectionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTION_STATE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ConnectionStateEnum::CONNECTIONSTATEUNSPECIFIED),
           "MISSING_CREDENTIALS" => Ok(GooglePrivacyDlpV2ConnectionStateEnum::MISSINGCREDENTIALS),
           "AVAILABLE" => Ok(GooglePrivacyDlpV2ConnectionStateEnum::AVAILABLE),
           "ERROR" => Ok(GooglePrivacyDlpV2ConnectionStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ConnectionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Common alphabets.
pub enum GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
    

    /// Unused.
    ///
    /// "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED"
    #[serde(rename="FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED")]
    FFXCOMMONNATIVEALPHABETUNSPECIFIED,
    

    /// `[0-9]` (radix of 10)
    ///
    /// "NUMERIC"
    #[serde(rename="NUMERIC")]
    NUMERIC,
    

    /// `[0-9A-F]` (radix of 16)
    ///
    /// "HEXADECIMAL"
    #[serde(rename="HEXADECIMAL")]
    HEXADECIMAL,
    

    /// `[0-9A-Z]` (radix of 36)
    ///
    /// "UPPER_CASE_ALPHA_NUMERIC"
    #[serde(rename="UPPER_CASE_ALPHA_NUMERIC")]
    UPPERCASEALPHANUMERIC,
    

    /// `[0-9A-Za-z]` (radix of 62)
    ///
    /// "ALPHA_NUMERIC"
    #[serde(rename="ALPHA_NUMERIC")]
    ALPHANUMERIC,
}

impl AsRef<str> for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum::FFXCOMMONNATIVEALPHABETUNSPECIFIED => "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED",
            GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum::NUMERIC => "NUMERIC",
            GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum::HEXADECIMAL => "HEXADECIMAL",
            GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum::UPPERCASEALPHANUMERIC => "UPPER_CASE_ALPHA_NUMERIC",
            GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum::ALPHANUMERIC => "ALPHA_NUMERIC",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED" => Ok(GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum::FFXCOMMONNATIVEALPHABETUNSPECIFIED),
           "NUMERIC" => Ok(GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum::NUMERIC),
           "HEXADECIMAL" => Ok(GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum::HEXADECIMAL),
           "UPPER_CASE_ALPHA_NUMERIC" => Ok(GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum::UPPERCASEALPHANUMERIC),
           "ALPHA_NUMERIC" => Ok(GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum::ALPHANUMERIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching.
pub enum GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum {
    

    /// A finding of this custom info type will not be excluded from results.
    ///
    /// "EXCLUSION_TYPE_UNSPECIFIED"
    #[serde(rename="EXCLUSION_TYPE_UNSPECIFIED")]
    EXCLUSIONTYPEUNSPECIFIED,
    

    /// A finding of this custom info type will be excluded from final results, but can still affect rule execution.
    ///
    /// "EXCLUSION_TYPE_EXCLUDE"
    #[serde(rename="EXCLUSION_TYPE_EXCLUDE")]
    EXCLUSIONTYPEEXCLUDE,
}

impl AsRef<str> for GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum::EXCLUSIONTYPEUNSPECIFIED => "EXCLUSION_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum::EXCLUSIONTYPEEXCLUDE => "EXCLUSION_TYPE_EXCLUDE",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXCLUSION_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum::EXCLUSIONTYPEUNSPECIFIED),
           "EXCLUSION_TYPE_EXCLUDE" => Ok(GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum::EXCLUSIONTYPEEXCLUDE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Likelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria specified by the rule. Defaults to `VERY_LIKELY` if not specified.
pub enum GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum {
    

    /// Default value; same as POSSIBLE.
    ///
    /// "LIKELIHOOD_UNSPECIFIED"
    #[serde(rename="LIKELIHOOD_UNSPECIFIED")]
    LIKELIHOODUNSPECIFIED,
    

    /// Highest chance of a false positive.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// High chance of a false positive.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// Some matching signals. The default value.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// Low chance of a false positive.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// Confidence level is high. Lowest chance of a false positive.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::LIKELIHOODUNSPECIFIED => "LIKELIHOOD_UNSPECIFIED",
            GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::UNLIKELY => "UNLIKELY",
            GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::POSSIBLE => "POSSIBLE",
            GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::LIKELY => "LIKELY",
            GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIKELIHOOD_UNSPECIFIED" => Ok(GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::LIKELIHOODUNSPECIFIED),
           "VERY_UNLIKELY" => Ok(GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DataRiskLevelScoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The score applied to the resource.
pub enum GooglePrivacyDlpV2DataRiskLevelScoreEnum {
    

    /// Unused.
    ///
    /// "RISK_SCORE_UNSPECIFIED"
    #[serde(rename="RISK_SCORE_UNSPECIFIED")]
    RISKSCOREUNSPECIFIED,
    

    /// Low risk - Lower indication of sensitive data that appears to have additional access restrictions in place or no indication of sensitive data found.
    ///
    /// "RISK_LOW"
    #[serde(rename="RISK_LOW")]
    RISKLOW,
    

    /// Medium risk - Sensitive data may be present but additional access or fine grain access restrictions appear to be present. Consider limiting access even further or transform data to mask.
    ///
    /// "RISK_MODERATE"
    #[serde(rename="RISK_MODERATE")]
    RISKMODERATE,
    

    /// High risk – SPII may be present. Access controls may include public ACLs. Exfiltration of data may lead to user data loss. Re-identification of users may be possible. Consider limiting usage and or removing SPII.
    ///
    /// "RISK_HIGH"
    #[serde(rename="RISK_HIGH")]
    RISKHIGH,
}

impl AsRef<str> for GooglePrivacyDlpV2DataRiskLevelScoreEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DataRiskLevelScoreEnum::RISKSCOREUNSPECIFIED => "RISK_SCORE_UNSPECIFIED",
            GooglePrivacyDlpV2DataRiskLevelScoreEnum::RISKLOW => "RISK_LOW",
            GooglePrivacyDlpV2DataRiskLevelScoreEnum::RISKMODERATE => "RISK_MODERATE",
            GooglePrivacyDlpV2DataRiskLevelScoreEnum::RISKHIGH => "RISK_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DataRiskLevelScoreEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RISK_SCORE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DataRiskLevelScoreEnum::RISKSCOREUNSPECIFIED),
           "RISK_LOW" => Ok(GooglePrivacyDlpV2DataRiskLevelScoreEnum::RISKLOW),
           "RISK_MODERATE" => Ok(GooglePrivacyDlpV2DataRiskLevelScoreEnum::RISKMODERATE),
           "RISK_HIGH" => Ok(GooglePrivacyDlpV2DataRiskLevelScoreEnum::RISKHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DataRiskLevelScoreEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DateTimeDayOfWeekEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Day of week
pub enum GooglePrivacyDlpV2DateTimeDayOfWeekEnum {
    

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

impl AsRef<str> for GooglePrivacyDlpV2DateTimeDayOfWeekEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DateTimeDayOfWeekEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            GooglePrivacyDlpV2DateTimeDayOfWeekEnum::MONDAY => "MONDAY",
            GooglePrivacyDlpV2DateTimeDayOfWeekEnum::TUESDAY => "TUESDAY",
            GooglePrivacyDlpV2DateTimeDayOfWeekEnum::WEDNESDAY => "WEDNESDAY",
            GooglePrivacyDlpV2DateTimeDayOfWeekEnum::THURSDAY => "THURSDAY",
            GooglePrivacyDlpV2DateTimeDayOfWeekEnum::FRIDAY => "FRIDAY",
            GooglePrivacyDlpV2DateTimeDayOfWeekEnum::SATURDAY => "SATURDAY",
            GooglePrivacyDlpV2DateTimeDayOfWeekEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DateTimeDayOfWeekEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DateTimeDayOfWeekEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(GooglePrivacyDlpV2DateTimeDayOfWeekEnum::MONDAY),
           "TUESDAY" => Ok(GooglePrivacyDlpV2DateTimeDayOfWeekEnum::TUESDAY),
           "WEDNESDAY" => Ok(GooglePrivacyDlpV2DateTimeDayOfWeekEnum::WEDNESDAY),
           "THURSDAY" => Ok(GooglePrivacyDlpV2DateTimeDayOfWeekEnum::THURSDAY),
           "FRIDAY" => Ok(GooglePrivacyDlpV2DateTimeDayOfWeekEnum::FRIDAY),
           "SATURDAY" => Ok(GooglePrivacyDlpV2DateTimeDayOfWeekEnum::SATURDAY),
           "SUNDAY" => Ok(GooglePrivacyDlpV2DateTimeDayOfWeekEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DateTimeDayOfWeekEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of user-specified file type groups to transform. If specified, only the files with these filetypes will be transformed. If empty, all supported files will be transformed. Supported types may be automatically added over time. If a file type is set in this field that isn't supported by the Deidentify action then the job will fail and will not be successfully created/started. Currently the only filetypes supported are: IMAGES, TEXT_FILES, CSV, TSV.
pub enum GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum {
    

    /// Includes all files.
    ///
    /// "FILE_TYPE_UNSPECIFIED"
    #[serde(rename="FILE_TYPE_UNSPECIFIED")]
    FILETYPEUNSPECIFIED,
    

    /// Includes all file extensions not covered by another entry. Binary scanning attempts to convert the content of the file to utf_8 to scan the file. If you wish to avoid this fall back, specify one or more of the other file types in your storage scan.
    ///
    /// "BINARY_FILE"
    #[serde(rename="BINARY_FILE")]
    BINARYFILE,
    

    /// Included file extensions: asc,asp, aspx, brf, c, cc,cfm, cgi, cpp, csv, cxx, c++, cs, css, dart, dat, dot, eml,, epbub, ged, go, h, hh, hpp, hxx, h++, hs, html, htm, mkd, markdown, m, ml, mli, perl, pl, plist, pm, php, phtml, pht, properties, py, pyw, rb, rbw, rs, rss, rc, scala, sh, sql, swift, tex, shtml, shtm, xhtml, lhs, ics, ini, java, js, json, jsonl, kix, kml, ocaml, md, txt, text, tsv, vb, vcard, vcs, wml, xcodeproj, xml, xsl, xsd, yml, yaml.
    ///
    /// "TEXT_FILE"
    #[serde(rename="TEXT_FILE")]
    TEXTFILE,
    

    /// Included file extensions: bmp, gif, jpg, jpeg, jpe, png. Setting bytes_limit_per_file or bytes_limit_per_file_percent has no effect on image files. Image inspection is restricted to the `global`, `us`, `asia`, and `europe` regions.
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// Microsoft Word files larger than 30 MB will be scanned as binary files. Included file extensions: docx, dotx, docm, dotm. Setting `bytes_limit_per_file` or `bytes_limit_per_file_percent` has no effect on Word files.
    ///
    /// "WORD"
    #[serde(rename="WORD")]
    WORD,
    

    /// PDF files larger than 30 MB will be scanned as binary files. Included file extensions: pdf. Setting `bytes_limit_per_file` or `bytes_limit_per_file_percent` has no effect on PDF files.
    ///
    /// "PDF"
    #[serde(rename="PDF")]
    PDF,
    

    /// Included file extensions: avro
    ///
    /// "AVRO"
    #[serde(rename="AVRO")]
    AVRO,
    

    /// Included file extensions: csv
    ///
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
    

    /// Included file extensions: tsv
    ///
    /// "TSV"
    #[serde(rename="TSV")]
    TSV,
    

    /// Microsoft PowerPoint files larger than 30 MB will be scanned as binary files. Included file extensions: pptx, pptm, potx, potm, pot. Setting `bytes_limit_per_file` or `bytes_limit_per_file_percent` has no effect on PowerPoint files.
    ///
    /// "POWERPOINT"
    #[serde(rename="POWERPOINT")]
    POWERPOINT,
    

    /// Microsoft Excel files larger than 30 MB will be scanned as binary files. Included file extensions: xlsx, xlsm, xltx, xltm. Setting `bytes_limit_per_file` or `bytes_limit_per_file_percent` has no effect on Excel files.
    ///
    /// "EXCEL"
    #[serde(rename="EXCEL")]
    EXCEL,
}

impl AsRef<str> for GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::FILETYPEUNSPECIFIED => "FILE_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::BINARYFILE => "BINARY_FILE",
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::TEXTFILE => "TEXT_FILE",
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::IMAGE => "IMAGE",
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::WORD => "WORD",
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::PDF => "PDF",
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::AVRO => "AVRO",
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::CSV => "CSV",
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::TSV => "TSV",
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::POWERPOINT => "POWERPOINT",
            GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::EXCEL => "EXCEL",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILE_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::FILETYPEUNSPECIFIED),
           "BINARY_FILE" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::BINARYFILE),
           "TEXT_FILE" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::TEXTFILE),
           "IMAGE" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::IMAGE),
           "WORD" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::WORD),
           "PDF" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::PDF),
           "AVRO" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::AVRO),
           "CSV" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::CSV),
           "TSV" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::TSV),
           "POWERPOINT" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::POWERPOINT),
           "EXCEL" => Ok(GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum::EXCEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DeidentifyFileTypesToTransformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restrict discovery to categories of table types.
pub enum GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum {
    

    /// Unused.
    ///
    /// "BIG_QUERY_COLLECTION_UNSPECIFIED"
    #[serde(rename="BIG_QUERY_COLLECTION_UNSPECIFIED")]
    BIGQUERYCOLLECTIONUNSPECIFIED,
    

    /// Automatically generate profiles for all tables, even if the table type is not yet fully supported for analysis. Profiles for unsupported tables will be generated with errors to indicate their partial support. When full support is added, the tables will automatically be profiled during the next scheduled run.
    ///
    /// "BIG_QUERY_COLLECTION_ALL_TYPES"
    #[serde(rename="BIG_QUERY_COLLECTION_ALL_TYPES")]
    BIGQUERYCOLLECTIONALLTYPES,
    

    /// Only those types fully supported will be profiled. Will expand automatically as Cloud DLP adds support for new table types. Unsupported table types will not have partial profiles generated.
    ///
    /// "BIG_QUERY_COLLECTION_ONLY_SUPPORTED_TYPES"
    #[serde(rename="BIG_QUERY_COLLECTION_ONLY_SUPPORTED_TYPES")]
    BIGQUERYCOLLECTIONONLYSUPPORTEDTYPES,
}

impl AsRef<str> for GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum::BIGQUERYCOLLECTIONUNSPECIFIED => "BIG_QUERY_COLLECTION_UNSPECIFIED",
            GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum::BIGQUERYCOLLECTIONALLTYPES => "BIG_QUERY_COLLECTION_ALL_TYPES",
            GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum::BIGQUERYCOLLECTIONONLYSUPPORTEDTYPES => "BIG_QUERY_COLLECTION_ONLY_SUPPORTED_TYPES",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BIG_QUERY_COLLECTION_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum::BIGQUERYCOLLECTIONUNSPECIFIED),
           "BIG_QUERY_COLLECTION_ALL_TYPES" => Ok(GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum::BIGQUERYCOLLECTIONALLTYPES),
           "BIG_QUERY_COLLECTION_ONLY_SUPPORTED_TYPES" => Ok(GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum::BIGQUERYCOLLECTIONONLYSUPPORTEDTYPES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DiscoveryBigQueryConditionTypeCollectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Database engines that should be profiled. Optional. Defaults to ALL_SUPPORTED_DATABASE_ENGINES if unspecified.
pub enum GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum {
    

    /// Unused.
    ///
    /// "DATABASE_ENGINE_UNSPECIFIED"
    #[serde(rename="DATABASE_ENGINE_UNSPECIFIED")]
    DATABASEENGINEUNSPECIFIED,
    

    /// Include all supported database engines.
    ///
    /// "ALL_SUPPORTED_DATABASE_ENGINES"
    #[serde(rename="ALL_SUPPORTED_DATABASE_ENGINES")]
    ALLSUPPORTEDDATABASEENGINES,
    

    /// MySql database.
    ///
    /// "MYSQL"
    #[serde(rename="MYSQL")]
    MYSQL,
    

    /// PostGres database.
    ///
    /// "POSTGRES"
    #[serde(rename="POSTGRES")]
    POSTGRES,
}

impl AsRef<str> for GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum::DATABASEENGINEUNSPECIFIED => "DATABASE_ENGINE_UNSPECIFIED",
            GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum::ALLSUPPORTEDDATABASEENGINES => "ALL_SUPPORTED_DATABASE_ENGINES",
            GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum::MYSQL => "MYSQL",
            GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum::POSTGRES => "POSTGRES",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENGINE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum::DATABASEENGINEUNSPECIFIED),
           "ALL_SUPPORTED_DATABASE_ENGINES" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum::ALLSUPPORTEDDATABASEENGINES),
           "MYSQL" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum::MYSQL),
           "POSTGRES" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum::POSTGRES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DiscoveryCloudSqlConditionDatabaseEnginesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Data profiles will only be generated for the database resource types specified in this field. If not specified, defaults to [DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES].
pub enum GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum {
    

    /// Unused.
    ///
    /// "DATABASE_RESOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_RESOURCE_TYPE_UNSPECIFIED")]
    DATABASERESOURCETYPEUNSPECIFIED,
    

    /// Includes database resource types that become supported at a later time.
    ///
    /// "DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES"
    #[serde(rename="DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES")]
    DATABASERESOURCETYPEALLSUPPORTEDTYPES,
    

    /// Tables.
    ///
    /// "DATABASE_RESOURCE_TYPE_TABLE"
    #[serde(rename="DATABASE_RESOURCE_TYPE_TABLE")]
    DATABASERESOURCETYPETABLE,
}

impl AsRef<str> for GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum::DATABASERESOURCETYPEUNSPECIFIED => "DATABASE_RESOURCE_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum::DATABASERESOURCETYPEALLSUPPORTEDTYPES => "DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES",
            GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum::DATABASERESOURCETYPETABLE => "DATABASE_RESOURCE_TYPE_TABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_RESOURCE_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum::DATABASERESOURCETYPEUNSPECIFIED),
           "DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum::DATABASERESOURCETYPEALLSUPPORTEDTYPES),
           "DATABASE_RESOURCE_TYPE_TABLE" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum::DATABASERESOURCETYPETABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DiscoveryCloudSqlConditionTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Data changes (non-schema changes) in Cloud SQL tables can't trigger reprofiling. If you set this field, profiles are refreshed at this frequency regardless of whether the underlying tables have changes. Defaults to never.
pub enum GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum {
    

    /// Unspecified.
    ///
    /// "UPDATE_FREQUENCY_UNSPECIFIED"
    #[serde(rename="UPDATE_FREQUENCY_UNSPECIFIED")]
    UPDATEFREQUENCYUNSPECIFIED,
    

    /// After the data profile is created, it will never be updated.
    ///
    /// "UPDATE_FREQUENCY_NEVER"
    #[serde(rename="UPDATE_FREQUENCY_NEVER")]
    UPDATEFREQUENCYNEVER,
    

    /// The data profile can be updated up to once every 24 hours.
    ///
    /// "UPDATE_FREQUENCY_DAILY"
    #[serde(rename="UPDATE_FREQUENCY_DAILY")]
    UPDATEFREQUENCYDAILY,
    

    /// The data profile can be updated up to once every 30 days. Default.
    ///
    /// "UPDATE_FREQUENCY_MONTHLY"
    #[serde(rename="UPDATE_FREQUENCY_MONTHLY")]
    UPDATEFREQUENCYMONTHLY,
}

impl AsRef<str> for GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum::UPDATEFREQUENCYUNSPECIFIED => "UPDATE_FREQUENCY_UNSPECIFIED",
            GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum::UPDATEFREQUENCYNEVER => "UPDATE_FREQUENCY_NEVER",
            GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum::UPDATEFREQUENCYDAILY => "UPDATE_FREQUENCY_DAILY",
            GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum::UPDATEFREQUENCYMONTHLY => "UPDATE_FREQUENCY_MONTHLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UPDATE_FREQUENCY_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum::UPDATEFREQUENCYUNSPECIFIED),
           "UPDATE_FREQUENCY_NEVER" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum::UPDATEFREQUENCYNEVER),
           "UPDATE_FREQUENCY_DAILY" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum::UPDATEFREQUENCYDAILY),
           "UPDATE_FREQUENCY_MONTHLY" => Ok(GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum::UPDATEFREQUENCYMONTHLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DiscoveryCloudSqlGenerationCadenceRefreshFrequencyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DiscoveryConfigStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. A status for this configuration.
pub enum GooglePrivacyDlpV2DiscoveryConfigStatusEnum {
    

    /// Unused
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The discovery config is currently active.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The discovery config is paused temporarily.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
}

impl AsRef<str> for GooglePrivacyDlpV2DiscoveryConfigStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DiscoveryConfigStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            GooglePrivacyDlpV2DiscoveryConfigStatusEnum::RUNNING => "RUNNING",
            GooglePrivacyDlpV2DiscoveryConfigStatusEnum::PAUSED => "PAUSED",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DiscoveryConfigStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DiscoveryConfigStatusEnum::STATUSUNSPECIFIED),
           "RUNNING" => Ok(GooglePrivacyDlpV2DiscoveryConfigStatusEnum::RUNNING),
           "PAUSED" => Ok(GooglePrivacyDlpV2DiscoveryConfigStatusEnum::PAUSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DiscoveryConfigStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How frequently profiles may be updated when schemas are modified. Defaults to monthly.
pub enum GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum {
    

    /// Unspecified.
    ///
    /// "UPDATE_FREQUENCY_UNSPECIFIED"
    #[serde(rename="UPDATE_FREQUENCY_UNSPECIFIED")]
    UPDATEFREQUENCYUNSPECIFIED,
    

    /// After the data profile is created, it will never be updated.
    ///
    /// "UPDATE_FREQUENCY_NEVER"
    #[serde(rename="UPDATE_FREQUENCY_NEVER")]
    UPDATEFREQUENCYNEVER,
    

    /// The data profile can be updated up to once every 24 hours.
    ///
    /// "UPDATE_FREQUENCY_DAILY"
    #[serde(rename="UPDATE_FREQUENCY_DAILY")]
    UPDATEFREQUENCYDAILY,
    

    /// The data profile can be updated up to once every 30 days. Default.
    ///
    /// "UPDATE_FREQUENCY_MONTHLY"
    #[serde(rename="UPDATE_FREQUENCY_MONTHLY")]
    UPDATEFREQUENCYMONTHLY,
}

impl AsRef<str> for GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYUNSPECIFIED => "UPDATE_FREQUENCY_UNSPECIFIED",
            GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYNEVER => "UPDATE_FREQUENCY_NEVER",
            GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYDAILY => "UPDATE_FREQUENCY_DAILY",
            GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYMONTHLY => "UPDATE_FREQUENCY_MONTHLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UPDATE_FREQUENCY_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYUNSPECIFIED),
           "UPDATE_FREQUENCY_NEVER" => Ok(GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYNEVER),
           "UPDATE_FREQUENCY_DAILY" => Ok(GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYDAILY),
           "UPDATE_FREQUENCY_MONTHLY" => Ok(GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYMONTHLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceFrequencyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of events to consider when deciding if the table's schema has been modified and should have the profile updated. Defaults to NEW_COLUMNS.
pub enum GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum {
    

    /// Unused
    ///
    /// "SCHEMA_MODIFICATION_UNSPECIFIED"
    #[serde(rename="SCHEMA_MODIFICATION_UNSPECIFIED")]
    SCHEMAMODIFICATIONUNSPECIFIED,
    

    /// Profiles should be regenerated when new columns are added to the table. Default.
    ///
    /// "SCHEMA_NEW_COLUMNS"
    #[serde(rename="SCHEMA_NEW_COLUMNS")]
    SCHEMANEWCOLUMNS,
    

    /// Profiles should be regenerated when columns are removed from the table.
    ///
    /// "SCHEMA_REMOVED_COLUMNS"
    #[serde(rename="SCHEMA_REMOVED_COLUMNS")]
    SCHEMAREMOVEDCOLUMNS,
}

impl AsRef<str> for GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum::SCHEMAMODIFICATIONUNSPECIFIED => "SCHEMA_MODIFICATION_UNSPECIFIED",
            GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum::SCHEMANEWCOLUMNS => "SCHEMA_NEW_COLUMNS",
            GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum::SCHEMAREMOVEDCOLUMNS => "SCHEMA_REMOVED_COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCHEMA_MODIFICATION_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum::SCHEMAMODIFICATIONUNSPECIFIED),
           "SCHEMA_NEW_COLUMNS" => Ok(GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum::SCHEMANEWCOLUMNS),
           "SCHEMA_REMOVED_COLUMNS" => Ok(GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum::SCHEMAREMOVEDCOLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DiscoverySchemaModifiedCadenceTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How frequently data profiles can be updated when tables are modified. Defaults to never.
pub enum GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum {
    

    /// Unspecified.
    ///
    /// "UPDATE_FREQUENCY_UNSPECIFIED"
    #[serde(rename="UPDATE_FREQUENCY_UNSPECIFIED")]
    UPDATEFREQUENCYUNSPECIFIED,
    

    /// After the data profile is created, it will never be updated.
    ///
    /// "UPDATE_FREQUENCY_NEVER"
    #[serde(rename="UPDATE_FREQUENCY_NEVER")]
    UPDATEFREQUENCYNEVER,
    

    /// The data profile can be updated up to once every 24 hours.
    ///
    /// "UPDATE_FREQUENCY_DAILY"
    #[serde(rename="UPDATE_FREQUENCY_DAILY")]
    UPDATEFREQUENCYDAILY,
    

    /// The data profile can be updated up to once every 30 days. Default.
    ///
    /// "UPDATE_FREQUENCY_MONTHLY"
    #[serde(rename="UPDATE_FREQUENCY_MONTHLY")]
    UPDATEFREQUENCYMONTHLY,
}

impl AsRef<str> for GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum::UPDATEFREQUENCYUNSPECIFIED => "UPDATE_FREQUENCY_UNSPECIFIED",
            GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum::UPDATEFREQUENCYNEVER => "UPDATE_FREQUENCY_NEVER",
            GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum::UPDATEFREQUENCYDAILY => "UPDATE_FREQUENCY_DAILY",
            GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum::UPDATEFREQUENCYMONTHLY => "UPDATE_FREQUENCY_MONTHLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UPDATE_FREQUENCY_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum::UPDATEFREQUENCYUNSPECIFIED),
           "UPDATE_FREQUENCY_NEVER" => Ok(GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum::UPDATEFREQUENCYNEVER),
           "UPDATE_FREQUENCY_DAILY" => Ok(GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum::UPDATEFREQUENCYDAILY),
           "UPDATE_FREQUENCY_MONTHLY" => Ok(GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum::UPDATEFREQUENCYMONTHLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DiscoveryTableModifiedCadenceFrequencyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DiscoveryTableModifiedCadenceTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of events to consider when deciding if the table has been modified and should have the profile updated. Defaults to MODIFIED_TIMESTAMP.
pub enum GooglePrivacyDlpV2DiscoveryTableModifiedCadenceTypesEnum {
    

    /// Unused.
    ///
    /// "TABLE_MODIFICATION_UNSPECIFIED"
    #[serde(rename="TABLE_MODIFICATION_UNSPECIFIED")]
    TABLEMODIFICATIONUNSPECIFIED,
    

    /// A table will be considered modified when the last_modified_time from BigQuery has been updated.
    ///
    /// "TABLE_MODIFIED_TIMESTAMP"
    #[serde(rename="TABLE_MODIFIED_TIMESTAMP")]
    TABLEMODIFIEDTIMESTAMP,
}

impl AsRef<str> for GooglePrivacyDlpV2DiscoveryTableModifiedCadenceTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DiscoveryTableModifiedCadenceTypesEnum::TABLEMODIFICATIONUNSPECIFIED => "TABLE_MODIFICATION_UNSPECIFIED",
            GooglePrivacyDlpV2DiscoveryTableModifiedCadenceTypesEnum::TABLEMODIFIEDTIMESTAMP => "TABLE_MODIFIED_TIMESTAMP",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DiscoveryTableModifiedCadenceTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TABLE_MODIFICATION_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DiscoveryTableModifiedCadenceTypesEnum::TABLEMODIFICATIONUNSPECIFIED),
           "TABLE_MODIFIED_TIMESTAMP" => Ok(GooglePrivacyDlpV2DiscoveryTableModifiedCadenceTypesEnum::TABLEMODIFIEDTIMESTAMP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DiscoveryTableModifiedCadenceTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DlpJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of a job.
pub enum GooglePrivacyDlpV2DlpJobStateEnum {
    

    /// Unused.
    ///
    /// "JOB_STATE_UNSPECIFIED"
    #[serde(rename="JOB_STATE_UNSPECIFIED")]
    JOBSTATEUNSPECIFIED,
    

    /// The job has not yet started.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The job is currently running. Once a job has finished it will transition to FAILED or DONE.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The job is no longer running.
    ///
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
    

    /// The job was canceled before it could be completed.
    ///
    /// "CANCELED"
    #[serde(rename="CANCELED")]
    CANCELED,
    

    /// The job had an error and did not complete.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The job is currently accepting findings via hybridInspect. A hybrid job in ACTIVE state may continue to have findings added to it through the calling of hybridInspect. After the job has finished no more calls to hybridInspect may be made. ACTIVE jobs can transition to DONE.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for GooglePrivacyDlpV2DlpJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DlpJobStateEnum::JOBSTATEUNSPECIFIED => "JOB_STATE_UNSPECIFIED",
            GooglePrivacyDlpV2DlpJobStateEnum::PENDING => "PENDING",
            GooglePrivacyDlpV2DlpJobStateEnum::RUNNING => "RUNNING",
            GooglePrivacyDlpV2DlpJobStateEnum::DONE => "DONE",
            GooglePrivacyDlpV2DlpJobStateEnum::CANCELED => "CANCELED",
            GooglePrivacyDlpV2DlpJobStateEnum::FAILED => "FAILED",
            GooglePrivacyDlpV2DlpJobStateEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DlpJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_STATE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DlpJobStateEnum::JOBSTATEUNSPECIFIED),
           "PENDING" => Ok(GooglePrivacyDlpV2DlpJobStateEnum::PENDING),
           "RUNNING" => Ok(GooglePrivacyDlpV2DlpJobStateEnum::RUNNING),
           "DONE" => Ok(GooglePrivacyDlpV2DlpJobStateEnum::DONE),
           "CANCELED" => Ok(GooglePrivacyDlpV2DlpJobStateEnum::CANCELED),
           "FAILED" => Ok(GooglePrivacyDlpV2DlpJobStateEnum::FAILED),
           "ACTIVE" => Ok(GooglePrivacyDlpV2DlpJobStateEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DlpJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2DlpJobTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of job.
pub enum GooglePrivacyDlpV2DlpJobTypeEnum {
    

    /// Defaults to INSPECT_JOB.
    ///
    /// "DLP_JOB_TYPE_UNSPECIFIED"
    #[serde(rename="DLP_JOB_TYPE_UNSPECIFIED")]
    DLPJOBTYPEUNSPECIFIED,
    

    /// The job inspected Google Cloud for sensitive data.
    ///
    /// "INSPECT_JOB"
    #[serde(rename="INSPECT_JOB")]
    INSPECTJOB,
    

    /// The job executed a Risk Analysis computation.
    ///
    /// "RISK_ANALYSIS_JOB"
    #[serde(rename="RISK_ANALYSIS_JOB")]
    RISKANALYSISJOB,
}

impl AsRef<str> for GooglePrivacyDlpV2DlpJobTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2DlpJobTypeEnum::DLPJOBTYPEUNSPECIFIED => "DLP_JOB_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2DlpJobTypeEnum::INSPECTJOB => "INSPECT_JOB",
            GooglePrivacyDlpV2DlpJobTypeEnum::RISKANALYSISJOB => "RISK_ANALYSIS_JOB",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2DlpJobTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DLP_JOB_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2DlpJobTypeEnum::DLPJOBTYPEUNSPECIFIED),
           "INSPECT_JOB" => Ok(GooglePrivacyDlpV2DlpJobTypeEnum::INSPECTJOB),
           "RISK_ANALYSIS_JOB" => Ok(GooglePrivacyDlpV2DlpJobTypeEnum::RISKANALYSISJOB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2DlpJobTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the rule is applied, see MatchingType documentation for details.
pub enum GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum {
    

    /// Invalid.
    ///
    /// "MATCHING_TYPE_UNSPECIFIED"
    #[serde(rename="MATCHING_TYPE_UNSPECIFIED")]
    MATCHINGTYPEUNSPECIFIED,
    

    /// Full match. - Dictionary: join of Dictionary results matched complete finding quote - Regex: all regex matches fill a finding quote start to end - Exclude info type: completely inside affecting info types findings
    ///
    /// "MATCHING_TYPE_FULL_MATCH"
    #[serde(rename="MATCHING_TYPE_FULL_MATCH")]
    MATCHINGTYPEFULLMATCH,
    

    /// Partial match. - Dictionary: at least one of the tokens in the finding matches - Regex: substring of the finding matches - Exclude info type: intersects with affecting info types findings
    ///
    /// "MATCHING_TYPE_PARTIAL_MATCH"
    #[serde(rename="MATCHING_TYPE_PARTIAL_MATCH")]
    MATCHINGTYPEPARTIALMATCH,
    

    /// Inverse match. - Dictionary: no tokens in the finding match the dictionary - Regex: finding doesn't match the regex - Exclude info type: no intersection with affecting info types findings
    ///
    /// "MATCHING_TYPE_INVERSE_MATCH"
    #[serde(rename="MATCHING_TYPE_INVERSE_MATCH")]
    MATCHINGTYPEINVERSEMATCH,
}

impl AsRef<str> for GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum::MATCHINGTYPEUNSPECIFIED => "MATCHING_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum::MATCHINGTYPEFULLMATCH => "MATCHING_TYPE_FULL_MATCH",
            GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum::MATCHINGTYPEPARTIALMATCH => "MATCHING_TYPE_PARTIAL_MATCH",
            GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum::MATCHINGTYPEINVERSEMATCH => "MATCHING_TYPE_INVERSE_MATCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MATCHING_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum::MATCHINGTYPEUNSPECIFIED),
           "MATCHING_TYPE_FULL_MATCH" => Ok(GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum::MATCHINGTYPEFULLMATCH),
           "MATCHING_TYPE_PARTIAL_MATCH" => Ok(GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum::MATCHINGTYPEPARTIALMATCH),
           "MATCHING_TYPE_INVERSE_MATCH" => Ok(GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum::MATCHINGTYPEINVERSEMATCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ExpressionLogicalOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator to apply to the result of conditions. Default and currently only supported value is `AND`.
pub enum GooglePrivacyDlpV2ExpressionLogicalOperatorEnum {
    

    /// Unused
    ///
    /// "LOGICAL_OPERATOR_UNSPECIFIED"
    #[serde(rename="LOGICAL_OPERATOR_UNSPECIFIED")]
    LOGICALOPERATORUNSPECIFIED,
    

    /// Conditional AND
    ///
    /// "AND"
    #[serde(rename="AND")]
    AND,
}

impl AsRef<str> for GooglePrivacyDlpV2ExpressionLogicalOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ExpressionLogicalOperatorEnum::LOGICALOPERATORUNSPECIFIED => "LOGICAL_OPERATOR_UNSPECIFIED",
            GooglePrivacyDlpV2ExpressionLogicalOperatorEnum::AND => "AND",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ExpressionLogicalOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOGICAL_OPERATOR_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ExpressionLogicalOperatorEnum::LOGICALOPERATORUNSPECIFIED),
           "AND" => Ok(GooglePrivacyDlpV2ExpressionLogicalOperatorEnum::AND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ExpressionLogicalOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2FindingLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Confidence of how likely it is that the `info_type` is correct.
pub enum GooglePrivacyDlpV2FindingLikelihoodEnum {
    

    /// Default value; same as POSSIBLE.
    ///
    /// "LIKELIHOOD_UNSPECIFIED"
    #[serde(rename="LIKELIHOOD_UNSPECIFIED")]
    LIKELIHOODUNSPECIFIED,
    

    /// Highest chance of a false positive.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// High chance of a false positive.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// Some matching signals. The default value.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// Low chance of a false positive.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// Confidence level is high. Lowest chance of a false positive.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for GooglePrivacyDlpV2FindingLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2FindingLikelihoodEnum::LIKELIHOODUNSPECIFIED => "LIKELIHOOD_UNSPECIFIED",
            GooglePrivacyDlpV2FindingLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            GooglePrivacyDlpV2FindingLikelihoodEnum::UNLIKELY => "UNLIKELY",
            GooglePrivacyDlpV2FindingLikelihoodEnum::POSSIBLE => "POSSIBLE",
            GooglePrivacyDlpV2FindingLikelihoodEnum::LIKELY => "LIKELY",
            GooglePrivacyDlpV2FindingLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2FindingLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIKELIHOOD_UNSPECIFIED" => Ok(GooglePrivacyDlpV2FindingLikelihoodEnum::LIKELIHOODUNSPECIFIED),
           "VERY_UNLIKELY" => Ok(GooglePrivacyDlpV2FindingLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(GooglePrivacyDlpV2FindingLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(GooglePrivacyDlpV2FindingLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(GooglePrivacyDlpV2FindingLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(GooglePrivacyDlpV2FindingLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2FindingLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The group of relevant businesses where this infoType is commonly used
pub enum GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum {
    

    /// Unused industry
    ///
    /// "INDUSTRY_UNSPECIFIED"
    #[serde(rename="INDUSTRY_UNSPECIFIED")]
    INDUSTRYUNSPECIFIED,
    

    /// The infoType is typically used in the finance industry.
    ///
    /// "FINANCE"
    #[serde(rename="FINANCE")]
    FINANCE,
    

    /// The infoType is typically used in the health industry.
    ///
    /// "HEALTH"
    #[serde(rename="HEALTH")]
    HEALTH,
    

    /// The infoType is typically used in the telecommunications industry.
    ///
    /// "TELECOMMUNICATIONS"
    #[serde(rename="TELECOMMUNICATIONS")]
    TELECOMMUNICATIONS,
}

impl AsRef<str> for GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum::INDUSTRYUNSPECIFIED => "INDUSTRY_UNSPECIFIED",
            GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum::FINANCE => "FINANCE",
            GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum::HEALTH => "HEALTH",
            GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum::TELECOMMUNICATIONS => "TELECOMMUNICATIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INDUSTRY_UNSPECIFIED" => Ok(GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum::INDUSTRYUNSPECIFIED),
           "FINANCE" => Ok(GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum::FINANCE),
           "HEALTH" => Ok(GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum::HEALTH),
           "TELECOMMUNICATIONS" => Ok(GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum::TELECOMMUNICATIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2InfoTypeCategoryIndustryCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The region or country that issued the ID or document represented by the infoType.
pub enum GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum {
    

    /// Unused location
    ///
    /// "LOCATION_UNSPECIFIED"
    #[serde(rename="LOCATION_UNSPECIFIED")]
    LOCATIONUNSPECIFIED,
    

    /// The infoType is not issued by or tied to a specific region, but is used almost everywhere.
    ///
    /// "GLOBAL"
    #[serde(rename="GLOBAL")]
    GLOBAL,
    

    /// The infoType is typically used in Argentina.
    ///
    /// "ARGENTINA"
    #[serde(rename="ARGENTINA")]
    ARGENTINA,
    

    /// The infoType is typically used in Australia.
    ///
    /// "AUSTRALIA"
    #[serde(rename="AUSTRALIA")]
    AUSTRALIA,
    

    /// The infoType is typically used in Belgium.
    ///
    /// "BELGIUM"
    #[serde(rename="BELGIUM")]
    BELGIUM,
    

    /// The infoType is typically used in Brazil.
    ///
    /// "BRAZIL"
    #[serde(rename="BRAZIL")]
    BRAZIL,
    

    /// The infoType is typically used in Canada.
    ///
    /// "CANADA"
    #[serde(rename="CANADA")]
    CANADA,
    

    /// The infoType is typically used in Chile.
    ///
    /// "CHILE"
    #[serde(rename="CHILE")]
    CHILE,
    

    /// The infoType is typically used in China.
    ///
    /// "CHINA"
    #[serde(rename="CHINA")]
    CHINA,
    

    /// The infoType is typically used in Colombia.
    ///
    /// "COLOMBIA"
    #[serde(rename="COLOMBIA")]
    COLOMBIA,
    

    /// The infoType is typically used in Croatia.
    ///
    /// "CROATIA"
    #[serde(rename="CROATIA")]
    CROATIA,
    

    /// The infoType is typically used in Denmark.
    ///
    /// "DENMARK"
    #[serde(rename="DENMARK")]
    DENMARK,
    

    /// The infoType is typically used in France.
    ///
    /// "FRANCE"
    #[serde(rename="FRANCE")]
    FRANCE,
    

    /// The infoType is typically used in Finland.
    ///
    /// "FINLAND"
    #[serde(rename="FINLAND")]
    FINLAND,
    

    /// The infoType is typically used in Germany.
    ///
    /// "GERMANY"
    #[serde(rename="GERMANY")]
    GERMANY,
    

    /// The infoType is typically used in Hong Kong.
    ///
    /// "HONG_KONG"
    #[serde(rename="HONG_KONG")]
    HONGKONG,
    

    /// The infoType is typically used in India.
    ///
    /// "INDIA"
    #[serde(rename="INDIA")]
    INDIA,
    

    /// The infoType is typically used in Indonesia.
    ///
    /// "INDONESIA"
    #[serde(rename="INDONESIA")]
    INDONESIA,
    

    /// The infoType is typically used in Ireland.
    ///
    /// "IRELAND"
    #[serde(rename="IRELAND")]
    IRELAND,
    

    /// The infoType is typically used in Israel.
    ///
    /// "ISRAEL"
    #[serde(rename="ISRAEL")]
    ISRAEL,
    

    /// The infoType is typically used in Italy.
    ///
    /// "ITALY"
    #[serde(rename="ITALY")]
    ITALY,
    

    /// The infoType is typically used in Japan.
    ///
    /// "JAPAN"
    #[serde(rename="JAPAN")]
    JAPAN,
    

    /// The infoType is typically used in Korea.
    ///
    /// "KOREA"
    #[serde(rename="KOREA")]
    KOREA,
    

    /// The infoType is typically used in Mexico.
    ///
    /// "MEXICO"
    #[serde(rename="MEXICO")]
    MEXICO,
    

    /// The infoType is typically used in the Netherlands.
    ///
    /// "THE_NETHERLANDS"
    #[serde(rename="THE_NETHERLANDS")]
    THENETHERLANDS,
    

    /// The infoType is typically used in New Zealand.
    ///
    /// "NEW_ZEALAND"
    #[serde(rename="NEW_ZEALAND")]
    NEWZEALAND,
    

    /// The infoType is typically used in Norway.
    ///
    /// "NORWAY"
    #[serde(rename="NORWAY")]
    NORWAY,
    

    /// The infoType is typically used in Paraguay.
    ///
    /// "PARAGUAY"
    #[serde(rename="PARAGUAY")]
    PARAGUAY,
    

    /// The infoType is typically used in Peru.
    ///
    /// "PERU"
    #[serde(rename="PERU")]
    PERU,
    

    /// The infoType is typically used in Poland.
    ///
    /// "POLAND"
    #[serde(rename="POLAND")]
    POLAND,
    

    /// The infoType is typically used in Portugal.
    ///
    /// "PORTUGAL"
    #[serde(rename="PORTUGAL")]
    PORTUGAL,
    

    /// The infoType is typically used in Russia.
    ///
    /// "RUSSIA"
    #[serde(rename="RUSSIA")]
    RUSSIA,
    

    /// The infoType is typically used in Singapore.
    ///
    /// "SINGAPORE"
    #[serde(rename="SINGAPORE")]
    SINGAPORE,
    

    /// The infoType is typically used in South Africa.
    ///
    /// "SOUTH_AFRICA"
    #[serde(rename="SOUTH_AFRICA")]
    SOUTHAFRICA,
    

    /// The infoType is typically used in Spain.
    ///
    /// "SPAIN"
    #[serde(rename="SPAIN")]
    SPAIN,
    

    /// The infoType is typically used in Sweden.
    ///
    /// "SWEDEN"
    #[serde(rename="SWEDEN")]
    SWEDEN,
    

    /// The infoType is typically used in Switzerland.
    ///
    /// "SWITZERLAND"
    #[serde(rename="SWITZERLAND")]
    SWITZERLAND,
    

    /// The infoType is typically used in Taiwan.
    ///
    /// "TAIWAN"
    #[serde(rename="TAIWAN")]
    TAIWAN,
    

    /// The infoType is typically used in Thailand.
    ///
    /// "THAILAND"
    #[serde(rename="THAILAND")]
    THAILAND,
    

    /// The infoType is typically used in Turkey.
    ///
    /// "TURKEY"
    #[serde(rename="TURKEY")]
    TURKEY,
    

    /// The infoType is typically used in Ukraine.
    ///
    /// "UKRAINE"
    #[serde(rename="UKRAINE")]
    UKRAINE,
    

    /// The infoType is typically used in the United Kingdom.
    ///
    /// "UNITED_KINGDOM"
    #[serde(rename="UNITED_KINGDOM")]
    UNITEDKINGDOM,
    

    /// The infoType is typically used in the United States.
    ///
    /// "UNITED_STATES"
    #[serde(rename="UNITED_STATES")]
    UNITEDSTATES,
    

    /// The infoType is typically used in Uruguay.
    ///
    /// "URUGUAY"
    #[serde(rename="URUGUAY")]
    URUGUAY,
    

    /// The infoType is typically used in Uzbekistan.
    ///
    /// "UZBEKISTAN"
    #[serde(rename="UZBEKISTAN")]
    UZBEKISTAN,
    

    /// The infoType is typically used in Venezuela.
    ///
    /// "VENEZUELA"
    #[serde(rename="VENEZUELA")]
    VENEZUELA,
    

    /// The infoType is typically used in Google internally.
    ///
    /// "INTERNAL"
    #[serde(rename="INTERNAL")]
    INTERNAL,
}

impl AsRef<str> for GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::LOCATIONUNSPECIFIED => "LOCATION_UNSPECIFIED",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::GLOBAL => "GLOBAL",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::ARGENTINA => "ARGENTINA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::AUSTRALIA => "AUSTRALIA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::BELGIUM => "BELGIUM",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::BRAZIL => "BRAZIL",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::CANADA => "CANADA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::CHILE => "CHILE",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::CHINA => "CHINA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::COLOMBIA => "COLOMBIA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::CROATIA => "CROATIA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::DENMARK => "DENMARK",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::FRANCE => "FRANCE",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::FINLAND => "FINLAND",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::GERMANY => "GERMANY",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::HONGKONG => "HONG_KONG",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::INDIA => "INDIA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::INDONESIA => "INDONESIA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::IRELAND => "IRELAND",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::ISRAEL => "ISRAEL",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::ITALY => "ITALY",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::JAPAN => "JAPAN",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::KOREA => "KOREA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::MEXICO => "MEXICO",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::THENETHERLANDS => "THE_NETHERLANDS",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::NEWZEALAND => "NEW_ZEALAND",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::NORWAY => "NORWAY",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::PARAGUAY => "PARAGUAY",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::PERU => "PERU",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::POLAND => "POLAND",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::PORTUGAL => "PORTUGAL",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::RUSSIA => "RUSSIA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::SINGAPORE => "SINGAPORE",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::SOUTHAFRICA => "SOUTH_AFRICA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::SPAIN => "SPAIN",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::SWEDEN => "SWEDEN",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::SWITZERLAND => "SWITZERLAND",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::TAIWAN => "TAIWAN",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::THAILAND => "THAILAND",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::TURKEY => "TURKEY",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::UKRAINE => "UKRAINE",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::UNITEDKINGDOM => "UNITED_KINGDOM",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::UNITEDSTATES => "UNITED_STATES",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::URUGUAY => "URUGUAY",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::UZBEKISTAN => "UZBEKISTAN",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::VENEZUELA => "VENEZUELA",
            GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::INTERNAL => "INTERNAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOCATION_UNSPECIFIED" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::LOCATIONUNSPECIFIED),
           "GLOBAL" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::GLOBAL),
           "ARGENTINA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::ARGENTINA),
           "AUSTRALIA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::AUSTRALIA),
           "BELGIUM" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::BELGIUM),
           "BRAZIL" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::BRAZIL),
           "CANADA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::CANADA),
           "CHILE" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::CHILE),
           "CHINA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::CHINA),
           "COLOMBIA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::COLOMBIA),
           "CROATIA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::CROATIA),
           "DENMARK" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::DENMARK),
           "FRANCE" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::FRANCE),
           "FINLAND" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::FINLAND),
           "GERMANY" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::GERMANY),
           "HONG_KONG" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::HONGKONG),
           "INDIA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::INDIA),
           "INDONESIA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::INDONESIA),
           "IRELAND" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::IRELAND),
           "ISRAEL" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::ISRAEL),
           "ITALY" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::ITALY),
           "JAPAN" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::JAPAN),
           "KOREA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::KOREA),
           "MEXICO" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::MEXICO),
           "THE_NETHERLANDS" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::THENETHERLANDS),
           "NEW_ZEALAND" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::NEWZEALAND),
           "NORWAY" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::NORWAY),
           "PARAGUAY" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::PARAGUAY),
           "PERU" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::PERU),
           "POLAND" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::POLAND),
           "PORTUGAL" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::PORTUGAL),
           "RUSSIA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::RUSSIA),
           "SINGAPORE" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::SINGAPORE),
           "SOUTH_AFRICA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::SOUTHAFRICA),
           "SPAIN" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::SPAIN),
           "SWEDEN" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::SWEDEN),
           "SWITZERLAND" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::SWITZERLAND),
           "TAIWAN" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::TAIWAN),
           "THAILAND" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::THAILAND),
           "TURKEY" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::TURKEY),
           "UKRAINE" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::UKRAINE),
           "UNITED_KINGDOM" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::UNITEDKINGDOM),
           "UNITED_STATES" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::UNITEDSTATES),
           "URUGUAY" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::URUGUAY),
           "UZBEKISTAN" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::UZBEKISTAN),
           "VENEZUELA" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::VENEZUELA),
           "INTERNAL" => Ok(GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum::INTERNAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2InfoTypeCategoryLocationCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The class of identifiers where this infoType belongs
pub enum GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum {
    

    /// Unused type
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Personally identifiable information, for example, a name or phone number
    ///
    /// "PII"
    #[serde(rename="PII")]
    PII,
    

    /// Personally identifiable information that is especially sensitive, for example, a passport number.
    ///
    /// "SPII"
    #[serde(rename="SPII")]
    SPII,
    

    /// Attributes that can partially identify someone, especially in combination with other attributes, like age, height, and gender.
    ///
    /// "DEMOGRAPHIC"
    #[serde(rename="DEMOGRAPHIC")]
    DEMOGRAPHIC,
    

    /// Confidential or secret information, for example, a password.
    ///
    /// "CREDENTIAL"
    #[serde(rename="CREDENTIAL")]
    CREDENTIAL,
    

    /// An identification document issued by a government.
    ///
    /// "GOVERNMENT_ID"
    #[serde(rename="GOVERNMENT_ID")]
    GOVERNMENTID,
    

    /// A document, for example, a resume or source code.
    ///
    /// "DOCUMENT"
    #[serde(rename="DOCUMENT")]
    DOCUMENT,
    

    /// Information that is not sensitive on its own, but provides details about the circumstances surrounding an entity or an event.
    ///
    /// "CONTEXTUAL_INFORMATION"
    #[serde(rename="CONTEXTUAL_INFORMATION")]
    CONTEXTUALINFORMATION,
}

impl AsRef<str> for GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::PII => "PII",
            GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::SPII => "SPII",
            GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::DEMOGRAPHIC => "DEMOGRAPHIC",
            GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::CREDENTIAL => "CREDENTIAL",
            GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::GOVERNMENTID => "GOVERNMENT_ID",
            GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::DOCUMENT => "DOCUMENT",
            GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::CONTEXTUALINFORMATION => "CONTEXTUAL_INFORMATION",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::TYPEUNSPECIFIED),
           "PII" => Ok(GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::PII),
           "SPII" => Ok(GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::SPII),
           "DEMOGRAPHIC" => Ok(GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::DEMOGRAPHIC),
           "CREDENTIAL" => Ok(GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::CREDENTIAL),
           "GOVERNMENT_ID" => Ok(GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::GOVERNMENTID),
           "DOCUMENT" => Ok(GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::DOCUMENT),
           "CONTEXTUAL_INFORMATION" => Ok(GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum::CONTEXTUALINFORMATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2InfoTypeCategoryTypeCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Which parts of the API supports this InfoType.
pub enum GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum {
    

    /// Unused.
    ///
    /// "ENUM_TYPE_UNSPECIFIED"
    #[serde(rename="ENUM_TYPE_UNSPECIFIED")]
    ENUMTYPEUNSPECIFIED,
    

    /// Supported by the inspect operations.
    ///
    /// "INSPECT"
    #[serde(rename="INSPECT")]
    INSPECT,
    

    /// Supported by the risk analysis operations.
    ///
    /// "RISK_ANALYSIS"
    #[serde(rename="RISK_ANALYSIS")]
    RISKANALYSIS,
}

impl AsRef<str> for GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum::ENUMTYPEUNSPECIFIED => "ENUM_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum::INSPECT => "INSPECT",
            GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum::RISKANALYSIS => "RISK_ANALYSIS",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENUM_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum::ENUMTYPEUNSPECIFIED),
           "INSPECT" => Ok(GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum::INSPECT),
           "RISK_ANALYSIS" => Ok(GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum::RISKANALYSIS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Only returns findings equal to or above this threshold. This field is required or else the configuration fails.
pub enum GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum {
    

    /// Default value; same as POSSIBLE.
    ///
    /// "LIKELIHOOD_UNSPECIFIED"
    #[serde(rename="LIKELIHOOD_UNSPECIFIED")]
    LIKELIHOODUNSPECIFIED,
    

    /// Highest chance of a false positive.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// High chance of a false positive.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// Some matching signals. The default value.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// Low chance of a false positive.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// Confidence level is high. Lowest chance of a false positive.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::LIKELIHOODUNSPECIFIED => "LIKELIHOOD_UNSPECIFIED",
            GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::UNLIKELY => "UNLIKELY",
            GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::POSSIBLE => "POSSIBLE",
            GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::LIKELY => "LIKELY",
            GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIKELIHOOD_UNSPECIFIED" => Ok(GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::LIKELIHOODUNSPECIFIED),
           "VERY_UNLIKELY" => Ok(GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2InfoTypeLikelihoodMinLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2InspectConfigContentOptionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated and unused.
pub enum GooglePrivacyDlpV2InspectConfigContentOptionsEnum {
    

    /// Includes entire content of a file or a data stream.
    ///
    /// "CONTENT_UNSPECIFIED"
    #[serde(rename="CONTENT_UNSPECIFIED")]
    CONTENTUNSPECIFIED,
    

    /// Text content within the data, excluding any metadata.
    ///
    /// "CONTENT_TEXT"
    #[serde(rename="CONTENT_TEXT")]
    CONTENTTEXT,
    

    /// Images found in the data.
    ///
    /// "CONTENT_IMAGE"
    #[serde(rename="CONTENT_IMAGE")]
    CONTENTIMAGE,
}

impl AsRef<str> for GooglePrivacyDlpV2InspectConfigContentOptionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2InspectConfigContentOptionsEnum::CONTENTUNSPECIFIED => "CONTENT_UNSPECIFIED",
            GooglePrivacyDlpV2InspectConfigContentOptionsEnum::CONTENTTEXT => "CONTENT_TEXT",
            GooglePrivacyDlpV2InspectConfigContentOptionsEnum::CONTENTIMAGE => "CONTENT_IMAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2InspectConfigContentOptionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_UNSPECIFIED" => Ok(GooglePrivacyDlpV2InspectConfigContentOptionsEnum::CONTENTUNSPECIFIED),
           "CONTENT_TEXT" => Ok(GooglePrivacyDlpV2InspectConfigContentOptionsEnum::CONTENTTEXT),
           "CONTENT_IMAGE" => Ok(GooglePrivacyDlpV2InspectConfigContentOptionsEnum::CONTENTIMAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2InspectConfigContentOptionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Only returns findings equal to or above this threshold. The default is POSSIBLE. In general, the highest likelihood setting yields the fewest findings in results and the lowest chance of a false positive. For more information, see [Match likelihood](https://cloud.google.com/sensitive-data-protection/docs/likelihood).
pub enum GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum {
    

    /// Default value; same as POSSIBLE.
    ///
    /// "LIKELIHOOD_UNSPECIFIED"
    #[serde(rename="LIKELIHOOD_UNSPECIFIED")]
    LIKELIHOODUNSPECIFIED,
    

    /// Highest chance of a false positive.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// High chance of a false positive.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// Some matching signals. The default value.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// Low chance of a false positive.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// Confidence level is high. Lowest chance of a false positive.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::LIKELIHOODUNSPECIFIED => "LIKELIHOOD_UNSPECIFIED",
            GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::UNLIKELY => "UNLIKELY",
            GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::POSSIBLE => "POSSIBLE",
            GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::LIKELY => "LIKELY",
            GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIKELIHOOD_UNSPECIFIED" => Ok(GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::LIKELIHOODUNSPECIFIED),
           "VERY_UNLIKELY" => Ok(GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2JobTriggerStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. A status for this trigger.
pub enum GooglePrivacyDlpV2JobTriggerStatusEnum {
    

    /// Unused.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// Trigger is healthy.
    ///
    /// "HEALTHY"
    #[serde(rename="HEALTHY")]
    HEALTHY,
    

    /// Trigger is temporarily paused.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// Trigger is cancelled and can not be resumed.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for GooglePrivacyDlpV2JobTriggerStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2JobTriggerStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            GooglePrivacyDlpV2JobTriggerStatusEnum::HEALTHY => "HEALTHY",
            GooglePrivacyDlpV2JobTriggerStatusEnum::PAUSED => "PAUSED",
            GooglePrivacyDlpV2JobTriggerStatusEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2JobTriggerStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(GooglePrivacyDlpV2JobTriggerStatusEnum::STATUSUNSPECIFIED),
           "HEALTHY" => Ok(GooglePrivacyDlpV2JobTriggerStatusEnum::HEALTHY),
           "PAUSED" => Ok(GooglePrivacyDlpV2JobTriggerStatusEnum::PAUSED),
           "CANCELLED" => Ok(GooglePrivacyDlpV2JobTriggerStatusEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2JobTriggerStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Set the likelihood of a finding to a fixed value.
pub enum GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum {
    

    /// Default value; same as POSSIBLE.
    ///
    /// "LIKELIHOOD_UNSPECIFIED"
    #[serde(rename="LIKELIHOOD_UNSPECIFIED")]
    LIKELIHOODUNSPECIFIED,
    

    /// Highest chance of a false positive.
    ///
    /// "VERY_UNLIKELY"
    #[serde(rename="VERY_UNLIKELY")]
    VERYUNLIKELY,
    

    /// High chance of a false positive.
    ///
    /// "UNLIKELY"
    #[serde(rename="UNLIKELY")]
    UNLIKELY,
    

    /// Some matching signals. The default value.
    ///
    /// "POSSIBLE"
    #[serde(rename="POSSIBLE")]
    POSSIBLE,
    

    /// Low chance of a false positive.
    ///
    /// "LIKELY"
    #[serde(rename="LIKELY")]
    LIKELY,
    

    /// Confidence level is high. Lowest chance of a false positive.
    ///
    /// "VERY_LIKELY"
    #[serde(rename="VERY_LIKELY")]
    VERYLIKELY,
}

impl AsRef<str> for GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::LIKELIHOODUNSPECIFIED => "LIKELIHOOD_UNSPECIFIED",
            GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::VERYUNLIKELY => "VERY_UNLIKELY",
            GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::UNLIKELY => "UNLIKELY",
            GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::POSSIBLE => "POSSIBLE",
            GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::LIKELY => "LIKELY",
            GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::VERYLIKELY => "VERY_LIKELY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIKELIHOOD_UNSPECIFIED" => Ok(GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::LIKELIHOODUNSPECIFIED),
           "VERY_UNLIKELY" => Ok(GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::VERYUNLIKELY),
           "UNLIKELY" => Ok(GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::UNLIKELY),
           "POSSIBLE" => Ok(GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::POSSIBLE),
           "LIKELY" => Ok(GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::LIKELY),
           "VERY_LIKELY" => Ok(GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum::VERYLIKELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2MetadataLocationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of metadata containing the finding.
pub enum GooglePrivacyDlpV2MetadataLocationTypeEnum {
    

    /// Unused
    ///
    /// "METADATATYPE_UNSPECIFIED"
    #[serde(rename="METADATATYPE_UNSPECIFIED")]
    METADATATYPEUNSPECIFIED,
    

    /// General file metadata provided by Cloud Storage.
    ///
    /// "STORAGE_METADATA"
    #[serde(rename="STORAGE_METADATA")]
    STORAGEMETADATA,
}

impl AsRef<str> for GooglePrivacyDlpV2MetadataLocationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2MetadataLocationTypeEnum::METADATATYPEUNSPECIFIED => "METADATATYPE_UNSPECIFIED",
            GooglePrivacyDlpV2MetadataLocationTypeEnum::STORAGEMETADATA => "STORAGE_METADATA",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2MetadataLocationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METADATATYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2MetadataLocationTypeEnum::METADATATYPEUNSPECIFIED),
           "STORAGE_METADATA" => Ok(GooglePrivacyDlpV2MetadataLocationTypeEnum::STORAGEMETADATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2MetadataLocationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Schema used for writing the findings for Inspect jobs. This field is only used for Inspect and must be unspecified for Risk jobs. Columns are derived from the `Finding` object. If appending to an existing table, any columns from the predefined schema that are missing will be added. No columns in the existing table will be deleted. If unspecified, then all available columns will be used for a new table or an (existing) table with no schema, and no changes will be made to an existing table that has a schema. Only for use with external storage.
pub enum GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum {
    

    /// Unused.
    ///
    /// "OUTPUT_SCHEMA_UNSPECIFIED"
    #[serde(rename="OUTPUT_SCHEMA_UNSPECIFIED")]
    OUTPUTSCHEMAUNSPECIFIED,
    

    /// Basic schema including only `info_type`, `quote`, `certainty`, and `timestamp`.
    ///
    /// "BASIC_COLUMNS"
    #[serde(rename="BASIC_COLUMNS")]
    BASICCOLUMNS,
    

    /// Schema tailored to findings from scanning Cloud Storage.
    ///
    /// "GCS_COLUMNS"
    #[serde(rename="GCS_COLUMNS")]
    GCSCOLUMNS,
    

    /// Schema tailored to findings from scanning Google Datastore.
    ///
    /// "DATASTORE_COLUMNS"
    #[serde(rename="DATASTORE_COLUMNS")]
    DATASTORECOLUMNS,
    

    /// Schema tailored to findings from scanning Google BigQuery.
    ///
    /// "BIG_QUERY_COLUMNS"
    #[serde(rename="BIG_QUERY_COLUMNS")]
    BIGQUERYCOLUMNS,
    

    /// Schema containing all columns.
    ///
    /// "ALL_COLUMNS"
    #[serde(rename="ALL_COLUMNS")]
    ALLCOLUMNS,
}

impl AsRef<str> for GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::OUTPUTSCHEMAUNSPECIFIED => "OUTPUT_SCHEMA_UNSPECIFIED",
            GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::BASICCOLUMNS => "BASIC_COLUMNS",
            GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::GCSCOLUMNS => "GCS_COLUMNS",
            GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::DATASTORECOLUMNS => "DATASTORE_COLUMNS",
            GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::BIGQUERYCOLUMNS => "BIG_QUERY_COLUMNS",
            GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::ALLCOLUMNS => "ALL_COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OUTPUT_SCHEMA_UNSPECIFIED" => Ok(GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::OUTPUTSCHEMAUNSPECIFIED),
           "BASIC_COLUMNS" => Ok(GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::BASICCOLUMNS),
           "GCS_COLUMNS" => Ok(GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::GCSCOLUMNS),
           "DATASTORE_COLUMNS" => Ok(GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::DATASTORECOLUMNS),
           "BIG_QUERY_COLUMNS" => Ok(GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::BIGQUERYCOLUMNS),
           "ALL_COLUMNS" => Ok(GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum::ALLCOLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The minimum data risk score that triggers the condition.
pub enum GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum {
    

    /// Unused.
    ///
    /// "PROFILE_SCORE_BUCKET_UNSPECIFIED"
    #[serde(rename="PROFILE_SCORE_BUCKET_UNSPECIFIED")]
    PROFILESCOREBUCKETUNSPECIFIED,
    

    /// High risk/sensitivity detected.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Medium or high risk/sensitivity detected.
    ///
    /// "MEDIUM_OR_HIGH"
    #[serde(rename="MEDIUM_OR_HIGH")]
    MEDIUMORHIGH,
}

impl AsRef<str> for GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum::PROFILESCOREBUCKETUNSPECIFIED => "PROFILE_SCORE_BUCKET_UNSPECIFIED",
            GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum::HIGH => "HIGH",
            GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum::MEDIUMORHIGH => "MEDIUM_OR_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_SCORE_BUCKET_UNSPECIFIED" => Ok(GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum::PROFILESCOREBUCKETUNSPECIFIED),
           "HIGH" => Ok(GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum::HIGH),
           "MEDIUM_OR_HIGH" => Ok(GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum::MEDIUMORHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2PubSubConditionMinimumRiskScoreEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The minimum sensitivity level that triggers the condition.
pub enum GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum {
    

    /// Unused.
    ///
    /// "PROFILE_SCORE_BUCKET_UNSPECIFIED"
    #[serde(rename="PROFILE_SCORE_BUCKET_UNSPECIFIED")]
    PROFILESCOREBUCKETUNSPECIFIED,
    

    /// High risk/sensitivity detected.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Medium or high risk/sensitivity detected.
    ///
    /// "MEDIUM_OR_HIGH"
    #[serde(rename="MEDIUM_OR_HIGH")]
    MEDIUMORHIGH,
}

impl AsRef<str> for GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum::PROFILESCOREBUCKETUNSPECIFIED => "PROFILE_SCORE_BUCKET_UNSPECIFIED",
            GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum::HIGH => "HIGH",
            GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum::MEDIUMORHIGH => "MEDIUM_OR_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_SCORE_BUCKET_UNSPECIFIED" => Ok(GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum::PROFILESCOREBUCKETUNSPECIFIED),
           "HIGH" => Ok(GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum::HIGH),
           "MEDIUM_OR_HIGH" => Ok(GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum::MEDIUMORHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2PubSubConditionMinimumSensitivityScoreEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator to apply to the collection of conditions.
pub enum GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum {
    

    /// Unused.
    ///
    /// "LOGICAL_OPERATOR_UNSPECIFIED"
    #[serde(rename="LOGICAL_OPERATOR_UNSPECIFIED")]
    LOGICALOPERATORUNSPECIFIED,
    

    /// Conditional OR.
    ///
    /// "OR"
    #[serde(rename="OR")]
    OR,
    

    /// Conditional AND.
    ///
    /// "AND"
    #[serde(rename="AND")]
    AND,
}

impl AsRef<str> for GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum::LOGICALOPERATORUNSPECIFIED => "LOGICAL_OPERATOR_UNSPECIFIED",
            GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum::OR => "OR",
            GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum::AND => "AND",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOGICAL_OPERATOR_UNSPECIFIED" => Ok(GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum::LOGICALOPERATORUNSPECIFIED),
           "OR" => Ok(GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum::OR),
           "AND" => Ok(GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum::AND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2PubSubExpressionLogicalOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How much data to include in the Pub/Sub message. If the user wishes to limit the size of the message, they can use resource_name and fetch the profile fields they wish to. Per table profile (not per column).
pub enum GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum {
    

    /// Unused.
    ///
    /// "DETAIL_LEVEL_UNSPECIFIED"
    #[serde(rename="DETAIL_LEVEL_UNSPECIFIED")]
    DETAILLEVELUNSPECIFIED,
    

    /// The full table data profile.
    ///
    /// "TABLE_PROFILE"
    #[serde(rename="TABLE_PROFILE")]
    TABLEPROFILE,
    

    /// The resource name of the table.
    ///
    /// "RESOURCE_NAME"
    #[serde(rename="RESOURCE_NAME")]
    RESOURCENAME,
}

impl AsRef<str> for GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum::DETAILLEVELUNSPECIFIED => "DETAIL_LEVEL_UNSPECIFIED",
            GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum::TABLEPROFILE => "TABLE_PROFILE",
            GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum::RESOURCENAME => "RESOURCE_NAME",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DETAIL_LEVEL_UNSPECIFIED" => Ok(GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum::DETAILLEVELUNSPECIFIED),
           "TABLE_PROFILE" => Ok(GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum::TABLEPROFILE),
           "RESOURCE_NAME" => Ok(GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum::RESOURCENAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2PubSubNotificationDetailOfMessageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2PubSubNotificationEventEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of event that triggers a Pub/Sub. At most one `PubSubNotification` per EventType is permitted.
pub enum GooglePrivacyDlpV2PubSubNotificationEventEnum {
    

    /// Unused.
    ///
    /// "EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="EVENT_TYPE_UNSPECIFIED")]
    EVENTTYPEUNSPECIFIED,
    

    /// New profile (not a re-profile).
    ///
    /// "NEW_PROFILE"
    #[serde(rename="NEW_PROFILE")]
    NEWPROFILE,
    

    /// Changed one of the following profile metrics: * Table data risk score * Table sensitivity score * Table resource visibility * Table encryption type * Table predicted infoTypes * Table other infoTypes
    ///
    /// "CHANGED_PROFILE"
    #[serde(rename="CHANGED_PROFILE")]
    CHANGEDPROFILE,
    

    /// Table data risk score or sensitivity score increased.
    ///
    /// "SCORE_INCREASED"
    #[serde(rename="SCORE_INCREASED")]
    SCOREINCREASED,
    

    /// A user (non-internal) error occurred.
    ///
    /// "ERROR_CHANGED"
    #[serde(rename="ERROR_CHANGED")]
    ERRORCHANGED,
}

impl AsRef<str> for GooglePrivacyDlpV2PubSubNotificationEventEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2PubSubNotificationEventEnum::EVENTTYPEUNSPECIFIED => "EVENT_TYPE_UNSPECIFIED",
            GooglePrivacyDlpV2PubSubNotificationEventEnum::NEWPROFILE => "NEW_PROFILE",
            GooglePrivacyDlpV2PubSubNotificationEventEnum::CHANGEDPROFILE => "CHANGED_PROFILE",
            GooglePrivacyDlpV2PubSubNotificationEventEnum::SCOREINCREASED => "SCORE_INCREASED",
            GooglePrivacyDlpV2PubSubNotificationEventEnum::ERRORCHANGED => "ERROR_CHANGED",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2PubSubNotificationEventEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_TYPE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2PubSubNotificationEventEnum::EVENTTYPEUNSPECIFIED),
           "NEW_PROFILE" => Ok(GooglePrivacyDlpV2PubSubNotificationEventEnum::NEWPROFILE),
           "CHANGED_PROFILE" => Ok(GooglePrivacyDlpV2PubSubNotificationEventEnum::CHANGEDPROFILE),
           "SCORE_INCREASED" => Ok(GooglePrivacyDlpV2PubSubNotificationEventEnum::SCOREINCREASED),
           "ERROR_CHANGED" => Ok(GooglePrivacyDlpV2PubSubNotificationEventEnum::ERRORCHANGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2PubSubNotificationEventEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Frequency to regenerate data profiles when the schema is modified. Defaults to monthly.
pub enum GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum {
    

    /// Unspecified.
    ///
    /// "UPDATE_FREQUENCY_UNSPECIFIED"
    #[serde(rename="UPDATE_FREQUENCY_UNSPECIFIED")]
    UPDATEFREQUENCYUNSPECIFIED,
    

    /// After the data profile is created, it will never be updated.
    ///
    /// "UPDATE_FREQUENCY_NEVER"
    #[serde(rename="UPDATE_FREQUENCY_NEVER")]
    UPDATEFREQUENCYNEVER,
    

    /// The data profile can be updated up to once every 24 hours.
    ///
    /// "UPDATE_FREQUENCY_DAILY"
    #[serde(rename="UPDATE_FREQUENCY_DAILY")]
    UPDATEFREQUENCYDAILY,
    

    /// The data profile can be updated up to once every 30 days. Default.
    ///
    /// "UPDATE_FREQUENCY_MONTHLY"
    #[serde(rename="UPDATE_FREQUENCY_MONTHLY")]
    UPDATEFREQUENCYMONTHLY,
}

impl AsRef<str> for GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYUNSPECIFIED => "UPDATE_FREQUENCY_UNSPECIFIED",
            GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYNEVER => "UPDATE_FREQUENCY_NEVER",
            GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYDAILY => "UPDATE_FREQUENCY_DAILY",
            GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYMONTHLY => "UPDATE_FREQUENCY_MONTHLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UPDATE_FREQUENCY_UNSPECIFIED" => Ok(GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYUNSPECIFIED),
           "UPDATE_FREQUENCY_NEVER" => Ok(GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYNEVER),
           "UPDATE_FREQUENCY_DAILY" => Ok(GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYDAILY),
           "UPDATE_FREQUENCY_MONTHLY" => Ok(GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum::UPDATEFREQUENCYMONTHLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2SchemaModifiedCadenceFrequencyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The types of schema modifications to consider. Defaults to NEW_COLUMNS.
pub enum GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum {
    

    /// Unused.
    ///
    /// "SQL_SCHEMA_MODIFICATION_UNSPECIFIED"
    #[serde(rename="SQL_SCHEMA_MODIFICATION_UNSPECIFIED")]
    SQLSCHEMAMODIFICATIONUNSPECIFIED,
    

    /// New columns has appeared.
    ///
    /// "NEW_COLUMNS"
    #[serde(rename="NEW_COLUMNS")]
    NEWCOLUMNS,
    

    /// Columns have been removed from the table.
    ///
    /// "REMOVED_COLUMNS"
    #[serde(rename="REMOVED_COLUMNS")]
    REMOVEDCOLUMNS,
}

impl AsRef<str> for GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum::SQLSCHEMAMODIFICATIONUNSPECIFIED => "SQL_SCHEMA_MODIFICATION_UNSPECIFIED",
            GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum::NEWCOLUMNS => "NEW_COLUMNS",
            GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum::REMOVEDCOLUMNS => "REMOVED_COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_SCHEMA_MODIFICATION_UNSPECIFIED" => Ok(GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum::SQLSCHEMAMODIFICATIONUNSPECIFIED),
           "NEW_COLUMNS" => Ok(GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum::NEWCOLUMNS),
           "REMOVED_COLUMNS" => Ok(GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum::REMOVEDCOLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2SchemaModifiedCadenceTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2SensitivityScoreScoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The sensitivity score applied to the resource.
pub enum GooglePrivacyDlpV2SensitivityScoreScoreEnum {
    

    /// Unused.
    ///
    /// "SENSITIVITY_SCORE_UNSPECIFIED"
    #[serde(rename="SENSITIVITY_SCORE_UNSPECIFIED")]
    SENSITIVITYSCOREUNSPECIFIED,
    

    /// No sensitive information detected. The resource isn't publicly accessible.
    ///
    /// "SENSITIVITY_LOW"
    #[serde(rename="SENSITIVITY_LOW")]
    SENSITIVITYLOW,
    

    /// Medium risk. Contains personally identifiable information (PII), potentially sensitive data, or fields with free-text data that are at a higher risk of having intermittent sensitive data. Consider limiting access.
    ///
    /// "SENSITIVITY_MODERATE"
    #[serde(rename="SENSITIVITY_MODERATE")]
    SENSITIVITYMODERATE,
    

    /// High risk. Sensitive personally identifiable information (SPII) can be present. Exfiltration of data can lead to user data loss. Re-identification of users might be possible. Consider limiting usage and or removing SPII.
    ///
    /// "SENSITIVITY_HIGH"
    #[serde(rename="SENSITIVITY_HIGH")]
    SENSITIVITYHIGH,
}

impl AsRef<str> for GooglePrivacyDlpV2SensitivityScoreScoreEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2SensitivityScoreScoreEnum::SENSITIVITYSCOREUNSPECIFIED => "SENSITIVITY_SCORE_UNSPECIFIED",
            GooglePrivacyDlpV2SensitivityScoreScoreEnum::SENSITIVITYLOW => "SENSITIVITY_LOW",
            GooglePrivacyDlpV2SensitivityScoreScoreEnum::SENSITIVITYMODERATE => "SENSITIVITY_MODERATE",
            GooglePrivacyDlpV2SensitivityScoreScoreEnum::SENSITIVITYHIGH => "SENSITIVITY_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2SensitivityScoreScoreEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SENSITIVITY_SCORE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2SensitivityScoreScoreEnum::SENSITIVITYSCOREUNSPECIFIED),
           "SENSITIVITY_LOW" => Ok(GooglePrivacyDlpV2SensitivityScoreScoreEnum::SENSITIVITYLOW),
           "SENSITIVITY_MODERATE" => Ok(GooglePrivacyDlpV2SensitivityScoreScoreEnum::SENSITIVITYMODERATE),
           "SENSITIVITY_HIGH" => Ok(GooglePrivacyDlpV2SensitivityScoreScoreEnum::SENSITIVITYHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2SensitivityScoreScoreEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Stored info type version state. Read-only, updated by the system during dictionary creation.
pub enum GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum {
    

    /// Unused
    ///
    /// "STORED_INFO_TYPE_STATE_UNSPECIFIED"
    #[serde(rename="STORED_INFO_TYPE_STATE_UNSPECIFIED")]
    STOREDINFOTYPESTATEUNSPECIFIED,
    

    /// StoredInfoType version is being created.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// StoredInfoType version is ready for use.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// StoredInfoType creation failed. All relevant error messages are returned in the `StoredInfoTypeVersion` message.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// StoredInfoType is no longer valid because artifacts stored in user-controlled storage were modified. To fix an invalid StoredInfoType, use the `UpdateStoredInfoType` method to create a new version.
    ///
    /// "INVALID"
    #[serde(rename="INVALID")]
    INVALID,
}

impl AsRef<str> for GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum::STOREDINFOTYPESTATEUNSPECIFIED => "STORED_INFO_TYPE_STATE_UNSPECIFIED",
            GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum::PENDING => "PENDING",
            GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum::READY => "READY",
            GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum::FAILED => "FAILED",
            GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum::INVALID => "INVALID",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STORED_INFO_TYPE_STATE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum::STOREDINFOTYPESTATEUNSPECIFIED),
           "PENDING" => Ok(GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum::PENDING),
           "READY" => Ok(GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum::READY),
           "FAILED" => Ok(GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum::FAILED),
           "INVALID" => Ok(GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum::INVALID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2SummaryResultCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Outcome of the transformation.
pub enum GooglePrivacyDlpV2SummaryResultCodeEnum {
    

    /// Unused
    ///
    /// "TRANSFORMATION_RESULT_CODE_UNSPECIFIED"
    #[serde(rename="TRANSFORMATION_RESULT_CODE_UNSPECIFIED")]
    TRANSFORMATIONRESULTCODEUNSPECIFIED,
    

    /// Transformation completed without an error.
    ///
    /// "SUCCESS"
    #[serde(rename="SUCCESS")]
    SUCCESS,
    

    /// Transformation had an error.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for GooglePrivacyDlpV2SummaryResultCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2SummaryResultCodeEnum::TRANSFORMATIONRESULTCODEUNSPECIFIED => "TRANSFORMATION_RESULT_CODE_UNSPECIFIED",
            GooglePrivacyDlpV2SummaryResultCodeEnum::SUCCESS => "SUCCESS",
            GooglePrivacyDlpV2SummaryResultCodeEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2SummaryResultCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFORMATION_RESULT_CODE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2SummaryResultCodeEnum::TRANSFORMATIONRESULTCODEUNSPECIFIED),
           "SUCCESS" => Ok(GooglePrivacyDlpV2SummaryResultCodeEnum::SUCCESS),
           "ERROR" => Ok(GooglePrivacyDlpV2SummaryResultCodeEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2SummaryResultCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the table is encrypted.
pub enum GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum {
    

    /// Unused.
    ///
    /// "ENCRYPTION_STATUS_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_STATUS_UNSPECIFIED")]
    ENCRYPTIONSTATUSUNSPECIFIED,
    

    /// Google manages server-side encryption keys on your behalf.
    ///
    /// "ENCRYPTION_GOOGLE_MANAGED"
    #[serde(rename="ENCRYPTION_GOOGLE_MANAGED")]
    ENCRYPTIONGOOGLEMANAGED,
    

    /// Customer provides the key.
    ///
    /// "ENCRYPTION_CUSTOMER_MANAGED"
    #[serde(rename="ENCRYPTION_CUSTOMER_MANAGED")]
    ENCRYPTIONCUSTOMERMANAGED,
}

impl AsRef<str> for GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum::ENCRYPTIONSTATUSUNSPECIFIED => "ENCRYPTION_STATUS_UNSPECIFIED",
            GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum::ENCRYPTIONGOOGLEMANAGED => "ENCRYPTION_GOOGLE_MANAGED",
            GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum::ENCRYPTIONCUSTOMERMANAGED => "ENCRYPTION_CUSTOMER_MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_STATUS_UNSPECIFIED" => Ok(GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum::ENCRYPTIONSTATUSUNSPECIFIED),
           "ENCRYPTION_GOOGLE_MANAGED" => Ok(GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum::ENCRYPTIONGOOGLEMANAGED),
           "ENCRYPTION_CUSTOMER_MANAGED" => Ok(GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum::ENCRYPTIONCUSTOMERMANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2TableDataProfileEncryptionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How broadly a resource has been shared.
pub enum GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum {
    

    /// Unused.
    ///
    /// "RESOURCE_VISIBILITY_UNSPECIFIED"
    #[serde(rename="RESOURCE_VISIBILITY_UNSPECIFIED")]
    RESOURCEVISIBILITYUNSPECIFIED,
    

    /// Visible to any user.
    ///
    /// "RESOURCE_VISIBILITY_PUBLIC"
    #[serde(rename="RESOURCE_VISIBILITY_PUBLIC")]
    RESOURCEVISIBILITYPUBLIC,
    

    /// Visible only to specific users.
    ///
    /// "RESOURCE_VISIBILITY_RESTRICTED"
    #[serde(rename="RESOURCE_VISIBILITY_RESTRICTED")]
    RESOURCEVISIBILITYRESTRICTED,
}

impl AsRef<str> for GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum::RESOURCEVISIBILITYUNSPECIFIED => "RESOURCE_VISIBILITY_UNSPECIFIED",
            GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum::RESOURCEVISIBILITYPUBLIC => "RESOURCE_VISIBILITY_PUBLIC",
            GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum::RESOURCEVISIBILITYRESTRICTED => "RESOURCE_VISIBILITY_RESTRICTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_VISIBILITY_UNSPECIFIED" => Ok(GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum::RESOURCEVISIBILITYUNSPECIFIED),
           "RESOURCE_VISIBILITY_PUBLIC" => Ok(GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum::RESOURCEVISIBILITYPUBLIC),
           "RESOURCE_VISIBILITY_RESTRICTED" => Ok(GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum::RESOURCEVISIBILITYRESTRICTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2TableDataProfileResourceVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2TableDataProfileStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of a profile.
pub enum GooglePrivacyDlpV2TableDataProfileStateEnum {
    

    /// Unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The profile is currently running. Once a profile has finished it will transition to DONE.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The profile is no longer generating. If profile_status.status.code is 0, the profile succeeded, otherwise, it failed.
    ///
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
}

impl AsRef<str> for GooglePrivacyDlpV2TableDataProfileStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2TableDataProfileStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GooglePrivacyDlpV2TableDataProfileStateEnum::RUNNING => "RUNNING",
            GooglePrivacyDlpV2TableDataProfileStateEnum::DONE => "DONE",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2TableDataProfileStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GooglePrivacyDlpV2TableDataProfileStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(GooglePrivacyDlpV2TableDataProfileStateEnum::RUNNING),
           "DONE" => Ok(GooglePrivacyDlpV2TableDataProfileStateEnum::DONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2TableDataProfileStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2TimePartConfigPartToExtractEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The part of the time to keep.
pub enum GooglePrivacyDlpV2TimePartConfigPartToExtractEnum {
    

    /// Unused
    ///
    /// "TIME_PART_UNSPECIFIED"
    #[serde(rename="TIME_PART_UNSPECIFIED")]
    TIMEPARTUNSPECIFIED,
    

    /// [0-9999]
    ///
    /// "YEAR"
    #[serde(rename="YEAR")]
    YEAR,
    

    /// [1-12]
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// [1-31]
    ///
    /// "DAY_OF_MONTH"
    #[serde(rename="DAY_OF_MONTH")]
    DAYOFMONTH,
    

    /// [1-7]
    ///
    /// "DAY_OF_WEEK"
    #[serde(rename="DAY_OF_WEEK")]
    DAYOFWEEK,
    

    /// [1-53]
    ///
    /// "WEEK_OF_YEAR"
    #[serde(rename="WEEK_OF_YEAR")]
    WEEKOFYEAR,
    

    /// [0-23]
    ///
    /// "HOUR_OF_DAY"
    #[serde(rename="HOUR_OF_DAY")]
    HOUROFDAY,
}

impl AsRef<str> for GooglePrivacyDlpV2TimePartConfigPartToExtractEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::TIMEPARTUNSPECIFIED => "TIME_PART_UNSPECIFIED",
            GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::YEAR => "YEAR",
            GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::MONTH => "MONTH",
            GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::DAYOFMONTH => "DAY_OF_MONTH",
            GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::DAYOFWEEK => "DAY_OF_WEEK",
            GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::WEEKOFYEAR => "WEEK_OF_YEAR",
            GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::HOUROFDAY => "HOUR_OF_DAY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2TimePartConfigPartToExtractEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_PART_UNSPECIFIED" => Ok(GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::TIMEPARTUNSPECIFIED),
           "YEAR" => Ok(GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::YEAR),
           "MONTH" => Ok(GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::MONTH),
           "DAY_OF_MONTH" => Ok(GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::DAYOFMONTH),
           "DAY_OF_WEEK" => Ok(GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::DAYOFWEEK),
           "WEEK_OF_YEAR" => Ok(GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::WEEKOFYEAR),
           "HOUR_OF_DAY" => Ok(GooglePrivacyDlpV2TimePartConfigPartToExtractEnum::HOUROFDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2TimePartConfigPartToExtractEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GooglePrivacyDlpV2ValueDayOfWeekValueEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// day of week
pub enum GooglePrivacyDlpV2ValueDayOfWeekValueEnum {
    

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

impl AsRef<str> for GooglePrivacyDlpV2ValueDayOfWeekValueEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GooglePrivacyDlpV2ValueDayOfWeekValueEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            GooglePrivacyDlpV2ValueDayOfWeekValueEnum::MONDAY => "MONDAY",
            GooglePrivacyDlpV2ValueDayOfWeekValueEnum::TUESDAY => "TUESDAY",
            GooglePrivacyDlpV2ValueDayOfWeekValueEnum::WEDNESDAY => "WEDNESDAY",
            GooglePrivacyDlpV2ValueDayOfWeekValueEnum::THURSDAY => "THURSDAY",
            GooglePrivacyDlpV2ValueDayOfWeekValueEnum::FRIDAY => "FRIDAY",
            GooglePrivacyDlpV2ValueDayOfWeekValueEnum::SATURDAY => "SATURDAY",
            GooglePrivacyDlpV2ValueDayOfWeekValueEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for GooglePrivacyDlpV2ValueDayOfWeekValueEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(GooglePrivacyDlpV2ValueDayOfWeekValueEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(GooglePrivacyDlpV2ValueDayOfWeekValueEnum::MONDAY),
           "TUESDAY" => Ok(GooglePrivacyDlpV2ValueDayOfWeekValueEnum::TUESDAY),
           "WEDNESDAY" => Ok(GooglePrivacyDlpV2ValueDayOfWeekValueEnum::WEDNESDAY),
           "THURSDAY" => Ok(GooglePrivacyDlpV2ValueDayOfWeekValueEnum::THURSDAY),
           "FRIDAY" => Ok(GooglePrivacyDlpV2ValueDayOfWeekValueEnum::FRIDAY),
           "SATURDAY" => Ok(GooglePrivacyDlpV2ValueDayOfWeekValueEnum::SATURDAY),
           "SUNDAY" => Ok(GooglePrivacyDlpV2ValueDayOfWeekValueEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GooglePrivacyDlpV2ValueDayOfWeekValueEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of jobs. Will use `DlpJobType.INSPECT` if not set.
pub enum OrganizationTypeEnum {
    

    /// Defaults to INSPECT_JOB.
    ///
    /// "DLP_JOB_TYPE_UNSPECIFIED"
    #[serde(rename="DLP_JOB_TYPE_UNSPECIFIED")]
    DLPJOBTYPEUNSPECIFIED,
    

    /// The job inspected Google Cloud for sensitive data.
    ///
    /// "INSPECT_JOB"
    #[serde(rename="INSPECT_JOB")]
    INSPECTJOB,
    

    /// The job executed a Risk Analysis computation.
    ///
    /// "RISK_ANALYSIS_JOB"
    #[serde(rename="RISK_ANALYSIS_JOB")]
    RISKANALYSISJOB,
}

impl AsRef<str> for OrganizationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationTypeEnum::DLPJOBTYPEUNSPECIFIED => "DLP_JOB_TYPE_UNSPECIFIED",
            OrganizationTypeEnum::INSPECTJOB => "INSPECT_JOB",
            OrganizationTypeEnum::RISKANALYSISJOB => "RISK_ANALYSIS_JOB",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DLP_JOB_TYPE_UNSPECIFIED" => Ok(OrganizationTypeEnum::DLPJOBTYPEUNSPECIFIED),
           "INSPECT_JOB" => Ok(OrganizationTypeEnum::INSPECTJOB),
           "RISK_ANALYSIS_JOB" => Ok(OrganizationTypeEnum::RISKANALYSISJOB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of jobs. Will use `DlpJobType.INSPECT` if not set.
pub enum ProjectTypeEnum {
    

    /// Defaults to INSPECT_JOB.
    ///
    /// "DLP_JOB_TYPE_UNSPECIFIED"
    #[serde(rename="DLP_JOB_TYPE_UNSPECIFIED")]
    DLPJOBTYPEUNSPECIFIED,
    

    /// The job inspected Google Cloud for sensitive data.
    ///
    /// "INSPECT_JOB"
    #[serde(rename="INSPECT_JOB")]
    INSPECTJOB,
    

    /// The job executed a Risk Analysis computation.
    ///
    /// "RISK_ANALYSIS_JOB"
    #[serde(rename="RISK_ANALYSIS_JOB")]
    RISKANALYSISJOB,
}

impl AsRef<str> for ProjectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectTypeEnum::DLPJOBTYPEUNSPECIFIED => "DLP_JOB_TYPE_UNSPECIFIED",
            ProjectTypeEnum::INSPECTJOB => "INSPECT_JOB",
            ProjectTypeEnum::RISKANALYSISJOB => "RISK_ANALYSIS_JOB",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DLP_JOB_TYPE_UNSPECIFIED" => Ok(ProjectTypeEnum::DLPJOBTYPEUNSPECIFIED),
           "INSPECT_JOB" => Ok(ProjectTypeEnum::INSPECTJOB),
           "RISK_ANALYSIS_JOB" => Ok(ProjectTypeEnum::RISKANALYSISJOB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


