# back-end Code Documentation

## Project Overview
This backend application, developed in Rust, is designed to handle various geospatial data formats and provide API endpoints for a frontend application. It focuses on processing spatial data, handling file inputs, and serving the processed data through structured APIs.

## APIs

### File: `index.rs`
- **Structs Definitions** 
  - `Node`:
  - Represents a node in the spatial index, containing metadata and a reference to a map.
    - Fields:
        `metadata`: Stores metadata of the node, including the region it covers and any tags associated with it.
        `map`: An `Arc` pointing to a `MapType`, ensuring thread-safe access to the map data.
  - `MetaData`
    - Stores metadata associated with a node.
    - Fields:
        - `region`: A `Region` struct defining the geographical area covered by the node.
        - `tags`: A vector of key-value pairs (`String`, `String`) representing tags associated with the node.
- **Associated Type**:
  - `Envelope = AABB<Coordinate>`: Specifies that the envelope (bounding box) for `Node` is an axis-aligned bounding box with `Coordinate` points.
- **Methods**:
  - `envelope()`: Returns the `Node`'s envelope as an `AABB` created from the `top_left` and `bottom_right` corners of the node's `region`. This method is essential for integrating the node into the R-tree, allowing it to be efficiently queried based on spatial relationships.

### File: `spatial.rs`
- **Type Aliases and Structs**
  - `Coordinate`: A type alias that represents a geographical coordinate, defined as a tuple `(f64, f64)`, representing longitude and latitude.
  - `Region`: A struct that represents a geographical area, defined by two `Coordinate` points: `top_left` and `bottom_right`.
- **Methods and Implementations**:
  - The `Region` struct implements the following methods:
    - bottom_left(): Calculates and returns the bottom left `coordinate` of the region.
    - bottom_right(): Returns the bottom right `coordinate` of the region.
    - top_left(): Returns the top left `coordinate` of the region.
    - top_right(): Calculates and returns the top right `coordinate` of the region.
- **Conversions**
  - The `Region` struct implements the `From` trait for various geospatial data format specific region types, allowing these types to be converted into a standard `Region` struct. This standardization facilitates interoperability within the system.
    - **From `GeoTiffRegion` for `Region`**
      - Converts a `GeoTiffRegion` to a `Region`.
    - **From `KMLRegion` for `Region`**
      - Converts a `KMLRegion` to a `Region`, adjusting coordinate pairs to fit the `Region`'s expected structure.
    - **From `GeoJSONRegion` for `Region`**
      - converts a `GeoJSONRegion` to a `Region`.
    - **From `DT2Region` for `Region`**
      - Converts a `DT2Region` (Digital Terrain Elevation Data) to a `Region`.
    - **From `QueryRegion` for `Region`**
      - Converts a `QueryRegion` (possibly related to database queries or user input) to a `Region`.
    - **From `MBTilesRegion` for `Region`**
      - Converts an `MBTilesRegion` (MapBox Tiles) to a `Region`.
    - **From `GPKGRegion` for `Region`**
      - Converts a `GPKGRegion` (GeoPackage) to a `Region`.

### File:`geojson.rs`

- **Functions**

  - **`get_boundaries(coordinates: Vec<[f64; 2]>) -> (Coordinate, Coordinate)`**
    - Calculates and returns the boundary coordinates for GeoJSON data.

  - **`parse_geojson(reader: &mut BufReader<File>) -> Result<GeoJSONMetaData, GeoJSONErrorState>`**
    - Parses GeoJSON from a file, extracting geographical boundaries and metadata, and handles parsing errors.

- **Structs and Enums**
  - **`GeoJSONErrorState`**: Enumerates errors that can occur while parsing GeoJSON data.
  - **`GEOJSONMap`**: Represents a GeoJSON map, storing the file path.
  - **`GeoJSONRegion`**: Defines a geographical region with top-right and bottom-left coordinates.
  - **`GeoJSONMetaData`**: Contains metadata for a GeoJSON region, including its geographical boundaries and tags.


### File: `kml.rs`

