use std::path::PathBuf;
use crate::parsing::{DataType, ParsingErrorState};

mod kml;
mod util;
mod spatial;
mod index;
mod parsing;
use geotiff::parse_tiff;


fn main() {
    let kml_file = parsing::Descriptor::new(PathBuf::from("/home/ben/uni/psd/teamproj/sh35-data-parsing/luciad_and_leuven.kml"));
    // TIFF Paths:
    // /home/ben/uni/psd/teamproj/sh35-data-parsing/planetsat.tif
    // /home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/Sat Imagery/PlanetSAT_10_0s3_N54W004.tif
    // /home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/OrdnanceSurveyOpenData/250k/SU.tif // Does not have this behaviour // HAS CUSTOM GEO SYSTEM.
    // /home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/OrdnanceSurveyOpenData/OpenMap Local/SU01NE.tif // Has off le/be behaviour
    // /home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/OrdnanceSurveyOpenData/miniscale/MiniScale_(standard)_R23.tif // Does not have odd behaviour
    // /home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/OrdnanceSurveyOpenData/VectorMapDistrict/SU01.tif // Has odd littleendian/bigendian behaviour.
    // /home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/Aerial Imagery/ST9143.tif // HAS SIDECAR - NOT WORKING
    let tiff_file = parsing::Descriptor {
        //path: PathBuf::from("/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/OrdnanceSurveyOpenData/250k/SU.tif"),
        // path: PathBuf::from("/home/ben/uni/psd/teamproj/sh35-data-parsing/planetsat.tif"),
        path: PathBuf::from("/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/OrdnanceSurveyOpenData/miniscale/MiniScale_(standard)_R23.tif"),
        data_type: Some(DataType::TIFF)
    };

    parsing::parse_from_descriptor(tiff_file);
    // match parsing::parse_from_descriptor(tiff_file) {
    //     Ok(region) => {
    //         println!("Result: {:?}", region);
    //     },
    //     Err(error_kind) => match error_kind {
    //         ParsingErrorState::UnknownExtension(descriptor) => {
    //             eprintln!("Encountered unknown extension {:?}", descriptor.data_type);
    //             panic!();
    //         },
    //         ParsingErrorState::NoExtension(descriptor) => {
    //             eprintln!("No file extension for file: {:?}", descriptor);
    //             panic!();
    //         },
    //         ParsingErrorState::FileError(descriptor, error_kind) => {
    //             eprintln!("Failed to open: {:?}", descriptor);
    //             eprintln!("ErrorKind: {:?}", error_kind);
    //             panic!();
    //         }
    //
    //         ParsingErrorState::InvalidOrUnhandledFormat(descriptor) => {
    //             eprintln!("Unexpected format for file: {:?}", descriptor);
    //             panic!();
    //         }
    //         ParsingErrorState::NoGeoData(descriptor) => {
    //             eprintln!("File does not contain required GeoData to get coordinate boundaries!");
    //             eprintln!("File: {:?}", descriptor);
    //             panic!();
    //         }
    //     }
    // };
}
