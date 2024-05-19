use super::*;



// region ActivationStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActivationStatusStateEnum {
    
    /// "UNKNOWN_STATE"
    #[serde(rename="UNKNOWN_STATE")]
    UNKNOWNSTATE,
    

    /// Not-Activated, this is the default status
    ///
    /// "NOT_ACTIVATED"
    #[serde(rename="NOT_ACTIVATED")]
    NOTACTIVATED,
    

    /// Legacy alias for `NOT_ACTIVATED`. Deprecated.
    ///
    /// "not_activated"
    #[serde(rename="not_activated")]
    NotActivated,
    

    /// Activated
    ///
    /// "ACTIVATED"
    #[serde(rename="ACTIVATED")]
    ACTIVATED,
    

    /// Legacy alias for `ACTIVATED`. Deprecated.
    ///
    /// "activated"
    #[serde(rename="activated")]
    Activated,
}

impl AsRef<str> for ActivationStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivationStatusStateEnum::UNKNOWNSTATE => "UNKNOWN_STATE",
            ActivationStatusStateEnum::NOTACTIVATED => "NOT_ACTIVATED",
            ActivationStatusStateEnum::NotActivated => "not_activated",
            ActivationStatusStateEnum::ACTIVATED => "ACTIVATED",
            ActivationStatusStateEnum::Activated => "activated",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivationStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_STATE" => Ok(ActivationStatusStateEnum::UNKNOWNSTATE),
           "NOT_ACTIVATED" => Ok(ActivationStatusStateEnum::NOTACTIVATED),
           "not_activated" => Ok(ActivationStatusStateEnum::NotActivated),
           "ACTIVATED" => Ok(ActivationStatusStateEnum::ACTIVATED),
           "activated" => Ok(ActivationStatusStateEnum::Activated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivationStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BarcodeRenderEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The render encoding for the barcode. When specified, barcode is rendered in the given encoding. Otherwise best known encoding is chosen by Google.
pub enum BarcodeRenderEncodingEnum {
    
    /// "RENDER_ENCODING_UNSPECIFIED"
    #[serde(rename="RENDER_ENCODING_UNSPECIFIED")]
    RENDERENCODINGUNSPECIFIED,
    

    /// UTF_8 encoding for barcodes. This is only supported for barcode type qrCode.
    ///
    /// "UTF_8"
    #[serde(rename="UTF_8")]
    UTF8,
}

impl AsRef<str> for BarcodeRenderEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BarcodeRenderEncodingEnum::RENDERENCODINGUNSPECIFIED => "RENDER_ENCODING_UNSPECIFIED",
            BarcodeRenderEncodingEnum::UTF8 => "UTF_8",
        }
    }
}

impl std::convert::TryFrom< &str> for BarcodeRenderEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RENDER_ENCODING_UNSPECIFIED" => Ok(BarcodeRenderEncodingEnum::RENDERENCODINGUNSPECIFIED),
           "UTF_8" => Ok(BarcodeRenderEncodingEnum::UTF8),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BarcodeRenderEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BarcodeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of barcode.
pub enum BarcodeTypeEnum {
    
    /// "BARCODE_TYPE_UNSPECIFIED"
    #[serde(rename="BARCODE_TYPE_UNSPECIFIED")]
    BARCODETYPEUNSPECIFIED,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "AZTEC"
    #[serde(rename="AZTEC")]
    AZTEC,
    

    /// Legacy alias for `AZTEC`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "aztec"
    #[serde(rename="aztec")]
    Aztec,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "CODE_39"
    #[serde(rename="CODE_39")]
    CODE39,
    

    /// Legacy alias for `CODE_39`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "code39"
    #[serde(rename="code39")]
    Code39,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "CODE_128"
    #[serde(rename="CODE_128")]
    CODE128,
    

    /// Legacy alias for `CODE_128`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "code128"
    #[serde(rename="code128")]
    Code128,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "CODABAR"
    #[serde(rename="CODABAR")]
    CODABAR,
    

    /// Legacy alias for `CODABAR`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "codabar"
    #[serde(rename="codabar")]
    Codabar,
    

    /// A 2D matrix barcode consisting of black and white. Cells or modules are arranged in either a square or rectangle. Not supported for Rotating Barcodes.
    ///
    /// "DATA_MATRIX"
    #[serde(rename="DATA_MATRIX")]
    DATAMATRIX,
    

    /// Legacy alias for `DATA_MATRIX`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "dataMatrix"
    #[serde(rename="dataMatrix")]
    DataMatrix,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "EAN_8"
    #[serde(rename="EAN_8")]
    EAN8,
    

    /// Legacy alias for `EAN_8`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "ean8"
    #[serde(rename="ean8")]
    Ean8,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "EAN_13"
    #[serde(rename="EAN_13")]
    EAN13,
    

    /// Legacy alias for `EAN_13`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "ean13"
    #[serde(rename="ean13")]
    Ean13,
    

    /// Legacy alias for `EAN_13`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "EAN13"
    #[serde(rename="EAN13")]
    EAN13,
    

    /// 14 digit ITF code Not supported for Rotating Barcodes.
    ///
    /// "ITF_14"
    #[serde(rename="ITF_14")]
    ITF14,
    

    /// Legacy alias for `ITF_14`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "itf14"
    #[serde(rename="itf14")]
    Itf14,
    

    /// Supported for Rotating Barcodes.
    ///
    /// "PDF_417"
    #[serde(rename="PDF_417")]
    PDF417,
    

    /// Legacy alias for `PDF_417`. Deprecated.
    ///
    /// "pdf417"
    #[serde(rename="pdf417")]
    Pdf417,
    

    /// Legacy alias for `PDF_417`. Deprecated.
    ///
    /// "PDF417"
    #[serde(rename="PDF417")]
    PDF417,
    

    /// Supported for Rotating Barcodes.
    ///
    /// "QR_CODE"
    #[serde(rename="QR_CODE")]
    QRCODE,
    

    /// Legacy alias for `QR_CODE`. Deprecated.
    ///
    /// "qrCode"
    #[serde(rename="qrCode")]
    QrCode,
    

    /// Legacy alias for `QR_CODE`. Deprecated.
    ///
    /// "qrcode"
    #[serde(rename="qrcode")]
    Qrcode,
    

    /// 11 or 12 digit codes Not supported for Rotating Barcodes.
    ///
    /// "UPC_A"
    #[serde(rename="UPC_A")]
    UPCA,
    

    /// Legacy alias for `UPC_A`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "upcA"
    #[serde(rename="upcA")]
    UpcA,
    

    /// Renders the field as a text field. The `alternateText` field may not be used with a barcode of type `textOnly`. Not supported for Rotating Barcodes.
    ///
    /// "TEXT_ONLY"
    #[serde(rename="TEXT_ONLY")]
    TEXTONLY,
    

    /// Legacy alias for `TEXT_ONLY`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "textOnly"
    #[serde(rename="textOnly")]
    TextOnly,
}

impl AsRef<str> for BarcodeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BarcodeTypeEnum::BARCODETYPEUNSPECIFIED => "BARCODE_TYPE_UNSPECIFIED",
            BarcodeTypeEnum::AZTEC => "AZTEC",
            BarcodeTypeEnum::Aztec => "aztec",
            BarcodeTypeEnum::CODE39 => "CODE_39",
            BarcodeTypeEnum::Code39 => "code39",
            BarcodeTypeEnum::CODE128 => "CODE_128",
            BarcodeTypeEnum::Code128 => "code128",
            BarcodeTypeEnum::CODABAR => "CODABAR",
            BarcodeTypeEnum::Codabar => "codabar",
            BarcodeTypeEnum::DATAMATRIX => "DATA_MATRIX",
            BarcodeTypeEnum::DataMatrix => "dataMatrix",
            BarcodeTypeEnum::EAN8 => "EAN_8",
            BarcodeTypeEnum::Ean8 => "ean8",
            BarcodeTypeEnum::EAN13 => "EAN_13",
            BarcodeTypeEnum::Ean13 => "ean13",
            BarcodeTypeEnum::EAN13 => "EAN13",
            BarcodeTypeEnum::ITF14 => "ITF_14",
            BarcodeTypeEnum::Itf14 => "itf14",
            BarcodeTypeEnum::PDF417 => "PDF_417",
            BarcodeTypeEnum::Pdf417 => "pdf417",
            BarcodeTypeEnum::PDF417 => "PDF417",
            BarcodeTypeEnum::QRCODE => "QR_CODE",
            BarcodeTypeEnum::QrCode => "qrCode",
            BarcodeTypeEnum::Qrcode => "qrcode",
            BarcodeTypeEnum::UPCA => "UPC_A",
            BarcodeTypeEnum::UpcA => "upcA",
            BarcodeTypeEnum::TEXTONLY => "TEXT_ONLY",
            BarcodeTypeEnum::TextOnly => "textOnly",
        }
    }
}

impl std::convert::TryFrom< &str> for BarcodeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BARCODE_TYPE_UNSPECIFIED" => Ok(BarcodeTypeEnum::BARCODETYPEUNSPECIFIED),
           "AZTEC" => Ok(BarcodeTypeEnum::AZTEC),
           "aztec" => Ok(BarcodeTypeEnum::Aztec),
           "CODE_39" => Ok(BarcodeTypeEnum::CODE39),
           "code39" => Ok(BarcodeTypeEnum::Code39),
           "CODE_128" => Ok(BarcodeTypeEnum::CODE128),
           "code128" => Ok(BarcodeTypeEnum::Code128),
           "CODABAR" => Ok(BarcodeTypeEnum::CODABAR),
           "codabar" => Ok(BarcodeTypeEnum::Codabar),
           "DATA_MATRIX" => Ok(BarcodeTypeEnum::DATAMATRIX),
           "dataMatrix" => Ok(BarcodeTypeEnum::DataMatrix),
           "EAN_8" => Ok(BarcodeTypeEnum::EAN8),
           "ean8" => Ok(BarcodeTypeEnum::Ean8),
           "EAN_13" => Ok(BarcodeTypeEnum::EAN13),
           "ean13" => Ok(BarcodeTypeEnum::Ean13),
           "EAN13" => Ok(BarcodeTypeEnum::EAN13),
           "ITF_14" => Ok(BarcodeTypeEnum::ITF14),
           "itf14" => Ok(BarcodeTypeEnum::Itf14),
           "PDF_417" => Ok(BarcodeTypeEnum::PDF417),
           "pdf417" => Ok(BarcodeTypeEnum::Pdf417),
           "PDF417" => Ok(BarcodeTypeEnum::PDF417),
           "QR_CODE" => Ok(BarcodeTypeEnum::QRCODE),
           "qrCode" => Ok(BarcodeTypeEnum::QrCode),
           "qrcode" => Ok(BarcodeTypeEnum::Qrcode),
           "UPC_A" => Ok(BarcodeTypeEnum::UPCA),
           "upcA" => Ok(BarcodeTypeEnum::UpcA),
           "TEXT_ONLY" => Ok(BarcodeTypeEnum::TEXTONLY),
           "textOnly" => Ok(BarcodeTypeEnum::TextOnly),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BarcodeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BoardingAndSeatingInfoBoardingDoorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Set this field only if this flight boards through more than one door or bridge and you want to explicitly print the door location on the boarding pass. Most airlines route their passengers to the right door or bridge by refering to doors/bridges by the `seatClass`. In those cases `boardingDoor` should not be set.
pub enum BoardingAndSeatingInfoBoardingDoorEnum {
    
    /// "BOARDING_DOOR_UNSPECIFIED"
    #[serde(rename="BOARDING_DOOR_UNSPECIFIED")]
    BOARDINGDOORUNSPECIFIED,
    
    /// "FRONT"
    #[serde(rename="FRONT")]
    FRONT,
    

    /// Legacy alias for `FRONT`. Deprecated.
    ///
    /// "front"
    #[serde(rename="front")]
    Front,
    
    /// "BACK"
    #[serde(rename="BACK")]
    BACK,
    

    /// Legacy alias for `BACK`. Deprecated.
    ///
    /// "back"
    #[serde(rename="back")]
    Back,
}

impl AsRef<str> for BoardingAndSeatingInfoBoardingDoorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BoardingAndSeatingInfoBoardingDoorEnum::BOARDINGDOORUNSPECIFIED => "BOARDING_DOOR_UNSPECIFIED",
            BoardingAndSeatingInfoBoardingDoorEnum::FRONT => "FRONT",
            BoardingAndSeatingInfoBoardingDoorEnum::Front => "front",
            BoardingAndSeatingInfoBoardingDoorEnum::BACK => "BACK",
            BoardingAndSeatingInfoBoardingDoorEnum::Back => "back",
        }
    }
}

impl std::convert::TryFrom< &str> for BoardingAndSeatingInfoBoardingDoorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BOARDING_DOOR_UNSPECIFIED" => Ok(BoardingAndSeatingInfoBoardingDoorEnum::BOARDINGDOORUNSPECIFIED),
           "FRONT" => Ok(BoardingAndSeatingInfoBoardingDoorEnum::FRONT),
           "front" => Ok(BoardingAndSeatingInfoBoardingDoorEnum::Front),
           "BACK" => Ok(BoardingAndSeatingInfoBoardingDoorEnum::BACK),
           "back" => Ok(BoardingAndSeatingInfoBoardingDoorEnum::Back),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BoardingAndSeatingInfoBoardingDoorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BoardingAndSeatingPolicyBoardingPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the policy the airline uses for boarding. If unset, Google will default to `zoneBased`.
pub enum BoardingAndSeatingPolicyBoardingPolicyEnum {
    
    /// "BOARDING_POLICY_UNSPECIFIED"
    #[serde(rename="BOARDING_POLICY_UNSPECIFIED")]
    BOARDINGPOLICYUNSPECIFIED,
    
    /// "ZONE_BASED"
    #[serde(rename="ZONE_BASED")]
    ZONEBASED,
    

    /// Legacy alias for `ZONE_BASED`. Deprecated.
    ///
    /// "zoneBased"
    #[serde(rename="zoneBased")]
    ZoneBased,
    
    /// "GROUP_BASED"
    #[serde(rename="GROUP_BASED")]
    GROUPBASED,
    

    /// Legacy alias for `GROUP_BASED`. Deprecated.
    ///
    /// "groupBased"
    #[serde(rename="groupBased")]
    GroupBased,
    
    /// "BOARDING_POLICY_OTHER"
    #[serde(rename="BOARDING_POLICY_OTHER")]
    BOARDINGPOLICYOTHER,
    

    /// Legacy alias for `BOARDING_POLICY_OTHER`. Deprecated.
    ///
    /// "boardingPolicyOther"
    #[serde(rename="boardingPolicyOther")]
    BoardingPolicyOther,
}

impl AsRef<str> for BoardingAndSeatingPolicyBoardingPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BoardingAndSeatingPolicyBoardingPolicyEnum::BOARDINGPOLICYUNSPECIFIED => "BOARDING_POLICY_UNSPECIFIED",
            BoardingAndSeatingPolicyBoardingPolicyEnum::ZONEBASED => "ZONE_BASED",
            BoardingAndSeatingPolicyBoardingPolicyEnum::ZoneBased => "zoneBased",
            BoardingAndSeatingPolicyBoardingPolicyEnum::GROUPBASED => "GROUP_BASED",
            BoardingAndSeatingPolicyBoardingPolicyEnum::GroupBased => "groupBased",
            BoardingAndSeatingPolicyBoardingPolicyEnum::BOARDINGPOLICYOTHER => "BOARDING_POLICY_OTHER",
            BoardingAndSeatingPolicyBoardingPolicyEnum::BoardingPolicyOther => "boardingPolicyOther",
        }
    }
}

impl std::convert::TryFrom< &str> for BoardingAndSeatingPolicyBoardingPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BOARDING_POLICY_UNSPECIFIED" => Ok(BoardingAndSeatingPolicyBoardingPolicyEnum::BOARDINGPOLICYUNSPECIFIED),
           "ZONE_BASED" => Ok(BoardingAndSeatingPolicyBoardingPolicyEnum::ZONEBASED),
           "zoneBased" => Ok(BoardingAndSeatingPolicyBoardingPolicyEnum::ZoneBased),
           "GROUP_BASED" => Ok(BoardingAndSeatingPolicyBoardingPolicyEnum::GROUPBASED),
           "groupBased" => Ok(BoardingAndSeatingPolicyBoardingPolicyEnum::GroupBased),
           "BOARDING_POLICY_OTHER" => Ok(BoardingAndSeatingPolicyBoardingPolicyEnum::BOARDINGPOLICYOTHER),
           "boardingPolicyOther" => Ok(BoardingAndSeatingPolicyBoardingPolicyEnum::BoardingPolicyOther),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BoardingAndSeatingPolicyBoardingPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BoardingAndSeatingPolicySeatClassPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Seating policy which dictates how we display the seat class. If unset, Google will default to `cabinBased`.
pub enum BoardingAndSeatingPolicySeatClassPolicyEnum {
    
    /// "SEAT_CLASS_POLICY_UNSPECIFIED"
    #[serde(rename="SEAT_CLASS_POLICY_UNSPECIFIED")]
    SEATCLASSPOLICYUNSPECIFIED,
    
    /// "CABIN_BASED"
    #[serde(rename="CABIN_BASED")]
    CABINBASED,
    

    /// Legacy alias for `CABIN_BASED`. Deprecated.
    ///
    /// "cabinBased"
    #[serde(rename="cabinBased")]
    CabinBased,
    
    /// "CLASS_BASED"
    #[serde(rename="CLASS_BASED")]
    CLASSBASED,
    

    /// Legacy alias for `CLASS_BASED`. Deprecated.
    ///
    /// "classBased"
    #[serde(rename="classBased")]
    ClassBased,
    
    /// "TIER_BASED"
    #[serde(rename="TIER_BASED")]
    TIERBASED,
    

    /// Legacy alias for `TIER_BASED`. Deprecated.
    ///
    /// "tierBased"
    #[serde(rename="tierBased")]
    TierBased,
    
    /// "SEAT_CLASS_POLICY_OTHER"
    #[serde(rename="SEAT_CLASS_POLICY_OTHER")]
    SEATCLASSPOLICYOTHER,
    

    /// Legacy alias for `SEAT_CLASS_POLICY_OTHER`. Deprecated.
    ///
    /// "seatClassPolicyOther"
    #[serde(rename="seatClassPolicyOther")]
    SeatClassPolicyOther,
}

impl AsRef<str> for BoardingAndSeatingPolicySeatClassPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BoardingAndSeatingPolicySeatClassPolicyEnum::SEATCLASSPOLICYUNSPECIFIED => "SEAT_CLASS_POLICY_UNSPECIFIED",
            BoardingAndSeatingPolicySeatClassPolicyEnum::CABINBASED => "CABIN_BASED",
            BoardingAndSeatingPolicySeatClassPolicyEnum::CabinBased => "cabinBased",
            BoardingAndSeatingPolicySeatClassPolicyEnum::CLASSBASED => "CLASS_BASED",
            BoardingAndSeatingPolicySeatClassPolicyEnum::ClassBased => "classBased",
            BoardingAndSeatingPolicySeatClassPolicyEnum::TIERBASED => "TIER_BASED",
            BoardingAndSeatingPolicySeatClassPolicyEnum::TierBased => "tierBased",
            BoardingAndSeatingPolicySeatClassPolicyEnum::SEATCLASSPOLICYOTHER => "SEAT_CLASS_POLICY_OTHER",
            BoardingAndSeatingPolicySeatClassPolicyEnum::SeatClassPolicyOther => "seatClassPolicyOther",
        }
    }
}

impl std::convert::TryFrom< &str> for BoardingAndSeatingPolicySeatClassPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEAT_CLASS_POLICY_UNSPECIFIED" => Ok(BoardingAndSeatingPolicySeatClassPolicyEnum::SEATCLASSPOLICYUNSPECIFIED),
           "CABIN_BASED" => Ok(BoardingAndSeatingPolicySeatClassPolicyEnum::CABINBASED),
           "cabinBased" => Ok(BoardingAndSeatingPolicySeatClassPolicyEnum::CabinBased),
           "CLASS_BASED" => Ok(BoardingAndSeatingPolicySeatClassPolicyEnum::CLASSBASED),
           "classBased" => Ok(BoardingAndSeatingPolicySeatClassPolicyEnum::ClassBased),
           "TIER_BASED" => Ok(BoardingAndSeatingPolicySeatClassPolicyEnum::TIERBASED),
           "tierBased" => Ok(BoardingAndSeatingPolicySeatClassPolicyEnum::TierBased),
           "SEAT_CLASS_POLICY_OTHER" => Ok(BoardingAndSeatingPolicySeatClassPolicyEnum::SEATCLASSPOLICYOTHER),
           "seatClassPolicyOther" => Ok(BoardingAndSeatingPolicySeatClassPolicyEnum::SeatClassPolicyOther),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BoardingAndSeatingPolicySeatClassPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompositeMediaReferenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Describes what the field reference contains.
pub enum CompositeMediaReferenceTypeEnum {
    

    /// Reference contains a GFS path or a local path.
    ///
    /// "PATH"
    #[serde(rename="PATH")]
    PATH,
    

    /// Reference points to a blobstore object. This could be either a v1 blob_ref or a v2 blobstore2_info. Clients should check blobstore2_info first, since v1 is being deprecated.
    ///
    /// "BLOB_REF"
    #[serde(rename="BLOB_REF")]
    BLOBREF,
    

