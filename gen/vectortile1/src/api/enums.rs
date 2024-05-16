use super::*;



// region AreaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The polygon encoding type used for this area.
pub enum AreaTypeEnum {
    

    /// The first vertex in vertex_offset is the center of a triangle fan. The other vertices are arranged around this vertex in a fan shape. The following diagram showes a triangle fan polygon with the vertices labelled with their indices in the vertex_offset list. Triangle fan polygons always have a single boundary loop. Vertices may be in either a clockwise or counterclockwise order. (1) / \ / \ / \ (0)-----(2) / \ / / \ / / \ / (4)-----(3)
    ///
    /// "TRIANGLE_FAN"
    #[serde(rename="TRIANGLE_FAN")]
    TRIANGLEFAN,
    

    /// The polygon is a set of triangles with three vertex indices per triangle. The vertex indices can be found in the triangle_indices field. Indexed triangle polygons also contain information about boundary loops. These identify the loops at the boundary of the polygon and may be used in conjunction with the internal_edges field for styling. Boundary loops may represent either a hole or a disconnected component of the polygon. The following diagram shows an indexed triangle polygon with two boundary loops. (0) (4) / \ / \ / \ / \ (1)----(2) (3)----(5)
    ///
    /// "INDEXED_TRIANGLES"
    #[serde(rename="INDEXED_TRIANGLES")]
    INDEXEDTRIANGLES,
    

    /// A strip of triangles, where each triangle uses the last edge of the previous triangle. Vertices may be in either a clockwise or counterclockwise order. Only polygons without the has_external_edges flag set will use triangle strips. (0) / \ / \ / \ (2)-----(1) / \ / / \ / / \ / (4)-----(3)
    ///
    /// "TRIANGLE_STRIP"
    #[serde(rename="TRIANGLE_STRIP")]
    TRIANGLESTRIP,
}

impl AsRef<str> for AreaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AreaTypeEnum::TRIANGLEFAN => "TRIANGLE_FAN",
            AreaTypeEnum::INDEXEDTRIANGLES => "INDEXED_TRIANGLES",
            AreaTypeEnum::TRIANGLESTRIP => "TRIANGLE_STRIP",
        }
    }
}

