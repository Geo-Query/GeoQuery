use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

mod kml;
mod spatial;
mod index;
use geotiff::{parse_tiff, TIFFErrorState, HeaderErrorState, IFDEntryErrorState};
use crate::kml::{KMLErrorState, parse_kml};
use crate::spatial::Region;


fn main() {
    // Set of test paths, will be gotten by a recursive directory search eventually.
    let paths = vec![
        "/home/ben/uni/psd/teamproj/sh35-data-parsing/planetsat.tif",
        "/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/Sat Imagery/PlanetSAT_10_0s3_N54W004.tif",
        "/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/OrdnanceSurveyOpenData/250k/SU.tif",
        "/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/OrdnanceSurveyOpenData/OpenMap Local/SU01NE.tif",
        "/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/OrdnanceSurveyOpenData/miniscale/MiniScale_(standard)_R23.tif",
        "/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/OrdnanceSurveyOpenData/VectorMapDistrict/SU01.tif",
        "/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/Aerial Imagery/ST9143.tif"
    ];

    // Convert raw strings to path buffers for opening.
    let paths: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();

    let mut regions: Vec<Box<dyn Region>> = Vec::with_capacity(paths.len());
    // Iterate over supplied files.
    for path in paths {
        // Get the file extension.
        match path.extension() {
            Some(ext) => { // Unwrap code to get OSStr -> str for comparison.
                match ext.to_str() {
                    Some(ext) => match File::open(&path) {
                        Ok(file) => match ext { // Once have extension, match against it.
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
                                } { regions.push(region); }
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
                                            None
                                        }
                                    }
                                } {regions.push(region)}
                            },
                            _ => {
                                eprintln!("File: {:?}", path);
                                eprintln!("Unhandled format: {ext}");
                                eprintln!("Ignored!"); // Ignore for debug purposes. Will be annoying having to remove unimplemented files.
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
            }
        }
    }
}