    /// Data is included into this proto buffer
    ///
    /// "INLINE"
    #[serde(rename="INLINE")]
    INLINE,
    

    /// Reference points to a bigstore object
    ///
    /// "BIGSTORE_REF"
    #[serde(rename="BIGSTORE_REF")]
    BIGSTOREREF,
    

    /// Indicates the data is stored in cosmo_binary_reference.
    ///
    /// "COSMO_BINARY_REFERENCE"
    #[serde(rename="COSMO_BINARY_REFERENCE")]
    COSMOBINARYREFERENCE,
}

impl AsRef<str> for CompositeMediaReferenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompositeMediaReferenceTypeEnum::PATH => "PATH",
            CompositeMediaReferenceTypeEnum::BLOBREF => "BLOB_REF",
            CompositeMediaReferenceTypeEnum::INLINE => "INLINE",
            CompositeMediaReferenceTypeEnum::BIGSTOREREF => "BIGSTORE_REF",
            CompositeMediaReferenceTypeEnum::COSMOBINARYREFERENCE => "COSMO_BINARY_REFERENCE",
        }
    }
}

impl std::convert::TryFrom< &str> for CompositeMediaReferenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PATH" => Ok(CompositeMediaReferenceTypeEnum::PATH),
           "BLOB_REF" => Ok(CompositeMediaReferenceTypeEnum::BLOBREF),
           "INLINE" => Ok(CompositeMediaReferenceTypeEnum::INLINE),
           "BIGSTORE_REF" => Ok(CompositeMediaReferenceTypeEnum::BIGSTOREREF),
           "COSMO_BINARY_REFERENCE" => Ok(CompositeMediaReferenceTypeEnum::COSMOBINARYREFERENCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompositeMediaReferenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DiscoverableProgramStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Visibility state of the discoverable program.
pub enum DiscoverableProgramStateEnum {
    
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Visible only to testers that have access to issuer account.
    ///
    /// "TRUSTED_TESTERS"
    #[serde(rename="TRUSTED_TESTERS")]
    TRUSTEDTESTERS,
    

    /// Legacy alias for `TRUSTED_TESTERS`. Deprecated.
    ///
    /// "trustedTesters"
    #[serde(rename="trustedTesters")]
    TrustedTesters,
    

    /// Visible to all.
    ///
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    

    /// Legacy alias for `LIVE`. Deprecated.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// Not visible.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Legacy alias for `DISABLED`. Deprecated.
    ///
    /// "disabled"
    #[serde(rename="disabled")]
    Disabled,
}

impl AsRef<str> for DiscoverableProgramStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DiscoverableProgramStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            DiscoverableProgramStateEnum::TRUSTEDTESTERS => "TRUSTED_TESTERS",
            DiscoverableProgramStateEnum::TrustedTesters => "trustedTesters",
            DiscoverableProgramStateEnum::LIVE => "LIVE",
            DiscoverableProgramStateEnum::Live => "live",
            DiscoverableProgramStateEnum::DISABLED => "DISABLED",
            DiscoverableProgramStateEnum::Disabled => "disabled",
        }
    }
}

impl std::convert::TryFrom< &str> for DiscoverableProgramStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(DiscoverableProgramStateEnum::STATEUNSPECIFIED),
           "TRUSTED_TESTERS" => Ok(DiscoverableProgramStateEnum::TRUSTEDTESTERS),
           "trustedTesters" => Ok(DiscoverableProgramStateEnum::TrustedTesters),
           "LIVE" => Ok(DiscoverableProgramStateEnum::LIVE),
           "live" => Ok(DiscoverableProgramStateEnum::Live),
           "DISABLED" => Ok(DiscoverableProgramStateEnum::DISABLED),
           "disabled" => Ok(DiscoverableProgramStateEnum::Disabled),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DiscoverableProgramStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
///  User data that is sent in a POST request to the signup website URL. This information is encoded and then shared so that the merchant's website can prefill fields used to enroll the user for the discoverable program.
pub enum DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum {
    
    /// "SHARED_DATA_TYPE_UNSPECIFIED"
    #[serde(rename="SHARED_DATA_TYPE_UNSPECIFIED")]
    SHAREDDATATYPEUNSPECIFIED,
    
    /// "FIRST_NAME"
    #[serde(rename="FIRST_NAME")]
    FIRSTNAME,
    
    /// "LAST_NAME"
    #[serde(rename="LAST_NAME")]
    LASTNAME,
    

    /// single line address field
    ///
    /// "STREET_ADDRESS"
    #[serde(rename="STREET_ADDRESS")]
    STREETADDRESS,
    

    /// multi line address fields
    ///
    /// "ADDRESS_LINE_1"
    #[serde(rename="ADDRESS_LINE_1")]
    ADDRESSLINE1,
    
    /// "ADDRESS_LINE_2"
    #[serde(rename="ADDRESS_LINE_2")]
    ADDRESSLINE2,
    
    /// "ADDRESS_LINE_3"
    #[serde(rename="ADDRESS_LINE_3")]
    ADDRESSLINE3,
    
    /// "CITY"
    #[serde(rename="CITY")]
    CITY,
    
    /// "STATE"
    #[serde(rename="STATE")]
    STATE,
    
    /// "ZIPCODE"
    #[serde(rename="ZIPCODE")]
    ZIPCODE,
    
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    
    /// "EMAIL"
    #[serde(rename="EMAIL")]
    EMAIL,
    
    /// "PHONE"
    #[serde(rename="PHONE")]
    PHONE,
}

impl AsRef<str> for DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::SHAREDDATATYPEUNSPECIFIED => "SHARED_DATA_TYPE_UNSPECIFIED",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::FIRSTNAME => "FIRST_NAME",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::LASTNAME => "LAST_NAME",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::STREETADDRESS => "STREET_ADDRESS",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::ADDRESSLINE1 => "ADDRESS_LINE_1",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::ADDRESSLINE2 => "ADDRESS_LINE_2",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::ADDRESSLINE3 => "ADDRESS_LINE_3",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::CITY => "CITY",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::STATE => "STATE",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::ZIPCODE => "ZIPCODE",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::COUNTRY => "COUNTRY",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::EMAIL => "EMAIL",
            DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::PHONE => "PHONE",
        }
    }
}

impl std::convert::TryFrom< &str> for DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SHARED_DATA_TYPE_UNSPECIFIED" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::SHAREDDATATYPEUNSPECIFIED),
           "FIRST_NAME" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::FIRSTNAME),
           "LAST_NAME" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::LASTNAME),
           "STREET_ADDRESS" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::STREETADDRESS),
           "ADDRESS_LINE_1" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::ADDRESSLINE1),
           "ADDRESS_LINE_2" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::ADDRESSLINE2),
           "ADDRESS_LINE_3" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::ADDRESSLINE3),
           "CITY" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::CITY),
           "STATE" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::STATE),
           "ZIPCODE" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::ZIPCODE),
           "COUNTRY" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::COUNTRY),
           "EMAIL" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::EMAIL),
           "PHONE" => Ok(DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum::PHONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventDateTimeDoorsOpenLabelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The label to use for the doors open value (`doorsOpen`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `doorsOpenLabel` and `customDoorsOpenLabel` may not be set. If neither is set, the label will default to "Doors Open", localized. If the doors open field is unset, this label will not be used.
pub enum EventDateTimeDoorsOpenLabelEnum {
    
    /// "DOORS_OPEN_LABEL_UNSPECIFIED"
    #[serde(rename="DOORS_OPEN_LABEL_UNSPECIFIED")]
    DOORSOPENLABELUNSPECIFIED,
    
    /// "DOORS_OPEN"
    #[serde(rename="DOORS_OPEN")]
    DOORSOPEN,
    

    /// Legacy alias for `DOORS_OPEN`. Deprecated.
    ///
    /// "doorsOpen"
    #[serde(rename="doorsOpen")]
    DoorsOpen,
    
    /// "GATES_OPEN"
    #[serde(rename="GATES_OPEN")]
    GATESOPEN,
    

    /// Legacy alias for `GATES_OPEN`. Deprecated.
    ///
    /// "gatesOpen"
    #[serde(rename="gatesOpen")]
    GatesOpen,
}

impl AsRef<str> for EventDateTimeDoorsOpenLabelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventDateTimeDoorsOpenLabelEnum::DOORSOPENLABELUNSPECIFIED => "DOORS_OPEN_LABEL_UNSPECIFIED",
            EventDateTimeDoorsOpenLabelEnum::DOORSOPEN => "DOORS_OPEN",
            EventDateTimeDoorsOpenLabelEnum::DoorsOpen => "doorsOpen",
            EventDateTimeDoorsOpenLabelEnum::GATESOPEN => "GATES_OPEN",
            EventDateTimeDoorsOpenLabelEnum::GatesOpen => "gatesOpen",
        }
    }
}

impl std::convert::TryFrom< &str> for EventDateTimeDoorsOpenLabelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DOORS_OPEN_LABEL_UNSPECIFIED" => Ok(EventDateTimeDoorsOpenLabelEnum::DOORSOPENLABELUNSPECIFIED),
           "DOORS_OPEN" => Ok(EventDateTimeDoorsOpenLabelEnum::DOORSOPEN),
           "doorsOpen" => Ok(EventDateTimeDoorsOpenLabelEnum::DoorsOpen),
           "GATES_OPEN" => Ok(EventDateTimeDoorsOpenLabelEnum::GATESOPEN),
           "gatesOpen" => Ok(EventDateTimeDoorsOpenLabelEnum::GatesOpen),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventDateTimeDoorsOpenLabelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTicketClasConfirmationCodeLabelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used.
pub enum EventTicketClasConfirmationCodeLabelEnum {
    
    /// "CONFIRMATION_CODE_LABEL_UNSPECIFIED"
    #[serde(rename="CONFIRMATION_CODE_LABEL_UNSPECIFIED")]
    CONFIRMATIONCODELABELUNSPECIFIED,
    
    /// "CONFIRMATION_CODE"
    #[serde(rename="CONFIRMATION_CODE")]
    CONFIRMATIONCODE,
    

    /// Legacy alias for `CONFIRMATION_CODE`. Deprecated.
    ///
    /// "confirmationCode"
    #[serde(rename="confirmationCode")]
    ConfirmationCode,
    
    /// "CONFIRMATION_NUMBER"
    #[serde(rename="CONFIRMATION_NUMBER")]
    CONFIRMATIONNUMBER,
    

    /// Legacy alias for `CONFIRMATION_NUMBER`. Deprecated.
    ///
    /// "confirmationNumber"
    #[serde(rename="confirmationNumber")]
    ConfirmationNumber,
    
    /// "ORDER_NUMBER"
    #[serde(rename="ORDER_NUMBER")]
    ORDERNUMBER,
    

    /// Legacy alias for `ORDER_NUMBER`. Deprecated.
    ///
    /// "orderNumber"
    #[serde(rename="orderNumber")]
    OrderNumber,
    
    /// "RESERVATION_NUMBER"
    #[serde(rename="RESERVATION_NUMBER")]
    RESERVATIONNUMBER,
    

    /// Legacy alias for `RESERVATION_NUMBER`. Deprecated.
    ///
    /// "reservationNumber"
    #[serde(rename="reservationNumber")]
    ReservationNumber,
}

impl AsRef<str> for EventTicketClasConfirmationCodeLabelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTicketClasConfirmationCodeLabelEnum::CONFIRMATIONCODELABELUNSPECIFIED => "CONFIRMATION_CODE_LABEL_UNSPECIFIED",
            EventTicketClasConfirmationCodeLabelEnum::CONFIRMATIONCODE => "CONFIRMATION_CODE",
            EventTicketClasConfirmationCodeLabelEnum::ConfirmationCode => "confirmationCode",
            EventTicketClasConfirmationCodeLabelEnum::CONFIRMATIONNUMBER => "CONFIRMATION_NUMBER",
            EventTicketClasConfirmationCodeLabelEnum::ConfirmationNumber => "confirmationNumber",
            EventTicketClasConfirmationCodeLabelEnum::ORDERNUMBER => "ORDER_NUMBER",
            EventTicketClasConfirmationCodeLabelEnum::OrderNumber => "orderNumber",
            EventTicketClasConfirmationCodeLabelEnum::RESERVATIONNUMBER => "RESERVATION_NUMBER",
            EventTicketClasConfirmationCodeLabelEnum::ReservationNumber => "reservationNumber",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTicketClasConfirmationCodeLabelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONFIRMATION_CODE_LABEL_UNSPECIFIED" => Ok(EventTicketClasConfirmationCodeLabelEnum::CONFIRMATIONCODELABELUNSPECIFIED),
           "CONFIRMATION_CODE" => Ok(EventTicketClasConfirmationCodeLabelEnum::CONFIRMATIONCODE),
           "confirmationCode" => Ok(EventTicketClasConfirmationCodeLabelEnum::ConfirmationCode),
           "CONFIRMATION_NUMBER" => Ok(EventTicketClasConfirmationCodeLabelEnum::CONFIRMATIONNUMBER),
           "confirmationNumber" => Ok(EventTicketClasConfirmationCodeLabelEnum::ConfirmationNumber),
           "ORDER_NUMBER" => Ok(EventTicketClasConfirmationCodeLabelEnum::ORDERNUMBER),
           "orderNumber" => Ok(EventTicketClasConfirmationCodeLabelEnum::OrderNumber),
           "RESERVATION_NUMBER" => Ok(EventTicketClasConfirmationCodeLabelEnum::RESERVATIONNUMBER),
           "reservationNumber" => Ok(EventTicketClasConfirmationCodeLabelEnum::ReservationNumber),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTicketClasConfirmationCodeLabelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTicketClasGateLabelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used.
pub enum EventTicketClasGateLabelEnum {
    
    /// "GATE_LABEL_UNSPECIFIED"
    #[serde(rename="GATE_LABEL_UNSPECIFIED")]
    GATELABELUNSPECIFIED,
    
    /// "GATE"
    #[serde(rename="GATE")]
    GATE,
    

    /// Legacy alias for `GATE`. Deprecated.
    ///
    /// "gate"
    #[serde(rename="gate")]
    Gate,
    
    /// "DOOR"
    #[serde(rename="DOOR")]
    DOOR,
    

    /// Legacy alias for `DOOR`. Deprecated.
    ///
    /// "door"
    #[serde(rename="door")]
    Door,
    
    /// "ENTRANCE"
    #[serde(rename="ENTRANCE")]
    ENTRANCE,
    

    /// Legacy alias for `ENTRANCE`. Deprecated.
    ///
    /// "entrance"
    #[serde(rename="entrance")]
    Entrance,
}

impl AsRef<str> for EventTicketClasGateLabelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTicketClasGateLabelEnum::GATELABELUNSPECIFIED => "GATE_LABEL_UNSPECIFIED",
            EventTicketClasGateLabelEnum::GATE => "GATE",
            EventTicketClasGateLabelEnum::Gate => "gate",
            EventTicketClasGateLabelEnum::DOOR => "DOOR",
            EventTicketClasGateLabelEnum::Door => "door",
            EventTicketClasGateLabelEnum::ENTRANCE => "ENTRANCE",
            EventTicketClasGateLabelEnum::Entrance => "entrance",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTicketClasGateLabelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GATE_LABEL_UNSPECIFIED" => Ok(EventTicketClasGateLabelEnum::GATELABELUNSPECIFIED),
           "GATE" => Ok(EventTicketClasGateLabelEnum::GATE),
           "gate" => Ok(EventTicketClasGateLabelEnum::Gate),
           "DOOR" => Ok(EventTicketClasGateLabelEnum::DOOR),
           "door" => Ok(EventTicketClasGateLabelEnum::Door),
           "ENTRANCE" => Ok(EventTicketClasGateLabelEnum::ENTRANCE),
           "entrance" => Ok(EventTicketClasGateLabelEnum::Entrance),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTicketClasGateLabelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Identifies whether multiple users and devices will save the same object referencing this class.
pub enum EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum {
    

    /// Unspecified preference.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The Pass object is shareable by a user and can be saved by any number of different users, and on any number of devices. Partners typically use this setup for passes that do not need to be restricted to a single user or pinned to a single device.
    ///
    /// "MULTIPLE_HOLDERS"
    #[serde(rename="MULTIPLE_HOLDERS")]
    MULTIPLEHOLDERS,
    

    /// An object can only be saved by one user, but this user can view and use it on multiple of their devices. Once the first user saves the object, no other user will be allowed to view or save it.
    ///
    /// "ONE_USER_ALL_DEVICES"
    #[serde(rename="ONE_USER_ALL_DEVICES")]
    ONEUSERALLDEVICES,
    

    /// An object can only be saved by one user on a single device. Intended for use by select partners in limited circumstances. An example use case is a transit ticket that should be "device pinned", meaning it can be saved, viewed and used only by a single user on a single device. Contact support for additional information.
    ///
    /// "ONE_USER_ONE_DEVICE"
    #[serde(rename="ONE_USER_ONE_DEVICE")]
    ONEUSERONEDEVICE,
    

    /// Legacy alias for `MULTIPLE_HOLDERS`. Deprecated.
    ///
    /// "multipleHolders"
    #[serde(rename="multipleHolders")]
    MultipleHolders,
    

    /// Legacy alias for `ONE_USER_ALL_DEVICES`. Deprecated.
    ///
    /// "oneUserAllDevices"
    #[serde(rename="oneUserAllDevices")]
    OneUserAllDevices,
    

    /// Legacy alias for `ONE_USER_ONE_DEVICE`. Deprecated.
    ///
    /// "oneUserOneDevice"
    #[serde(rename="oneUserOneDevice")]
    OneUserOneDevice,
}