impl std::convert::TryFrom< &str> for AreaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRIANGLE_FAN" => Ok(AreaTypeEnum::TRIANGLEFAN),
           "INDEXED_TRIANGLES" => Ok(AreaTypeEnum::INDEXEDTRIANGLES),
           "TRIANGLE_STRIP" => Ok(AreaTypeEnum::TRIANGLESTRIP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AreaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FeatureTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this feature.
pub enum FeatureTypeEnum {
    

    /// Unknown feature type.
    ///
    /// "FEATURE_TYPE_UNSPECIFIED"
    #[serde(rename="FEATURE_TYPE_UNSPECIFIED")]
    FEATURETYPEUNSPECIFIED,
    

    /// Structures such as buildings and bridges.
    ///
    /// "STRUCTURE"
    #[serde(rename="STRUCTURE")]
    STRUCTURE,
    

    /// A business serving alcoholic drinks to be consumed onsite.
    ///
    /// "BAR"
    #[serde(rename="BAR")]
    BAR,
    

    /// A financial institution that offers services to the general public.
    ///
    /// "BANK"
    #[serde(rename="BANK")]
    BANK,
    

    /// A place that provides any type of lodging for travelers.
    ///
    /// "LODGING"
    #[serde(rename="LODGING")]
    LODGING,
    

    /// A business that sells coffee, tea, and sometimes small meals.
    ///
    /// "CAFE"
    #[serde(rename="CAFE")]
    CAFE,
    

    /// A business that prepares meals on-site for service to customers.
    ///
    /// "RESTAURANT"
    #[serde(rename="RESTAURANT")]
    RESTAURANT,
    

    /// A venue for private and public events.
    ///
    /// "EVENT_VENUE"
    #[serde(rename="EVENT_VENUE")]
    EVENTVENUE,
    

    /// Place of interest to tourists, typically for natural or cultural value.
    ///
    /// "TOURIST_DESTINATION"
    #[serde(rename="TOURIST_DESTINATION")]
    TOURISTDESTINATION,
    

    /// A structure containing a business or businesses that sell goods.
    ///
    /// "SHOPPING"
    #[serde(rename="SHOPPING")]
    SHOPPING,
    

    /// Institution where young people receive general (not vocation or professional) education.
    ///
    /// "SCHOOL"
    #[serde(rename="SCHOOL")]
    SCHOOL,
    

    /// Segments such as roads and train lines.
    ///
    /// "SEGMENT"
    #[serde(rename="SEGMENT")]
    SEGMENT,
    

    /// A way leading from one place to another intended for use by vehicles.
    ///
    /// "ROAD"
    #[serde(rename="ROAD")]
    ROAD,
    

    /// A small city street, typically for travel in a residential neighborhood.
    ///
    /// "LOCAL_ROAD"
    #[serde(rename="LOCAL_ROAD")]
    LOCALROAD,
    

    /// Major through road that's expected to carry large volumes of traffic.
    ///
    /// "ARTERIAL_ROAD"
    #[serde(rename="ARTERIAL_ROAD")]
    ARTERIALROAD,
    

    /// A major road including freeways and state highways.
    ///
    /// "HIGHWAY"
    #[serde(rename="HIGHWAY")]
    HIGHWAY,
    

    /// A highway with grade-separated crossings that is accessed exclusively by ramps. These are usually called "freeways" or "motorways". The enable_detailed_highway_types request flag must be set in order for this type to be returned.
    ///
    /// "CONTROLLED_ACCESS_HIGHWAY"
    #[serde(rename="CONTROLLED_ACCESS_HIGHWAY")]
    CONTROLLEDACCESSHIGHWAY,
    

    /// A path that's primarily intended for use by pedestrians and/or cyclists.
    ///
    /// "FOOTPATH"
    #[serde(rename="FOOTPATH")]
    FOOTPATH,
    

    /// Tracks intended for use by trains.
    ///
    /// "RAIL"
    #[serde(rename="RAIL")]
    RAIL,
    

    /// Services which are part of the road network but are not roads.
    ///
    /// "FERRY"
    #[serde(rename="FERRY")]
    FERRY,
    

    /// Non-water areas such as parks and forest.
    ///
    /// "REGION"
    #[serde(rename="REGION")]
    REGION,
    

    /// Outdoor areas such as parks and botanical gardens.
    ///
    /// "PARK"
    #[serde(rename="PARK")]
    PARK,
    

    /// A pebbly or sandy shore along the edge of a sea or lake.
    ///
    /// "BEACH"
    #[serde(rename="BEACH")]
    BEACH,
    

    /// Area of land covered by trees.
    ///
    /// "FOREST"
    #[serde(rename="FOREST")]
    FOREST,
    

    /// Political entities, such as provinces and districts.
    ///
    /// "POLITICAL"
    #[serde(rename="POLITICAL")]
    POLITICAL,
    

    /// Top-level divisions within a country, such as prefectures or states.
    ///
    /// "ADMINISTRATIVE_AREA1"
    #[serde(rename="ADMINISTRATIVE_AREA1")]
    ADMINISTRATIVEAREA1,
    

    /// Cities, towns, and other municipalities.
    ///
    /// "LOCALITY"
    #[serde(rename="LOCALITY")]
    LOCALITY,
    

    /// Divisions within a locality like a borough or ward.
    ///
    /// "SUBLOCALITY"
    #[serde(rename="SUBLOCALITY")]
    SUBLOCALITY,
    

    /// Water features such as rivers and lakes.
    ///
    /// "WATER"
    #[serde(rename="WATER")]
    WATER,
}

impl AsRef<str> for FeatureTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FeatureTypeEnum::FEATURETYPEUNSPECIFIED => "FEATURE_TYPE_UNSPECIFIED",
            FeatureTypeEnum::STRUCTURE => "STRUCTURE",
            FeatureTypeEnum::BAR => "BAR",
            FeatureTypeEnum::BANK => "BANK",
            FeatureTypeEnum::LODGING => "LODGING",
            FeatureTypeEnum::CAFE => "CAFE",
            FeatureTypeEnum::RESTAURANT => "RESTAURANT",
            FeatureTypeEnum::EVENTVENUE => "EVENT_VENUE",
            FeatureTypeEnum::TOURISTDESTINATION => "TOURIST_DESTINATION",
            FeatureTypeEnum::SHOPPING => "SHOPPING",
            FeatureTypeEnum::SCHOOL => "SCHOOL",
            FeatureTypeEnum::SEGMENT => "SEGMENT",
            FeatureTypeEnum::ROAD => "ROAD",
            FeatureTypeEnum::LOCALROAD => "LOCAL_ROAD",
            FeatureTypeEnum::ARTERIALROAD => "ARTERIAL_ROAD",
            FeatureTypeEnum::HIGHWAY => "HIGHWAY",
            FeatureTypeEnum::CONTROLLEDACCESSHIGHWAY => "CONTROLLED_ACCESS_HIGHWAY",
            FeatureTypeEnum::FOOTPATH => "FOOTPATH",
            FeatureTypeEnum::RAIL => "RAIL",
            FeatureTypeEnum::FERRY => "FERRY",
            FeatureTypeEnum::REGION => "REGION",
            FeatureTypeEnum::PARK => "PARK",
            FeatureTypeEnum::BEACH => "BEACH",
            FeatureTypeEnum::FOREST => "FOREST",
            FeatureTypeEnum::POLITICAL => "POLITICAL",
            FeatureTypeEnum::ADMINISTRATIVEAREA1 => "ADMINISTRATIVE_AREA1",
            FeatureTypeEnum::LOCALITY => "LOCALITY",
            FeatureTypeEnum::SUBLOCALITY => "SUBLOCALITY",
            FeatureTypeEnum::WATER => "WATER",
        }
    }
}

