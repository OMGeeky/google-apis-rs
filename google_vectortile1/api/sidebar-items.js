initSidebarItems({"struct":[["Area","Represents an area. Used to represent regions such as water, parks, etc. Next ID: 10"],["BasemapZOrder","Metadata necessary to determine the ordering of a particular basemap element relative to others. To render the basemap correctly, sort by z-plane, then z-grade, then z-within-grade."],["ExtrudedArea","Represents a height-extruded area: a 3D prism with a constant X-Y plane cross section. Used to represent extruded buildings. A single building may consist of several extruded areas. The min_z and max_z fields are scaled to the size of the tile. An extruded area with a max_z value of 4096 has the same height as the width of the tile that it is on."],["Feature","A feature representing a single geographic entity."],["FeatureTile","A tile containing information about the map features located in the region it covers."],["FeaturetileGetCall","Gets a feature tile by its tile resource name."],["FeaturetileMethods","A builder providing access to all methods supported on featuretile resources. It is not used directly, but through the `SemanticTile` hub."],["FirstDerivativeElevationGrid","A packed representation of a 2D grid of uniformly spaced points containing elevation data. Each point within the grid represents the altitude in meters above average sea level at that location within the tile. Elevations provided are (generally) relative to the EGM96 geoid, however some areas will be relative to NAVD88. EGM96 and NAVD88 are off by no more than 2 meters. The grid is oriented north-west to south-east, as illustrated: rows[0].a[0] rows[0].a[m] +-----------------+ | | | N | | ^ | | | | | W <-----> E | | | | | v | | S | | | +-----------------+ rows[n].a[0] rows[n].a[m] Rather than storing the altitudes directly, we store the diffs between them as integers at some requested level of precision to take advantage of integer packing. The actual altitude values a[] can be reconstructed using the scale and each row's first_altitude and altitude_diff fields."],["Geometry","Represents the geometry of a feature, that is, the shape that it has on the map. The local tile coordinate system has the origin at the north-west (upper-left) corner of the tile, and is scaled to 4096 units across each edge. The height (Z) axis has the same scale factor: an extruded area with a max_z value of 4096 has the same height as the width of the tile that it is on. There is no clipping boundary, so it is possible that some coordinates will lie outside the tile boundaries."],["Line","Represents a 2D polyline. Used to represent segments such as roads, train tracks, etc."],["ModeledVolume","Represents a modeled volume in 3D space. Used to represent 3D buildings."],["ProviderInfo","Information about the data providers that should be included in the attribution string shown by the client."],["Relation","Represents a relation to another feature in the tile. For example, a building might be occupied by a given POI. The related feature can be retrieved using the related feature index."],["RoadInfo","Extra metadata relating to roads."],["Row","A row of altitude points in the elevation grid, ordered from west to east."],["SecondDerivativeElevationGrid","A packed representation of a 2D grid of uniformly spaced points containing elevation data. Each point within the grid represents the altitude in meters above average sea level at that location within the tile. Elevations provided are (generally) relative to the EGM96 geoid, however some areas will be relative to NAVD88. EGM96 and NAVD88 are off by no more than 2 meters. The grid is oriented north-west to south-east, as illustrated: rows[0].a[0] rows[0].a[m] +-----------------+ | | | N | | ^ | | | | | W <-----> E | | | | | v | | S | | | +-----------------+ rows[n].a[0] rows[n].a[m] Rather than storing the altitudes directly, we store the diffs of the diffs between them as integers at some requested level of precision to take advantage of integer packing. Note that the data is packed in such a way that is fast to decode in Unity and that further optimizes wire size."],["SegmentInfo","Extra metadata relating to segments."],["SemanticTile","Central instance to access all SemanticTile related resource activities"],["TerrainTile","A tile containing information about the terrain located in the region it covers."],["TerraintileGetCall","Gets a terrain tile by its tile resource name."],["TerraintileMethods","A builder providing access to all methods supported on terraintile resources. It is not used directly, but through the `SemanticTile` hub."],["TileCoordinates","Global tile coordinates. Global tile coordinates reference a specific tile on the map at a specific zoom level. The origin of this coordinate system is always at the northwest corner of the map, with x values increasing from west to east and y values increasing from north to south. Tiles are indexed using x, y coordinates from that origin. The zoom level containing the entire world in a tile is 0, and it increases as you zoom in. Zoom level n + 1 will contain 4 times as many tiles as zoom level n. The zoom level controls the level of detail of the data that is returned. In particular, this affects the set of feature types returned, their density, and geometry simplification. The exact tile contents may change over time, but care will be taken to keep supporting the most important use cases. For example, zoom level 15 shows roads for orientation and planning in the local neighborhood and zoom level 17 shows buildings to give users on foot a sense of situational awareness."],["TriangleStrip","Represents a strip of triangles. Each triangle uses the last edge of the previous one. The following diagram shows an example of a triangle strip, with each vertex labeled with its index in the vertex_index array. (1)-----(3) / \\ / \\ / \\ / \\ / \\ / \\ (0)-----(2)-----(4) Vertices may be in either clockwise or counter-clockwise order."],["Vertex2DList","2D vertex list used for lines and areas. Each entry represents an offset from the previous one in local tile coordinates. The first entry is offset from (0, 0). For example, the list of vertices [(1,1), (2, 2), (1, 2)] would be encoded in vertex offsets as [(1, 1), (1, 1), (-1, 0)]."],["Vertex3DList","3D vertex list used for modeled volumes. Each entry represents an offset from the previous one in local tile coordinates. The first coordinate is offset from (0, 0, 0)."]]});