impl AsRef<str> for EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS => "MULTIPLE_HOLDERS",
            EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES => "ONE_USER_ALL_DEVICES",
            EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE => "ONE_USER_ONE_DEVICE",
            EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders => "multipleHolders",
            EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices => "oneUserAllDevices",
            EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice => "oneUserOneDevice",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED),
           "MULTIPLE_HOLDERS" => Ok(EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS),
           "ONE_USER_ALL_DEVICES" => Ok(EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES),
           "ONE_USER_ONE_DEVICE" => Ok(EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE),
           "multipleHolders" => Ok(EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders),
           "oneUserAllDevices" => Ok(EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices),
           "oneUserOneDevice" => Ok(EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTicketClasReviewStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
pub enum EventTicketClasReviewStatusEnum {
    
    /// "REVIEW_STATUS_UNSPECIFIED"
    #[serde(rename="REVIEW_STATUS_UNSPECIFIED")]
    REVIEWSTATUSUNSPECIFIED,
    
    /// "UNDER_REVIEW"
    #[serde(rename="UNDER_REVIEW")]
    UNDERREVIEW,
    

    /// Legacy alias for `UNDER_REVIEW`. Deprecated.
    ///
    /// "underReview"
    #[serde(rename="underReview")]
    UnderReview,
    
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// Legacy alias for `APPROVED`. Deprecated.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// Legacy alias for `REJECTED`. Deprecated.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Legacy alias for `DRAFT`. Deprecated.
    ///
    /// "draft"
    #[serde(rename="draft")]
    Draft,
}

impl AsRef<str> for EventTicketClasReviewStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTicketClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED => "REVIEW_STATUS_UNSPECIFIED",
            EventTicketClasReviewStatusEnum::UNDERREVIEW => "UNDER_REVIEW",
            EventTicketClasReviewStatusEnum::UnderReview => "underReview",
            EventTicketClasReviewStatusEnum::APPROVED => "APPROVED",
            EventTicketClasReviewStatusEnum::Approved => "approved",
            EventTicketClasReviewStatusEnum::REJECTED => "REJECTED",
            EventTicketClasReviewStatusEnum::Rejected => "rejected",
            EventTicketClasReviewStatusEnum::DRAFT => "DRAFT",
            EventTicketClasReviewStatusEnum::Draft => "draft",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTicketClasReviewStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVIEW_STATUS_UNSPECIFIED" => Ok(EventTicketClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED),
           "UNDER_REVIEW" => Ok(EventTicketClasReviewStatusEnum::UNDERREVIEW),
           "underReview" => Ok(EventTicketClasReviewStatusEnum::UnderReview),
           "APPROVED" => Ok(EventTicketClasReviewStatusEnum::APPROVED),
           "approved" => Ok(EventTicketClasReviewStatusEnum::Approved),
           "REJECTED" => Ok(EventTicketClasReviewStatusEnum::REJECTED),
           "rejected" => Ok(EventTicketClasReviewStatusEnum::Rejected),
           "DRAFT" => Ok(EventTicketClasReviewStatusEnum::DRAFT),
           "draft" => Ok(EventTicketClasReviewStatusEnum::Draft),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTicketClasReviewStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTicketClasRowLabelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used.
pub enum EventTicketClasRowLabelEnum {
    
    /// "ROW_LABEL_UNSPECIFIED"
    #[serde(rename="ROW_LABEL_UNSPECIFIED")]
    ROWLABELUNSPECIFIED,
    
    /// "ROW"
    #[serde(rename="ROW")]
    ROW,
    

    /// Legacy alias for `ROW`. Deprecated.
    ///
    /// "row"
    #[serde(rename="row")]
    Row,
}

impl AsRef<str> for EventTicketClasRowLabelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTicketClasRowLabelEnum::ROWLABELUNSPECIFIED => "ROW_LABEL_UNSPECIFIED",
            EventTicketClasRowLabelEnum::ROW => "ROW",
            EventTicketClasRowLabelEnum::Row => "row",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTicketClasRowLabelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROW_LABEL_UNSPECIFIED" => Ok(EventTicketClasRowLabelEnum::ROWLABELUNSPECIFIED),
           "ROW" => Ok(EventTicketClasRowLabelEnum::ROW),
           "row" => Ok(EventTicketClasRowLabelEnum::Row),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTicketClasRowLabelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTicketClasSeatLabelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used.
pub enum EventTicketClasSeatLabelEnum {
    
    /// "SEAT_LABEL_UNSPECIFIED"
    #[serde(rename="SEAT_LABEL_UNSPECIFIED")]
    SEATLABELUNSPECIFIED,
    
    /// "SEAT"
    #[serde(rename="SEAT")]
    SEAT,
    

    /// Legacy alias for `SEAT`. Deprecated.
    ///
    /// "seat"
    #[serde(rename="seat")]
    Seat,
}

impl AsRef<str> for EventTicketClasSeatLabelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTicketClasSeatLabelEnum::SEATLABELUNSPECIFIED => "SEAT_LABEL_UNSPECIFIED",
            EventTicketClasSeatLabelEnum::SEAT => "SEAT",
            EventTicketClasSeatLabelEnum::Seat => "seat",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTicketClasSeatLabelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEAT_LABEL_UNSPECIFIED" => Ok(EventTicketClasSeatLabelEnum::SEATLABELUNSPECIFIED),
           "SEAT" => Ok(EventTicketClasSeatLabelEnum::SEAT),
           "seat" => Ok(EventTicketClasSeatLabelEnum::Seat),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTicketClasSeatLabelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTicketClasSectionLabelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used.
pub enum EventTicketClasSectionLabelEnum {
    
    /// "SECTION_LABEL_UNSPECIFIED"
    #[serde(rename="SECTION_LABEL_UNSPECIFIED")]
    SECTIONLABELUNSPECIFIED,
    
    /// "SECTION"
    #[serde(rename="SECTION")]
    SECTION,
    

    /// Legacy alias for `SECTION`. Deprecated.
    ///
    /// "section"
    #[serde(rename="section")]
    Section,
    
    /// "THEATER"
    #[serde(rename="THEATER")]
    THEATER,
    

    /// Legacy alias for `THEATER`. Deprecated.
    ///
    /// "theater"
    #[serde(rename="theater")]
    Theater,
}

impl AsRef<str> for EventTicketClasSectionLabelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTicketClasSectionLabelEnum::SECTIONLABELUNSPECIFIED => "SECTION_LABEL_UNSPECIFIED",
            EventTicketClasSectionLabelEnum::SECTION => "SECTION",
            EventTicketClasSectionLabelEnum::Section => "section",
            EventTicketClasSectionLabelEnum::THEATER => "THEATER",
            EventTicketClasSectionLabelEnum::Theater => "theater",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTicketClasSectionLabelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_LABEL_UNSPECIFIED" => Ok(EventTicketClasSectionLabelEnum::SECTIONLABELUNSPECIFIED),
           "SECTION" => Ok(EventTicketClasSectionLabelEnum::SECTION),
           "section" => Ok(EventTicketClasSectionLabelEnum::Section),
           "THEATER" => Ok(EventTicketClasSectionLabelEnum::THEATER),
           "theater" => Ok(EventTicketClasSectionLabelEnum::Theater),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTicketClasSectionLabelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTicketClasViewUnlockRequirementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// View Unlock Requirement options for the event ticket.
pub enum EventTicketClasViewUnlockRequirementEnum {
    

    /// Default value, same as UNLOCK_NOT_REQUIRED.
    ///
    /// "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED"
    #[serde(rename="VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED")]
    VIEWUNLOCKREQUIREMENTUNSPECIFIED,
    

    /// Default behavior for all the existing Passes if ViewUnlockRequirement is not set.
    ///
    /// "UNLOCK_NOT_REQUIRED"
    #[serde(rename="UNLOCK_NOT_REQUIRED")]
    UNLOCKNOTREQUIRED,
    

    /// Requires the user to unlock their device each time the pass is viewed. If the user removes their device lock after saving the pass, then they will be prompted to create a device lock before the pass can be viewed.
    ///
    /// "UNLOCK_REQUIRED_TO_VIEW"
    #[serde(rename="UNLOCK_REQUIRED_TO_VIEW")]
    UNLOCKREQUIREDTOVIEW,
}

impl AsRef<str> for EventTicketClasViewUnlockRequirementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTicketClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED => "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED",
            EventTicketClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED => "UNLOCK_NOT_REQUIRED",
            EventTicketClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW => "UNLOCK_REQUIRED_TO_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTicketClasViewUnlockRequirementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED" => Ok(EventTicketClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED),
           "UNLOCK_NOT_REQUIRED" => Ok(EventTicketClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED),
           "UNLOCK_REQUIRED_TO_VIEW" => Ok(EventTicketClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTicketClasViewUnlockRequirementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTicketObjectStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
pub enum EventTicketObjectStateEnum {
    
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Object is active and displayed to with other active objects.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Legacy alias for `ACTIVE`. Deprecated.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// Legacy alias for `COMPLETED`. Deprecated.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
    

    /// Object is no longer valid (`validTimeInterval` passed).
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// Legacy alias for `EXPIRED`. Deprecated.
    ///
    /// "expired"
    #[serde(rename="expired")]
    Expired,
    
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// Legacy alias for `INACTIVE`. Deprecated.
    ///
    /// "inactive"
    #[serde(rename="inactive")]
    Inactive,
}

impl AsRef<str> for EventTicketObjectStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTicketObjectStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            EventTicketObjectStateEnum::ACTIVE => "ACTIVE",
            EventTicketObjectStateEnum::Active => "active",
            EventTicketObjectStateEnum::COMPLETED => "COMPLETED",
            EventTicketObjectStateEnum::Completed => "completed",
            EventTicketObjectStateEnum::EXPIRED => "EXPIRED",
            EventTicketObjectStateEnum::Expired => "expired",
            EventTicketObjectStateEnum::INACTIVE => "INACTIVE",
            EventTicketObjectStateEnum::Inactive => "inactive",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTicketObjectStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(EventTicketObjectStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(EventTicketObjectStateEnum::ACTIVE),
           "active" => Ok(EventTicketObjectStateEnum::Active),
           "COMPLETED" => Ok(EventTicketObjectStateEnum::COMPLETED),
           "completed" => Ok(EventTicketObjectStateEnum::Completed),
           "EXPIRED" => Ok(EventTicketObjectStateEnum::EXPIRED),
           "expired" => Ok(EventTicketObjectStateEnum::Expired),
           "INACTIVE" => Ok(EventTicketObjectStateEnum::INACTIVE),
           "inactive" => Ok(EventTicketObjectStateEnum::Inactive),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTicketObjectStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FieldReferenceDateFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Only valid if the `fieldPath` references a date field. Chooses how the date field will be formatted and displayed in the UI.
pub enum FieldReferenceDateFormatEnum {
    

    /// Default option when no format is specified, when selected, no formatting will be applied.
    ///
    /// "DATE_FORMAT_UNSPECIFIED"
    #[serde(rename="DATE_FORMAT_UNSPECIFIED")]
    DATEFORMATUNSPECIFIED,
    

    /// Renders `2018-12-14T13:00:00` as `Dec 14, 1:00 PM` in `en_US`.
    ///
    /// "DATE_TIME"
    #[serde(rename="DATE_TIME")]
    DATETIME,
    

    /// Legacy alias for `DATE_TIME`. Deprecated.
    ///
    /// "dateTime"
    #[serde(rename="dateTime")]
    DateTime,
    

    /// Renders `2018-12-14T13:00:00` as `Dec 14` in `en_US`.
    ///
    /// "DATE_ONLY"
    #[serde(rename="DATE_ONLY")]
    DATEONLY,
    

    /// Legacy alias for `DATE_ONLY`. Deprecated.
    ///
    /// "dateOnly"
    #[serde(rename="dateOnly")]
    DateOnly,
    

    /// Renders `2018-12-14T13:00:00` as `1:00 PM` in `en_US`.
    ///
    /// "TIME_ONLY"
    #[serde(rename="TIME_ONLY")]
    TIMEONLY,
    

    /// Legacy alias for `TIME_ONLY`. Deprecated.
    ///
    /// "timeOnly"
    #[serde(rename="timeOnly")]
    TimeOnly,
    

    /// Renders `2018-12-14T13:00:00` as `Dec 14, 2018, 1:00 PM` in `en_US`.
    ///
    /// "DATE_TIME_YEAR"
    #[serde(rename="DATE_TIME_YEAR")]
    DATETIMEYEAR,
    

    /// Legacy alias for `DATE_TIME_YEAR`. Deprecated.
    ///
    /// "dateTimeYear"
    #[serde(rename="dateTimeYear")]
    DateTimeYear,
    

    /// Renders `2018-12-14T13:00:00` as `Dec 14, 2018` in `en_US`.
    ///
    /// "DATE_YEAR"
    #[serde(rename="DATE_YEAR")]
    DATEYEAR,
    

    /// Legacy alias for `DATE_YEAR`. Deprecated.
    ///
    /// "dateYear"
    #[serde(rename="dateYear")]
    DateYear,
    

    /// Renders `2018-12-14T13:00:00` as `2018-12`.
    ///
    /// "YEAR_MONTH"
    #[serde(rename="YEAR_MONTH")]
    YEARMONTH,
    

    /// Renders `2018-12-14T13:00:00` as `2018-12-14`.
    ///
    /// "YEAR_MONTH_DAY"
    #[serde(rename="YEAR_MONTH_DAY")]
    YEARMONTHDAY,
}

impl AsRef<str> for FieldReferenceDateFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FieldReferenceDateFormatEnum::DATEFORMATUNSPECIFIED => "DATE_FORMAT_UNSPECIFIED",
            FieldReferenceDateFormatEnum::DATETIME => "DATE_TIME",
            FieldReferenceDateFormatEnum::DateTime => "dateTime",
            FieldReferenceDateFormatEnum::DATEONLY => "DATE_ONLY",
            FieldReferenceDateFormatEnum::DateOnly => "dateOnly",
            FieldReferenceDateFormatEnum::TIMEONLY => "TIME_ONLY",
            FieldReferenceDateFormatEnum::TimeOnly => "timeOnly",
            FieldReferenceDateFormatEnum::DATETIMEYEAR => "DATE_TIME_YEAR",
            FieldReferenceDateFormatEnum::DateTimeYear => "dateTimeYear",
            FieldReferenceDateFormatEnum::DATEYEAR => "DATE_YEAR",
            FieldReferenceDateFormatEnum::DateYear => "dateYear",
            FieldReferenceDateFormatEnum::YEARMONTH => "YEAR_MONTH",
            FieldReferenceDateFormatEnum::YEARMONTHDAY => "YEAR_MONTH_DAY",
        }
    }
}

impl std::convert::TryFrom< &str> for FieldReferenceDateFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATE_FORMAT_UNSPECIFIED" => Ok(FieldReferenceDateFormatEnum::DATEFORMATUNSPECIFIED),
           "DATE_TIME" => Ok(FieldReferenceDateFormatEnum::DATETIME),
           "dateTime" => Ok(FieldReferenceDateFormatEnum::DateTime),
           "DATE_ONLY" => Ok(FieldReferenceDateFormatEnum::DATEONLY),
           "dateOnly" => Ok(FieldReferenceDateFormatEnum::DateOnly),
           "TIME_ONLY" => Ok(FieldReferenceDateFormatEnum::TIMEONLY),
           "timeOnly" => Ok(FieldReferenceDateFormatEnum::TimeOnly),
           "DATE_TIME_YEAR" => Ok(FieldReferenceDateFormatEnum::DATETIMEYEAR),
           "dateTimeYear" => Ok(FieldReferenceDateFormatEnum::DateTimeYear),
           "DATE_YEAR" => Ok(FieldReferenceDateFormatEnum::DATEYEAR),
           "dateYear" => Ok(FieldReferenceDateFormatEnum::DateYear),
           "YEAR_MONTH" => Ok(FieldReferenceDateFormatEnum::YEARMONTH),
           "YEAR_MONTH_DAY" => Ok(FieldReferenceDateFormatEnum::YEARMONTHDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FieldReferenceDateFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirstRowOptionTransitOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum FirstRowOptionTransitOptionEnum {
    
    /// "TRANSIT_OPTION_UNSPECIFIED"
    #[serde(rename="TRANSIT_OPTION_UNSPECIFIED")]
    TRANSITOPTIONUNSPECIFIED,
    
    /// "ORIGIN_AND_DESTINATION_NAMES"
    #[serde(rename="ORIGIN_AND_DESTINATION_NAMES")]
    ORIGINANDDESTINATIONNAMES,
    

    /// Legacy alias for `ORIGIN_AND_DESTINATION_NAMES`. Deprecated.
    ///
    /// "originAndDestinationNames"
    #[serde(rename="originAndDestinationNames")]
    OriginAndDestinationNames,
    
    /// "ORIGIN_AND_DESTINATION_CODES"
    #[serde(rename="ORIGIN_AND_DESTINATION_CODES")]
    ORIGINANDDESTINATIONCODES,
    

    /// Legacy alias for `ORIGIN_AND_DESTINATION_CODES`. Deprecated.
    ///
    /// "originAndDestinationCodes"
    #[serde(rename="originAndDestinationCodes")]
    OriginAndDestinationCodes,
    
    /// "ORIGIN_NAME"
    #[serde(rename="ORIGIN_NAME")]
    ORIGINNAME,
    

    /// Legacy alias for `ORIGIN_NAME`. Deprecated.
    ///
    /// "originName"
    #[serde(rename="originName")]
    OriginName,
}

impl AsRef<str> for FirstRowOptionTransitOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirstRowOptionTransitOptionEnum::TRANSITOPTIONUNSPECIFIED => "TRANSIT_OPTION_UNSPECIFIED",
            FirstRowOptionTransitOptionEnum::ORIGINANDDESTINATIONNAMES => "ORIGIN_AND_DESTINATION_NAMES",
            FirstRowOptionTransitOptionEnum::OriginAndDestinationNames => "originAndDestinationNames",
            FirstRowOptionTransitOptionEnum::ORIGINANDDESTINATIONCODES => "ORIGIN_AND_DESTINATION_CODES",
            FirstRowOptionTransitOptionEnum::OriginAndDestinationCodes => "originAndDestinationCodes",
            FirstRowOptionTransitOptionEnum::ORIGINNAME => "ORIGIN_NAME",
            FirstRowOptionTransitOptionEnum::OriginName => "originName",
        }
    }
}

impl std::convert::TryFrom< &str> for FirstRowOptionTransitOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSIT_OPTION_UNSPECIFIED" => Ok(FirstRowOptionTransitOptionEnum::TRANSITOPTIONUNSPECIFIED),
           "ORIGIN_AND_DESTINATION_NAMES" => Ok(FirstRowOptionTransitOptionEnum::ORIGINANDDESTINATIONNAMES),
           "originAndDestinationNames" => Ok(FirstRowOptionTransitOptionEnum::OriginAndDestinationNames),
           "ORIGIN_AND_DESTINATION_CODES" => Ok(FirstRowOptionTransitOptionEnum::ORIGINANDDESTINATIONCODES),
           "originAndDestinationCodes" => Ok(FirstRowOptionTransitOptionEnum::OriginAndDestinationCodes),
           "ORIGIN_NAME" => Ok(FirstRowOptionTransitOptionEnum::ORIGINNAME),
           "originName" => Ok(FirstRowOptionTransitOptionEnum::OriginName),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirstRowOptionTransitOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FlightClasFlightStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of this flight. If unset, Google will compute status based on data from other sources, such as FlightStats, etc. Note: Google-computed status will not be returned in API responses.
pub enum FlightClasFlightStatusEnum {
    
    /// "FLIGHT_STATUS_UNSPECIFIED"
    #[serde(rename="FLIGHT_STATUS_UNSPECIFIED")]
    FLIGHTSTATUSUNSPECIFIED,
    

    /// Flight is on time, early, or delayed.
    ///
    /// "SCHEDULED"
    #[serde(rename="SCHEDULED")]
    SCHEDULED,
    

    /// Legacy alias for `SCHEDULED`. Deprecated.
    ///
    /// "scheduled"
    #[serde(rename="scheduled")]
    Scheduled,
    

    /// Flight is in progress (taxiing, taking off, landing, airborne).
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Legacy alias for `ACTIVE`. Deprecated.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    

    /// Flight landed at the original destination.
    ///
    /// "LANDED"
    #[serde(rename="LANDED")]
    LANDED,
    

    /// Legacy alias for `LANDED`. Deprecated.
    ///
    /// "landed"
    #[serde(rename="landed")]
    Landed,
    

    /// Flight is cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// Legacy alias for `CANCELLED`. Deprecated.
    ///
    /// "cancelled"
    #[serde(rename="cancelled")]
    Cancelled,
    

    /// Flight is airborne but heading to a different airport than the original destination.
    ///
    /// "REDIRECTED"
    #[serde(rename="REDIRECTED")]
    REDIRECTED,
    

    /// Legacy alias for `REDIRECTED`. Deprecated.
    ///
    /// "redirected"
    #[serde(rename="redirected")]
    Redirected,
    

    /// Flight has already landed at a different airport than the original destination.
    ///
    /// "DIVERTED"
    #[serde(rename="DIVERTED")]
    DIVERTED,
    

    /// Legacy alias for `DIVERTED`. Deprecated.
    ///
    /// "diverted"
    #[serde(rename="diverted")]
    Diverted,
}

impl AsRef<str> for FlightClasFlightStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FlightClasFlightStatusEnum::FLIGHTSTATUSUNSPECIFIED => "FLIGHT_STATUS_UNSPECIFIED",
            FlightClasFlightStatusEnum::SCHEDULED => "SCHEDULED",
            FlightClasFlightStatusEnum::Scheduled => "scheduled",
            FlightClasFlightStatusEnum::ACTIVE => "ACTIVE",
            FlightClasFlightStatusEnum::Active => "active",
            FlightClasFlightStatusEnum::LANDED => "LANDED",
            FlightClasFlightStatusEnum::Landed => "landed",
            FlightClasFlightStatusEnum::CANCELLED => "CANCELLED",
            FlightClasFlightStatusEnum::Cancelled => "cancelled",
            FlightClasFlightStatusEnum::REDIRECTED => "REDIRECTED",
            FlightClasFlightStatusEnum::Redirected => "redirected",
            FlightClasFlightStatusEnum::DIVERTED => "DIVERTED",
            FlightClasFlightStatusEnum::Diverted => "diverted",
        }
    }
}

impl std::convert::TryFrom< &str> for FlightClasFlightStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FLIGHT_STATUS_UNSPECIFIED" => Ok(FlightClasFlightStatusEnum::FLIGHTSTATUSUNSPECIFIED),
           "SCHEDULED" => Ok(FlightClasFlightStatusEnum::SCHEDULED),
           "scheduled" => Ok(FlightClasFlightStatusEnum::Scheduled),
           "ACTIVE" => Ok(FlightClasFlightStatusEnum::ACTIVE),
           "active" => Ok(FlightClasFlightStatusEnum::Active),
           "LANDED" => Ok(FlightClasFlightStatusEnum::LANDED),
           "landed" => Ok(FlightClasFlightStatusEnum::Landed),
           "CANCELLED" => Ok(FlightClasFlightStatusEnum::CANCELLED),
           "cancelled" => Ok(FlightClasFlightStatusEnum::Cancelled),
           "REDIRECTED" => Ok(FlightClasFlightStatusEnum::REDIRECTED),
           "redirected" => Ok(FlightClasFlightStatusEnum::Redirected),
           "DIVERTED" => Ok(FlightClasFlightStatusEnum::DIVERTED),
           "diverted" => Ok(FlightClasFlightStatusEnum::Diverted),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FlightClasFlightStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FlightClasMultipleDevicesAndHoldersAllowedStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Identifies whether multiple users and devices will save the same object referencing this class.
pub enum FlightClasMultipleDevicesAndHoldersAllowedStatusEnum {
    

    /// Unspecified preference.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The Pass object is shareable by a user and can be saved by any number of different users, and on any number of devices. Partners typically use this setup for passes that do not need to be restricted to a single user or pinned to a single device.
    ///
    /// "MULTIPLE_HOLDERS"
    #[serde(rename="MULTIPLE_HOLDERS")]
    MULTIPLEHOLDERS,
    

    /// An object can only be saved by one user, but this user can view and use it on multiple of their devices. Once the first user saves the object, no other user will be allowed to view or save it.
    ///
    /// "ONE_USER_ALL_DEVICES"
    #[serde(rename="ONE_USER_ALL_DEVICES")]
    ONEUSERALLDEVICES,
    

    /// An object can only be saved by one user on a single device. Intended for use by select partners in limited circumstances. An example use case is a transit ticket that should be "device pinned", meaning it can be saved, viewed and used only by a single user on a single device. Contact support for additional information.
    ///
    /// "ONE_USER_ONE_DEVICE"
    #[serde(rename="ONE_USER_ONE_DEVICE")]
    ONEUSERONEDEVICE,
    

    /// Legacy alias for `MULTIPLE_HOLDERS`. Deprecated.
    ///
    /// "multipleHolders"
    #[serde(rename="multipleHolders")]
    MultipleHolders,
    