impl std::convert::TryFrom< &str> for FeatureTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEATURE_TYPE_UNSPECIFIED" => Ok(FeatureTypeEnum::FEATURETYPEUNSPECIFIED),
           "STRUCTURE" => Ok(FeatureTypeEnum::STRUCTURE),
           "BAR" => Ok(FeatureTypeEnum::BAR),
           "BANK" => Ok(FeatureTypeEnum::BANK),
           "LODGING" => Ok(FeatureTypeEnum::LODGING),
           "CAFE" => Ok(FeatureTypeEnum::CAFE),
           "RESTAURANT" => Ok(FeatureTypeEnum::RESTAURANT),
           "EVENT_VENUE" => Ok(FeatureTypeEnum::EVENTVENUE),
           "TOURIST_DESTINATION" => Ok(FeatureTypeEnum::TOURISTDESTINATION),
           "SHOPPING" => Ok(FeatureTypeEnum::SHOPPING),
           "SCHOOL" => Ok(FeatureTypeEnum::SCHOOL),
           "SEGMENT" => Ok(FeatureTypeEnum::SEGMENT),
           "ROAD" => Ok(FeatureTypeEnum::ROAD),
           "LOCAL_ROAD" => Ok(FeatureTypeEnum::LOCALROAD),
           "ARTERIAL_ROAD" => Ok(FeatureTypeEnum::ARTERIALROAD),
           "HIGHWAY" => Ok(FeatureTypeEnum::HIGHWAY),
           "CONTROLLED_ACCESS_HIGHWAY" => Ok(FeatureTypeEnum::CONTROLLEDACCESSHIGHWAY),
           "FOOTPATH" => Ok(FeatureTypeEnum::FOOTPATH),
           "RAIL" => Ok(FeatureTypeEnum::RAIL),
           "FERRY" => Ok(FeatureTypeEnum::FERRY),
           "REGION" => Ok(FeatureTypeEnum::REGION),
           "PARK" => Ok(FeatureTypeEnum::PARK),
           "BEACH" => Ok(FeatureTypeEnum::BEACH),
           "FOREST" => Ok(FeatureTypeEnum::FOREST),
           "POLITICAL" => Ok(FeatureTypeEnum::POLITICAL),
           "ADMINISTRATIVE_AREA1" => Ok(FeatureTypeEnum::ADMINISTRATIVEAREA1),
           "LOCALITY" => Ok(FeatureTypeEnum::LOCALITY),
           "SUBLOCALITY" => Ok(FeatureTypeEnum::SUBLOCALITY),
           "WATER" => Ok(FeatureTypeEnum::WATER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FeatureTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FeatureTileStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tile response status code to support tile caching.
pub enum FeatureTileStatusEnum {
    

    /// Everything worked out OK. The cache-control header determines how long this Tile response may be cached by the client. See also version_id and STATUS_OK_DATA_UNCHANGED.
    ///
    /// "STATUS_OK"
    #[serde(rename="STATUS_OK")]
    STATUSOK,
    

    /// Indicates that the request was processed successfully and that the tile data that would have been returned are identical to the data already in the client's cache, as specified by the value of client_tile_version_id contained in GetFeatureTileRequest. In particular, the tile's features and providers will not be populated when the tile data is identical. However, the cache-control header and version_id can still change even when the tile contents itself does not, so clients should always use the most recent values returned by the API.
    ///
    /// "STATUS_OK_DATA_UNCHANGED"
    #[serde(rename="STATUS_OK_DATA_UNCHANGED")]
    STATUSOKDATAUNCHANGED,
}

impl AsRef<str> for FeatureTileStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FeatureTileStatusEnum::STATUSOK => "STATUS_OK",
            FeatureTileStatusEnum::STATUSOKDATAUNCHANGED => "STATUS_OK_DATA_UNCHANGED",
        }
    }
}

