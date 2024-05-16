use super::*;



// region AlbumPositionPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of position, for a media or enrichment item.
pub enum AlbumPositionPositionEnum {
    

    /// Default value if this enum isn't set.
    ///
    /// "POSITION_TYPE_UNSPECIFIED"
    #[serde(rename="POSITION_TYPE_UNSPECIFIED")]
    POSITIONTYPEUNSPECIFIED,
    

    /// At the beginning of the album.
    ///
    /// "FIRST_IN_ALBUM"
    #[serde(rename="FIRST_IN_ALBUM")]
    FIRSTINALBUM,
    

    /// At the end of the album.
    ///
    /// "LAST_IN_ALBUM"
    #[serde(rename="LAST_IN_ALBUM")]
    LASTINALBUM,
    

    /// After a media item.
    ///
    /// "AFTER_MEDIA_ITEM"
    #[serde(rename="AFTER_MEDIA_ITEM")]
    AFTERMEDIAITEM,
    

    /// After an enrichment item.
    ///
    /// "AFTER_ENRICHMENT_ITEM"
    #[serde(rename="AFTER_ENRICHMENT_ITEM")]
    AFTERENRICHMENTITEM,
}

impl AsRef<str> for AlbumPositionPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AlbumPositionPositionEnum::POSITIONTYPEUNSPECIFIED => "POSITION_TYPE_UNSPECIFIED",
            AlbumPositionPositionEnum::FIRSTINALBUM => "FIRST_IN_ALBUM",
            AlbumPositionPositionEnum::LASTINALBUM => "LAST_IN_ALBUM",
            AlbumPositionPositionEnum::AFTERMEDIAITEM => "AFTER_MEDIA_ITEM",
            AlbumPositionPositionEnum::AFTERENRICHMENTITEM => "AFTER_ENRICHMENT_ITEM",
        }
    }
}