    /// Legacy alias for `ONE_USER_ALL_DEVICES`. Deprecated.
    ///
    /// "oneUserAllDevices"
    #[serde(rename="oneUserAllDevices")]
    OneUserAllDevices,
    

    /// Legacy alias for `ONE_USER_ONE_DEVICE`. Deprecated.
    ///
    /// "oneUserOneDevice"
    #[serde(rename="oneUserOneDevice")]
    OneUserOneDevice,
}

impl AsRef<str> for FlightClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS => "MULTIPLE_HOLDERS",
            FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES => "ONE_USER_ALL_DEVICES",
            FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE => "ONE_USER_ONE_DEVICE",
            FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders => "multipleHolders",
            FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices => "oneUserAllDevices",
            FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice => "oneUserOneDevice",
        }
    }
}

impl std::convert::TryFrom< &str> for FlightClasMultipleDevicesAndHoldersAllowedStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED),
           "MULTIPLE_HOLDERS" => Ok(FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS),
           "ONE_USER_ALL_DEVICES" => Ok(FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES),
           "ONE_USER_ONE_DEVICE" => Ok(FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE),
           "multipleHolders" => Ok(FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders),
           "oneUserAllDevices" => Ok(FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices),
           "oneUserOneDevice" => Ok(FlightClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FlightClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FlightClasReviewStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
pub enum FlightClasReviewStatusEnum {
    
    /// "REVIEW_STATUS_UNSPECIFIED"
    #[serde(rename="REVIEW_STATUS_UNSPECIFIED")]
    REVIEWSTATUSUNSPECIFIED,
    
    /// "UNDER_REVIEW"
    #[serde(rename="UNDER_REVIEW")]
    UNDERREVIEW,
    

    /// Legacy alias for `UNDER_REVIEW`. Deprecated.
    ///
    /// "underReview"
    #[serde(rename="underReview")]
    UnderReview,
    
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// Legacy alias for `APPROVED`. Deprecated.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// Legacy alias for `REJECTED`. Deprecated.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Legacy alias for `DRAFT`. Deprecated.
    ///
    /// "draft"
    #[serde(rename="draft")]
    Draft,
}

impl AsRef<str> for FlightClasReviewStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FlightClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED => "REVIEW_STATUS_UNSPECIFIED",
            FlightClasReviewStatusEnum::UNDERREVIEW => "UNDER_REVIEW",
            FlightClasReviewStatusEnum::UnderReview => "underReview",
            FlightClasReviewStatusEnum::APPROVED => "APPROVED",
            FlightClasReviewStatusEnum::Approved => "approved",
            FlightClasReviewStatusEnum::REJECTED => "REJECTED",
            FlightClasReviewStatusEnum::Rejected => "rejected",
            FlightClasReviewStatusEnum::DRAFT => "DRAFT",
            FlightClasReviewStatusEnum::Draft => "draft",
        }
    }
}

impl std::convert::TryFrom< &str> for FlightClasReviewStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVIEW_STATUS_UNSPECIFIED" => Ok(FlightClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED),
           "UNDER_REVIEW" => Ok(FlightClasReviewStatusEnum::UNDERREVIEW),
           "underReview" => Ok(FlightClasReviewStatusEnum::UnderReview),
           "APPROVED" => Ok(FlightClasReviewStatusEnum::APPROVED),
           "approved" => Ok(FlightClasReviewStatusEnum::Approved),
           "REJECTED" => Ok(FlightClasReviewStatusEnum::REJECTED),
           "rejected" => Ok(FlightClasReviewStatusEnum::Rejected),
           "DRAFT" => Ok(FlightClasReviewStatusEnum::DRAFT),
           "draft" => Ok(FlightClasReviewStatusEnum::Draft),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FlightClasReviewStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FlightClasViewUnlockRequirementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// View Unlock Requirement options for the boarding pass.
pub enum FlightClasViewUnlockRequirementEnum {
    

    /// Default value, same as UNLOCK_NOT_REQUIRED.
    ///
    /// "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED"
    #[serde(rename="VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED")]
    VIEWUNLOCKREQUIREMENTUNSPECIFIED,
    

    /// Default behavior for all the existing Passes if ViewUnlockRequirement is not set.
    ///
    /// "UNLOCK_NOT_REQUIRED"
    #[serde(rename="UNLOCK_NOT_REQUIRED")]
    UNLOCKNOTREQUIRED,
    

    /// Requires the user to unlock their device each time the pass is viewed. If the user removes their device lock after saving the pass, then they will be prompted to create a device lock before the pass can be viewed.
    ///
    /// "UNLOCK_REQUIRED_TO_VIEW"
    #[serde(rename="UNLOCK_REQUIRED_TO_VIEW")]
    UNLOCKREQUIREDTOVIEW,
}

impl AsRef<str> for FlightClasViewUnlockRequirementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FlightClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED => "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED",
            FlightClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED => "UNLOCK_NOT_REQUIRED",
            FlightClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW => "UNLOCK_REQUIRED_TO_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for FlightClasViewUnlockRequirementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED" => Ok(FlightClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED),
           "UNLOCK_NOT_REQUIRED" => Ok(FlightClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED),
           "UNLOCK_REQUIRED_TO_VIEW" => Ok(FlightClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FlightClasViewUnlockRequirementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FlightObjectStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
pub enum FlightObjectStateEnum {
    
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Object is active and displayed to with other active objects.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Legacy alias for `ACTIVE`. Deprecated.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// Legacy alias for `COMPLETED`. Deprecated.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
    

    /// Object is no longer valid (`validTimeInterval` passed).
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// Legacy alias for `EXPIRED`. Deprecated.
    ///
    /// "expired"
    #[serde(rename="expired")]
    Expired,
    
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// Legacy alias for `INACTIVE`. Deprecated.
    ///
    /// "inactive"
    #[serde(rename="inactive")]
    Inactive,
}

impl AsRef<str> for FlightObjectStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FlightObjectStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            FlightObjectStateEnum::ACTIVE => "ACTIVE",
            FlightObjectStateEnum::Active => "active",
            FlightObjectStateEnum::COMPLETED => "COMPLETED",
            FlightObjectStateEnum::Completed => "completed",
            FlightObjectStateEnum::EXPIRED => "EXPIRED",
            FlightObjectStateEnum::Expired => "expired",
            FlightObjectStateEnum::INACTIVE => "INACTIVE",
            FlightObjectStateEnum::Inactive => "inactive",
        }
    }
}

impl std::convert::TryFrom< &str> for FlightObjectStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(FlightObjectStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(FlightObjectStateEnum::ACTIVE),
           "active" => Ok(FlightObjectStateEnum::Active),
           "COMPLETED" => Ok(FlightObjectStateEnum::COMPLETED),
           "completed" => Ok(FlightObjectStateEnum::Completed),
           "EXPIRED" => Ok(FlightObjectStateEnum::EXPIRED),
           "expired" => Ok(FlightObjectStateEnum::Expired),
           "INACTIVE" => Ok(FlightObjectStateEnum::INACTIVE),
           "inactive" => Ok(FlightObjectStateEnum::Inactive),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FlightObjectStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GenericClasMultipleDevicesAndHoldersAllowedStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Identifies whether multiple users and devices will save the same object referencing this class.
pub enum GenericClasMultipleDevicesAndHoldersAllowedStatusEnum {
    

    /// Unspecified preference.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The Pass object is shareable by a user and can be saved by any number of different users, and on any number of devices. Partners typically use this setup for passes that do not need to be restricted to a single user or pinned to a single device.
    ///
    /// "MULTIPLE_HOLDERS"
    #[serde(rename="MULTIPLE_HOLDERS")]
    MULTIPLEHOLDERS,
    

    /// An object can only be saved by one user, but this user can view and use it on multiple of their devices. Once the first user saves the object, no other user will be allowed to view or save it.
    ///
    /// "ONE_USER_ALL_DEVICES"
    #[serde(rename="ONE_USER_ALL_DEVICES")]
    ONEUSERALLDEVICES,
    

    /// An object can only be saved by one user on a single device. Intended for use by select partners in limited circumstances. An example use case is a transit ticket that should be "device pinned", meaning it can be saved, viewed and used only by a single user on a single device. Contact support for additional information.
    ///
    /// "ONE_USER_ONE_DEVICE"
    #[serde(rename="ONE_USER_ONE_DEVICE")]
    ONEUSERONEDEVICE,
    

    /// Legacy alias for `MULTIPLE_HOLDERS`. Deprecated.
    ///
    /// "multipleHolders"
    #[serde(rename="multipleHolders")]
    MultipleHolders,
    

    /// Legacy alias for `ONE_USER_ALL_DEVICES`. Deprecated.
    ///
    /// "oneUserAllDevices"
    #[serde(rename="oneUserAllDevices")]
    OneUserAllDevices,
    

    /// Legacy alias for `ONE_USER_ONE_DEVICE`. Deprecated.
    ///
    /// "oneUserOneDevice"
    #[serde(rename="oneUserOneDevice")]
    OneUserOneDevice,
}

impl AsRef<str> for GenericClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS => "MULTIPLE_HOLDERS",
            GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES => "ONE_USER_ALL_DEVICES",
            GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE => "ONE_USER_ONE_DEVICE",
            GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders => "multipleHolders",
            GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices => "oneUserAllDevices",
            GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice => "oneUserOneDevice",
        }
    }
}

impl std::convert::TryFrom< &str> for GenericClasMultipleDevicesAndHoldersAllowedStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED),
           "MULTIPLE_HOLDERS" => Ok(GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS),
           "ONE_USER_ALL_DEVICES" => Ok(GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES),
           "ONE_USER_ONE_DEVICE" => Ok(GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE),
           "multipleHolders" => Ok(GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders),
           "oneUserAllDevices" => Ok(GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices),
           "oneUserOneDevice" => Ok(GenericClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GenericClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GenericClasViewUnlockRequirementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// View Unlock Requirement options for the generic pass.
pub enum GenericClasViewUnlockRequirementEnum {
    

    /// Default value, same as UNLOCK_NOT_REQUIRED.
    ///
    /// "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED"
    #[serde(rename="VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED")]
    VIEWUNLOCKREQUIREMENTUNSPECIFIED,
    

    /// Default behavior for all the existing Passes if ViewUnlockRequirement is not set.
    ///
    /// "UNLOCK_NOT_REQUIRED"
    #[serde(rename="UNLOCK_NOT_REQUIRED")]
    UNLOCKNOTREQUIRED,
    

    /// Requires the user to unlock their device each time the pass is viewed. If the user removes their device lock after saving the pass, then they will be prompted to create a device lock before the pass can be viewed.
    ///
    /// "UNLOCK_REQUIRED_TO_VIEW"
    #[serde(rename="UNLOCK_REQUIRED_TO_VIEW")]
    UNLOCKREQUIREDTOVIEW,
}

impl AsRef<str> for GenericClasViewUnlockRequirementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GenericClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED => "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED",
            GenericClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED => "UNLOCK_NOT_REQUIRED",
            GenericClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW => "UNLOCK_REQUIRED_TO_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for GenericClasViewUnlockRequirementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED" => Ok(GenericClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED),
           "UNLOCK_NOT_REQUIRED" => Ok(GenericClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED),
           "UNLOCK_REQUIRED_TO_VIEW" => Ok(GenericClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GenericClasViewUnlockRequirementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GenericObjectGenericTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specify which `GenericType` the card belongs to.
pub enum GenericObjectGenericTypeEnum {
    

    /// Unspecified generic type.
    ///
    /// "GENERIC_TYPE_UNSPECIFIED"
    #[serde(rename="GENERIC_TYPE_UNSPECIFIED")]
    GENERICTYPEUNSPECIFIED,
    

    /// Season pass
    ///
    /// "GENERIC_SEASON_PASS"
    #[serde(rename="GENERIC_SEASON_PASS")]
    GENERICSEASONPASS,
    

    /// Utility bills
    ///
    /// "GENERIC_UTILITY_BILLS"
    #[serde(rename="GENERIC_UTILITY_BILLS")]
    GENERICUTILITYBILLS,
    

    /// Parking pass
    ///
    /// "GENERIC_PARKING_PASS"
    #[serde(rename="GENERIC_PARKING_PASS")]
    GENERICPARKINGPASS,
    

    /// Voucher
    ///
    /// "GENERIC_VOUCHER"
    #[serde(rename="GENERIC_VOUCHER")]
    GENERICVOUCHER,
    

    /// Gym membership cards
    ///
    /// "GENERIC_GYM_MEMBERSHIP"
    #[serde(rename="GENERIC_GYM_MEMBERSHIP")]
    GENERICGYMMEMBERSHIP,
    

    /// Library membership cards
    ///
    /// "GENERIC_LIBRARY_MEMBERSHIP"
    #[serde(rename="GENERIC_LIBRARY_MEMBERSHIP")]
    GENERICLIBRARYMEMBERSHIP,
    

    /// Reservations
    ///
    /// "GENERIC_RESERVATIONS"
    #[serde(rename="GENERIC_RESERVATIONS")]
    GENERICRESERVATIONS,
    

    /// Auto-insurance cards
    ///
    /// "GENERIC_AUTO_INSURANCE"
    #[serde(rename="GENERIC_AUTO_INSURANCE")]
    GENERICAUTOINSURANCE,
    

    /// Home-insurance cards
    ///
    /// "GENERIC_HOME_INSURANCE"
    #[serde(rename="GENERIC_HOME_INSURANCE")]
    GENERICHOMEINSURANCE,
    

    /// Entry tickets
    ///
    /// "GENERIC_ENTRY_TICKET"
    #[serde(rename="GENERIC_ENTRY_TICKET")]
    GENERICENTRYTICKET,
    

    /// Receipts
    ///
    /// "GENERIC_RECEIPT"
    #[serde(rename="GENERIC_RECEIPT")]
    GENERICRECEIPT,
    

    /// Other type
    ///
    /// "GENERIC_OTHER"
    #[serde(rename="GENERIC_OTHER")]
    GENERICOTHER,
}

impl AsRef<str> for GenericObjectGenericTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GenericObjectGenericTypeEnum::GENERICTYPEUNSPECIFIED => "GENERIC_TYPE_UNSPECIFIED",
            GenericObjectGenericTypeEnum::GENERICSEASONPASS => "GENERIC_SEASON_PASS",
            GenericObjectGenericTypeEnum::GENERICUTILITYBILLS => "GENERIC_UTILITY_BILLS",
            GenericObjectGenericTypeEnum::GENERICPARKINGPASS => "GENERIC_PARKING_PASS",
            GenericObjectGenericTypeEnum::GENERICVOUCHER => "GENERIC_VOUCHER",
            GenericObjectGenericTypeEnum::GENERICGYMMEMBERSHIP => "GENERIC_GYM_MEMBERSHIP",
            GenericObjectGenericTypeEnum::GENERICLIBRARYMEMBERSHIP => "GENERIC_LIBRARY_MEMBERSHIP",
            GenericObjectGenericTypeEnum::GENERICRESERVATIONS => "GENERIC_RESERVATIONS",
            GenericObjectGenericTypeEnum::GENERICAUTOINSURANCE => "GENERIC_AUTO_INSURANCE",
            GenericObjectGenericTypeEnum::GENERICHOMEINSURANCE => "GENERIC_HOME_INSURANCE",
            GenericObjectGenericTypeEnum::GENERICENTRYTICKET => "GENERIC_ENTRY_TICKET",
            GenericObjectGenericTypeEnum::GENERICRECEIPT => "GENERIC_RECEIPT",
            GenericObjectGenericTypeEnum::GENERICOTHER => "GENERIC_OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for GenericObjectGenericTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GENERIC_TYPE_UNSPECIFIED" => Ok(GenericObjectGenericTypeEnum::GENERICTYPEUNSPECIFIED),
           "GENERIC_SEASON_PASS" => Ok(GenericObjectGenericTypeEnum::GENERICSEASONPASS),
           "GENERIC_UTILITY_BILLS" => Ok(GenericObjectGenericTypeEnum::GENERICUTILITYBILLS),
           "GENERIC_PARKING_PASS" => Ok(GenericObjectGenericTypeEnum::GENERICPARKINGPASS),
           "GENERIC_VOUCHER" => Ok(GenericObjectGenericTypeEnum::GENERICVOUCHER),
           "GENERIC_GYM_MEMBERSHIP" => Ok(GenericObjectGenericTypeEnum::GENERICGYMMEMBERSHIP),
           "GENERIC_LIBRARY_MEMBERSHIP" => Ok(GenericObjectGenericTypeEnum::GENERICLIBRARYMEMBERSHIP),
           "GENERIC_RESERVATIONS" => Ok(GenericObjectGenericTypeEnum::GENERICRESERVATIONS),
           "GENERIC_AUTO_INSURANCE" => Ok(GenericObjectGenericTypeEnum::GENERICAUTOINSURANCE),
           "GENERIC_HOME_INSURANCE" => Ok(GenericObjectGenericTypeEnum::GENERICHOMEINSURANCE),
           "GENERIC_ENTRY_TICKET" => Ok(GenericObjectGenericTypeEnum::GENERICENTRYTICKET),
           "GENERIC_RECEIPT" => Ok(GenericObjectGenericTypeEnum::GENERICRECEIPT),
           "GENERIC_OTHER" => Ok(GenericObjectGenericTypeEnum::GENERICOTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GenericObjectGenericTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GenericObjectStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. If this is not provided, the object would be considered `ACTIVE`.
pub enum GenericObjectStateEnum {
    
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Object is active and displayed to with other active objects.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Legacy alias for `ACTIVE`. Deprecated.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// Legacy alias for `COMPLETED`. Deprecated.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
    

    /// Object is no longer valid (`validTimeInterval` passed).
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// Legacy alias for `EXPIRED`. Deprecated.
    ///
    /// "expired"
    #[serde(rename="expired")]
    Expired,
    
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// Legacy alias for `INACTIVE`. Deprecated.
    ///
    /// "inactive"
    #[serde(rename="inactive")]
    Inactive,
}

impl AsRef<str> for GenericObjectStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GenericObjectStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GenericObjectStateEnum::ACTIVE => "ACTIVE",
            GenericObjectStateEnum::Active => "active",
            GenericObjectStateEnum::COMPLETED => "COMPLETED",
            GenericObjectStateEnum::Completed => "completed",
            GenericObjectStateEnum::EXPIRED => "EXPIRED",
            GenericObjectStateEnum::Expired => "expired",
            GenericObjectStateEnum::INACTIVE => "INACTIVE",
            GenericObjectStateEnum::Inactive => "inactive",
        }
    }
}

impl std::convert::TryFrom< &str> for GenericObjectStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GenericObjectStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GenericObjectStateEnum::ACTIVE),
           "active" => Ok(GenericObjectStateEnum::Active),
           "COMPLETED" => Ok(GenericObjectStateEnum::COMPLETED),
           "completed" => Ok(GenericObjectStateEnum::Completed),
           "EXPIRED" => Ok(GenericObjectStateEnum::EXPIRED),
           "expired" => Ok(GenericObjectStateEnum::Expired),
           "INACTIVE" => Ok(GenericObjectStateEnum::INACTIVE),
           "inactive" => Ok(GenericObjectStateEnum::Inactive),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GenericObjectStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Identifies whether multiple users and devices will save the same object referencing this class.
pub enum GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum {
    

    /// Unspecified preference.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The Pass object is shareable by a user and can be saved by any number of different users, and on any number of devices. Partners typically use this setup for passes that do not need to be restricted to a single user or pinned to a single device.
    ///
    /// "MULTIPLE_HOLDERS"
    #[serde(rename="MULTIPLE_HOLDERS")]
    MULTIPLEHOLDERS,
    

    /// An object can only be saved by one user, but this user can view and use it on multiple of their devices. Once the first user saves the object, no other user will be allowed to view or save it.
    ///
    /// "ONE_USER_ALL_DEVICES"
    #[serde(rename="ONE_USER_ALL_DEVICES")]
    ONEUSERALLDEVICES,
    

    /// An object can only be saved by one user on a single device. Intended for use by select partners in limited circumstances. An example use case is a transit ticket that should be "device pinned", meaning it can be saved, viewed and used only by a single user on a single device. Contact support for additional information.
    ///
    /// "ONE_USER_ONE_DEVICE"
    #[serde(rename="ONE_USER_ONE_DEVICE")]
    ONEUSERONEDEVICE,
    

    /// Legacy alias for `MULTIPLE_HOLDERS`. Deprecated.
    ///
    /// "multipleHolders"
    #[serde(rename="multipleHolders")]
    MultipleHolders,
    

    /// Legacy alias for `ONE_USER_ALL_DEVICES`. Deprecated.
    ///
    /// "oneUserAllDevices"
    #[serde(rename="oneUserAllDevices")]
    OneUserAllDevices,
    

    /// Legacy alias for `ONE_USER_ONE_DEVICE`. Deprecated.
    ///
    /// "oneUserOneDevice"
    #[serde(rename="oneUserOneDevice")]
    OneUserOneDevice,
}

impl AsRef<str> for GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS => "MULTIPLE_HOLDERS",
            GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES => "ONE_USER_ALL_DEVICES",
            GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE => "ONE_USER_ONE_DEVICE",
            GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders => "multipleHolders",
            GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices => "oneUserAllDevices",
            GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice => "oneUserOneDevice",
        }
    }
}