- **Functions**

  - `get_boundaries(coordinates: Vec<Coordinate>) -> (Coordinate, Coordinate)`
    - Calculates and returns the minimum and maximum coordinates from a vector of `Coordinate` tuples, effectively defining a bounding box around the given coordinates.

  - `parse_kml(reader: &mut BufReader<File>) -> Result<KMLMetadata, KMLErrorState>`
    - Parses a KML (Keyhole Markup Language) file from a buffered reader, extracting geographical coordinates and metadata. It returns either `KMLMetadata` on success or a `KMLErrorState` on failure, handling specific parsing errors like unexpected formats or insufficient geographic data.

- **Structs and Enums**

  - **`KMLRegion`**
    - Represents a geographical region defined by top-right and bottom-left `Coordinate` points.

  - **`KMLMetadata`**
    - Contains metadata for a KML file, including the geographical region (as `KMLRegion`) and any additional tags as a vector of string tuples.

  - **`KMLErrorState`**
    - Enumerates possible errors encountered during KML parsing, such as:
      - `UnexpectedFormat(String)`: Indicates an unexpected format was encountered during parsing, with a message detailing the issue.
      - `NotEnoughGeoData`: Indicates that not enough geographic data was present in the file to establish a boundary.

  - **`KMLMap`**
    - Represents a KML map, storing the path to the KML file as a `PathBuf`.

### File: `gpkg.rs`

- **Structs**
  - **`GPKG`**
    - Represents the geographic bounds of a GeoPackage (GPKG) file.
    - Fields:
      - `min_x`: The minimum X coordinate (longitude).
      - `min_y`: The minimum Y coordinate (latitude).
      - `max_x`: The maximum X coordinate (longitude).
      - `max_y`: The maximum Y coordinate (latitude).

  - **`GPKGRegion`**
    - Represents a specific geographic region within a GeoPackage file.
    - Fields:
      - `top_left`: Coordinates of the top-left corner of the region (longitude, latitude).
      - `bottom_right`: Coordinates of the bottom-right corner of the region (longitude, latitude).

  - **`GPKGMetaData`**
    - Contains metadata about a GeoPackage file, including its geographic region and associated tags.
    - Fields:
      - `region`: A `GPKGRegion` object representing the geographic region covered by the GeoPackage.
      - `tags`: A vector of key-value pairs (as `String` tuples) representing additional metadata tags.

  - **`GPKGMap`**
    - Represents a map stored within a GeoPackage file.
    - Fields:
      - `path`: A `PathBuf` indicating the file path to the GeoPackage file.

- **Functions**
  - **`parse_gpkg(filepath: &str) -> Result<GPKGMetaData>`**
    - Parses a GeoPackage file to extract metadata and geographic bounds.
    - Parameters:
      - `filepath`: A string slice representing the file path to the GeoPackage file.
    - Returns:
      - A `Result` containing `GPKGMetaData` if successful, or an error if not.

### File: `shapefile.rs`

- **Traits**
  - **`FromBytes`**
    - Defines a method for converting a byte array to a specific type.
    - **Implemented for:**
      - `f64`: Converts an 8-byte array into an `f64` value using little-endian byte order.

- **Structs**
  - **`ShapeFileMap`**
    - Represents a Shapefile, including paths to its `.shp`, optional `.prj`, and optional `.tfw` files.
    - Fields:
      - `shp`: Path to the `.shp` file.
      - `prj`: Optional path to the `.prj` projection file.
      - `tfw`: Optional path to the `.tfw` world file.
  - **`ShapeFileMetaData`**
    - Contains metadata extracted from a Shapefile, including its geographical region and tags.
    - Fields:
      - `region`: The geographic region covered by the Shapefile.
      - `tags`: A vector of key-value pairs representing metadata tags.
  - **`ShapeFileHeader`**
    - Represents the header of a Shapefile, containing geographic bounding coordinates.
    - Fields:
      - `x_min`, `y_min`: The minimum X and Y coordinates.
      - `x_max`, `y_max`: The maximum X and Y coordinates.