impl std::convert::TryFrom< &str> for AlbumPositionPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POSITION_TYPE_UNSPECIFIED" => Ok(AlbumPositionPositionEnum::POSITIONTYPEUNSPECIFIED),
           "FIRST_IN_ALBUM" => Ok(AlbumPositionPositionEnum::FIRSTINALBUM),
           "LAST_IN_ALBUM" => Ok(AlbumPositionPositionEnum::LASTINALBUM),
           "AFTER_MEDIA_ITEM" => Ok(AlbumPositionPositionEnum::AFTERMEDIAITEM),
           "AFTER_ENRICHMENT_ITEM" => Ok(AlbumPositionPositionEnum::AFTERENRICHMENTITEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AlbumPositionPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentFilterExcludedContentCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The set of categories which are not to be included in the media item search results. The items in the set are ORed. There's a maximum of 10 `excludedContentCategories` per request.
pub enum ContentFilterExcludedContentCategoriesEnum {
    

    /// Default content category. This category is ignored when any other category is used in the filter.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Media items containing landscapes.
    ///
    /// "LANDSCAPES"
    #[serde(rename="LANDSCAPES")]
    LANDSCAPES,
    

    /// Media items containing receipts.
    ///
    /// "RECEIPTS"
    #[serde(rename="RECEIPTS")]
    RECEIPTS,
    

    /// Media items containing cityscapes.
    ///
    /// "CITYSCAPES"
    #[serde(rename="CITYSCAPES")]
    CITYSCAPES,
    

    /// Media items containing landmarks.
    ///
    /// "LANDMARKS"
    #[serde(rename="LANDMARKS")]
    LANDMARKS,
    

    /// Media items that are selfies.
    ///
    /// "SELFIES"
    #[serde(rename="SELFIES")]
    SELFIES,
    

    /// Media items containing people.
    ///
    /// "PEOPLE"
    #[serde(rename="PEOPLE")]
    PEOPLE,
    

    /// Media items containing pets.
    ///
    /// "PETS"
    #[serde(rename="PETS")]
    PETS,
    

    /// Media items from weddings.
    ///
    /// "WEDDINGS"
    #[serde(rename="WEDDINGS")]
    WEDDINGS,
    

    /// Media items from birthdays.
    ///
    /// "BIRTHDAYS"
    #[serde(rename="BIRTHDAYS")]
    BIRTHDAYS,
    

    /// Media items containing documents.
    ///
    /// "DOCUMENTS"
    #[serde(rename="DOCUMENTS")]
    DOCUMENTS,
    

    /// Media items taken during travel.
    ///
    /// "TRAVEL"
    #[serde(rename="TRAVEL")]
    TRAVEL,
    

    /// Media items containing animals.
    ///
    /// "ANIMALS"
    #[serde(rename="ANIMALS")]
    ANIMALS,
    

    /// Media items containing food.
    ///
    /// "FOOD"
    #[serde(rename="FOOD")]
    FOOD,
    

    /// Media items from sporting events.
    ///
    /// "SPORT"
    #[serde(rename="SPORT")]
    SPORT,
    

    /// Media items taken at night.
    ///
    /// "NIGHT"
    #[serde(rename="NIGHT")]
    NIGHT,
    

    /// Media items from performances.
    ///
    /// "PERFORMANCES"
    #[serde(rename="PERFORMANCES")]
    PERFORMANCES,
    

    /// Media items containing whiteboards.
    ///
    /// "WHITEBOARDS"
    #[serde(rename="WHITEBOARDS")]
    WHITEBOARDS,
    

    /// Media items that are screenshots.
    ///
    /// "SCREENSHOTS"
    #[serde(rename="SCREENSHOTS")]
    SCREENSHOTS,
    

    /// Media items that are considered to be utility. These include, but aren't limited to documents, screenshots, whiteboards etc.
    ///
    /// "UTILITY"
    #[serde(rename="UTILITY")]
    UTILITY,
    

    /// Media items containing art.
    ///
    /// "ARTS"
    #[serde(rename="ARTS")]
    ARTS,
    

    /// Media items containing crafts.
    ///
    /// "CRAFTS"
    #[serde(rename="CRAFTS")]
    CRAFTS,
    

    /// Media items related to fashion.
    ///
    /// "FASHION"
    #[serde(rename="FASHION")]
    FASHION,
    

    /// Media items containing houses.
    ///
    /// "HOUSES"
    #[serde(rename="HOUSES")]
    HOUSES,
    

    /// Media items containing gardens.
    ///
    /// "GARDENS"
    #[serde(rename="GARDENS")]
    GARDENS,
    

    /// Media items containing flowers.
    ///
    /// "FLOWERS"
    #[serde(rename="FLOWERS")]
    FLOWERS,
    

    /// Media items taken of holidays.
    ///
    /// "HOLIDAYS"
    #[serde(rename="HOLIDAYS")]
    HOLIDAYS,
}

impl AsRef<str> for ContentFilterExcludedContentCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentFilterExcludedContentCategoriesEnum::NONE => "NONE",
            ContentFilterExcludedContentCategoriesEnum::LANDSCAPES => "LANDSCAPES",
            ContentFilterExcludedContentCategoriesEnum::RECEIPTS => "RECEIPTS",
            ContentFilterExcludedContentCategoriesEnum::CITYSCAPES => "CITYSCAPES",
            ContentFilterExcludedContentCategoriesEnum::LANDMARKS => "LANDMARKS",
            ContentFilterExcludedContentCategoriesEnum::SELFIES => "SELFIES",
            ContentFilterExcludedContentCategoriesEnum::PEOPLE => "PEOPLE",
            ContentFilterExcludedContentCategoriesEnum::PETS => "PETS",
            ContentFilterExcludedContentCategoriesEnum::WEDDINGS => "WEDDINGS",
            ContentFilterExcludedContentCategoriesEnum::BIRTHDAYS => "BIRTHDAYS",
            ContentFilterExcludedContentCategoriesEnum::DOCUMENTS => "DOCUMENTS",
            ContentFilterExcludedContentCategoriesEnum::TRAVEL => "TRAVEL",
            ContentFilterExcludedContentCategoriesEnum::ANIMALS => "ANIMALS",
            ContentFilterExcludedContentCategoriesEnum::FOOD => "FOOD",
            ContentFilterExcludedContentCategoriesEnum::SPORT => "SPORT",
            ContentFilterExcludedContentCategoriesEnum::NIGHT => "NIGHT",
            ContentFilterExcludedContentCategoriesEnum::PERFORMANCES => "PERFORMANCES",
            ContentFilterExcludedContentCategoriesEnum::WHITEBOARDS => "WHITEBOARDS",
            ContentFilterExcludedContentCategoriesEnum::SCREENSHOTS => "SCREENSHOTS",
            ContentFilterExcludedContentCategoriesEnum::UTILITY => "UTILITY",
            ContentFilterExcludedContentCategoriesEnum::ARTS => "ARTS",
            ContentFilterExcludedContentCategoriesEnum::CRAFTS => "CRAFTS",
            ContentFilterExcludedContentCategoriesEnum::FASHION => "FASHION",
            ContentFilterExcludedContentCategoriesEnum::HOUSES => "HOUSES",
            ContentFilterExcludedContentCategoriesEnum::GARDENS => "GARDENS",
            ContentFilterExcludedContentCategoriesEnum::FLOWERS => "FLOWERS",
            ContentFilterExcludedContentCategoriesEnum::HOLIDAYS => "HOLIDAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentFilterExcludedContentCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(ContentFilterExcludedContentCategoriesEnum::NONE),
           "LANDSCAPES" => Ok(ContentFilterExcludedContentCategoriesEnum::LANDSCAPES),
           "RECEIPTS" => Ok(ContentFilterExcludedContentCategoriesEnum::RECEIPTS),
           "CITYSCAPES" => Ok(ContentFilterExcludedContentCategoriesEnum::CITYSCAPES),
           "LANDMARKS" => Ok(ContentFilterExcludedContentCategoriesEnum::LANDMARKS),
           "SELFIES" => Ok(ContentFilterExcludedContentCategoriesEnum::SELFIES),
           "PEOPLE" => Ok(ContentFilterExcludedContentCategoriesEnum::PEOPLE),
           "PETS" => Ok(ContentFilterExcludedContentCategoriesEnum::PETS),
           "WEDDINGS" => Ok(ContentFilterExcludedContentCategoriesEnum::WEDDINGS),
           "BIRTHDAYS" => Ok(ContentFilterExcludedContentCategoriesEnum::BIRTHDAYS),
           "DOCUMENTS" => Ok(ContentFilterExcludedContentCategoriesEnum::DOCUMENTS),
           "TRAVEL" => Ok(ContentFilterExcludedContentCategoriesEnum::TRAVEL),
           "ANIMALS" => Ok(ContentFilterExcludedContentCategoriesEnum::ANIMALS),
           "FOOD" => Ok(ContentFilterExcludedContentCategoriesEnum::FOOD),
           "SPORT" => Ok(ContentFilterExcludedContentCategoriesEnum::SPORT),
           "NIGHT" => Ok(ContentFilterExcludedContentCategoriesEnum::NIGHT),
           "PERFORMANCES" => Ok(ContentFilterExcludedContentCategoriesEnum::PERFORMANCES),
           "WHITEBOARDS" => Ok(ContentFilterExcludedContentCategoriesEnum::WHITEBOARDS),
           "SCREENSHOTS" => Ok(ContentFilterExcludedContentCategoriesEnum::SCREENSHOTS),
           "UTILITY" => Ok(ContentFilterExcludedContentCategoriesEnum::UTILITY),
           "ARTS" => Ok(ContentFilterExcludedContentCategoriesEnum::ARTS),
           "CRAFTS" => Ok(ContentFilterExcludedContentCategoriesEnum::CRAFTS),
           "FASHION" => Ok(ContentFilterExcludedContentCategoriesEnum::FASHION),
           "HOUSES" => Ok(ContentFilterExcludedContentCategoriesEnum::HOUSES),
           "GARDENS" => Ok(ContentFilterExcludedContentCategoriesEnum::GARDENS),
           "FLOWERS" => Ok(ContentFilterExcludedContentCategoriesEnum::FLOWERS),
           "HOLIDAYS" => Ok(ContentFilterExcludedContentCategoriesEnum::HOLIDAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentFilterExcludedContentCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentFilterIncludedContentCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The set of categories to be included in the media item search results. The items in the set are ORed. There's a maximum of 10 `includedContentCategories` per request.
pub enum ContentFilterIncludedContentCategoriesEnum {
    

    /// Default content category. This category is ignored when any other category is used in the filter.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Media items containing landscapes.
    ///
    /// "LANDSCAPES"
    #[serde(rename="LANDSCAPES")]
    LANDSCAPES,
    

    /// Media items containing receipts.
    ///
    /// "RECEIPTS"
    #[serde(rename="RECEIPTS")]
    RECEIPTS,
    

    /// Media items containing cityscapes.
    ///
    /// "CITYSCAPES"
    #[serde(rename="CITYSCAPES")]
    CITYSCAPES,
    

    /// Media items containing landmarks.
    ///
    /// "LANDMARKS"
    #[serde(rename="LANDMARKS")]
    LANDMARKS,
    

    /// Media items that are selfies.
    ///
    /// "SELFIES"
    #[serde(rename="SELFIES")]
    SELFIES,
    

    /// Media items containing people.
    ///
    /// "PEOPLE"
    #[serde(rename="PEOPLE")]
    PEOPLE,
    

    /// Media items containing pets.
    ///
    /// "PETS"
    #[serde(rename="PETS")]
    PETS,
    

    /// Media items from weddings.
    ///
    /// "WEDDINGS"
    #[serde(rename="WEDDINGS")]
    WEDDINGS,
    

    /// Media items from birthdays.
    ///
    /// "BIRTHDAYS"
    #[serde(rename="BIRTHDAYS")]
    BIRTHDAYS,
    

    /// Media items containing documents.
    ///
    /// "DOCUMENTS"
    #[serde(rename="DOCUMENTS")]
    DOCUMENTS,
    

    /// Media items taken during travel.
    ///
    /// "TRAVEL"
    #[serde(rename="TRAVEL")]
    TRAVEL,
    

    /// Media items containing animals.
    ///
    /// "ANIMALS"
    #[serde(rename="ANIMALS")]
    ANIMALS,
    

    /// Media items containing food.
    ///
    /// "FOOD"
    #[serde(rename="FOOD")]
    FOOD,
    

    /// Media items from sporting events.
    ///
    /// "SPORT"
    #[serde(rename="SPORT")]
    SPORT,
    

    /// Media items taken at night.
    ///
    /// "NIGHT"
    #[serde(rename="NIGHT")]
    NIGHT,
    

    /// Media items from performances.
    ///
    /// "PERFORMANCES"
    #[serde(rename="PERFORMANCES")]
    PERFORMANCES,
    

    /// Media items containing whiteboards.
    ///
    /// "WHITEBOARDS"
    #[serde(rename="WHITEBOARDS")]
    WHITEBOARDS,
    

    /// Media items that are screenshots.
    ///
    /// "SCREENSHOTS"
    #[serde(rename="SCREENSHOTS")]
    SCREENSHOTS,
    

    /// Media items that are considered to be utility. These include, but aren't limited to documents, screenshots, whiteboards etc.
    ///
    /// "UTILITY"
    #[serde(rename="UTILITY")]
    UTILITY,
    

    /// Media items containing art.
    ///
    /// "ARTS"
    #[serde(rename="ARTS")]
    ARTS,
    

    /// Media items containing crafts.
    ///
    /// "CRAFTS"
    #[serde(rename="CRAFTS")]
    CRAFTS,
    

    /// Media items related to fashion.
    ///
    /// "FASHION"
    #[serde(rename="FASHION")]
    FASHION,
    

    /// Media items containing houses.
    ///
    /// "HOUSES"
    #[serde(rename="HOUSES")]
    HOUSES,
    

    /// Media items containing gardens.
    ///
    /// "GARDENS"
    #[serde(rename="GARDENS")]
    GARDENS,
    

    /// Media items containing flowers.
    ///
    /// "FLOWERS"
    #[serde(rename="FLOWERS")]
    FLOWERS,
    

    /// Media items taken of holidays.
    ///
    /// "HOLIDAYS"
    #[serde(rename="HOLIDAYS")]
    HOLIDAYS,
}

impl AsRef<str> for ContentFilterIncludedContentCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentFilterIncludedContentCategoriesEnum::NONE => "NONE",
            ContentFilterIncludedContentCategoriesEnum::LANDSCAPES => "LANDSCAPES",
            ContentFilterIncludedContentCategoriesEnum::RECEIPTS => "RECEIPTS",
            ContentFilterIncludedContentCategoriesEnum::CITYSCAPES => "CITYSCAPES",
            ContentFilterIncludedContentCategoriesEnum::LANDMARKS => "LANDMARKS",
            ContentFilterIncludedContentCategoriesEnum::SELFIES => "SELFIES",
            ContentFilterIncludedContentCategoriesEnum::PEOPLE => "PEOPLE",
            ContentFilterIncludedContentCategoriesEnum::PETS => "PETS",
            ContentFilterIncludedContentCategoriesEnum::WEDDINGS => "WEDDINGS",
            ContentFilterIncludedContentCategoriesEnum::BIRTHDAYS => "BIRTHDAYS",
            ContentFilterIncludedContentCategoriesEnum::DOCUMENTS => "DOCUMENTS",
            ContentFilterIncludedContentCategoriesEnum::TRAVEL => "TRAVEL",
            ContentFilterIncludedContentCategoriesEnum::ANIMALS => "ANIMALS",
            ContentFilterIncludedContentCategoriesEnum::FOOD => "FOOD",
            ContentFilterIncludedContentCategoriesEnum::SPORT => "SPORT",
            ContentFilterIncludedContentCategoriesEnum::NIGHT => "NIGHT",
            ContentFilterIncludedContentCategoriesEnum::PERFORMANCES => "PERFORMANCES",
            ContentFilterIncludedContentCategoriesEnum::WHITEBOARDS => "WHITEBOARDS",
            ContentFilterIncludedContentCategoriesEnum::SCREENSHOTS => "SCREENSHOTS",
            ContentFilterIncludedContentCategoriesEnum::UTILITY => "UTILITY",
            ContentFilterIncludedContentCategoriesEnum::ARTS => "ARTS",
            ContentFilterIncludedContentCategoriesEnum::CRAFTS => "CRAFTS",
            ContentFilterIncludedContentCategoriesEnum::FASHION => "FASHION",
            ContentFilterIncludedContentCategoriesEnum::HOUSES => "HOUSES",
            ContentFilterIncludedContentCategoriesEnum::GARDENS => "GARDENS",
            ContentFilterIncludedContentCategoriesEnum::FLOWERS => "FLOWERS",
            ContentFilterIncludedContentCategoriesEnum::HOLIDAYS => "HOLIDAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentFilterIncludedContentCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(ContentFilterIncludedContentCategoriesEnum::NONE),
           "LANDSCAPES" => Ok(ContentFilterIncludedContentCategoriesEnum::LANDSCAPES),
           "RECEIPTS" => Ok(ContentFilterIncludedContentCategoriesEnum::RECEIPTS),
           "CITYSCAPES" => Ok(ContentFilterIncludedContentCategoriesEnum::CITYSCAPES),
           "LANDMARKS" => Ok(ContentFilterIncludedContentCategoriesEnum::LANDMARKS),
           "SELFIES" => Ok(ContentFilterIncludedContentCategoriesEnum::SELFIES),
           "PEOPLE" => Ok(ContentFilterIncludedContentCategoriesEnum::PEOPLE),
           "PETS" => Ok(ContentFilterIncludedContentCategoriesEnum::PETS),
           "WEDDINGS" => Ok(ContentFilterIncludedContentCategoriesEnum::WEDDINGS),
           "BIRTHDAYS" => Ok(ContentFilterIncludedContentCategoriesEnum::BIRTHDAYS),
           "DOCUMENTS" => Ok(ContentFilterIncludedContentCategoriesEnum::DOCUMENTS),
           "TRAVEL" => Ok(ContentFilterIncludedContentCategoriesEnum::TRAVEL),
           "ANIMALS" => Ok(ContentFilterIncludedContentCategoriesEnum::ANIMALS),
           "FOOD" => Ok(ContentFilterIncludedContentCategoriesEnum::FOOD),
           "SPORT" => Ok(ContentFilterIncludedContentCategoriesEnum::SPORT),
           "NIGHT" => Ok(ContentFilterIncludedContentCategoriesEnum::NIGHT),
           "PERFORMANCES" => Ok(ContentFilterIncludedContentCategoriesEnum::PERFORMANCES),
           "WHITEBOARDS" => Ok(ContentFilterIncludedContentCategoriesEnum::WHITEBOARDS),
           "SCREENSHOTS" => Ok(ContentFilterIncludedContentCategoriesEnum::SCREENSHOTS),
           "UTILITY" => Ok(ContentFilterIncludedContentCategoriesEnum::UTILITY),
           "ARTS" => Ok(ContentFilterIncludedContentCategoriesEnum::ARTS),
           "CRAFTS" => Ok(ContentFilterIncludedContentCategoriesEnum::CRAFTS),
           "FASHION" => Ok(ContentFilterIncludedContentCategoriesEnum::FASHION),
           "HOUSES" => Ok(ContentFilterIncludedContentCategoriesEnum::HOUSES),
           "GARDENS" => Ok(ContentFilterIncludedContentCategoriesEnum::GARDENS),
           "FLOWERS" => Ok(ContentFilterIncludedContentCategoriesEnum::FLOWERS),
           "HOLIDAYS" => Ok(ContentFilterIncludedContentCategoriesEnum::HOLIDAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentFilterIncludedContentCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FeatureFilterIncludedFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The set of features to be included in the media item search results. The items in the set are ORed and may match any of the specified features.
pub enum FeatureFilterIncludedFeaturesEnum {
    

    /// Treated as if no filters are applied. All features are included.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Media items that the user has marked as favorites in the Google Photos app.
    ///
    /// "FAVORITES"
    #[serde(rename="FAVORITES")]
    FAVORITES,
}

impl AsRef<str> for FeatureFilterIncludedFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FeatureFilterIncludedFeaturesEnum::NONE => "NONE",
            FeatureFilterIncludedFeaturesEnum::FAVORITES => "FAVORITES",
        }
    }
}