impl std::convert::TryFrom< &str> for FeatureTileStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_OK" => Ok(FeatureTileStatusEnum::STATUSOK),
           "STATUS_OK_DATA_UNCHANGED" => Ok(FeatureTileStatusEnum::STATUSOKDATAUNCHANGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FeatureTileStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RelationRelationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Relation type between the origin feature to the related feature.
pub enum RelationRelationTypeEnum {
    

    /// Unspecified relation type. Should never happen.
    ///
    /// "RELATION_TYPE_UNSPECIFIED"
    #[serde(rename="RELATION_TYPE_UNSPECIFIED")]
    RELATIONTYPEUNSPECIFIED,
    

    /// The origin feature occupies the related feature.
    ///
    /// "OCCUPIES"
    #[serde(rename="OCCUPIES")]
    OCCUPIES,
    

    /// The origin feature is primarily occupied by the related feature.
    ///
    /// "PRIMARILY_OCCUPIED_BY"
    #[serde(rename="PRIMARILY_OCCUPIED_BY")]
    PRIMARILYOCCUPIEDBY,
}

impl AsRef<str> for RelationRelationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RelationRelationTypeEnum::RELATIONTYPEUNSPECIFIED => "RELATION_TYPE_UNSPECIFIED",
            RelationRelationTypeEnum::OCCUPIES => "OCCUPIES",
            RelationRelationTypeEnum::PRIMARILYOCCUPIEDBY => "PRIMARILY_OCCUPIED_BY",
        }
    }
}

impl std::convert::TryFrom< &str> for RelationRelationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELATION_TYPE_UNSPECIFIED" => Ok(RelationRelationTypeEnum::RELATIONTYPEUNSPECIFIED),
           "OCCUPIES" => Ok(RelationRelationTypeEnum::OCCUPIES),
           "PRIMARILY_OCCUPIED_BY" => Ok(RelationRelationTypeEnum::PRIMARILYOCCUPIEDBY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RelationRelationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FeaturetileClientInfoPlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Platform where the application is running.
pub enum FeaturetileClientInfoPlatformEnum {
    

    /// Unspecified or unknown OS.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// Development environment.
    ///
    /// "EDITOR"
    #[serde(rename="EDITOR")]
    EDITOR,
    

    /// macOS.
    ///
    /// "MAC_OS"
    #[serde(rename="MAC_OS")]
    MACOS,
    

    /// Windows.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
    

    /// Linux
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Android
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// iOS
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// WebGL.
    ///
    /// "WEB_GL"
    #[serde(rename="WEB_GL")]
    WEBGL,
}

impl AsRef<str> for FeaturetileClientInfoPlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FeaturetileClientInfoPlatformEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            FeaturetileClientInfoPlatformEnum::EDITOR => "EDITOR",
            FeaturetileClientInfoPlatformEnum::MACOS => "MAC_OS",
            FeaturetileClientInfoPlatformEnum::WINDOWS => "WINDOWS",
            FeaturetileClientInfoPlatformEnum::LINUX => "LINUX",
            FeaturetileClientInfoPlatformEnum::ANDROID => "ANDROID",
            FeaturetileClientInfoPlatformEnum::IOS => "IOS",
            FeaturetileClientInfoPlatformEnum::WEBGL => "WEB_GL",
        }
    }
}

