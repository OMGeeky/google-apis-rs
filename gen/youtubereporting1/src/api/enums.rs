use super::*;



// region GdataCompositeMediaReferenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// gdata
pub enum GdataCompositeMediaReferenceTypeEnum {
    

    /// gdata
    ///
    /// "PATH"
    #[serde(rename="PATH")]
    PATH,
    

    /// gdata
    ///
    /// "BLOB_REF"
    #[serde(rename="BLOB_REF")]
    BLOBREF,
    

    /// gdata
    ///
    /// "INLINE"
    #[serde(rename="INLINE")]
    INLINE,
    

    /// gdata
    ///
    /// "BIGSTORE_REF"
    #[serde(rename="BIGSTORE_REF")]
    BIGSTOREREF,
    

    /// gdata
    ///
    /// "COSMO_BINARY_REFERENCE"
    #[serde(rename="COSMO_BINARY_REFERENCE")]
    COSMOBINARYREFERENCE,
}

impl AsRef<str> for GdataCompositeMediaReferenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GdataCompositeMediaReferenceTypeEnum::PATH => "PATH",
            GdataCompositeMediaReferenceTypeEnum::BLOBREF => "BLOB_REF",
            GdataCompositeMediaReferenceTypeEnum::INLINE => "INLINE",
            GdataCompositeMediaReferenceTypeEnum::BIGSTOREREF => "BIGSTORE_REF",
            GdataCompositeMediaReferenceTypeEnum::COSMOBINARYREFERENCE => "COSMO_BINARY_REFERENCE",
        }
    }
}

impl std::convert::TryFrom< &str> for GdataCompositeMediaReferenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PATH" => Ok(GdataCompositeMediaReferenceTypeEnum::PATH),
           "BLOB_REF" => Ok(GdataCompositeMediaReferenceTypeEnum::BLOBREF),
           "INLINE" => Ok(GdataCompositeMediaReferenceTypeEnum::INLINE),
           "BIGSTORE_REF" => Ok(GdataCompositeMediaReferenceTypeEnum::BIGSTOREREF),
           "COSMO_BINARY_REFERENCE" => Ok(GdataCompositeMediaReferenceTypeEnum::COSMOBINARYREFERENCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GdataCompositeMediaReferenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GdataMediaReferenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// gdata
pub enum GdataMediaReferenceTypeEnum {
    

    /// gdata
    ///
    /// "PATH"
    #[serde(rename="PATH")]
    PATH,
    

    /// gdata
    ///
    /// "BLOB_REF"
    #[serde(rename="BLOB_REF")]
    BLOBREF,
    

    /// gdata
    ///
    /// "INLINE"
    #[serde(rename="INLINE")]
    INLINE,
    

    /// gdata
    ///
    /// "GET_MEDIA"
    #[serde(rename="GET_MEDIA")]
    GETMEDIA,
    

    /// gdata
    ///
    /// "COMPOSITE_MEDIA"
    #[serde(rename="COMPOSITE_MEDIA")]
    COMPOSITEMEDIA,
    

    /// gdata
    ///
    /// "BIGSTORE_REF"
    #[serde(rename="BIGSTORE_REF")]
    BIGSTOREREF,
    

    /// gdata
    ///
    /// "DIFF_VERSION_RESPONSE"
    #[serde(rename="DIFF_VERSION_RESPONSE")]
    DIFFVERSIONRESPONSE,
    

    /// gdata
    ///
    /// "DIFF_CHECKSUMS_RESPONSE"
    #[serde(rename="DIFF_CHECKSUMS_RESPONSE")]
    DIFFCHECKSUMSRESPONSE,
    

    /// gdata
    ///
    /// "DIFF_DOWNLOAD_RESPONSE"
    #[serde(rename="DIFF_DOWNLOAD_RESPONSE")]
    DIFFDOWNLOADRESPONSE,
    

    /// gdata
    ///
    /// "DIFF_UPLOAD_REQUEST"
    #[serde(rename="DIFF_UPLOAD_REQUEST")]
    DIFFUPLOADREQUEST,
    

    /// gdata
    ///
    /// "DIFF_UPLOAD_RESPONSE"
    #[serde(rename="DIFF_UPLOAD_RESPONSE")]
    DIFFUPLOADRESPONSE,
    

    /// gdata
    ///
    /// "COSMO_BINARY_REFERENCE"
    #[serde(rename="COSMO_BINARY_REFERENCE")]
    COSMOBINARYREFERENCE,
    

    /// gdata
    ///
    /// "ARBITRARY_BYTES"
    #[serde(rename="ARBITRARY_BYTES")]
    ARBITRARYBYTES,
}

impl AsRef<str> for GdataMediaReferenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GdataMediaReferenceTypeEnum::PATH => "PATH",
            GdataMediaReferenceTypeEnum::BLOBREF => "BLOB_REF",
            GdataMediaReferenceTypeEnum::INLINE => "INLINE",
            GdataMediaReferenceTypeEnum::GETMEDIA => "GET_MEDIA",
            GdataMediaReferenceTypeEnum::COMPOSITEMEDIA => "COMPOSITE_MEDIA",
            GdataMediaReferenceTypeEnum::BIGSTOREREF => "BIGSTORE_REF",
            GdataMediaReferenceTypeEnum::DIFFVERSIONRESPONSE => "DIFF_VERSION_RESPONSE",
            GdataMediaReferenceTypeEnum::DIFFCHECKSUMSRESPONSE => "DIFF_CHECKSUMS_RESPONSE",
            GdataMediaReferenceTypeEnum::DIFFDOWNLOADRESPONSE => "DIFF_DOWNLOAD_RESPONSE",
            GdataMediaReferenceTypeEnum::DIFFUPLOADREQUEST => "DIFF_UPLOAD_REQUEST",
            GdataMediaReferenceTypeEnum::DIFFUPLOADRESPONSE => "DIFF_UPLOAD_RESPONSE",
            GdataMediaReferenceTypeEnum::COSMOBINARYREFERENCE => "COSMO_BINARY_REFERENCE",
            GdataMediaReferenceTypeEnum::ARBITRARYBYTES => "ARBITRARY_BYTES",
        }
    }
}