impl std::convert::TryFrom< &str> for FeatureFilterIncludedFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(FeatureFilterIncludedFeaturesEnum::NONE),
           "FAVORITES" => Ok(FeatureFilterIncludedFeaturesEnum::FAVORITES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FeatureFilterIncludedFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediaTypeFilterMediaTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The types of media items to be included. This field should be populated with only one media type. If you specify multiple media types, it results in an error.
pub enum MediaTypeFilterMediaTypesEnum {
    

    /// Treated as if no filters are applied. All media types are included.
    ///
    /// "ALL_MEDIA"
    #[serde(rename="ALL_MEDIA")]
    ALLMEDIA,
    

    /// All media items that are considered videos. This also includes movies the user has created using the Google Photos app.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// All media items that are considered photos. This includes .bmp, .gif, .ico, .jpg (and other spellings), .tiff, .webp and special photo types such as iOS live photos, Android motion photos, panoramas, photospheres.
    ///
    /// "PHOTO"
    #[serde(rename="PHOTO")]
    PHOTO,
}

impl AsRef<str> for MediaTypeFilterMediaTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediaTypeFilterMediaTypesEnum::ALLMEDIA => "ALL_MEDIA",
            MediaTypeFilterMediaTypesEnum::VIDEO => "VIDEO",
            MediaTypeFilterMediaTypesEnum::PHOTO => "PHOTO",
        }
    }
}