impl std::convert::TryFrom< &str> for GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED),
           "MULTIPLE_HOLDERS" => Ok(GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS),
           "ONE_USER_ALL_DEVICES" => Ok(GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES),
           "ONE_USER_ONE_DEVICE" => Ok(GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE),
           "multipleHolders" => Ok(GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders),
           "oneUserAllDevices" => Ok(GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices),
           "oneUserOneDevice" => Ok(GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GiftCardClasReviewStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
pub enum GiftCardClasReviewStatusEnum {
    
    /// "REVIEW_STATUS_UNSPECIFIED"
    #[serde(rename="REVIEW_STATUS_UNSPECIFIED")]
    REVIEWSTATUSUNSPECIFIED,
    
    /// "UNDER_REVIEW"
    #[serde(rename="UNDER_REVIEW")]
    UNDERREVIEW,
    

    /// Legacy alias for `UNDER_REVIEW`. Deprecated.
    ///
    /// "underReview"
    #[serde(rename="underReview")]
    UnderReview,
    
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// Legacy alias for `APPROVED`. Deprecated.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// Legacy alias for `REJECTED`. Deprecated.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Legacy alias for `DRAFT`. Deprecated.
    ///
    /// "draft"
    #[serde(rename="draft")]
    Draft,
}

impl AsRef<str> for GiftCardClasReviewStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GiftCardClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED => "REVIEW_STATUS_UNSPECIFIED",
            GiftCardClasReviewStatusEnum::UNDERREVIEW => "UNDER_REVIEW",
            GiftCardClasReviewStatusEnum::UnderReview => "underReview",
            GiftCardClasReviewStatusEnum::APPROVED => "APPROVED",
            GiftCardClasReviewStatusEnum::Approved => "approved",
            GiftCardClasReviewStatusEnum::REJECTED => "REJECTED",
            GiftCardClasReviewStatusEnum::Rejected => "rejected",
            GiftCardClasReviewStatusEnum::DRAFT => "DRAFT",
            GiftCardClasReviewStatusEnum::Draft => "draft",
        }
    }
}

impl std::convert::TryFrom< &str> for GiftCardClasReviewStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVIEW_STATUS_UNSPECIFIED" => Ok(GiftCardClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED),
           "UNDER_REVIEW" => Ok(GiftCardClasReviewStatusEnum::UNDERREVIEW),
           "underReview" => Ok(GiftCardClasReviewStatusEnum::UnderReview),
           "APPROVED" => Ok(GiftCardClasReviewStatusEnum::APPROVED),
           "approved" => Ok(GiftCardClasReviewStatusEnum::Approved),
           "REJECTED" => Ok(GiftCardClasReviewStatusEnum::REJECTED),
           "rejected" => Ok(GiftCardClasReviewStatusEnum::Rejected),
           "DRAFT" => Ok(GiftCardClasReviewStatusEnum::DRAFT),
           "draft" => Ok(GiftCardClasReviewStatusEnum::Draft),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GiftCardClasReviewStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GiftCardClasViewUnlockRequirementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// View Unlock Requirement options for the gift card.
pub enum GiftCardClasViewUnlockRequirementEnum {
    

    /// Default value, same as UNLOCK_NOT_REQUIRED.
    ///
    /// "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED"
    #[serde(rename="VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED")]
    VIEWUNLOCKREQUIREMENTUNSPECIFIED,
    

    /// Default behavior for all the existing Passes if ViewUnlockRequirement is not set.
    ///
    /// "UNLOCK_NOT_REQUIRED"
    #[serde(rename="UNLOCK_NOT_REQUIRED")]
    UNLOCKNOTREQUIRED,
    

    /// Requires the user to unlock their device each time the pass is viewed. If the user removes their device lock after saving the pass, then they will be prompted to create a device lock before the pass can be viewed.
    ///
    /// "UNLOCK_REQUIRED_TO_VIEW"
    #[serde(rename="UNLOCK_REQUIRED_TO_VIEW")]
    UNLOCKREQUIREDTOVIEW,
}

impl AsRef<str> for GiftCardClasViewUnlockRequirementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GiftCardClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED => "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED",
            GiftCardClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED => "UNLOCK_NOT_REQUIRED",
            GiftCardClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW => "UNLOCK_REQUIRED_TO_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for GiftCardClasViewUnlockRequirementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED" => Ok(GiftCardClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED),
           "UNLOCK_NOT_REQUIRED" => Ok(GiftCardClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED),
           "UNLOCK_REQUIRED_TO_VIEW" => Ok(GiftCardClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GiftCardClasViewUnlockRequirementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GiftCardObjectStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
pub enum GiftCardObjectStateEnum {
    
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Object is active and displayed to with other active objects.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Legacy alias for `ACTIVE`. Deprecated.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// Legacy alias for `COMPLETED`. Deprecated.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
    

    /// Object is no longer valid (`validTimeInterval` passed).
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// Legacy alias for `EXPIRED`. Deprecated.
    ///
    /// "expired"
    #[serde(rename="expired")]
    Expired,
    
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// Legacy alias for `INACTIVE`. Deprecated.
    ///
    /// "inactive"
    #[serde(rename="inactive")]
    Inactive,
}

impl AsRef<str> for GiftCardObjectStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GiftCardObjectStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GiftCardObjectStateEnum::ACTIVE => "ACTIVE",
            GiftCardObjectStateEnum::Active => "active",
            GiftCardObjectStateEnum::COMPLETED => "COMPLETED",
            GiftCardObjectStateEnum::Completed => "completed",
            GiftCardObjectStateEnum::EXPIRED => "EXPIRED",
            GiftCardObjectStateEnum::Expired => "expired",
            GiftCardObjectStateEnum::INACTIVE => "INACTIVE",
            GiftCardObjectStateEnum::Inactive => "inactive",
        }
    }
}

impl std::convert::TryFrom< &str> for GiftCardObjectStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GiftCardObjectStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GiftCardObjectStateEnum::ACTIVE),
           "active" => Ok(GiftCardObjectStateEnum::Active),
           "COMPLETED" => Ok(GiftCardObjectStateEnum::COMPLETED),
           "completed" => Ok(GiftCardObjectStateEnum::Completed),
           "EXPIRED" => Ok(GiftCardObjectStateEnum::EXPIRED),
           "expired" => Ok(GiftCardObjectStateEnum::Expired),
           "INACTIVE" => Ok(GiftCardObjectStateEnum::INACTIVE),
           "inactive" => Ok(GiftCardObjectStateEnum::Inactive),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GiftCardObjectStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IssuerToUserInfoActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum IssuerToUserInfoActionEnum {
    
    /// "ACTION_UNSPECIFIED"
    #[serde(rename="ACTION_UNSPECIFIED")]
    ACTIONUNSPECIFIED,
    
    /// "S2AP"
    #[serde(rename="S2AP")]
    S2AP,
    

    /// Legacy alias for `S2AP`. Deprecated.
    ///
    /// "s2ap"
    #[serde(rename="s2ap")]
    S2ap,
    
    /// "SIGN_UP"
    #[serde(rename="SIGN_UP")]
    SIGNUP,
    

    /// Legacy alias for `SIGN_UP`. Deprecated.
    ///
    /// "signUp"
    #[serde(rename="signUp")]
    SignUp,
}

impl AsRef<str> for IssuerToUserInfoActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IssuerToUserInfoActionEnum::ACTIONUNSPECIFIED => "ACTION_UNSPECIFIED",
            IssuerToUserInfoActionEnum::S2AP => "S2AP",
            IssuerToUserInfoActionEnum::S2ap => "s2ap",
            IssuerToUserInfoActionEnum::SIGNUP => "SIGN_UP",
            IssuerToUserInfoActionEnum::SignUp => "signUp",
        }
    }
}

impl std::convert::TryFrom< &str> for IssuerToUserInfoActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_UNSPECIFIED" => Ok(IssuerToUserInfoActionEnum::ACTIONUNSPECIFIED),
           "S2AP" => Ok(IssuerToUserInfoActionEnum::S2AP),
           "s2ap" => Ok(IssuerToUserInfoActionEnum::S2ap),
           "SIGN_UP" => Ok(IssuerToUserInfoActionEnum::SIGNUP),
           "signUp" => Ok(IssuerToUserInfoActionEnum::SignUp),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IssuerToUserInfoActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Identifies whether multiple users and devices will save the same object referencing this class.
pub enum LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum {
    

    /// Unspecified preference.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The Pass object is shareable by a user and can be saved by any number of different users, and on any number of devices. Partners typically use this setup for passes that do not need to be restricted to a single user or pinned to a single device.
    ///
    /// "MULTIPLE_HOLDERS"
    #[serde(rename="MULTIPLE_HOLDERS")]
    MULTIPLEHOLDERS,
    

    /// An object can only be saved by one user, but this user can view and use it on multiple of their devices. Once the first user saves the object, no other user will be allowed to view or save it.
    ///
    /// "ONE_USER_ALL_DEVICES"
    #[serde(rename="ONE_USER_ALL_DEVICES")]
    ONEUSERALLDEVICES,
    

    /// An object can only be saved by one user on a single device. Intended for use by select partners in limited circumstances. An example use case is a transit ticket that should be "device pinned", meaning it can be saved, viewed and used only by a single user on a single device. Contact support for additional information.
    ///
    /// "ONE_USER_ONE_DEVICE"
    #[serde(rename="ONE_USER_ONE_DEVICE")]
    ONEUSERONEDEVICE,
    

    /// Legacy alias for `MULTIPLE_HOLDERS`. Deprecated.
    ///
    /// "multipleHolders"
    #[serde(rename="multipleHolders")]
    MultipleHolders,
    

    /// Legacy alias for `ONE_USER_ALL_DEVICES`. Deprecated.
    ///
    /// "oneUserAllDevices"
    #[serde(rename="oneUserAllDevices")]
    OneUserAllDevices,
    

    /// Legacy alias for `ONE_USER_ONE_DEVICE`. Deprecated.
    ///
    /// "oneUserOneDevice"
    #[serde(rename="oneUserOneDevice")]
    OneUserOneDevice,
}

impl AsRef<str> for LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS => "MULTIPLE_HOLDERS",
            LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES => "ONE_USER_ALL_DEVICES",
            LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE => "ONE_USER_ONE_DEVICE",
            LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders => "multipleHolders",
            LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices => "oneUserAllDevices",
            LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice => "oneUserOneDevice",
        }
    }
}

impl std::convert::TryFrom< &str> for LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED),
           "MULTIPLE_HOLDERS" => Ok(LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS),
           "ONE_USER_ALL_DEVICES" => Ok(LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES),
           "ONE_USER_ONE_DEVICE" => Ok(LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE),
           "multipleHolders" => Ok(LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders),
           "oneUserAllDevices" => Ok(LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices),
           "oneUserOneDevice" => Ok(LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoyaltyClasReviewStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
pub enum LoyaltyClasReviewStatusEnum {
    
    /// "REVIEW_STATUS_UNSPECIFIED"
    #[serde(rename="REVIEW_STATUS_UNSPECIFIED")]
    REVIEWSTATUSUNSPECIFIED,
    
    /// "UNDER_REVIEW"
    #[serde(rename="UNDER_REVIEW")]
    UNDERREVIEW,
    

    /// Legacy alias for `UNDER_REVIEW`. Deprecated.
    ///
    /// "underReview"
    #[serde(rename="underReview")]
    UnderReview,
    
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// Legacy alias for `APPROVED`. Deprecated.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// Legacy alias for `REJECTED`. Deprecated.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Legacy alias for `DRAFT`. Deprecated.
    ///
    /// "draft"
    #[serde(rename="draft")]
    Draft,
}

impl AsRef<str> for LoyaltyClasReviewStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoyaltyClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED => "REVIEW_STATUS_UNSPECIFIED",
            LoyaltyClasReviewStatusEnum::UNDERREVIEW => "UNDER_REVIEW",
            LoyaltyClasReviewStatusEnum::UnderReview => "underReview",
            LoyaltyClasReviewStatusEnum::APPROVED => "APPROVED",
            LoyaltyClasReviewStatusEnum::Approved => "approved",
            LoyaltyClasReviewStatusEnum::REJECTED => "REJECTED",
            LoyaltyClasReviewStatusEnum::Rejected => "rejected",
            LoyaltyClasReviewStatusEnum::DRAFT => "DRAFT",
            LoyaltyClasReviewStatusEnum::Draft => "draft",
        }
    }
}

impl std::convert::TryFrom< &str> for LoyaltyClasReviewStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVIEW_STATUS_UNSPECIFIED" => Ok(LoyaltyClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED),
           "UNDER_REVIEW" => Ok(LoyaltyClasReviewStatusEnum::UNDERREVIEW),
           "underReview" => Ok(LoyaltyClasReviewStatusEnum::UnderReview),
           "APPROVED" => Ok(LoyaltyClasReviewStatusEnum::APPROVED),
           "approved" => Ok(LoyaltyClasReviewStatusEnum::Approved),
           "REJECTED" => Ok(LoyaltyClasReviewStatusEnum::REJECTED),
           "rejected" => Ok(LoyaltyClasReviewStatusEnum::Rejected),
           "DRAFT" => Ok(LoyaltyClasReviewStatusEnum::DRAFT),
           "draft" => Ok(LoyaltyClasReviewStatusEnum::Draft),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoyaltyClasReviewStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoyaltyClasViewUnlockRequirementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// View Unlock Requirement options for the loyalty card.
pub enum LoyaltyClasViewUnlockRequirementEnum {
    

    /// Default value, same as UNLOCK_NOT_REQUIRED.
    ///
    /// "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED"
    #[serde(rename="VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED")]
    VIEWUNLOCKREQUIREMENTUNSPECIFIED,
    

    /// Default behavior for all the existing Passes if ViewUnlockRequirement is not set.
    ///
    /// "UNLOCK_NOT_REQUIRED"
    #[serde(rename="UNLOCK_NOT_REQUIRED")]
    UNLOCKNOTREQUIRED,
    

    /// Requires the user to unlock their device each time the pass is viewed. If the user removes their device lock after saving the pass, then they will be prompted to create a device lock before the pass can be viewed.
    ///
    /// "UNLOCK_REQUIRED_TO_VIEW"
    #[serde(rename="UNLOCK_REQUIRED_TO_VIEW")]
    UNLOCKREQUIREDTOVIEW,
}

impl AsRef<str> for LoyaltyClasViewUnlockRequirementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoyaltyClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED => "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED",
            LoyaltyClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED => "UNLOCK_NOT_REQUIRED",
            LoyaltyClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW => "UNLOCK_REQUIRED_TO_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for LoyaltyClasViewUnlockRequirementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED" => Ok(LoyaltyClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED),
           "UNLOCK_NOT_REQUIRED" => Ok(LoyaltyClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED),
           "UNLOCK_REQUIRED_TO_VIEW" => Ok(LoyaltyClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoyaltyClasViewUnlockRequirementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoyaltyObjectStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
pub enum LoyaltyObjectStateEnum {
    
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Object is active and displayed to with other active objects.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Legacy alias for `ACTIVE`. Deprecated.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// Legacy alias for `COMPLETED`. Deprecated.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
    

    /// Object is no longer valid (`validTimeInterval` passed).
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// Legacy alias for `EXPIRED`. Deprecated.
    ///
    /// "expired"
    #[serde(rename="expired")]
    Expired,
    
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// Legacy alias for `INACTIVE`. Deprecated.
    ///
    /// "inactive"
    #[serde(rename="inactive")]
    Inactive,
}

impl AsRef<str> for LoyaltyObjectStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoyaltyObjectStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            LoyaltyObjectStateEnum::ACTIVE => "ACTIVE",
            LoyaltyObjectStateEnum::Active => "active",
            LoyaltyObjectStateEnum::COMPLETED => "COMPLETED",
            LoyaltyObjectStateEnum::Completed => "completed",
            LoyaltyObjectStateEnum::EXPIRED => "EXPIRED",
            LoyaltyObjectStateEnum::Expired => "expired",
            LoyaltyObjectStateEnum::INACTIVE => "INACTIVE",
            LoyaltyObjectStateEnum::Inactive => "inactive",
        }
    }
}

impl std::convert::TryFrom< &str> for LoyaltyObjectStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(LoyaltyObjectStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(LoyaltyObjectStateEnum::ACTIVE),
           "active" => Ok(LoyaltyObjectStateEnum::Active),
           "COMPLETED" => Ok(LoyaltyObjectStateEnum::COMPLETED),
           "completed" => Ok(LoyaltyObjectStateEnum::Completed),
           "EXPIRED" => Ok(LoyaltyObjectStateEnum::EXPIRED),
           "expired" => Ok(LoyaltyObjectStateEnum::Expired),
           "INACTIVE" => Ok(LoyaltyObjectStateEnum::INACTIVE),
           "inactive" => Ok(LoyaltyObjectStateEnum::Inactive),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoyaltyObjectStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediaReferenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Describes what the field reference contains.
pub enum MediaReferenceTypeEnum {
    

    /// Reference contains a GFS path or a local path.
    ///
    /// "PATH"
    #[serde(rename="PATH")]
    PATH,
    

    /// Reference points to a blobstore object. This could be either a v1 blob_ref or a v2 blobstore2_info. Clients should check blobstore2_info first, since v1 is being deprecated.
    ///
    /// "BLOB_REF"
    #[serde(rename="BLOB_REF")]
    BLOBREF,
    

    /// Data is included into this proto buffer
    ///
    /// "INLINE"
    #[serde(rename="INLINE")]
    INLINE,
    

    /// Data should be accessed from the current service using the operation GetMedia.
    ///
    /// "GET_MEDIA"
    #[serde(rename="GET_MEDIA")]
    GETMEDIA,
    

    /// The content for this media object is stored across multiple partial media objects under the composite_media field.
    ///
    /// "COMPOSITE_MEDIA"
    #[serde(rename="COMPOSITE_MEDIA")]
    COMPOSITEMEDIA,
    

    /// Reference points to a bigstore object
    ///
    /// "BIGSTORE_REF"
    #[serde(rename="BIGSTORE_REF")]
    BIGSTOREREF,
    

    /// Indicates the data is stored in diff_version_response.
    ///
    /// "DIFF_VERSION_RESPONSE"
    #[serde(rename="DIFF_VERSION_RESPONSE")]
    DIFFVERSIONRESPONSE,
    

    /// Indicates the data is stored in diff_checksums_response.
    ///
    /// "DIFF_CHECKSUMS_RESPONSE"
    #[serde(rename="DIFF_CHECKSUMS_RESPONSE")]
    DIFFCHECKSUMSRESPONSE,
    

    /// Indicates the data is stored in diff_download_response.
    ///
    /// "DIFF_DOWNLOAD_RESPONSE"
    #[serde(rename="DIFF_DOWNLOAD_RESPONSE")]
    DIFFDOWNLOADRESPONSE,
    

    /// Indicates the data is stored in diff_upload_request.
    ///
    /// "DIFF_UPLOAD_REQUEST"
    #[serde(rename="DIFF_UPLOAD_REQUEST")]
    DIFFUPLOADREQUEST,
    

    /// Indicates the data is stored in diff_upload_response.
    ///
    /// "DIFF_UPLOAD_RESPONSE"
    #[serde(rename="DIFF_UPLOAD_RESPONSE")]
    DIFFUPLOADRESPONSE,
    

    /// Indicates the data is stored in cosmo_binary_reference.
    ///
    /// "COSMO_BINARY_REFERENCE"
    #[serde(rename="COSMO_BINARY_REFERENCE")]
    COSMOBINARYREFERENCE,
    

    /// Informs Scotty to generate a response payload with the size specified in the length field. The contents of the payload are generated by Scotty and are undefined. This is useful for testing download speeds between the user and Scotty without involving a real payload source. Note: range is not supported when using arbitrary_bytes.
    ///
    /// "ARBITRARY_BYTES"
    #[serde(rename="ARBITRARY_BYTES")]
    ARBITRARYBYTES,
}

impl AsRef<str> for MediaReferenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediaReferenceTypeEnum::PATH => "PATH",
            MediaReferenceTypeEnum::BLOBREF => "BLOB_REF",
            MediaReferenceTypeEnum::INLINE => "INLINE",
            MediaReferenceTypeEnum::GETMEDIA => "GET_MEDIA",
            MediaReferenceTypeEnum::COMPOSITEMEDIA => "COMPOSITE_MEDIA",
            MediaReferenceTypeEnum::BIGSTOREREF => "BIGSTORE_REF",
            MediaReferenceTypeEnum::DIFFVERSIONRESPONSE => "DIFF_VERSION_RESPONSE",
            MediaReferenceTypeEnum::DIFFCHECKSUMSRESPONSE => "DIFF_CHECKSUMS_RESPONSE",
            MediaReferenceTypeEnum::DIFFDOWNLOADRESPONSE => "DIFF_DOWNLOAD_RESPONSE",
            MediaReferenceTypeEnum::DIFFUPLOADREQUEST => "DIFF_UPLOAD_REQUEST",
            MediaReferenceTypeEnum::DIFFUPLOADRESPONSE => "DIFF_UPLOAD_RESPONSE",
            MediaReferenceTypeEnum::COSMOBINARYREFERENCE => "COSMO_BINARY_REFERENCE",
            MediaReferenceTypeEnum::ARBITRARYBYTES => "ARBITRARY_BYTES",
        }
    }
}