- **Enums**
  - **`ShapeFileErrorKind`**
    - Enumerates specific errors that can occur when parsing Shapefile data.
    - Variants:
      - `UnexpectedMagicNumber`: Indicates the file does not start with the expected magic number.

- **Functions**
  - **`parse_header(buffer: &[u8]) -> Result<ShapeFileHeader, Box<dyn Error>>`**
    - Parses the header of a Shapefile from a given byte buffer.
    - Asserts that the buffer length is 100 bytes and checks for the correct magic number.
    - Extracts and returns the geographic bounding coordinates as a `ShapeFileHeader`.
  - **`parse_shapefile(shp_reader: &mut BufReader<File>, prj_reader: Option<&mut BufReader<File>>) -> Result<ShapeFileMetaData, Box<dyn Error>>`**
    - Parses a Shapefile and its optional projection file to extract metadata.
    - Reads and parses the Shapefile header for geographic bounds.
    - If a projection file is provided, reads and applies the projection to the geographic bounds.
    - Returns `ShapeFileMetaData` containing the projected geographic region and metadata tags.
    - Handles potential errors, including unsupported projections and issues with reading files.

### File: `dted.rs`

- **Functions**
  - **`parse_dddmmssh(dddmmssh: &str) -> Result<f64, DT2ErrorState>`**
    - Parses a coordinate in DDDMMSSH format and returns it as a floating-point number, handling potential errors.
  - **`parse_ddmmssh(ddmmssh: &str) -> Result<f64, DT2ErrorState>`**
    - Converts a coordinate in DDMMSSH format to a floating-point number, with error handling for parsing issues.
  - **`from_bytes(bytes: &[u8]) -> Result<UserHeaderLabel, UHLErrorState>`**
    - Extracts a User Header Label from a byte slice, providing detailed error states for parsing failures.
  - **`from_bytes(bytes: &[u8]) -> Result<DataSetIdentification, DSIErrorState>`**
    - Parses DataSet Identification from a byte array, offering granular error categorization for various parsing errors.

- **Structs and Enums**
  - **`DT2ErrorState`**
    - Enumerates general errors encountered during DTED parsing, such as format and specific section errors.
  - **`UHLErrorState`**
    - Details errors specifically related to parsing the User Header Label section of DTED data.
  - **`DSIErrorState`**
    - Categorizes errors found during the parsing of the DataSet Identification section.
  - **`DTEDMap`**
    - Represents a DTED map, encapsulating the file path and potentially other metadata.
  - **`DT2Region`**
    - Defines a region within a DTED map, likely including coordinates and boundary information.
  - **`DT2MetaData`**
    - Contains metadata about a DTED region, possibly involving geographical boundaries and descriptive tags.
  - **`UserHeaderLabel`**
    - Structures the User Header Label section of a DTED file, facilitating its parsing and interpretation.
  - **`DataSetIdentification`**
    - Represents the DataSet Identification section, crucial for understanding the DTED file's content and boundaries.

### File: `conversions.rs`

- **Trait Implementations for `MetaData` Conversion**
  - This file defines how metadata from various geographic data formats is converted into a unified `MetaData` structure.

- **Type Conversions Implemented**
  - **`KMLMetadata -> MetaData`**
    - Converts KML metadata to the unified `MetaData` format, preserving region and tags.
  - **`GeoTiffMetaData -> MetaData`**
    - Converts GeoTiff metadata to `MetaData`, including region and tags.
  - **`DT2MetaData -> MetaData`**
    - Transforms DT2 metadata into `MetaData`, maintaining region and tag information.
  - **`GeoJSONMetaData -> MetaData`**
    - Converts GeoJSON metadata into the general `MetaData` structure, with region and tags.
  - **`MBTilesMetaData -> MetaData`**
    - Adapts MBTiles metadata for the unified `MetaData` format, including region and tags.
  - **`GPKGMetaData -> MetaData`**
    - Transforms GPKG metadata into `MetaData`, capturing both region and tags.
  - **`ShapeFileMetaData -> MetaData`**
    - Converts ShapeFile metadata to `MetaData`, retaining the region and tags.

