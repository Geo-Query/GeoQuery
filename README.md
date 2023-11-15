Language: Rust
---------------------------------------------------------------------------------------------------
Progression:
- RStar Implementation [x]
- Region Parsing: [ ]
  - KML [x]
  - GEOTIFF w/o sidecar [x]
  - DTED [x]
  - GEOTIFF w/ sidecar [ ]
  - GEOJSON [ ]
  - GPKG [ ]
  - MBTILES [ ]
  - OSM/PBF [ ]
  - SHAPEFILE [ ]
- File Searching [ ]
- Index save/read [ ]
- I/O for Flask server

Requirements:
- Must run in acceptable time, over multiple TB of raw data.
- Must not modify the data directly (Can have a local working dir)
- Must read files of various types (unknown as of yet; including TIFF), and find the lat/long they contain.
- Will be interfacing with a local python HTTP API, should have some form of IPC.

---------------------------------------------------------------------------------------------------
Solutions:
- In order to run in acceptable time, we must build an index of the raw-unordered data.
- This index will store 2 sets of lat/long coordinates for each file, and must be queried with 2 sets of coordinates, to find all files that intersect.
- This index will follow the R* Tree data structure, will store opposite corners of each file as seperate points, if any point is contained within the queried range, this is an overlap.
- How will we manipulate this data structure in Rust? Use the [rstar](https://docs.rs/rstar/latest/rstar/) library.
- This structure can implement serde, to allow for saving of index to disk.

- Interfacing with python server... Could be called using subprocess directly as a cmd line, could be a daemon and use IPC to communicate with py-server, or could host a rudimentary unsecured/or maybe even secured, local http server.

- Reading files will be handled by pluggable readers, all implementing a common trait defining a std input of file descriptors, and a std output of 2 lat/long coords representing the boundaries of the file.

## Implementation Notes:
--------------------------
Kinds of failure:
    UnexpectedFormat - Should be displayed clearly to the user.
    MissingRequiredData - Should be ignored, and a small warning. In the case of GeoTiff, should look for a sidecar?