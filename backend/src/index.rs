use std::io::{BufReader};
use std::fs::File;
use std::path::PathBuf;
use tracing::{event, Level, span};
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

pub fn parse(path: PathBuf) -> Option<Region> {
    let span = span!(Level::INFO, "Parsing");
    let _guard = span.enter();
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
                                        event!(Level::ERROR, "KML File Parsing Failure");
                                        event!(Level::ERROR, "Due to error: {:?}", e);
                                        event!(Level::ERROR, "This is an unhandled format error, please contact developer.");
                                        event!(Level::ERROR, "Panic!");
                                        panic!();
                                    }
                                    KMLErrorState::NotEnoughGeoData => {
                                        event!(Level::ERROR, "File: {:?}", path);
                                        event!(Level::ERROR, "Does not contain any coordinate data.");
                                        event!(Level::ERROR, "Ignoring."); // Ignore as non-fatal.
                                        None
                                    }
                                }
                            } {
                                event!(Level::DEBUG, "Parsed {path:?}");
                                event!(Level::DEBUG, "found region: {region:?}");
                                return Some(region.into());
                            }
                        },
                        "tif" => {
                            let mut reader = BufReader::new(file);
                            if let Some(region) = match parse_tiff(&mut reader) {
                                Ok(r) => Some(r),
                                Err(e) => match e {
                                    TIFFErrorState::HeaderError(header_err) => match header_err {
                                        HeaderErrorState::UnexpectedByteOrder(b) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Has unexpected byte_order value: {:?}", b);
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        HeaderErrorState::UnexpectedMagicNumber(b) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Has unexpected magic number value: {:?}", b);
                                            event!(Level::ERROR, "This file may not be a real TIFF");
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        HeaderErrorState::InvalidLength(length) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Was passed invalid header buffer, of length {} (should be 8)", length);
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                    },
                                    TIFFErrorState::IFDEntryError(ifd_error_state) => match ifd_error_state {
                                        IFDEntryErrorState::UnexpectedEntryType(t) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Contains unimplemented tag type: {}", t);
                                            event!(Level::ERROR, "This tag type needs to be implemented!");
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        IFDEntryErrorState::MissingAssociatedValue(t) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Has tag: {}, but the value is inaccessible!", t);
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        IFDEntryErrorState::InvalidLength(length) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Was passed invalid tag buffer, of length {} (should be 12)", length);
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                    },
                                    TIFFErrorState::UnexpectedFormat(s) => {
                                        event!(Level::ERROR, "Unexpected TIFF Format Error!");
                                        event!(Level::ERROR, "Reason: {}", s);
                                        panic!();
                                    },
                                    TIFFErrorState::NotEnoughGeoData => {
                                        event!(Level::WARN, "Encountered tiff file without GeoData, ignoring as will be caught by sidecar if present.");
                                        return None;
                                    }
                                    TIFFErrorState::GeoKeyDirectoryError(geo_key_directory_error) => match geo_key_directory_error {
                                        GeoKeyDirectoryErrorState::ProjectionError(p) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Failed to create projection for file, see: {:?}", p);
                                            event!(Level::ERROR, "Likely that a new coordinate system needs to be implemented.");
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        GeoKeyDirectoryErrorState::UnexpectedFormat(s) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Failed to read GeoKeyDirectory, Reason: {}", s);
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                    },

                                    TIFFErrorState::ProjectionError(p) => {
                                        event!(Level::ERROR, "File: {:?}", path);
                                        event!(Level::ERROR, "Failed to perform final projection, Error: {}", p);
                                        event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                        panic!();
                                    }
                                }
                            } {
                                event!(Level::DEBUG, "Parsed {path:?}");
                                event!(Level::DEBUG, "found region: {region:?}");
                                return Some(region.into());
                            }
                        },
                        "dt2" | "dt1" => {
                            let mut reader = BufReader::new(file);
                            match parse_dt2(&mut reader) {
                                Ok(v) => {
                                    event!(Level::DEBUG, "Parsed {path:?}");
                                    event!(Level::DEBUG, "found region: {v:?}");
                                    return Some(v.into());
                                },
                                Err(e) => match e {
                                    DT2ErrorState::UnexpectedFormat(f) => {
                                        event!(Level::ERROR, "Unexpected DT2 Format Error!");
                                        event!(Level::ERROR, "Reason: {f}");
                                        panic!();
                                    }
                                    DT2ErrorState::UHLError(e) => match e {
                                        UHLErrorState::InvalidLength(l) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Failed to read User Header Label, was passed wrong length byte buffer: {l}");
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        UHLErrorState::InvalidSentinel(s) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Failed to read User Header Label, Sentinel value was invalid, is this really a .dt2? {s:?}");
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        UHLErrorState::InvalidDDMMSSH(ddmmssh) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Failed to read User Header Label, Encountered unparsable coordinate: {ddmmssh:?}");
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                    }
                                    DT2ErrorState::DSIError(e) => match e {
                                        DSIErrorState::InvalidLength(l) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Failed to read DSI, was passed wrong length byte buffer: {l}");
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        DSIErrorState::InvalidSentinel(s) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Failed to read DSI, Sentinel value was invalid, is this really a .dt2? {s:?}");
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        }
                                        DSIErrorState::InvalidDDMMSSH(ddmmssh) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Failed to read DSI, Encountered unparsable coordinate: {ddmmssh:?}");
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                            panic!();
                                        },
                                        DSIErrorState::InvalidDDDMMSSH(dddmmssh) => {
                                            event!(Level::ERROR, "File: {:?}", path);
                                            event!(Level::ERROR, "Failed to read DSI, Encountered unparsable coordinate: {dddmmssh:?}");
                                            event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
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
                                    event!(Level::DEBUG, "Parsed {path:?}");
                                    event!(Level::DEBUG, "found region: {region:?}");
                                    return Some(region.into());
                                }
                                Err(e) => match e {
                                    GeoJSONErrorState::InvalidJSON(e) => {
                                        event!(Level::ERROR, "File: {:?}", path);
                                        event!(Level::ERROR, "Failed to parse, Invalid JSON, Error: {e:?}");
                                        event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                        panic!();
                                    }
                                    GeoJSONErrorState::UnparsableCoordinate(c) => {
                                        event!(Level::ERROR, "File: {:?}", path);
                                        event!(Level::ERROR, "Failed to parse number, see value: {c:?}");
                                        event!(Level::ERROR, "Panic! Please contact developer, this is a breaking issue."); // Panic as parsing error
                                        panic!();
                                    }
                                }
                            }
                        }
                        _ => {
                            event!(Level::WARN, "File: {:?}", path);
                            event!(Level::WARN, "Unhandled format: {ext}");
                            event!(Level::WARN, "Ignored!"); // Ignore for debug purposes. Will be annoying having to remove unimplemented files.
                            return None;
                        }
                    },
                    Err(e) => {
                        event!(Level::ERROR, "File: {:?}", path);
                        event!(Level::ERROR, "Could not be opened: {:?}", e);
                        event!(Level::ERROR, "Throwing Panic!");
                        panic!();
                    }
                },
                None => {
                    event!(Level::ERROR, "File: {:?}", path);
                    event!(Level::ERROR, "Has extension: {:?}", ext);
                    event!(Level::ERROR, "But cannnot get the extension from wrapping OSStr.");
                    event!(Level::ERROR, "This should not happen, thus panic! Please contact developer.");
                    panic!(); // Panic as should be unreachable.
                }
            }
        },
        None => {
            event!(Level::WARN, "File: {:?}", path);
            event!(Level::WARN, "Has no EXTENSION! Thus ignored!"); // Files without extensions are ignored, but a log is made.
            return None;
        }
    }
    panic!();
}