impl std::convert::TryFrom< &str> for MediaReferenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PATH" => Ok(MediaReferenceTypeEnum::PATH),
           "BLOB_REF" => Ok(MediaReferenceTypeEnum::BLOBREF),
           "INLINE" => Ok(MediaReferenceTypeEnum::INLINE),
           "GET_MEDIA" => Ok(MediaReferenceTypeEnum::GETMEDIA),
           "COMPOSITE_MEDIA" => Ok(MediaReferenceTypeEnum::COMPOSITEMEDIA),
           "BIGSTORE_REF" => Ok(MediaReferenceTypeEnum::BIGSTOREREF),
           "DIFF_VERSION_RESPONSE" => Ok(MediaReferenceTypeEnum::DIFFVERSIONRESPONSE),
           "DIFF_CHECKSUMS_RESPONSE" => Ok(MediaReferenceTypeEnum::DIFFCHECKSUMSRESPONSE),
           "DIFF_DOWNLOAD_RESPONSE" => Ok(MediaReferenceTypeEnum::DIFFDOWNLOADRESPONSE),
           "DIFF_UPLOAD_REQUEST" => Ok(MediaReferenceTypeEnum::DIFFUPLOADREQUEST),
           "DIFF_UPLOAD_RESPONSE" => Ok(MediaReferenceTypeEnum::DIFFUPLOADRESPONSE),
           "COSMO_BINARY_REFERENCE" => Ok(MediaReferenceTypeEnum::COSMOBINARYREFERENCE),
           "ARBITRARY_BYTES" => Ok(MediaReferenceTypeEnum::ARBITRARYBYTES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediaReferenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediaRequestInfoNotificationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of notification received from Scotty.
pub enum MediaRequestInfoNotificationTypeEnum {
    

    /// Such requests signals the start of a request containing media upload. Only the media field(s) in the inserted/updated resource are set. The response should either return an error or succeed. On success, responses don't need to contain anything.
    ///
    /// "START"
    #[serde(rename="START")]
    START,
    

    /// Such requests signals that the upload has progressed and that the backend might want to access the media file specified in relevant fields in the resource. Only the media field(s) in the inserted/updated resource are set. The response should either return an error or succeed. On success, responses don't need to contain anything.
    ///
    /// "PROGRESS"
    #[serde(rename="PROGRESS")]
    PROGRESS,
    

    /// Such requests signals the end of a request containing media upload. END should be handled just like normal Insert/Upload requests, that is, they should process the request and return a complete resource in the response. Pointers to media data (a GFS path usually) appear in the relevant fields in the inserted/updated resource. See gdata.Media in data.proto.
    ///
    /// "END"
    #[serde(rename="END")]
    END,
    

    /// Such requests occur after an END and signal that the response has been sent back to the client. RESPONSE_SENT is only sent to the backend if it is configured to receive them. The response does not need to contain anything.
    ///
    /// "RESPONSE_SENT"
    #[serde(rename="RESPONSE_SENT")]
    RESPONSESENT,
    

    /// Such requests indicate that an error occurred while processing the request. ERROR is only sent to the backend if it is configured to receive them. It is not guaranteed that all errors will result in this notification to the backend, even if the backend requests them. Since these requests are just for informational purposes, the response does not need to contain anything.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for MediaRequestInfoNotificationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediaRequestInfoNotificationTypeEnum::START => "START",
            MediaRequestInfoNotificationTypeEnum::PROGRESS => "PROGRESS",
            MediaRequestInfoNotificationTypeEnum::END => "END",
            MediaRequestInfoNotificationTypeEnum::RESPONSESENT => "RESPONSE_SENT",
            MediaRequestInfoNotificationTypeEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for MediaRequestInfoNotificationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "START" => Ok(MediaRequestInfoNotificationTypeEnum::START),
           "PROGRESS" => Ok(MediaRequestInfoNotificationTypeEnum::PROGRESS),
           "END" => Ok(MediaRequestInfoNotificationTypeEnum::END),
           "RESPONSE_SENT" => Ok(MediaRequestInfoNotificationTypeEnum::RESPONSESENT),
           "ERROR" => Ok(MediaRequestInfoNotificationTypeEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediaRequestInfoNotificationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MessageMessageTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The message type.
pub enum MessageMessageTypeEnum {
    
    /// "MESSAGE_TYPE_UNSPECIFIED"
    #[serde(rename="MESSAGE_TYPE_UNSPECIFIED")]
    MESSAGETYPEUNSPECIFIED,
    

    /// Renders the message as text on the card details screen. This is the default message type.
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// Legacy alias for `TEXT`. Deprecated.
    ///
    /// "text"
    #[serde(rename="text")]
    Text,
    

    /// Note: This enum is currently not supported.
    ///
    /// "EXPIRATION_NOTIFICATION"
    #[serde(rename="EXPIRATION_NOTIFICATION")]
    EXPIRATIONNOTIFICATION,
    

    /// Legacy alias for `EXPIRATION_NOTIFICATION`. Deprecated.
    ///
    /// "expirationNotification"
    #[serde(rename="expirationNotification")]
    ExpirationNotification,
}

impl AsRef<str> for MessageMessageTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MessageMessageTypeEnum::MESSAGETYPEUNSPECIFIED => "MESSAGE_TYPE_UNSPECIFIED",
            MessageMessageTypeEnum::TEXT => "TEXT",
            MessageMessageTypeEnum::Text => "text",
            MessageMessageTypeEnum::EXPIRATIONNOTIFICATION => "EXPIRATION_NOTIFICATION",
            MessageMessageTypeEnum::ExpirationNotification => "expirationNotification",
        }
    }
}

impl std::convert::TryFrom< &str> for MessageMessageTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MESSAGE_TYPE_UNSPECIFIED" => Ok(MessageMessageTypeEnum::MESSAGETYPEUNSPECIFIED),
           "TEXT" => Ok(MessageMessageTypeEnum::TEXT),
           "text" => Ok(MessageMessageTypeEnum::Text),
           "EXPIRATION_NOTIFICATION" => Ok(MessageMessageTypeEnum::EXPIRATIONNOTIFICATION),
           "expirationNotification" => Ok(MessageMessageTypeEnum::ExpirationNotification),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MessageMessageTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OfferClasMultipleDevicesAndHoldersAllowedStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Identifies whether multiple users and devices will save the same object referencing this class.
pub enum OfferClasMultipleDevicesAndHoldersAllowedStatusEnum {
    

    /// Unspecified preference.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The Pass object is shareable by a user and can be saved by any number of different users, and on any number of devices. Partners typically use this setup for passes that do not need to be restricted to a single user or pinned to a single device.
    ///
    /// "MULTIPLE_HOLDERS"
    #[serde(rename="MULTIPLE_HOLDERS")]
    MULTIPLEHOLDERS,
    

    /// An object can only be saved by one user, but this user can view and use it on multiple of their devices. Once the first user saves the object, no other user will be allowed to view or save it.
    ///
    /// "ONE_USER_ALL_DEVICES"
    #[serde(rename="ONE_USER_ALL_DEVICES")]
    ONEUSERALLDEVICES,
    

    /// An object can only be saved by one user on a single device. Intended for use by select partners in limited circumstances. An example use case is a transit ticket that should be "device pinned", meaning it can be saved, viewed and used only by a single user on a single device. Contact support for additional information.
    ///
    /// "ONE_USER_ONE_DEVICE"
    #[serde(rename="ONE_USER_ONE_DEVICE")]
    ONEUSERONEDEVICE,
    

    /// Legacy alias for `MULTIPLE_HOLDERS`. Deprecated.
    ///
    /// "multipleHolders"
    #[serde(rename="multipleHolders")]
    MultipleHolders,
    

    /// Legacy alias for `ONE_USER_ALL_DEVICES`. Deprecated.
    ///
    /// "oneUserAllDevices"
    #[serde(rename="oneUserAllDevices")]
    OneUserAllDevices,
    

    /// Legacy alias for `ONE_USER_ONE_DEVICE`. Deprecated.
    ///
    /// "oneUserOneDevice"
    #[serde(rename="oneUserOneDevice")]
    OneUserOneDevice,
}

impl AsRef<str> for OfferClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS => "MULTIPLE_HOLDERS",
            OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES => "ONE_USER_ALL_DEVICES",
            OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE => "ONE_USER_ONE_DEVICE",
            OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders => "multipleHolders",
            OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices => "oneUserAllDevices",
            OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice => "oneUserOneDevice",
        }
    }
}

impl std::convert::TryFrom< &str> for OfferClasMultipleDevicesAndHoldersAllowedStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED),
           "MULTIPLE_HOLDERS" => Ok(OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS),
           "ONE_USER_ALL_DEVICES" => Ok(OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES),
           "ONE_USER_ONE_DEVICE" => Ok(OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE),
           "multipleHolders" => Ok(OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders),
           "oneUserAllDevices" => Ok(OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices),
           "oneUserOneDevice" => Ok(OfferClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OfferClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OfferClasRedemptionChannelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The redemption channels applicable to this offer.
pub enum OfferClasRedemptionChannelEnum {
    
    /// "REDEMPTION_CHANNEL_UNSPECIFIED"
    #[serde(rename="REDEMPTION_CHANNEL_UNSPECIFIED")]
    REDEMPTIONCHANNELUNSPECIFIED,
    
    /// "INSTORE"
    #[serde(rename="INSTORE")]
    INSTORE,
    

    /// Legacy alias for `INSTORE`. Deprecated.
    ///
    /// "instore"
    #[serde(rename="instore")]
    Instore,
    
    /// "ONLINE"
    #[serde(rename="ONLINE")]
    ONLINE,
    

    /// Legacy alias for `ONLINE`. Deprecated.
    ///
    /// "online"
    #[serde(rename="online")]
    Online,
    
    /// "BOTH"
    #[serde(rename="BOTH")]
    BOTH,
    

    /// Legacy alias for `BOTH`. Deprecated.
    ///
    /// "both"
    #[serde(rename="both")]
    Both,
    
    /// "TEMPORARY_PRICE_REDUCTION"
    #[serde(rename="TEMPORARY_PRICE_REDUCTION")]
    TEMPORARYPRICEREDUCTION,
    

    /// Legacy alias for `TEMPORARY_PRICE_REDUCTION`. Deprecated.
    ///
    /// "temporaryPriceReduction"
    #[serde(rename="temporaryPriceReduction")]
    TemporaryPriceReduction,
}

impl AsRef<str> for OfferClasRedemptionChannelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OfferClasRedemptionChannelEnum::REDEMPTIONCHANNELUNSPECIFIED => "REDEMPTION_CHANNEL_UNSPECIFIED",
            OfferClasRedemptionChannelEnum::INSTORE => "INSTORE",
            OfferClasRedemptionChannelEnum::Instore => "instore",
            OfferClasRedemptionChannelEnum::ONLINE => "ONLINE",
            OfferClasRedemptionChannelEnum::Online => "online",
            OfferClasRedemptionChannelEnum::BOTH => "BOTH",
            OfferClasRedemptionChannelEnum::Both => "both",
            OfferClasRedemptionChannelEnum::TEMPORARYPRICEREDUCTION => "TEMPORARY_PRICE_REDUCTION",
            OfferClasRedemptionChannelEnum::TemporaryPriceReduction => "temporaryPriceReduction",
        }
    }
}

impl std::convert::TryFrom< &str> for OfferClasRedemptionChannelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDEMPTION_CHANNEL_UNSPECIFIED" => Ok(OfferClasRedemptionChannelEnum::REDEMPTIONCHANNELUNSPECIFIED),
           "INSTORE" => Ok(OfferClasRedemptionChannelEnum::INSTORE),
           "instore" => Ok(OfferClasRedemptionChannelEnum::Instore),
           "ONLINE" => Ok(OfferClasRedemptionChannelEnum::ONLINE),
           "online" => Ok(OfferClasRedemptionChannelEnum::Online),
           "BOTH" => Ok(OfferClasRedemptionChannelEnum::BOTH),
           "both" => Ok(OfferClasRedemptionChannelEnum::Both),
           "TEMPORARY_PRICE_REDUCTION" => Ok(OfferClasRedemptionChannelEnum::TEMPORARYPRICEREDUCTION),
           "temporaryPriceReduction" => Ok(OfferClasRedemptionChannelEnum::TemporaryPriceReduction),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OfferClasRedemptionChannelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OfferClasReviewStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The status of the class. This field can be set to `draft` or The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
pub enum OfferClasReviewStatusEnum {
    
    /// "REVIEW_STATUS_UNSPECIFIED"
    #[serde(rename="REVIEW_STATUS_UNSPECIFIED")]
    REVIEWSTATUSUNSPECIFIED,
    
    /// "UNDER_REVIEW"
    #[serde(rename="UNDER_REVIEW")]
    UNDERREVIEW,
    

    /// Legacy alias for `UNDER_REVIEW`. Deprecated.
    ///
    /// "underReview"
    #[serde(rename="underReview")]
    UnderReview,
    
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// Legacy alias for `APPROVED`. Deprecated.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// Legacy alias for `REJECTED`. Deprecated.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Legacy alias for `DRAFT`. Deprecated.
    ///
    /// "draft"
    #[serde(rename="draft")]
    Draft,
}

impl AsRef<str> for OfferClasReviewStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OfferClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED => "REVIEW_STATUS_UNSPECIFIED",
            OfferClasReviewStatusEnum::UNDERREVIEW => "UNDER_REVIEW",
            OfferClasReviewStatusEnum::UnderReview => "underReview",
            OfferClasReviewStatusEnum::APPROVED => "APPROVED",
            OfferClasReviewStatusEnum::Approved => "approved",
            OfferClasReviewStatusEnum::REJECTED => "REJECTED",
            OfferClasReviewStatusEnum::Rejected => "rejected",
            OfferClasReviewStatusEnum::DRAFT => "DRAFT",
            OfferClasReviewStatusEnum::Draft => "draft",
        }
    }
}

impl std::convert::TryFrom< &str> for OfferClasReviewStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVIEW_STATUS_UNSPECIFIED" => Ok(OfferClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED),
           "UNDER_REVIEW" => Ok(OfferClasReviewStatusEnum::UNDERREVIEW),
           "underReview" => Ok(OfferClasReviewStatusEnum::UnderReview),
           "APPROVED" => Ok(OfferClasReviewStatusEnum::APPROVED),
           "approved" => Ok(OfferClasReviewStatusEnum::Approved),
           "REJECTED" => Ok(OfferClasReviewStatusEnum::REJECTED),
           "rejected" => Ok(OfferClasReviewStatusEnum::Rejected),
           "DRAFT" => Ok(OfferClasReviewStatusEnum::DRAFT),
           "draft" => Ok(OfferClasReviewStatusEnum::Draft),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OfferClasReviewStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OfferClasViewUnlockRequirementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// View Unlock Requirement options for the offer.
pub enum OfferClasViewUnlockRequirementEnum {
    

    /// Default value, same as UNLOCK_NOT_REQUIRED.
    ///
    /// "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED"
    #[serde(rename="VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED")]
    VIEWUNLOCKREQUIREMENTUNSPECIFIED,
    

    /// Default behavior for all the existing Passes if ViewUnlockRequirement is not set.
    ///
    /// "UNLOCK_NOT_REQUIRED"
    #[serde(rename="UNLOCK_NOT_REQUIRED")]
    UNLOCKNOTREQUIRED,
    

    /// Requires the user to unlock their device each time the pass is viewed. If the user removes their device lock after saving the pass, then they will be prompted to create a device lock before the pass can be viewed.
    ///
    /// "UNLOCK_REQUIRED_TO_VIEW"
    #[serde(rename="UNLOCK_REQUIRED_TO_VIEW")]
    UNLOCKREQUIREDTOVIEW,
}

impl AsRef<str> for OfferClasViewUnlockRequirementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OfferClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED => "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED",
            OfferClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED => "UNLOCK_NOT_REQUIRED",
            OfferClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW => "UNLOCK_REQUIRED_TO_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for OfferClasViewUnlockRequirementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED" => Ok(OfferClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED),
           "UNLOCK_NOT_REQUIRED" => Ok(OfferClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED),
           "UNLOCK_REQUIRED_TO_VIEW" => Ok(OfferClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OfferClasViewUnlockRequirementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OfferObjectStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
pub enum OfferObjectStateEnum {
    
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Object is active and displayed to with other active objects.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Legacy alias for `ACTIVE`. Deprecated.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// Legacy alias for `COMPLETED`. Deprecated.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
    

    /// Object is no longer valid (`validTimeInterval` passed).
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// Legacy alias for `EXPIRED`. Deprecated.
    ///
    /// "expired"
    #[serde(rename="expired")]
    Expired,
    
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// Legacy alias for `INACTIVE`. Deprecated.
    ///
    /// "inactive"
    #[serde(rename="inactive")]
    Inactive,
}

impl AsRef<str> for OfferObjectStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OfferObjectStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            OfferObjectStateEnum::ACTIVE => "ACTIVE",
            OfferObjectStateEnum::Active => "active",
            OfferObjectStateEnum::COMPLETED => "COMPLETED",
            OfferObjectStateEnum::Completed => "completed",
            OfferObjectStateEnum::EXPIRED => "EXPIRED",
            OfferObjectStateEnum::Expired => "expired",
            OfferObjectStateEnum::INACTIVE => "INACTIVE",
            OfferObjectStateEnum::Inactive => "inactive",
        }
    }
}

impl std::convert::TryFrom< &str> for OfferObjectStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(OfferObjectStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(OfferObjectStateEnum::ACTIVE),
           "active" => Ok(OfferObjectStateEnum::Active),
           "COMPLETED" => Ok(OfferObjectStateEnum::COMPLETED),
           "completed" => Ok(OfferObjectStateEnum::Completed),
           "EXPIRED" => Ok(OfferObjectStateEnum::EXPIRED),
           "expired" => Ok(OfferObjectStateEnum::Expired),
           "INACTIVE" => Ok(OfferObjectStateEnum::INACTIVE),
           "inactive" => Ok(OfferObjectStateEnum::Inactive),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OfferObjectStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PassConstraintNfcConstraintEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The NFC constraints for the pass.
pub enum PassConstraintNfcConstraintEnum {
    

    /// Default value, no specified constraint.
    ///
    /// "NFC_CONSTRAINT_UNSPECIFIED"
    #[serde(rename="NFC_CONSTRAINT_UNSPECIFIED")]
    NFCCONSTRAINTUNSPECIFIED,
    

    /// Payment cards will not be conveyed while the pass is open.
    ///
    /// "BLOCK_PAYMENT"
    #[serde(rename="BLOCK_PAYMENT")]
    BLOCKPAYMENT,
    

    /// Closed loop transit cards will not be conveyed while the pass is open.
    ///
    /// "BLOCK_CLOSED_LOOP_TRANSIT"
    #[serde(rename="BLOCK_CLOSED_LOOP_TRANSIT")]
    BLOCKCLOSEDLOOPTRANSIT,
}

impl AsRef<str> for PassConstraintNfcConstraintEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PassConstraintNfcConstraintEnum::NFCCONSTRAINTUNSPECIFIED => "NFC_CONSTRAINT_UNSPECIFIED",
            PassConstraintNfcConstraintEnum::BLOCKPAYMENT => "BLOCK_PAYMENT",
            PassConstraintNfcConstraintEnum::BLOCKCLOSEDLOOPTRANSIT => "BLOCK_CLOSED_LOOP_TRANSIT",
        }
    }
}

