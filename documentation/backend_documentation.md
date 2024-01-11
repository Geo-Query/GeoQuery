# back-end Code Documentation

## APIs

### File: `index.rs`
- **`parse(path: PathBuf) -> Region`**
  - Parses a file at the given path and returns a `Region` object based on the geospatial data format specified in the file.

### File: `spatial.rs`
- **`bottom_left(&self) -> Coordinate`**
  - Returns the bottom-left coordinate of a spatial region.
- **`bottom_right(&self) -> Coordinate`**
  - Returns the bottom-right coordinate of a spatial region.
- **`top_left(&self) -> Coordinate`**
  - Returns the top-left coordinate of a spatial region.
- **`top_right(&self) -> Coordinate`**
  - Returns the top-right coordinate of a spatial region.

### File: `geojson.rs`
- **`get_boundaries(coordinates: Vec<[f64; 2]>) -> (Coordinate, Coordinate)`**
  - Calculates and returns the boundary coordinates for GeoJSON data.
- **`parse_geojson(reader: &mut BufReader<File>) -> Result<GeoJSONRegion, GeoJSONErrorState>`**
  - Parses GeoJSON files and returns a `GeoJSONRegion` object.

### File: `kml.rs`
- **`get_boundaries(coordinates: Vec<Coordinate>) -> (Coordinate, Coordinate)`**
  - Calculates and returns boundary coordinates from a series of KML points.
- **`parse_kml(reader: &mut BufReader<File>) -> Result<KMLRegion, KMLErrorState>`**
  - Parses KML files and returns a `KMLRegion` object.

### File: `dt2.rs`
- **`from_bytes(buffer: &[u8]) -> Result<UserHeaderLabel, DT2ErrorState>`**
  - Parses DT2 user header labels from byte data.
- **`from_bytes(buffer: &[u8]) -> Result<DataSetIdentification, DT2ErrorState>`**
  - Parses DT2 dataset identification information from byte data.
- **`parse_dt2(reader: &mut BufReader<File>) -> Result<DT2Region, DT2ErrorState>`**
  - Parses DT2 files and returns a `DT2Region` object.

### File: `header.rs`
- **`parse_header(buffer: &[u8]) -> Result<(ByteOrder, SeekFrom), TIFFErrorState>`**
  - Parses the header of a TIFF file, returning the byte order and a position in the file.

### File: `lib.rs`
- **`parse_tiff(reader: &mut BufReader<File>) -> Result<GeoTiffRegion, TIFFErrorState>`**
  - Parses a TIFF file and returns a `GeoTiffRegion` object.

### File: `entry.rs`
- **`new(entry_buf: &[u8], byte_order: &ByteOrder) -> Result<IFDEntry, TIFFErrorState>`**
  - Creates a new `IFDEntry` object from a byte buffer and a byte order.
- **`resolve(&mut self, byte_order: &ByteOrder, reader: &mut BufReader<File>) -> Result<&EntryValue, TIFFErrorState>`**
  - Resolves the value of an IFD entry.

### File: `geokeydirectory.rs`
- **`from_shorts(shorts: &[u16]) -> Result<GeoKeyDirectoryHeader, TIFFErrorState>`**
  - Converts an array of shorts to a `GeoKeyDirectoryHeader` structure for GeoTIFF files.
- **`from_shorts(shorts: &[u16]) -> Result<GeoKey, TIFFErrorState>`**
  - Converts an array of shorts to a `GeoKey` object for GeoTIFF files.
- **`from_shorts(shorts: &Vec<u16>) -> Result<GeoKeyDirectory, TIFFErrorState>`**
  - Converts a vector of shorts to a `GeoKeyDirectory` structure for GeoTIFF files.
- **`get_projection(&self, target_epsg: &str) -> Result<Proj, TIFFErrorState>`**
  - Retrieves projection information for GeoTIFF files based on EPSG code.



## Potential Error Outputs
- **KMLErrorState**
  - Errors specific to KML file parsing, such as unexpected format or insufficient geographic data.
- **GeoJSONErrorState**
  - Errors related to GeoJSON file parsing, including invalid JSON structure or unparsable coordinates.
- **DT2ErrorState**
  - Errors encountered while parsing DT2 files, including unexpected formats or user header label issues.
- **TIFFErrorState**
  - TIFF-specific errors, such as header parsing issues, IFD entry errors, and GeoKey directory errors.
- **HeaderErrorState, IFDEntryErrorState, GeoKeyDirectoryErrorState**
  - Errors related to parsing different aspects of TIFF files, including headers, IFD entries, and GeoKey directories.
- **ProjCreateError**
  - Errors occurring during the creation of projection objects, typically related to unsupported coordinate systems or malformed projection data.



## Overall Data Flow
1. **File Parsing**
   - Files are parsed based on their formats (KML, GeoJSON, DT2, TIFF) using respective modules (`kml.rs`, `geojson.rs`, `dt2.rs`, `lib.rs`).
2. **Data Representation and Handling**
   - Parsed data is converted into structured formats such as `Region`, `Coordinate`, and specific region objects for each file type.
3. **Spatial Processing**
   - The `spatial.rs` module handles the representation of geographic regions and coordinates, providing foundational spatial data structures.
4. **Error Handling**
   - Each parsing module incorporates specific error handling mechanisms to address format-specific parsing issues and data integrity problems.
5. **Indexing and Spatial Queries (if applicable)**
   - The `index.rs` module, if present, provides functionality for spatial indexing, supporting efficient querying and data retrieval.
6. **Integration and Utilization**
   - The `lib.rs` and `mod.rs` modules integrate different parsing and processing modules, orchestrating the overall data flow within the application.
7. **Utility Functions and Detailed Data Handling**
   - Modules like `util.rs` and `entry.rs` provide utility functions and detailed handling of specific data types, particularly for complex formats like TIFF.

## Link to other documentation
[Link to frontend File](./frontend_documentation.md)
[Link to web-api File](./web-api_documentation.md)