- **Common Fields in `MetaData`**
  - **`region`**: Transformed from the source metadata's region representation into a unified format.
  - **`tags`**: A collection of tags or metadata properties carried over from the original format.

### File: `mbtiles.rs`
- **Structs**
  - **`MBTilesRegion`**
    - Represents the geographic region covered by an MBTiles file.
    - Fields:
      - `top_left`: `Coordinate` (longitude, latitude) of the top-left corner.
      - `bottom_right`: `Coordinate` (longitude, latitude) of the bottom-right corner.
  - **`MBTilesMetaData`**
    - Contains metadata information for an MBTiles file, including its geographic region and additional tags.
    - Fields:
      - `region`: An instance of `MBTilesRegion` indicating the geographic area covered.
      - `tags`: A vector of key-value pairs (`Vec<(String, String)>`) representing metadata tags.
  - **`MBTilesMap`**
    - Represents an MBTiles file, primarily holding its file path.
    - Fields:
      - `path`: `PathBuf` indicating the file path of the MBTiles file.

- **Functions**
  - **`parse_mbtiles(filepath: &str) -> Result<MBTilesMetaData>`**
    - Parses an MBTiles file to extract its metadata, including geographic bounds and additional tags.
    - Parameters:
      - `filepath`: A string slice that holds the file path of the MBTiles file to be parsed.
    - Returns:
      - A `Result` containing `MBTilesMetaData` on success, encapsulating the geographic region covered by the MBTiles file and any additional metadata tags.
    - Behavior:
      - Opens a connection to the MBTiles SQLite database.
      - Executes a query to retrieve the bounds of the map from the metadata table.
      - Parses the bounds string to extract geographic coordinates (longitude and latitude) for the top-left and bottom-right corners.
      - Constructs and returns an `MBTilesMetaData` instance with the parsed geographic region and a default tag indicating the filetype as "MBTiles".

### File: `header.rs`

- **Functions**

  - `parse_header(buffer: &[u8]) -> Result<(ByteOrder, SeekFrom), TIFFErrorState>`
    - Analyzes the header of a TIFF file provided as a byte slice. It validates the byte order, magic numbers, and calculates the offset to the Image File Directory (IFD). Returns the byte order and IFD offset on success, or a `TIFFErrorState` on failure.

- **Structs and Enums**

  - **`ByteOrder`**
    - Enumerates the possible byte orders in a TIFF file, either `LittleEndian` or `BigEndian`.

  - **`TIFFErrorState`**
    - Enumerates possible errors encountered during TIFF parsing, such as:
      - `HeaderError(HeaderErrorState)`: Indicates an error related to the TIFF header, with a specific `HeaderErrorState`.


### File: `lib.rs`
- **Functions**

  - `parse_tiff(reader: &mut BufReader<File>, tfw_reader: Option<&mut BufReader<File>>) -> Result<GeoTiffMetaData, TIFFErrorState>`
    Parses a GeoTIFF file and an optional associated TFW (world file) to extract geographic metadata and region information. Returns `GeoTiffMetaData` on success or `TIFFErrorState` on failure, detailing issues such as unexpected format or insufficient geo-data.

  - `calculate_extent(top_left: (f64, f64), bottom_right: (f64, f64), resolution: (f64, f64)) -> GeoRegion`
    Calculates the geographic extent of an area given its top-left and bottom-right coordinates along with the resolution. Returns a `GeoRegion` representing the geographic area covered.

- **Structs and Enums**

  - **`GeoTiffMetaData`**
    Contains metadata extracted from a GeoTIFF file, including geographic information like coordinate reference system (CRS), extents, and pixel resolution.

  - **`GeoRegion`**
    Represents a geographic region, containing fields such as the coordinates for the top-left and bottom-right corners, and the resolution of the region.