impl std::convert::TryFrom< &str> for PassConstraintNfcConstraintEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NFC_CONSTRAINT_UNSPECIFIED" => Ok(PassConstraintNfcConstraintEnum::NFCCONSTRAINTUNSPECIFIED),
           "BLOCK_PAYMENT" => Ok(PassConstraintNfcConstraintEnum::BLOCKPAYMENT),
           "BLOCK_CLOSED_LOOP_TRANSIT" => Ok(PassConstraintNfcConstraintEnum::BLOCKCLOSEDLOOPTRANSIT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PassConstraintNfcConstraintEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PassConstraintScreenshotEligibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The screenshot eligibility for the pass.
pub enum PassConstraintScreenshotEligibilityEnum {
    

    /// Default value, same as ELIGIBLE.
    ///
    /// "SCREENSHOT_ELIGIBILITY_UNSPECIFIED"
    #[serde(rename="SCREENSHOT_ELIGIBILITY_UNSPECIFIED")]
    SCREENSHOTELIGIBILITYUNSPECIFIED,
    

    /// Default behavior for all existing Passes if ScreenshotEligibility is not set. Allows screenshots to be taken on Android devices.
    ///
    /// "ELIGIBLE"
    #[serde(rename="ELIGIBLE")]
    ELIGIBLE,
    

    /// Disallows screenshots to be taken on Android devices. Note that older versions of Wallet may still allow screenshots to be taken.
    ///
    /// "INELIGIBLE"
    #[serde(rename="INELIGIBLE")]
    INELIGIBLE,
}

impl AsRef<str> for PassConstraintScreenshotEligibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PassConstraintScreenshotEligibilityEnum::SCREENSHOTELIGIBILITYUNSPECIFIED => "SCREENSHOT_ELIGIBILITY_UNSPECIFIED",
            PassConstraintScreenshotEligibilityEnum::ELIGIBLE => "ELIGIBLE",
            PassConstraintScreenshotEligibilityEnum::INELIGIBLE => "INELIGIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for PassConstraintScreenshotEligibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCREENSHOT_ELIGIBILITY_UNSPECIFIED" => Ok(PassConstraintScreenshotEligibilityEnum::SCREENSHOTELIGIBILITYUNSPECIFIED),
           "ELIGIBLE" => Ok(PassConstraintScreenshotEligibilityEnum::ELIGIBLE),
           "INELIGIBLE" => Ok(PassConstraintScreenshotEligibilityEnum::INELIGIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PassConstraintScreenshotEligibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PermissionRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role granted by this permission.
pub enum PermissionRoleEnum {
    
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
    

    /// Legacy alias for `OWNER`. Deprecated.
    ///
    /// "owner"
    #[serde(rename="owner")]
    Owner,
    
    /// "READER"
    #[serde(rename="READER")]
    READER,
    

    /// Legacy alias for `READER`. Deprecated.
    ///
    /// "reader"
    #[serde(rename="reader")]
    Reader,
    
    /// "WRITER"
    #[serde(rename="WRITER")]
    WRITER,
    

    /// Legacy alias for `WRITER`. Deprecated.
    ///
    /// "writer"
    #[serde(rename="writer")]
    Writer,
}

impl AsRef<str> for PermissionRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PermissionRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            PermissionRoleEnum::OWNER => "OWNER",
            PermissionRoleEnum::Owner => "owner",
            PermissionRoleEnum::READER => "READER",
            PermissionRoleEnum::Reader => "reader",
            PermissionRoleEnum::WRITER => "WRITER",
            PermissionRoleEnum::Writer => "writer",
        }
    }
}

impl std::convert::TryFrom< &str> for PermissionRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(PermissionRoleEnum::ROLEUNSPECIFIED),
           "OWNER" => Ok(PermissionRoleEnum::OWNER),
           "owner" => Ok(PermissionRoleEnum::Owner),
           "READER" => Ok(PermissionRoleEnum::READER),
           "reader" => Ok(PermissionRoleEnum::Reader),
           "WRITER" => Ok(PermissionRoleEnum::WRITER),
           "writer" => Ok(PermissionRoleEnum::Writer),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PermissionRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RotatingBarcodeRenderEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The render encoding for the barcode. When specified, barcode is rendered in the given encoding. Otherwise best known encoding is chosen by Google.
pub enum RotatingBarcodeRenderEncodingEnum {
    
    /// "RENDER_ENCODING_UNSPECIFIED"
    #[serde(rename="RENDER_ENCODING_UNSPECIFIED")]
    RENDERENCODINGUNSPECIFIED,
    

    /// UTF_8 encoding for barcodes. This is only supported for barcode type qrCode.
    ///
    /// "UTF_8"
    #[serde(rename="UTF_8")]
    UTF8,
}

impl AsRef<str> for RotatingBarcodeRenderEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RotatingBarcodeRenderEncodingEnum::RENDERENCODINGUNSPECIFIED => "RENDER_ENCODING_UNSPECIFIED",
            RotatingBarcodeRenderEncodingEnum::UTF8 => "UTF_8",
        }
    }
}

impl std::convert::TryFrom< &str> for RotatingBarcodeRenderEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RENDER_ENCODING_UNSPECIFIED" => Ok(RotatingBarcodeRenderEncodingEnum::RENDERENCODINGUNSPECIFIED),
           "UTF_8" => Ok(RotatingBarcodeRenderEncodingEnum::UTF8),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RotatingBarcodeRenderEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RotatingBarcodeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this barcode.
pub enum RotatingBarcodeTypeEnum {
    
    /// "BARCODE_TYPE_UNSPECIFIED"
    #[serde(rename="BARCODE_TYPE_UNSPECIFIED")]
    BARCODETYPEUNSPECIFIED,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "AZTEC"
    #[serde(rename="AZTEC")]
    AZTEC,
    

    /// Legacy alias for `AZTEC`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "aztec"
    #[serde(rename="aztec")]
    Aztec,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "CODE_39"
    #[serde(rename="CODE_39")]
    CODE39,
    

    /// Legacy alias for `CODE_39`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "code39"
    #[serde(rename="code39")]
    Code39,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "CODE_128"
    #[serde(rename="CODE_128")]
    CODE128,
    

    /// Legacy alias for `CODE_128`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "code128"
    #[serde(rename="code128")]
    Code128,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "CODABAR"
    #[serde(rename="CODABAR")]
    CODABAR,
    

    /// Legacy alias for `CODABAR`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "codabar"
    #[serde(rename="codabar")]
    Codabar,
    

    /// A 2D matrix barcode consisting of black and white. Cells or modules are arranged in either a square or rectangle. Not supported for Rotating Barcodes.
    ///
    /// "DATA_MATRIX"
    #[serde(rename="DATA_MATRIX")]
    DATAMATRIX,
    

    /// Legacy alias for `DATA_MATRIX`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "dataMatrix"
    #[serde(rename="dataMatrix")]
    DataMatrix,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "EAN_8"
    #[serde(rename="EAN_8")]
    EAN8,
    

    /// Legacy alias for `EAN_8`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "ean8"
    #[serde(rename="ean8")]
    Ean8,
    

    /// Not supported for Rotating Barcodes.
    ///
    /// "EAN_13"
    #[serde(rename="EAN_13")]
    EAN13,
    

    /// Legacy alias for `EAN_13`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "ean13"
    #[serde(rename="ean13")]
    Ean13,
    

    /// Legacy alias for `EAN_13`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "EAN13"
    #[serde(rename="EAN13")]
    EAN13,
    

    /// 14 digit ITF code Not supported for Rotating Barcodes.
    ///
    /// "ITF_14"
    #[serde(rename="ITF_14")]
    ITF14,
    

    /// Legacy alias for `ITF_14`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "itf14"
    #[serde(rename="itf14")]
    Itf14,
    

    /// Supported for Rotating Barcodes.
    ///
    /// "PDF_417"
    #[serde(rename="PDF_417")]
    PDF417,
    

    /// Legacy alias for `PDF_417`. Deprecated.
    ///
    /// "pdf417"
    #[serde(rename="pdf417")]
    Pdf417,
    

    /// Legacy alias for `PDF_417`. Deprecated.
    ///
    /// "PDF417"
    #[serde(rename="PDF417")]
    PDF417,
    

    /// Supported for Rotating Barcodes.
    ///
    /// "QR_CODE"
    #[serde(rename="QR_CODE")]
    QRCODE,
    

    /// Legacy alias for `QR_CODE`. Deprecated.
    ///
    /// "qrCode"
    #[serde(rename="qrCode")]
    QrCode,
    

    /// Legacy alias for `QR_CODE`. Deprecated.
    ///
    /// "qrcode"
    #[serde(rename="qrcode")]
    Qrcode,
    

    /// 11 or 12 digit codes Not supported for Rotating Barcodes.
    ///
    /// "UPC_A"
    #[serde(rename="UPC_A")]
    UPCA,
    

    /// Legacy alias for `UPC_A`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "upcA"
    #[serde(rename="upcA")]
    UpcA,
    

    /// Renders the field as a text field. The `alternateText` field may not be used with a barcode of type `textOnly`. Not supported for Rotating Barcodes.
    ///
    /// "TEXT_ONLY"
    #[serde(rename="TEXT_ONLY")]
    TEXTONLY,
    

    /// Legacy alias for `TEXT_ONLY`. Deprecated. Not supported for Rotating Barcodes.
    ///
    /// "textOnly"
    #[serde(rename="textOnly")]
    TextOnly,
}

impl AsRef<str> for RotatingBarcodeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RotatingBarcodeTypeEnum::BARCODETYPEUNSPECIFIED => "BARCODE_TYPE_UNSPECIFIED",
            RotatingBarcodeTypeEnum::AZTEC => "AZTEC",
            RotatingBarcodeTypeEnum::Aztec => "aztec",
            RotatingBarcodeTypeEnum::CODE39 => "CODE_39",
            RotatingBarcodeTypeEnum::Code39 => "code39",
            RotatingBarcodeTypeEnum::CODE128 => "CODE_128",
            RotatingBarcodeTypeEnum::Code128 => "code128",
            RotatingBarcodeTypeEnum::CODABAR => "CODABAR",
            RotatingBarcodeTypeEnum::Codabar => "codabar",
            RotatingBarcodeTypeEnum::DATAMATRIX => "DATA_MATRIX",
            RotatingBarcodeTypeEnum::DataMatrix => "dataMatrix",
            RotatingBarcodeTypeEnum::EAN8 => "EAN_8",
            RotatingBarcodeTypeEnum::Ean8 => "ean8",
            RotatingBarcodeTypeEnum::EAN13 => "EAN_13",
            RotatingBarcodeTypeEnum::Ean13 => "ean13",
            RotatingBarcodeTypeEnum::EAN13 => "EAN13",
            RotatingBarcodeTypeEnum::ITF14 => "ITF_14",
            RotatingBarcodeTypeEnum::Itf14 => "itf14",
            RotatingBarcodeTypeEnum::PDF417 => "PDF_417",
            RotatingBarcodeTypeEnum::Pdf417 => "pdf417",
            RotatingBarcodeTypeEnum::PDF417 => "PDF417",
            RotatingBarcodeTypeEnum::QRCODE => "QR_CODE",
            RotatingBarcodeTypeEnum::QrCode => "qrCode",
            RotatingBarcodeTypeEnum::Qrcode => "qrcode",
            RotatingBarcodeTypeEnum::UPCA => "UPC_A",
            RotatingBarcodeTypeEnum::UpcA => "upcA",
            RotatingBarcodeTypeEnum::TEXTONLY => "TEXT_ONLY",
            RotatingBarcodeTypeEnum::TextOnly => "textOnly",
        }
    }
}

impl std::convert::TryFrom< &str> for RotatingBarcodeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BARCODE_TYPE_UNSPECIFIED" => Ok(RotatingBarcodeTypeEnum::BARCODETYPEUNSPECIFIED),
           "AZTEC" => Ok(RotatingBarcodeTypeEnum::AZTEC),
           "aztec" => Ok(RotatingBarcodeTypeEnum::Aztec),
           "CODE_39" => Ok(RotatingBarcodeTypeEnum::CODE39),
           "code39" => Ok(RotatingBarcodeTypeEnum::Code39),
           "CODE_128" => Ok(RotatingBarcodeTypeEnum::CODE128),
           "code128" => Ok(RotatingBarcodeTypeEnum::Code128),
           "CODABAR" => Ok(RotatingBarcodeTypeEnum::CODABAR),
           "codabar" => Ok(RotatingBarcodeTypeEnum::Codabar),
           "DATA_MATRIX" => Ok(RotatingBarcodeTypeEnum::DATAMATRIX),
           "dataMatrix" => Ok(RotatingBarcodeTypeEnum::DataMatrix),
           "EAN_8" => Ok(RotatingBarcodeTypeEnum::EAN8),
           "ean8" => Ok(RotatingBarcodeTypeEnum::Ean8),
           "EAN_13" => Ok(RotatingBarcodeTypeEnum::EAN13),
           "ean13" => Ok(RotatingBarcodeTypeEnum::Ean13),
           "EAN13" => Ok(RotatingBarcodeTypeEnum::EAN13),
           "ITF_14" => Ok(RotatingBarcodeTypeEnum::ITF14),
           "itf14" => Ok(RotatingBarcodeTypeEnum::Itf14),
           "PDF_417" => Ok(RotatingBarcodeTypeEnum::PDF417),
           "pdf417" => Ok(RotatingBarcodeTypeEnum::Pdf417),
           "PDF417" => Ok(RotatingBarcodeTypeEnum::PDF417),
           "QR_CODE" => Ok(RotatingBarcodeTypeEnum::QRCODE),
           "qrCode" => Ok(RotatingBarcodeTypeEnum::QrCode),
           "qrcode" => Ok(RotatingBarcodeTypeEnum::Qrcode),
           "UPC_A" => Ok(RotatingBarcodeTypeEnum::UPCA),
           "upcA" => Ok(RotatingBarcodeTypeEnum::UpcA),
           "TEXT_ONLY" => Ok(RotatingBarcodeTypeEnum::TEXTONLY),
           "textOnly" => Ok(RotatingBarcodeTypeEnum::TextOnly),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RotatingBarcodeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RotatingBarcodeTotpDetailAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The TOTP algorithm used to generate the OTP.
pub enum RotatingBarcodeTotpDetailAlgorithmEnum {
    
    /// "TOTP_ALGORITHM_UNSPECIFIED"
    #[serde(rename="TOTP_ALGORITHM_UNSPECIFIED")]
    TOTPALGORITHMUNSPECIFIED,
    

    /// TOTP algorithm from RFC 6238 with the SHA1 hash function
    ///
    /// "TOTP_SHA1"
    #[serde(rename="TOTP_SHA1")]
    TOTPSHA1,
}

impl AsRef<str> for RotatingBarcodeTotpDetailAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RotatingBarcodeTotpDetailAlgorithmEnum::TOTPALGORITHMUNSPECIFIED => "TOTP_ALGORITHM_UNSPECIFIED",
            RotatingBarcodeTotpDetailAlgorithmEnum::TOTPSHA1 => "TOTP_SHA1",
        }
    }
}

impl std::convert::TryFrom< &str> for RotatingBarcodeTotpDetailAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TOTP_ALGORITHM_UNSPECIFIED" => Ok(RotatingBarcodeTotpDetailAlgorithmEnum::TOTPALGORITHMUNSPECIFIED),
           "TOTP_SHA1" => Ok(RotatingBarcodeTotpDetailAlgorithmEnum::TOTPSHA1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RotatingBarcodeTotpDetailAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SecurityAnimationAnimationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of animation.
pub enum SecurityAnimationAnimationTypeEnum {
    
    /// "ANIMATION_UNSPECIFIED"
    #[serde(rename="ANIMATION_UNSPECIFIED")]
    ANIMATIONUNSPECIFIED,
    

    /// Default Foil & Shimmer animation
    ///
    /// "FOIL_SHIMMER"
    #[serde(rename="FOIL_SHIMMER")]
    FOILSHIMMER,
    

    /// Legacy alias for `FOIL_SHIMMER`. Deprecated.
    ///
    /// "foilShimmer"
    #[serde(rename="foilShimmer")]
    FoilShimmer,
}

impl AsRef<str> for SecurityAnimationAnimationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SecurityAnimationAnimationTypeEnum::ANIMATIONUNSPECIFIED => "ANIMATION_UNSPECIFIED",
            SecurityAnimationAnimationTypeEnum::FOILSHIMMER => "FOIL_SHIMMER",
            SecurityAnimationAnimationTypeEnum::FoilShimmer => "foilShimmer",
        }
    }
}

impl std::convert::TryFrom< &str> for SecurityAnimationAnimationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANIMATION_UNSPECIFIED" => Ok(SecurityAnimationAnimationTypeEnum::ANIMATIONUNSPECIFIED),
           "FOIL_SHIMMER" => Ok(SecurityAnimationAnimationTypeEnum::FOILSHIMMER),
           "foilShimmer" => Ok(SecurityAnimationAnimationTypeEnum::FoilShimmer),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SecurityAnimationAnimationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TemplateItemPredefinedItemEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A predefined item to display. Only one of `firstValue` or `predefinedItem` may be set.
pub enum TemplateItemPredefinedItemEnum {
    
    /// "PREDEFINED_ITEM_UNSPECIFIED"
    #[serde(rename="PREDEFINED_ITEM_UNSPECIFIED")]
    PREDEFINEDITEMUNSPECIFIED,
    
    /// "FREQUENT_FLYER_PROGRAM_NAME_AND_NUMBER"
    #[serde(rename="FREQUENT_FLYER_PROGRAM_NAME_AND_NUMBER")]
    FREQUENTFLYERPROGRAMNAMEANDNUMBER,
    

    /// Legacy alias for `FREQUENT_FLYER_PROGRAM_NAME_AND_NUMBER`. Deprecated.
    ///
    /// "frequentFlyerProgramNameAndNumber"
    #[serde(rename="frequentFlyerProgramNameAndNumber")]
    FrequentFlyerProgramNameAndNumber,
    
    /// "FLIGHT_NUMBER_AND_OPERATING_FLIGHT_NUMBER"
    #[serde(rename="FLIGHT_NUMBER_AND_OPERATING_FLIGHT_NUMBER")]
    FLIGHTNUMBERANDOPERATINGFLIGHTNUMBER,
    

    /// Legacy alias for `FLIGHT_NUMBER_AND_OPERATING_FLIGHT_NUMBER`. Deprecated.
    ///
    /// "flightNumberAndOperatingFlightNumber"
    #[serde(rename="flightNumberAndOperatingFlightNumber")]
    FlightNumberAndOperatingFlightNumber,
}

impl AsRef<str> for TemplateItemPredefinedItemEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TemplateItemPredefinedItemEnum::PREDEFINEDITEMUNSPECIFIED => "PREDEFINED_ITEM_UNSPECIFIED",
            TemplateItemPredefinedItemEnum::FREQUENTFLYERPROGRAMNAMEANDNUMBER => "FREQUENT_FLYER_PROGRAM_NAME_AND_NUMBER",
            TemplateItemPredefinedItemEnum::FrequentFlyerProgramNameAndNumber => "frequentFlyerProgramNameAndNumber",
            TemplateItemPredefinedItemEnum::FLIGHTNUMBERANDOPERATINGFLIGHTNUMBER => "FLIGHT_NUMBER_AND_OPERATING_FLIGHT_NUMBER",
            TemplateItemPredefinedItemEnum::FlightNumberAndOperatingFlightNumber => "flightNumberAndOperatingFlightNumber",
        }
    }
}

impl std::convert::TryFrom< &str> for TemplateItemPredefinedItemEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PREDEFINED_ITEM_UNSPECIFIED" => Ok(TemplateItemPredefinedItemEnum::PREDEFINEDITEMUNSPECIFIED),
           "FREQUENT_FLYER_PROGRAM_NAME_AND_NUMBER" => Ok(TemplateItemPredefinedItemEnum::FREQUENTFLYERPROGRAMNAMEANDNUMBER),
           "frequentFlyerProgramNameAndNumber" => Ok(TemplateItemPredefinedItemEnum::FrequentFlyerProgramNameAndNumber),
           "FLIGHT_NUMBER_AND_OPERATING_FLIGHT_NUMBER" => Ok(TemplateItemPredefinedItemEnum::FLIGHTNUMBERANDOPERATINGFLIGHTNUMBER),
           "flightNumberAndOperatingFlightNumber" => Ok(TemplateItemPredefinedItemEnum::FlightNumberAndOperatingFlightNumber),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TemplateItemPredefinedItemEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TicketSeatFareClassEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The fare class of the ticketed seat.
pub enum TicketSeatFareClassEnum {
    
    /// "FARE_CLASS_UNSPECIFIED"
    #[serde(rename="FARE_CLASS_UNSPECIFIED")]
    FARECLASSUNSPECIFIED,
    
    /// "ECONOMY"
    #[serde(rename="ECONOMY")]
    ECONOMY,
    

    /// Legacy alias for `ECONOMY`. Deprecated.
    ///
    /// "economy"
    #[serde(rename="economy")]
    Economy,
    
    /// "FIRST"
    #[serde(rename="FIRST")]
    FIRST,
    

    /// Legacy alias for `FIRST`. Deprecated.
    ///
    /// "first"
    #[serde(rename="first")]
    First,
    
    /// "BUSINESS"
    #[serde(rename="BUSINESS")]
    BUSINESS,
    

    /// Legacy alias for `BUSINESS`. Deprecated.
    ///
    /// "business"
    #[serde(rename="business")]
    Business,
}

impl AsRef<str> for TicketSeatFareClassEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TicketSeatFareClassEnum::FARECLASSUNSPECIFIED => "FARE_CLASS_UNSPECIFIED",
            TicketSeatFareClassEnum::ECONOMY => "ECONOMY",
            TicketSeatFareClassEnum::Economy => "economy",
            TicketSeatFareClassEnum::FIRST => "FIRST",
            TicketSeatFareClassEnum::First => "first",
            TicketSeatFareClassEnum::BUSINESS => "BUSINESS",
            TicketSeatFareClassEnum::Business => "business",
        }
    }
}

impl std::convert::TryFrom< &str> for TicketSeatFareClassEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FARE_CLASS_UNSPECIFIED" => Ok(TicketSeatFareClassEnum::FARECLASSUNSPECIFIED),
           "ECONOMY" => Ok(TicketSeatFareClassEnum::ECONOMY),
           "economy" => Ok(TicketSeatFareClassEnum::Economy),
           "FIRST" => Ok(TicketSeatFareClassEnum::FIRST),
           "first" => Ok(TicketSeatFareClassEnum::First),
           "BUSINESS" => Ok(TicketSeatFareClassEnum::BUSINESS),
           "business" => Ok(TicketSeatFareClassEnum::Business),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TicketSeatFareClassEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransitClasMultipleDevicesAndHoldersAllowedStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Identifies whether multiple users and devices will save the same object referencing this class.
pub enum TransitClasMultipleDevicesAndHoldersAllowedStatusEnum {
    

