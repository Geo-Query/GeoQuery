# SH35 Main Repository
------------------------

Contains two subprojects...
frontend & backend.

# Installation & Usage
Please consult the [User Manual](manual.pdf)

Team Members:
------------------
Ruari O'Hara, 2666047o@student.gla.ac.uk, 07939544510

Benjamin Parsons-Willis, 2664758P@student.gla.ac.uk, 07529606324

Anwar Abdullah, 2666539a@student.gla.ac.uk, 07709939510

Finlay Gray, 2677288g@student.gla.ac.uk, 07938625301

Calum Robertson, 2665639r@student.gla.ac.uk, 07368119260

Yangruizhe Jiang, 2665049j@student.gla.ac.uk, 07421723178

## Frontend
-----------------
Frontend in react, allows for queries to be made to backend.

Requirements:
  - Select Bounding Box
  - Convert Box to coordinates in Longitude & Latitude decimal format
  - Allow direct inputs of various longitude/latitude formats, skipping the box step if user should so choose.
  - Save this query to history.
  - Send this query onwards to the backend. (Via REST API)
  - Poll/Check for results in progress, and display them to the user.
  - Upon full results, should be able to export to some predefined structure.

## Backend
-----------------
Backend in Rust, allows for search/query of files based on geospatial criterion.

Requirements:
  - Should be able to take a directory and recursively explore it, indexing geospatial files based on 
    their overall longitude/latitude decimal extent.
    - Requires parsing of various formats, see <----------->
    - Use of R* Tree enables quick indexing.
    - Use of Serde will enable quick saving/loading of index.

  - Receive coordinates in standard longitude/latitude decimal format
  - From these coordinates, should be able to find all files which overlap the given area.
  - Return these to the frontend in a standardized format, with enough information to fetch/export these files.


Progression:
- RStar Implementation [x]
- Region Parsing: [ X]
  - KML [x]
  - GEOTIFF w/o sidecar [x]
  - DTED [x]
  - GEOJSON [x]
  - GPKG [X] 
  - MBTILES [X]
  - SHAPEFILE [X ]
- File Searching [X]
- Index save/read [X ]

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

- Interfacing with python server will be done over ZMQ.

- Reading files will be handled by pluggable readers, all implementing a common trait defining a std input of file descriptors, and a std output of 2 lat/long coords representing the boundaries of the file.

## Implementation Notes:
--------------------------
Kinds of failure:
  - UnexpectedFormat - Should be displayed clearly to the user.
  - MissingRequiredData - Should be ignored, and a small warning. ( In the case of GeoTiff, should look for a sidecar? )