### File: `entry.rs`
- **Enums**

  - **`EntryType`**
    - Represents the type of data stored in an IFD (Image File Directory) entry. Possible values include `BYTES`, `ASCII`, `SHORT`, `LONG`, `RATIONAL`, `UNDEFINED`, and `DOUBLE`.

  - **`EntryValue`**
    - Represents the actual data stored in an IFD entry, which can vary in type. It includes variants like `BYTES(Vec<u8>)`, `ASCII(Vec<String>)`, `SHORT(Vec<u16>)`, `LONG(Vec<u32>)`, `RATIONAL(Vec<(u32, u32)>)`, `UNDEFINED(Vec<u8>)`, and `DOUBLE(Vec<f64>)`.

- **Structs**

  - **`IFDEntry`**
    - Represents a single entry within the IFD of a TIFF file. It contains the tag (identifier), count of values, the type of data (`EntryType`), associated bytes (either direct value or offset), and optionally the resolved value (`EntryValue`).

- **Functions**

  - **`new(entry_buf: &[u8], byte_order: &ByteOrder) -> Result<IFDEntry, TIFFErrorState>`**
    - Constructs a new `IFDEntry` from a buffer slice, interpreting the data based on the specified byte order. It validates the buffer length, extracts and validates the tag and data type, and initializes the entry with these and the raw bytes. It returns either an `IFDEntry` or a `TIFFErrorState` on failure, covering scenarios like invalid buffer length or unexpected entry type.

  - **`resolve(&mut self, byte_order: &ByteOrder, reader: &mut BufReader<File>) -> Result<&EntryValue, TIFFErrorState>`**
    - Resolves the actual value for the IFD entry, converting the raw or offset bytes into the typed value specified by `EntryType`. It handles different data sizes and types, potentially reading from the file if the data is not directly stored in the entry. Returns either a reference to the `EntryValue` on success or a `TIFFErrorState` on failure, which can include errors like missing associated value or IO errors during file access.

### File: `geokeydirectory.rs`
- **Structs**

  - **`GeoKeyDirectoryHeader`**
    - Represents the header of a GeoKeyDirectory, containing key revision, minor revision, and key count.
    - **Fields:**
      - `key_revision`: The major version of the GeoKeyDirectory.
      - `minor_revision`: The minor version of the GeoKeyDirectory.
      - `count`: The number of GeoKeys in the directory.

  - **`GeoKey`**
    - Represents a single GeoKey, including its ID, location, count, and value.
    - **Fields:**
      - `id`: The identifier of the GeoKey.
      - `location`: Specifies where the GeoKey's data is stored.
      - `count`: The number of values or the length of the value array for the key.
      - `value`: The actual value of the GeoKey if it is directly embedded (optional).

  - **`GeoKeyDirectory`**
    - Encapsulates the entire GeoKeyDirectory, including its header and a collection of GeoKeys.
    - **Fields:**
      - `header`: The `GeoKeyDirectoryHeader` of the directory.
      - `keys`: A `HashMap` mapping GeoKey IDs to their corresponding `GeoKey` structs.

- **Functions**

  - **`GeoKeyDirectoryHeader::from_shorts(shorts: &[u16]) -> Result<GeoKeyDirectoryHeader, TIFFErrorState>`**
    - Creates a `GeoKeyDirectoryHeader` from a slice of `u16` values. Validates the expected format and returns an error if the format does not match expectations.

  - **`GeoKey::from_shorts(shorts: &[u16]) -> Result<GeoKey, TIFFErrorState>`**
    - Creates a `GeoKey` from a slice of `u16` values, handling the parsing and validation of the input to ensure it matches the expected GeoKey structure.

  - **`GeoKeyDirectory::from_shorts(shorts: &Vec<u16>) -> Result<GeoKeyDirectory, TIFFErrorState>`**
    - Constructs a `GeoKeyDirectory` from a vector of `u16` values, parsing the header and each GeoKey in turn, and organizing them into a coherent structure. It includes extensive error checking to ensure the input data is correctly formatted.

  - **`GeoKeyDirectory::get_projection(&self) -> Result<Proj, TIFFErrorState>`**
    - Attempts to determine the geospatial projection used by the TIFF file based on its GeoKeys. It looks for specific keys representing the CRS (Coordinate Reference System) code and uses the `proj4rs` library to create a `Proj` object that can be used for coordinate transformations. This method involves matching the CRS code against known definitions and handling various error conditions related to projection lookup and initialization.

