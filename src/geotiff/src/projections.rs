use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use proj::Proj;
use crate::geokeydirectory::GeoKeyDirectory;
use crate::tag::IFDEntry;
use crate::util::ByteOrder;

#[derive(Debug)]
pub enum ProjectionErrorState {
    UnexpectedFormat(String),
    NotEnoughGeoData
}

pub enum ModelType {
    Projected,
    Geographic,
    Geocentric
}
pub fn get_proj_from_key_directory(geo_key_directory: GeoKeyDirectory, tag: &mut HashMap<u16, IFDEntry>, byte_order: &ByteOrder, reader: &mut BufReader<File>, target_espg: &str) -> Result<proj::Proj, ProjectionErrorState> {
    // Get ModelTypeGeoKey...
    let type_geo_key = match geo_key_directory.keys.get(&1024u16) {
        Some(k) => if k.location == 0 {
            match k.value {
                Some(v) => v,
                None => {
                    return Err(ProjectionErrorState::UnexpectedFormat(String::from("Location was 0, expected value for geokey!")));
                }
            }
        } else {
            return Err(ProjectionErrorState::UnexpectedFormat(String::from("Expected geo-key to only be a single short!")));
        },
        None => {
            return Err(ProjectionErrorState::NotEnoughGeoData);
        }
    };

    let model_type = match type_geo_key {
        1 => ModelType::Projected,
        2 => ModelType::Geographic,
        3 => ModelType::Geocentric,
        _ => {
            return Err(ProjectionErrorState::UnexpectedFormat(String::from(format!("Expected model type to be 1,2,3, found unhandled: {}", type_geo_key))));
        }
    };

    let epsg_code = match model_type {
        ModelType::Projected => match geo_key_directory.keys.get(&2048) {
            Some(v) => if v.location == 0 {
                match v.value {
                    Some(v) => v,
                    None => {
                        return Err(ProjectionErrorState::UnexpectedFormat(String::from("Location was 0 for projected ESPG code! Expected value!")))
                    }
                }
            } else {
                return Err(ProjectionErrorState::UnexpectedFormat(String::from("Expected Projected ESPG code to be single short!")));
            },
            None => {
                return Err(ProjectionErrorState::NotEnoughGeoData);
            }
        }
        ModelType::Geographic => match geo_key_directory.keys.get(&3072) {
            Some(v) => if v.location == 0 {
                match v.value {
                    Some(v) => v,
                    None => {
                        return Err(ProjectionErrorState::UnexpectedFormat(String::from("Location was 0 for Geographic ESPG code! Expected value!")))
                    }
                }
            } else {
                return Err(ProjectionErrorState::UnexpectedFormat(String::from("Expected Geographic ESPG code to be single short!")));
            },
            None => {
                return Err(ProjectionErrorState::NotEnoughGeoData);
            }
        },
        ModelType::Geocentric => {
            eprintln!("Unhandled GEOCENTRIC data! Throwing err!");
            return Err(ProjectionErrorState::UnexpectedFormat(String::from("Unhandled GEOCENTRIC data type.")))
        }
    };

    return match Proj::new_known_crs(
        format!("EPSG:{}", epsg_code).as_str(),
        target_espg,
        None
    ) {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("Failed to create Projection! Error: {:?}", e);
            Err(ProjectionErrorState::UnexpectedFormat(String::from("Failed to create proj!")))
        }
    }
}