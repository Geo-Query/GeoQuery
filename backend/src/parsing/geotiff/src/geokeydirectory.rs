use std::collections::HashMap;
use std::fmt::Display;
use crate::error::GeoKeyDirectoryErrorState::UnexpectedFormat;
use proj4rs::Proj;
use crate::error::TIFFErrorState;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_key_directory_header_from_shorts_valid() {
        let shorts = [1, 0, 0, 0]; // Valid input data
        let result = GeoKeyDirectoryHeader::from_shorts(&shorts);
        assert!(result.is_ok());
        let header = result.unwrap();
        assert_eq!(header.key_revision, 0);
        assert_eq!(header.minor_revision, 0);
        assert_eq!(header.count, 0);
    }

    #[test]
    fn test_geo_key_directory_header_from_shorts_invalid_length() {
        let shorts = [1, 0, 0]; // Invalid input data (incorrect length)
        let result = GeoKeyDirectoryHeader::from_shorts(&shorts);
        assert!(result.is_err());
    }

    #[test]
    fn test_geo_key_from_shorts_valid() {
        let shorts = [1, 0, 1, 10]; // Assumed valid input data
        let result = GeoKey::from_shorts(&shorts);
        assert!(result.is_ok());
        let key = result.unwrap();
        assert_eq!(key.id, 1);
        assert_eq!(key.location, 0);
        assert_eq!(key.count, 1);
        assert_eq!(key.value, Some(10));
    }

    #[test]
    fn test_geo_key_from_shorts_invalid_length() {
        let shorts = [1, 0, 1]; // Invalid input data (incorrect length)
        let result = GeoKey::from_shorts(&shorts);
        assert!(result.is_err());
    }

    #[test]
    fn test_geo_key_directory_from_shorts_valid() {
        let shorts = vec![1, 0, 0, 1, 1, 0, 1, 10]; // Assumed valid input data
        let result = GeoKeyDirectory::from_shorts(&shorts);
        assert!(result.is_ok());
        let directory = result.unwrap();
        assert_eq!(directory.keys.len(), 1);
        assert!(directory.keys.contains_key(&1));
    }

    #[test]
    fn test_geo_key_directory_from_shorts_invalid() {
        let shorts = vec![1, 0, 0]; // Invalid input data (incorrect length)
        let result = GeoKeyDirectory::from_shorts(&shorts);
        assert!(result.is_err());
    }
    #[test]
    fn test_geo_key_directory_header_unexpected_version() {
        let shorts = [2, 0, 0, 0]; // Unexpected version
        let result = GeoKeyDirectoryHeader::from_shorts(&shorts);
        assert!(result.is_err());
    }

    #[test]
    fn test_geo_key_directory_header_unexpected_format() {
        let shorts = [1, 0, 0, 0, 0]; // Unexpected format with extra shorts
        let result = GeoKeyDirectoryHeader::from_shorts(&shorts);
        assert!(result.is_err());
    }
    #[test]
    fn test_geo_key_directory_from_shorts_with_multiple_keys() {
        let shorts = vec![1, 0, 0, 2, 1, 0, 1, 10, 2, 0, 1, 20]; // Two valid keys
        let result = GeoKeyDirectory::from_shorts(&shorts);
        assert!(result.is_ok());
        let directory = result.unwrap();
        assert_eq!(directory.keys.len(), 2);
        assert!(directory.keys.contains_key(&1) && directory.keys.contains_key(&2));
    }

    
    fn prepare_geo_key_directory_with_valid_crs() -> GeoKeyDirectory {
        let header = GeoKeyDirectoryHeader {
            key_revision: 1,
            minor_revision: 0,
            count: 1,
        };

        // Assume a GeoKey for EPSG:4326, the GeoKey ID for GeographicTypeGeoKey might need adjustment
        let geographic_type_geo_key = GeoKey {
            id: 2048, // Hypothetical GeoKey ID for GeographicTypeGeoKey
            location: 0,
            count: 1,
            value: Some(4326), // WGS 84
        };

        let mut keys = HashMap::new();
        keys.insert(geographic_type_geo_key.id, geographic_type_geo_key);

        GeoKeyDirectory {
            header,
            keys,
        }
    }

    #[test]
    fn test_get_projection_for_valid_crs_code() {
        let directory = prepare_geo_key_directory_with_valid_crs();
        let target_epsg = "EPSG:4326"; // The target CRS code, used here as an example

        let projection_result = directory.get_projection(target_epsg);

        // Check if projection_result is Ok, the exact type of Ok value depends on the return type of Proj::from_proj_string
        assert!(projection_result.is_ok(), "Failed to get projection for a valid CRS code");
        
    }    


}