impl std::convert::TryFrom< &str> for GdataMediaReferenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PATH" => Ok(GdataMediaReferenceTypeEnum::PATH),
           "BLOB_REF" => Ok(GdataMediaReferenceTypeEnum::BLOBREF),
           "INLINE" => Ok(GdataMediaReferenceTypeEnum::INLINE),
           "GET_MEDIA" => Ok(GdataMediaReferenceTypeEnum::GETMEDIA),
           "COMPOSITE_MEDIA" => Ok(GdataMediaReferenceTypeEnum::COMPOSITEMEDIA),
           "BIGSTORE_REF" => Ok(GdataMediaReferenceTypeEnum::BIGSTOREREF),
           "DIFF_VERSION_RESPONSE" => Ok(GdataMediaReferenceTypeEnum::DIFFVERSIONRESPONSE),
           "DIFF_CHECKSUMS_RESPONSE" => Ok(GdataMediaReferenceTypeEnum::DIFFCHECKSUMSRESPONSE),
           "DIFF_DOWNLOAD_RESPONSE" => Ok(GdataMediaReferenceTypeEnum::DIFFDOWNLOADRESPONSE),
           "DIFF_UPLOAD_REQUEST" => Ok(GdataMediaReferenceTypeEnum::DIFFUPLOADREQUEST),
           "DIFF_UPLOAD_RESPONSE" => Ok(GdataMediaReferenceTypeEnum::DIFFUPLOADRESPONSE),
           "COSMO_BINARY_REFERENCE" => Ok(GdataMediaReferenceTypeEnum::COSMOBINARYREFERENCE),
           "ARBITRARY_BYTES" => Ok(GdataMediaReferenceTypeEnum::ARBITRARYBYTES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GdataMediaReferenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