impl std::convert::TryFrom< &str> for MediaTypeFilterMediaTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_MEDIA" => Ok(MediaTypeFilterMediaTypesEnum::ALLMEDIA),
           "VIDEO" => Ok(MediaTypeFilterMediaTypesEnum::VIDEO),
           "PHOTO" => Ok(MediaTypeFilterMediaTypesEnum::PHOTO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediaTypeFilterMediaTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Processing status of the video.
pub enum VideoStatusEnum {
    

    /// Video processing status is unknown.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Video is being processed. The user sees an icon for this video in the Google Photos app; however, it isn't playable yet.
    ///
    /// "PROCESSING"
    #[serde(rename="PROCESSING")]
    PROCESSING,
    

    /// Video processing is complete and it is now ready for viewing. Important: attempting to download a video not in the READY state may fail.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Something has gone wrong and the video has failed to process.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for VideoStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatusEnum::UNSPECIFIED => "UNSPECIFIED",
            VideoStatusEnum::PROCESSING => "PROCESSING",
            VideoStatusEnum::READY => "READY",
            VideoStatusEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(VideoStatusEnum::UNSPECIFIED),
           "PROCESSING" => Ok(VideoStatusEnum::PROCESSING),
           "READY" => Ok(VideoStatusEnum::READY),
           "FAILED" => Ok(VideoStatusEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


