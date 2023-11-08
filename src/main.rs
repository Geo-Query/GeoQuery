use std::arch::x86_64::_mm256_abs_epi16;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use proj::ProjError::Path;

mod kml;
mod spatial;
mod index;
use geotiff::{parse_tiff, TIFFErrorState};
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
                                        TIFFErrorState::HeaderError(_) => None, // TODO: Handle ErrorStates neatly.
                                        TIFFErrorState::IFDEntryError(_) => None,
                                        TIFFErrorState::UnexpectedFormat(_) => None,
                                        TIFFErrorState::NotEnoughGeoData => None
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