impl std::convert::TryFrom< &str> for FeaturetileClientInfoPlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(FeaturetileClientInfoPlatformEnum::PLATFORMUNSPECIFIED),
           "EDITOR" => Ok(FeaturetileClientInfoPlatformEnum::EDITOR),
           "MAC_OS" => Ok(FeaturetileClientInfoPlatformEnum::MACOS),
           "WINDOWS" => Ok(FeaturetileClientInfoPlatformEnum::WINDOWS),
           "LINUX" => Ok(FeaturetileClientInfoPlatformEnum::LINUX),
           "ANDROID" => Ok(FeaturetileClientInfoPlatformEnum::ANDROID),
           "IOS" => Ok(FeaturetileClientInfoPlatformEnum::IOS),
           "WEB_GL" => Ok(FeaturetileClientInfoPlatformEnum::WEBGL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FeaturetileClientInfoPlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TerraintileClientInfoPlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Platform where the application is running.
pub enum TerraintileClientInfoPlatformEnum {
    

    /// Unspecified or unknown OS.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// Development environment.
    ///
    /// "EDITOR"
    #[serde(rename="EDITOR")]
    EDITOR,
    

    /// macOS.
    ///
    /// "MAC_OS"
    #[serde(rename="MAC_OS")]
    MACOS,
    

    /// Windows.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
    

    /// Linux
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Android
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// iOS
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// WebGL.
    ///
    /// "WEB_GL"
    #[serde(rename="WEB_GL")]
    WEBGL,
}

impl AsRef<str> for TerraintileClientInfoPlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TerraintileClientInfoPlatformEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            TerraintileClientInfoPlatformEnum::EDITOR => "EDITOR",
            TerraintileClientInfoPlatformEnum::MACOS => "MAC_OS",
            TerraintileClientInfoPlatformEnum::WINDOWS => "WINDOWS",
            TerraintileClientInfoPlatformEnum::LINUX => "LINUX",
            TerraintileClientInfoPlatformEnum::ANDROID => "ANDROID",
            TerraintileClientInfoPlatformEnum::IOS => "IOS",
            TerraintileClientInfoPlatformEnum::WEBGL => "WEB_GL",
        }
    }
}

impl std::convert::TryFrom< &str> for TerraintileClientInfoPlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(TerraintileClientInfoPlatformEnum::PLATFORMUNSPECIFIED),
           "EDITOR" => Ok(TerraintileClientInfoPlatformEnum::EDITOR),
           "MAC_OS" => Ok(TerraintileClientInfoPlatformEnum::MACOS),
           "WINDOWS" => Ok(TerraintileClientInfoPlatformEnum::WINDOWS),
           "LINUX" => Ok(TerraintileClientInfoPlatformEnum::LINUX),
           "ANDROID" => Ok(TerraintileClientInfoPlatformEnum::ANDROID),
           "IOS" => Ok(TerraintileClientInfoPlatformEnum::IOS),
           "WEB_GL" => Ok(TerraintileClientInfoPlatformEnum::WEBGL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TerraintileClientInfoPlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TerraintileTerrainFormatsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Terrain formats that the client understands.
pub enum TerraintileTerrainFormatsEnum {
    

    /// An unknown or unspecified terrain format.
    ///
    /// "TERRAIN_FORMAT_UNKNOWN"
    #[serde(rename="TERRAIN_FORMAT_UNKNOWN")]
    TERRAINFORMATUNKNOWN,
    

    /// Terrain elevation data encoded as a FirstDerivativeElevationGrid. .
    ///
    /// "FIRST_DERIVATIVE"
    #[serde(rename="FIRST_DERIVATIVE")]
    FIRSTDERIVATIVE,
    

    /// Terrain elevation data encoded as a SecondDerivativeElevationGrid.
    ///
    /// "SECOND_DERIVATIVE"
    #[serde(rename="SECOND_DERIVATIVE")]
    SECONDDERIVATIVE,
}

impl AsRef<str> for TerraintileTerrainFormatsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TerraintileTerrainFormatsEnum::TERRAINFORMATUNKNOWN => "TERRAIN_FORMAT_UNKNOWN",
            TerraintileTerrainFormatsEnum::FIRSTDERIVATIVE => "FIRST_DERIVATIVE",
            TerraintileTerrainFormatsEnum::SECONDDERIVATIVE => "SECOND_DERIVATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for TerraintileTerrainFormatsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TERRAIN_FORMAT_UNKNOWN" => Ok(TerraintileTerrainFormatsEnum::TERRAINFORMATUNKNOWN),
           "FIRST_DERIVATIVE" => Ok(TerraintileTerrainFormatsEnum::FIRSTDERIVATIVE),
           "SECOND_DERIVATIVE" => Ok(TerraintileTerrainFormatsEnum::SECONDDERIVATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TerraintileTerrainFormatsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