### File: `util.rs`
- **Enums**
- **`ByteOrder`**
  - Represents the byte order of data (little-endian or big-endian).
  - **Variants:**
    - `LittleEndian`: Indicates that the least significant byte is stored first.
    - `BigEndian`: Indicates that the most significant byte is stored first.

- **Traits**

- **`FromBytes`**
  - A trait for types that can be constructed from a slice of bytes, taking into account the byte order.
  - **Associated Functions:**
    - `fn from_bytes(bytes: &[u8], byte_order: &ByteOrder) -> Self;`: Constructs an instance of the implementing type from a byte slice, using the specified byte order.

- **Implementations**

- **For `u16`**:
  - Parses a `u16` integer from a byte slice, respecting the specified byte order.
- **For `u32`**:
  - Parses a `u32` integer from a byte slice, respecting the specified byte order.
- **For `f64`**:
  - Parses an `f64` floating-point number from a byte slice, respecting the specified byte order.

  
### File: `TFW.rs`

- **Structs**
- **`TFWData`**
  - 
  - **Fields:** 

#### Functions

- **`parse_tfw(reader: &mut BufReader<File>) -> Result<TFWData, Box<dyn Error>>`**
  - Aims to parse TFW data from a `BufReader<File>` and return a `Result` containing either the parsed `TFWData` or an error.
  - **Parameters:**
    - `reader`: A mutable reference to a `BufReader<File>` from which the TFW data is read.
  - **Returns:** Currently, this function does not perform parsing as intended. Instead, it immediately returns an error indicating that the feature is not fully implemented.


### File: `route.rs`
- This file is related to the web API. We have created an additional documentation for the web API.[Link to web-api File](./web-api_documentation.md)



## Potential Error Outputs
- **FileParsingAndFormatErrorState**
  - Errors specific to parsing various file formats such as GeoJSON, KML, Shapefiles, and TIFF files, including unrecognized formats, corruption, or missing metadata.
- **DataConversionErrorState**
  - Errors that occur during the transformation of data between different formats or coordinate systems, possibly due to unsupported conversion paths or inaccuracies in the source data.
- **IOAndFileSystemErrorState**
  - Errors related to file input/output operations, including issues with permissions, nonexistent files, or disk space limitations.
- **NetworkAndServiceCommunicationErrorState**
  - Failures due to network issues, API rate limits, or unexpected responses when the backend interacts with external services or APIs.
- **GeospatialDataIntegrityAndValidationErrorState**
  - Errors arising from invalid or incomplete geospatial data, such as missing coordinates, invalid geometries, or unsupported projection systems.
- **HeaderAndMetadataErrorState**
  - Errors caused by incorrect, missing, or corrupted header information in TIFF and other image formats, including issues with the GeoKey Directory.



## Overall Data Flow

1. **Configuration Loading** (`config.rs`)
   - Loads application configuration at startup, including database connections, service ports, etc.

2. **Spatial Data Processing**
   - **Data Parsing** (`geokeydirectory.rs`, `header.rs`, `lib.rs`, etc.)
     - Parses various geographical data formats, such as TIFF, GeoJSON, KML, etc.
   - **Spatial Index Construction** (`index.rs`)
     - Indexes geographical data using spatial indexing structures like R-trees to support efficient spatial queries.

3. **Web Service Startup** (`main.rs`)
   - Configures routes (`routes.rs`) and launches the asynchronous web service.

4. **Query Processing**
   - **Receiving Query Requests** (`routes.rs`)
     - Receives query requests through HTTP endpoints.
   - **Background Query Execution** (`worker.rs`)
     - Asynchronously executes query tasks, including spatial queries and data retrieval.
   - **Returning Query Results** (`io.rs`)
     - Serializes query results into formats like JSON and returns them to clients through web interfaces.

5. **Logging and Error Handling**
   - Logs key events and potential errors throughout the application, providing error feedback to clients.
## Link to other documentation
[Link to frontend File](./frontend_documentation.md)
[Link to web-api File](./web-api_documentation.md)