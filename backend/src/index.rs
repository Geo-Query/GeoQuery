use std::io::{BufReader};
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use crate::spatial::{Region, Coordinate};
use rstar::{RTreeObject, AABB};
use serde::{Deserialize, Serialize};
use geotiff::{GeoKeyDirectoryErrorState, HeaderErrorState, IFDEntryErrorState, parse_tiff, TIFFErrorState};
use crate::FileMeta;
use crate::parsing::dt2::{DSIErrorState, DT2ErrorState, parse_dt2, UHLErrorState};
use crate::parsing::geojson::{GeoJSONErrorState, parse_geojson};
use crate::parsing::kml::{KMLErrorState, parse_kml};

// Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub region: Region,
    pub file: Arc<FileMeta>
}

// Implement RTreeObject on Node.
impl RTreeObject for Node {
    type Envelope = AABB<Coordinate>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.region.bottom_left(), self.region.top_right())
    }
}

pub fn parse(path: PathBuf) -> Region {
    match path.extension() {
        Some(ext) => { // Unwrap code to get OSStr -> str for comparison.
            match ext.to_str() {
                Some(ext) => match File::open(&path) {
                    Ok(file) => match ext.to_ascii_lowercase().as_str() { // Once have extension, match against it.
                        "kml" => {
                            let mut reader = BufReader::new(file);
                            if let Some(region) = match parse_kml(&mut reader) {
                                Ok(r) => Some(r),
                                Err(e) => match e {
                                    KMLErrorState::UnexpectedFormat(e) => {
                                        eprintln!("KML File Parsing Failure");
                                        eprintln!("Due to error: {:?}", e);
                                        eprintln!("This is an unhandled format error, please contact developer.");
                                        eprintln!("Panic!");
                                        panic!();
                                    }
                                    KMLErrorState::NotEnoughGeoData => {
                                        eprintln!("File: {:?}", path);
                                        eprintln!("Does not contain any coordinate data.");
                                        eprintln!("Ignoring."); // Ignore as non-fatal.
                                        None
                                    }
                                }
                            } {
                                println!("{region:?}");
                                return region.into();
                            }
                        },
                        "tif" => {
                            let mut reader = BufReader::new(file);
                            if let Some(region) = match parse_tiff(&mut reader) {
                                Ok(r) => Some(r),
                                Err(e) => match e {
                                    TIFFErrorState::HeaderError(header_err) => match header_err {
                                        HeaderErrorState::UnexpectedByteOrder(b) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Has unexpected byte_order value: {:?}", b);
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        HeaderErrorState::UnexpectedMagicNumber(b) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Has unexpected magic number value: {:?}", b);
                                            eprintln!("This file may not be a real TIFF");
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        HeaderErrorState::InvalidLength(length) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Was passed invalid header buffer, of length {} (should be 8)", length);
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                    },
                                    TIFFErrorState::IFDEntryError(ifd_error_state) => match ifd_error_state {
                                        IFDEntryErrorState::UnexpectedEntryType(t) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Contains unimplemented tag type: {}", t);
                                            eprintln!("This tag type needs to be implemented!");
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        IFDEntryErrorState::MissingAssociatedValue(t) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Has tag: {}, but the value is inaccessible!", t);
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        IFDEntryErrorState::InvalidLength(length) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Was passed invalid tag buffer, of length {} (should be 12)", length);
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                    },
                                    TIFFErrorState::UnexpectedFormat(s) => {
                                        eprintln!("Unexpected TIFF Format Error!");
                                        eprintln!("Reason: {}", s);
                                        panic!();
                                    },
                                    TIFFErrorState::NotEnoughGeoData => {
                                        println!("Encountered tiff file without GeoData, ignoring as will be caught by sidecar if present.");
                                        panic!();
                                    }
                                    TIFFErrorState::GeoKeyDirectoryError(geo_key_directory_error) => match geo_key_directory_error {
                                        GeoKeyDirectoryErrorState::ProjectionError(p) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Failed to create projection for file, see: {:?}", p);
                                            eprintln!("Likely that a new coordinate system needs to be implemented.");
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        GeoKeyDirectoryErrorState::UnexpectedFormat(s) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Failed to read GeoKeyDirectory, Reason: {}", s);
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                    },

                                    TIFFErrorState::ProjectionError(p) => {
                                        eprintln!("File: {:?}", path);
                                        eprintln!("Failed to perform final projection, Error: {}", p);
                                        eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                        panic!();
                                    }
                                }
                            } {
                                println!("{region:?}");
                                return region.into();
                            }
                        },
                        "dt2" | "dt1" => {
                            let mut reader = BufReader::new(file);
                            match parse_dt2(&mut reader) {
                                Ok(v) => {
                                    println!("{v:?}");
                                    return v.into();
                                },
                                Err(e) => match e {
                                    DT2ErrorState::UnexpectedFormat(f) => {
                                        eprintln!("Unexpected DT2 Format Error!");
                                        eprintln!("Reason: {f}");
                                        panic!();
                                    }
                                    DT2ErrorState::UHLError(e) => match e {
                                        UHLErrorState::InvalidLength(l) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Failed to read User Header Label, was passed wrong length byte buffer: {l}");
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        UHLErrorState::InvalidSentinel(s) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Failed to read User Header Label, Sentinel value was invalid, is this really a .dt2? {s:?}");
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        UHLErrorState::InvalidDDMMSSH(ddmmssh) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Failed to read User Header Label, Encountered unparsable coordinate: {ddmmssh:?}");
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                    }
                                    DT2ErrorState::DSIError(e) => match e {
                                        DSIErrorState::InvalidLength(l) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Failed to read DSI, was passed wrong length byte buffer: {l}");
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        DSIErrorState::InvalidSentinel(s) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Failed to read DSI, Sentinel value was invalid, is this really a .dt2? {s:?}");
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        DSIErrorState::InvalidDDMMSSH(ddmmssh) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Failed to read DSI, Encountered unparsable coordinate: {ddmmssh:?}");
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        },
                                        DSIErrorState::InvalidDDDMMSSH(dddmmssh) => {
                                            eprintln!("File: {:?}", path);
                                            eprintln!("Failed to read DSI, Encountered unparsable coordinate: {dddmmssh:?}");
                                            eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                    }
                                }
                            }
                        },
                        "geojson" => {
                            let mut reader = BufReader::new(file);
                            match parse_geojson(&mut reader) {
                                Ok(region) => {
                                    println!("{region:?}");
                                    return region.into();
                                }
                                Err(e) => match e {
                                    GeoJSONErrorState::InvalidJSON(e) => {
                                        eprintln!("File: {:?}", path);
                                        eprintln!("Failed to parse, Invalid JSON, Error: {e:?}");
                                        eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                        panic!();
                                    }
                                    GeoJSONErrorState::UnparsableCoordinate(c) => {
                                        eprintln!("File: {:?}", path);
                                        eprintln!("Failed to parse number, see value: {c:?}");
                                        eprintln!("Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                        panic!();
                                    }
                                }
                            }
                        }
                        _ => {
                            eprintln!("File: {:?}", path);
                            eprintln!("Unhandled format: {ext}");
                            eprintln!("Ignored!"); // Ignore for debug purposes. Will be annoying having to remove unimplemented files.
                            panic!();
                        }
                    },
                    Err(e) => {
                        eprintln!("File: {:?}", path);
                        eprintln!("Could not be opened: {:?}", e);
                        eprintln!("Throwing Panic!");
                        panic!();
                    }
                },
                None => {
                    eprintln!("File: {:?}", path);
                    eprintln!("Has extension: {:?}", ext);
                    eprintln!("But cannnot get the extension from wrapping OSStr.");
                    eprintln!("This should not happen, thus panic! Please contact developer.");
                    panic!(); // Panic as should be unreachable.
                }
            }
        },
        None => {
            eprintln!("File: {:?}", path);
            eprintln!("Has no EXTENSION! Thus ignored!"); // Files without extensions are ignored, but a log is made.
            panic!();
        }
    }
    panic!();
}