use std::collections::HashMap;
use std::fmt::format;
use crate::GeoKeyDirectoryErrorState::{ProjectionError, UnexpectedFormat};
use proj4rs::Proj;
use crate::TIFFErrorState;

#[derive(Debug)]
pub enum GeoKeyDirectoryErrorState {
    ProjectionError(String),
    UnexpectedFormat(String)
}

#[derive(Debug)]
pub struct GeoKeyDirectoryHeader {
    pub key_revision: u16,
    pub minor_revision: u16,
    pub count: u16
}


impl GeoKeyDirectoryHeader {
    pub fn from_shorts(shorts: &[u16]) -> Result<GeoKeyDirectoryHeader, TIFFErrorState> {
        if shorts.len() != 4 {
            return Err(TIFFErrorState::GeoKeyDirectoryError(UnexpectedFormat(String::from("Unexpected length header!"))));
        }
        if shorts[0] != 1 {
            return Err(TIFFErrorState::GeoKeyDirectoryError(UnexpectedFormat(String::from(format!("Unexpected Key Directory Version! {:?}", shorts[0])))));
        }

        return Ok(GeoKeyDirectoryHeader {
            key_revision: shorts[1],
            minor_revision: shorts[2],
            count: shorts[3],
        });
    }
}

#[derive(Debug)]
pub struct GeoKey {
    pub id: u16,
    pub location: u16,
    pub count: u16,
    pub value: Option<u16>
}

impl GeoKey {
    pub fn from_shorts(shorts: &[u16]) -> Result<GeoKey, TIFFErrorState>{
        if shorts.len() != 4 {
            return Err(TIFFErrorState::GeoKeyDirectoryError(UnexpectedFormat(String::from("Unexpected length key!"))));
        }
        let id = shorts[0];
        let location = shorts[1];
        let count = shorts[2];
        let value = if location == 0 {Some(shorts[3])} else {None};

        return Ok(GeoKey {
            id,
            location,
            count,
            value
        });
    }
}


#[derive(Debug)]
pub struct GeoKeyDirectory {
    pub header: GeoKeyDirectoryHeader,
    pub keys: HashMap<u16, GeoKey>
}

impl GeoKeyDirectory {
    pub fn from_shorts(shorts: &Vec<u16>) -> Result<GeoKeyDirectory, TIFFErrorState> {
        if shorts.len() < 4 {
            return Err(TIFFErrorState::GeoKeyDirectoryError(UnexpectedFormat(String::from("Unexpected number of shorts! No Header!"))));
        }

        let header = match GeoKeyDirectoryHeader::from_shorts(&shorts[0..4]) {
            Ok(h) => h,
            Err(e) => {
                return Err(e);
            }
        };

        if shorts.len() != (4 + (header.count * 4)) as usize {
            return Err(TIFFErrorState::GeoKeyDirectoryError(UnexpectedFormat(String::from("Unexpected number of shorts! Should be a multiple of 4!"))));
        };
        let mut map = HashMap::with_capacity(header.count as usize);
        for chunk in shorts[4..shorts.len()].chunks(4) {
            let key = match GeoKey::from_shorts(&chunk) {
                Ok(k) => k,
                Err(e) => {
                    return Err(e);
                }
            };
            map.insert(key.id, key);
        }

        return Ok(GeoKeyDirectory {
            header,
            keys: map,
        });
    }

    pub fn get_projection(&self, target_epsg: &str) -> Result<Proj, TIFFErrorState> {
        let crs_code = if let Some(v) = self.keys.get(&2048) {
            if v.location == 0 {
                match v.value {
                    Some(v) => if v == 4277 {
                        27700
                    } else {
                        v
                    },
                    None => {
                        return Err(TIFFErrorState::GeoKeyDirectoryError(UnexpectedFormat(String::from("Location for Geographic EPSG code was 0! Expected value! But none found."))));
                    }
                }
            } else {
                return Err(TIFFErrorState::GeoKeyDirectoryError(UnexpectedFormat(String::from(format!("Expected Geographic EPSG code to be single short! Actual got tag location: {}", v.location)))));
            }
        } else if let Some(v) = self.keys.get(&3072) {
            if v.location == 0 {
                match v.value {
                    Some(v) => v,
                    None => return Err(TIFFErrorState::GeoKeyDirectoryError(UnexpectedFormat(String::from("Location for Projected EPSG code was 0! Expected value! But none found."))))
                }

            } else {
                return Err(TIFFErrorState::GeoKeyDirectoryError(UnexpectedFormat(String::from(format!("Expected Projected EPSG code to be single short! Actual got tag location: {}", v.location)))));
            }
        } else {
            return Err(TIFFErrorState::NotEnoughGeoData);
        };

        let def = if let Some(v) = crs_definitions::from_code(crs_code) {
            v
        } else {
            return Err(TIFFErrorState::ProjectionError(format!("Unsupported CRS: {crs_code}")));
        };

        return Ok(match Proj::from_proj_string(def.proj4) {
            Ok(v) => v,
            Err(e) => {
                return Err(TIFFErrorState::ProjectionError(format!("Projection Error: {e:?}")))
            }
        });
    }
}