    /// Unspecified preference.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The Pass object is shareable by a user and can be saved by any number of different users, and on any number of devices. Partners typically use this setup for passes that do not need to be restricted to a single user or pinned to a single device.
    ///
    /// "MULTIPLE_HOLDERS"
    #[serde(rename="MULTIPLE_HOLDERS")]
    MULTIPLEHOLDERS,
    

    /// An object can only be saved by one user, but this user can view and use it on multiple of their devices. Once the first user saves the object, no other user will be allowed to view or save it.
    ///
    /// "ONE_USER_ALL_DEVICES"
    #[serde(rename="ONE_USER_ALL_DEVICES")]
    ONEUSERALLDEVICES,
    

    /// An object can only be saved by one user on a single device. Intended for use by select partners in limited circumstances. An example use case is a transit ticket that should be "device pinned", meaning it can be saved, viewed and used only by a single user on a single device. Contact support for additional information.
    ///
    /// "ONE_USER_ONE_DEVICE"
    #[serde(rename="ONE_USER_ONE_DEVICE")]
    ONEUSERONEDEVICE,
    

    /// Legacy alias for `MULTIPLE_HOLDERS`. Deprecated.
    ///
    /// "multipleHolders"
    #[serde(rename="multipleHolders")]
    MultipleHolders,
    

    /// Legacy alias for `ONE_USER_ALL_DEVICES`. Deprecated.
    ///
    /// "oneUserAllDevices"
    #[serde(rename="oneUserAllDevices")]
    OneUserAllDevices,
    

    /// Legacy alias for `ONE_USER_ONE_DEVICE`. Deprecated.
    ///
    /// "oneUserOneDevice"
    #[serde(rename="oneUserOneDevice")]
    OneUserOneDevice,
}

impl AsRef<str> for TransitClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS => "MULTIPLE_HOLDERS",
            TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES => "ONE_USER_ALL_DEVICES",
            TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE => "ONE_USER_ONE_DEVICE",
            TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders => "multipleHolders",
            TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices => "oneUserAllDevices",
            TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice => "oneUserOneDevice",
        }
    }
}

impl std::convert::TryFrom< &str> for TransitClasMultipleDevicesAndHoldersAllowedStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::STATUSUNSPECIFIED),
           "MULTIPLE_HOLDERS" => Ok(TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::MULTIPLEHOLDERS),
           "ONE_USER_ALL_DEVICES" => Ok(TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERALLDEVICES),
           "ONE_USER_ONE_DEVICE" => Ok(TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::ONEUSERONEDEVICE),
           "multipleHolders" => Ok(TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::MultipleHolders),
           "oneUserAllDevices" => Ok(TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserAllDevices),
           "oneUserOneDevice" => Ok(TransitClasMultipleDevicesAndHoldersAllowedStatusEnum::OneUserOneDevice),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransitClasMultipleDevicesAndHoldersAllowedStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransitClasReviewStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
pub enum TransitClasReviewStatusEnum {
    
    /// "REVIEW_STATUS_UNSPECIFIED"
    #[serde(rename="REVIEW_STATUS_UNSPECIFIED")]
    REVIEWSTATUSUNSPECIFIED,
    
    /// "UNDER_REVIEW"
    #[serde(rename="UNDER_REVIEW")]
    UNDERREVIEW,
    

    /// Legacy alias for `UNDER_REVIEW`. Deprecated.
    ///
    /// "underReview"
    #[serde(rename="underReview")]
    UnderReview,
    
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// Legacy alias for `APPROVED`. Deprecated.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// Legacy alias for `REJECTED`. Deprecated.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Legacy alias for `DRAFT`. Deprecated.
    ///
    /// "draft"
    #[serde(rename="draft")]
    Draft,
}

impl AsRef<str> for TransitClasReviewStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransitClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED => "REVIEW_STATUS_UNSPECIFIED",
            TransitClasReviewStatusEnum::UNDERREVIEW => "UNDER_REVIEW",
            TransitClasReviewStatusEnum::UnderReview => "underReview",
            TransitClasReviewStatusEnum::APPROVED => "APPROVED",
            TransitClasReviewStatusEnum::Approved => "approved",
            TransitClasReviewStatusEnum::REJECTED => "REJECTED",
            TransitClasReviewStatusEnum::Rejected => "rejected",
            TransitClasReviewStatusEnum::DRAFT => "DRAFT",
            TransitClasReviewStatusEnum::Draft => "draft",
        }
    }
}

impl std::convert::TryFrom< &str> for TransitClasReviewStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVIEW_STATUS_UNSPECIFIED" => Ok(TransitClasReviewStatusEnum::REVIEWSTATUSUNSPECIFIED),
           "UNDER_REVIEW" => Ok(TransitClasReviewStatusEnum::UNDERREVIEW),
           "underReview" => Ok(TransitClasReviewStatusEnum::UnderReview),
           "APPROVED" => Ok(TransitClasReviewStatusEnum::APPROVED),
           "approved" => Ok(TransitClasReviewStatusEnum::Approved),
           "REJECTED" => Ok(TransitClasReviewStatusEnum::REJECTED),
           "rejected" => Ok(TransitClasReviewStatusEnum::Rejected),
           "DRAFT" => Ok(TransitClasReviewStatusEnum::DRAFT),
           "draft" => Ok(TransitClasReviewStatusEnum::Draft),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransitClasReviewStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransitClasTransitTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of transit this class represents, such as "bus".
pub enum TransitClasTransitTypeEnum {
    
    /// "TRANSIT_TYPE_UNSPECIFIED"
    #[serde(rename="TRANSIT_TYPE_UNSPECIFIED")]
    TRANSITTYPEUNSPECIFIED,
    
    /// "BUS"
    #[serde(rename="BUS")]
    BUS,
    

    /// Legacy alias for `BUS`. Deprecated.
    ///
    /// "bus"
    #[serde(rename="bus")]
    Bus,
    
    /// "RAIL"
    #[serde(rename="RAIL")]
    RAIL,
    

    /// Legacy alias for `RAIL`. Deprecated.
    ///
    /// "rail"
    #[serde(rename="rail")]
    Rail,
    
    /// "TRAM"
    #[serde(rename="TRAM")]
    TRAM,
    

    /// Legacy alias for `TRAM`. Deprecated.
    ///
    /// "tram"
    #[serde(rename="tram")]
    Tram,
    
    /// "FERRY"
    #[serde(rename="FERRY")]
    FERRY,
    

    /// Legacy alias for `FERRY`. Deprecated.
    ///
    /// "ferry"
    #[serde(rename="ferry")]
    Ferry,
    
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
    

    /// Legacy alias for `OTHER`. Deprecated.
    ///
    /// "other"
    #[serde(rename="other")]
    Other,
}

impl AsRef<str> for TransitClasTransitTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransitClasTransitTypeEnum::TRANSITTYPEUNSPECIFIED => "TRANSIT_TYPE_UNSPECIFIED",
            TransitClasTransitTypeEnum::BUS => "BUS",
            TransitClasTransitTypeEnum::Bus => "bus",
            TransitClasTransitTypeEnum::RAIL => "RAIL",
            TransitClasTransitTypeEnum::Rail => "rail",
            TransitClasTransitTypeEnum::TRAM => "TRAM",
            TransitClasTransitTypeEnum::Tram => "tram",
            TransitClasTransitTypeEnum::FERRY => "FERRY",
            TransitClasTransitTypeEnum::Ferry => "ferry",
            TransitClasTransitTypeEnum::OTHER => "OTHER",
            TransitClasTransitTypeEnum::Other => "other",
        }
    }
}

impl std::convert::TryFrom< &str> for TransitClasTransitTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSIT_TYPE_UNSPECIFIED" => Ok(TransitClasTransitTypeEnum::TRANSITTYPEUNSPECIFIED),
           "BUS" => Ok(TransitClasTransitTypeEnum::BUS),
           "bus" => Ok(TransitClasTransitTypeEnum::Bus),
           "RAIL" => Ok(TransitClasTransitTypeEnum::RAIL),
           "rail" => Ok(TransitClasTransitTypeEnum::Rail),
           "TRAM" => Ok(TransitClasTransitTypeEnum::TRAM),
           "tram" => Ok(TransitClasTransitTypeEnum::Tram),
           "FERRY" => Ok(TransitClasTransitTypeEnum::FERRY),
           "ferry" => Ok(TransitClasTransitTypeEnum::Ferry),
           "OTHER" => Ok(TransitClasTransitTypeEnum::OTHER),
           "other" => Ok(TransitClasTransitTypeEnum::Other),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransitClasTransitTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransitClasViewUnlockRequirementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// View Unlock Requirement options for the transit ticket.
pub enum TransitClasViewUnlockRequirementEnum {
    

    /// Default value, same as UNLOCK_NOT_REQUIRED.
    ///
    /// "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED"
    #[serde(rename="VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED")]
    VIEWUNLOCKREQUIREMENTUNSPECIFIED,
    

    /// Default behavior for all the existing Passes if ViewUnlockRequirement is not set.
    ///
    /// "UNLOCK_NOT_REQUIRED"
    #[serde(rename="UNLOCK_NOT_REQUIRED")]
    UNLOCKNOTREQUIRED,
    

    /// Requires the user to unlock their device each time the pass is viewed. If the user removes their device lock after saving the pass, then they will be prompted to create a device lock before the pass can be viewed.
    ///
    /// "UNLOCK_REQUIRED_TO_VIEW"
    #[serde(rename="UNLOCK_REQUIRED_TO_VIEW")]
    UNLOCKREQUIREDTOVIEW,
}

impl AsRef<str> for TransitClasViewUnlockRequirementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransitClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED => "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED",
            TransitClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED => "UNLOCK_NOT_REQUIRED",
            TransitClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW => "UNLOCK_REQUIRED_TO_VIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for TransitClasViewUnlockRequirementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNLOCK_REQUIREMENT_UNSPECIFIED" => Ok(TransitClasViewUnlockRequirementEnum::VIEWUNLOCKREQUIREMENTUNSPECIFIED),
           "UNLOCK_NOT_REQUIRED" => Ok(TransitClasViewUnlockRequirementEnum::UNLOCKNOTREQUIRED),
           "UNLOCK_REQUIRED_TO_VIEW" => Ok(TransitClasViewUnlockRequirementEnum::UNLOCKREQUIREDTOVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransitClasViewUnlockRequirementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransitObjectConcessionCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The concession category for the ticket.
pub enum TransitObjectConcessionCategoryEnum {
    
    /// "CONCESSION_CATEGORY_UNSPECIFIED"
    #[serde(rename="CONCESSION_CATEGORY_UNSPECIFIED")]
    CONCESSIONCATEGORYUNSPECIFIED,
    
    /// "ADULT"
    #[serde(rename="ADULT")]
    ADULT,
    

    /// Legacy alias for `ADULT`. Deprecated.
    ///
    /// "adult"
    #[serde(rename="adult")]
    Adult,
    
    /// "CHILD"
    #[serde(rename="CHILD")]
    CHILD,
    

    /// Legacy alias for `CHILD`. Deprecated.
    ///
    /// "child"
    #[serde(rename="child")]
    Child,
    
    /// "SENIOR"
    #[serde(rename="SENIOR")]
    SENIOR,
    

    /// Legacy alias for `SENIOR`. Deprecated.
    ///
    /// "senior"
    #[serde(rename="senior")]
    Senior,
}

impl AsRef<str> for TransitObjectConcessionCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransitObjectConcessionCategoryEnum::CONCESSIONCATEGORYUNSPECIFIED => "CONCESSION_CATEGORY_UNSPECIFIED",
            TransitObjectConcessionCategoryEnum::ADULT => "ADULT",
            TransitObjectConcessionCategoryEnum::Adult => "adult",
            TransitObjectConcessionCategoryEnum::CHILD => "CHILD",
            TransitObjectConcessionCategoryEnum::Child => "child",
            TransitObjectConcessionCategoryEnum::SENIOR => "SENIOR",
            TransitObjectConcessionCategoryEnum::Senior => "senior",
        }
    }
}

impl std::convert::TryFrom< &str> for TransitObjectConcessionCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONCESSION_CATEGORY_UNSPECIFIED" => Ok(TransitObjectConcessionCategoryEnum::CONCESSIONCATEGORYUNSPECIFIED),
           "ADULT" => Ok(TransitObjectConcessionCategoryEnum::ADULT),
           "adult" => Ok(TransitObjectConcessionCategoryEnum::Adult),
           "CHILD" => Ok(TransitObjectConcessionCategoryEnum::CHILD),
           "child" => Ok(TransitObjectConcessionCategoryEnum::Child),
           "SENIOR" => Ok(TransitObjectConcessionCategoryEnum::SENIOR),
           "senior" => Ok(TransitObjectConcessionCategoryEnum::Senior),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransitObjectConcessionCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransitObjectPassengerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The number of passengers.
pub enum TransitObjectPassengerTypeEnum {
    
    /// "PASSENGER_TYPE_UNSPECIFIED"
    #[serde(rename="PASSENGER_TYPE_UNSPECIFIED")]
    PASSENGERTYPEUNSPECIFIED,
    
    /// "SINGLE_PASSENGER"
    #[serde(rename="SINGLE_PASSENGER")]
    SINGLEPASSENGER,
    

    /// Legacy alias for `SINGLE_PASSENGER`. Deprecated.
    ///
    /// "singlePassenger"
    #[serde(rename="singlePassenger")]
    SinglePassenger,
    
    /// "MULTIPLE_PASSENGERS"
    #[serde(rename="MULTIPLE_PASSENGERS")]
    MULTIPLEPASSENGERS,
    

    /// Legacy alias for `MULTIPLE_PASSENGERS`. Deprecated.
    ///
    /// "multiplePassengers"
    #[serde(rename="multiplePassengers")]
    MultiplePassengers,
}

impl AsRef<str> for TransitObjectPassengerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransitObjectPassengerTypeEnum::PASSENGERTYPEUNSPECIFIED => "PASSENGER_TYPE_UNSPECIFIED",
            TransitObjectPassengerTypeEnum::SINGLEPASSENGER => "SINGLE_PASSENGER",
            TransitObjectPassengerTypeEnum::SinglePassenger => "singlePassenger",
            TransitObjectPassengerTypeEnum::MULTIPLEPASSENGERS => "MULTIPLE_PASSENGERS",
            TransitObjectPassengerTypeEnum::MultiplePassengers => "multiplePassengers",
        }
    }
}

impl std::convert::TryFrom< &str> for TransitObjectPassengerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PASSENGER_TYPE_UNSPECIFIED" => Ok(TransitObjectPassengerTypeEnum::PASSENGERTYPEUNSPECIFIED),
           "SINGLE_PASSENGER" => Ok(TransitObjectPassengerTypeEnum::SINGLEPASSENGER),
           "singlePassenger" => Ok(TransitObjectPassengerTypeEnum::SinglePassenger),
           "MULTIPLE_PASSENGERS" => Ok(TransitObjectPassengerTypeEnum::MULTIPLEPASSENGERS),
           "multiplePassengers" => Ok(TransitObjectPassengerTypeEnum::MultiplePassengers),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransitObjectPassengerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransitObjectStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
pub enum TransitObjectStateEnum {
    
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Object is active and displayed to with other active objects.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Legacy alias for `ACTIVE`. Deprecated.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// Legacy alias for `COMPLETED`. Deprecated.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
    

    /// Object is no longer valid (`validTimeInterval` passed).
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// Legacy alias for `EXPIRED`. Deprecated.
    ///
    /// "expired"
    #[serde(rename="expired")]
    Expired,
    
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// Legacy alias for `INACTIVE`. Deprecated.
    ///
    /// "inactive"
    #[serde(rename="inactive")]
    Inactive,
}

impl AsRef<str> for TransitObjectStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransitObjectStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            TransitObjectStateEnum::ACTIVE => "ACTIVE",
            TransitObjectStateEnum::Active => "active",
            TransitObjectStateEnum::COMPLETED => "COMPLETED",
            TransitObjectStateEnum::Completed => "completed",
            TransitObjectStateEnum::EXPIRED => "EXPIRED",
            TransitObjectStateEnum::Expired => "expired",
            TransitObjectStateEnum::INACTIVE => "INACTIVE",
            TransitObjectStateEnum::Inactive => "inactive",
        }
    }
}

impl std::convert::TryFrom< &str> for TransitObjectStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(TransitObjectStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(TransitObjectStateEnum::ACTIVE),
           "active" => Ok(TransitObjectStateEnum::Active),
           "COMPLETED" => Ok(TransitObjectStateEnum::COMPLETED),
           "completed" => Ok(TransitObjectStateEnum::Completed),
           "EXPIRED" => Ok(TransitObjectStateEnum::EXPIRED),
           "expired" => Ok(TransitObjectStateEnum::Expired),
           "INACTIVE" => Ok(TransitObjectStateEnum::INACTIVE),
           "inactive" => Ok(TransitObjectStateEnum::Inactive),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransitObjectStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransitObjectTicketStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the ticket. For states which affect display, use the `state` field instead.
pub enum TransitObjectTicketStatusEnum {
    
    /// "TICKET_STATUS_UNSPECIFIED"
    #[serde(rename="TICKET_STATUS_UNSPECIFIED")]
    TICKETSTATUSUNSPECIFIED,
    
    /// "USED"
    #[serde(rename="USED")]
    USED,
    

    /// Legacy alias for `USED`. Deprecated.
    ///
    /// "used"
    #[serde(rename="used")]
    Used,
    
    /// "REFUNDED"
    #[serde(rename="REFUNDED")]
    REFUNDED,
    

    /// Legacy alias for `REFUNDED`. Deprecated.
    ///
    /// "refunded"
    #[serde(rename="refunded")]
    Refunded,
    
    /// "EXCHANGED"
    #[serde(rename="EXCHANGED")]
    EXCHANGED,
    

    /// Legacy alias for `EXCHANGED`. Deprecated.
    ///
    /// "exchanged"
    #[serde(rename="exchanged")]
    Exchanged,
}

impl AsRef<str> for TransitObjectTicketStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransitObjectTicketStatusEnum::TICKETSTATUSUNSPECIFIED => "TICKET_STATUS_UNSPECIFIED",
            TransitObjectTicketStatusEnum::USED => "USED",
            TransitObjectTicketStatusEnum::Used => "used",
            TransitObjectTicketStatusEnum::REFUNDED => "REFUNDED",
            TransitObjectTicketStatusEnum::Refunded => "refunded",
            TransitObjectTicketStatusEnum::EXCHANGED => "EXCHANGED",
            TransitObjectTicketStatusEnum::Exchanged => "exchanged",
        }
    }
}

impl std::convert::TryFrom< &str> for TransitObjectTicketStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TICKET_STATUS_UNSPECIFIED" => Ok(TransitObjectTicketStatusEnum::TICKETSTATUSUNSPECIFIED),
           "USED" => Ok(TransitObjectTicketStatusEnum::USED),
           "used" => Ok(TransitObjectTicketStatusEnum::Used),
           "REFUNDED" => Ok(TransitObjectTicketStatusEnum::REFUNDED),
           "refunded" => Ok(TransitObjectTicketStatusEnum::Refunded),
           "EXCHANGED" => Ok(TransitObjectTicketStatusEnum::EXCHANGED),
           "exchanged" => Ok(TransitObjectTicketStatusEnum::Exchanged),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransitObjectTicketStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransitObjectTripTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of trip this transit object represents. Used to determine the pass title and/or which symbol to use between the origin and destination.
pub enum TransitObjectTripTypeEnum {
    
    /// "TRIP_TYPE_UNSPECIFIED"
    #[serde(rename="TRIP_TYPE_UNSPECIFIED")]
    TRIPTYPEUNSPECIFIED,
    
    /// "ROUND_TRIP"
    #[serde(rename="ROUND_TRIP")]
    ROUNDTRIP,
    

    /// Legacy alias for `ROUND_TRIP`. Deprecated.
    ///
    /// "roundTrip"
    #[serde(rename="roundTrip")]
    RoundTrip,
    
    /// "ONE_WAY"
    #[serde(rename="ONE_WAY")]
    ONEWAY,
    

    /// Legacy alias for `ONE_WAY`. Deprecated.
    ///
    /// "oneWay"
    #[serde(rename="oneWay")]
    OneWay,
}

impl AsRef<str> for TransitObjectTripTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransitObjectTripTypeEnum::TRIPTYPEUNSPECIFIED => "TRIP_TYPE_UNSPECIFIED",
            TransitObjectTripTypeEnum::ROUNDTRIP => "ROUND_TRIP",
            TransitObjectTripTypeEnum::RoundTrip => "roundTrip",
            TransitObjectTripTypeEnum::ONEWAY => "ONE_WAY",
            TransitObjectTripTypeEnum::OneWay => "oneWay",
        }
    }
}

impl std::convert::TryFrom< &str> for TransitObjectTripTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRIP_TYPE_UNSPECIFIED" => Ok(TransitObjectTripTypeEnum::TRIPTYPEUNSPECIFIED),
           "ROUND_TRIP" => Ok(TransitObjectTripTypeEnum::ROUNDTRIP),
           "roundTrip" => Ok(TransitObjectTripTypeEnum::RoundTrip),
           "ONE_WAY" => Ok(TransitObjectTripTypeEnum::ONEWAY),
           "oneWay" => Ok(TransitObjectTripTypeEnum::OneWay),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransitObjectTripTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


