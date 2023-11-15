use std::fs::File;
use std::io::{BufReader, Read};
use crate::spatial::{Coordinate, Region};

#[derive(Debug)]
pub enum DT2ErrorState {
    UnexpectedFormat(String),
    UHLError(UHLErrorState),
    DSIError(DSIErrorState)
}

#[derive(Debug)]
pub enum UHLErrorState {
    InvalidLength(usize),
    InvalidSentinel([u8; 4]),
    InvalidDDMMSSH([u8; 8])
}

#[derive(Debug)]
pub enum DSIErrorState {
    InvalidLength(usize),
    InvalidSentinel([u8; 4]),
    InvalidDDMMSSH([u8; 8])
}

fn parse_dddmmssh(data: &[u8]) -> Result<f64, &'static str> {
    if data.len() != 8 {
        return Err("Invalid length of data");
    }

    let degrees = String::from_utf8(data[0..3].to_vec())
        .map_err(|_| "Invalid degrees")?
        .parse::<f64>()
        .map_err(|_| "Invalid degrees")?;

    let minutes = String::from_utf8(data[3..5].to_vec())
        .map_err(|_| "Invalid minutes")?
        .parse::<f64>()
        .map_err(|_| "Invalid minutes")?;

    let seconds = String::from_utf8(data[5..7].to_vec())
        .map_err(|_| "Invalid seconds")?
        .parse::<f64>()
        .map_err(|_| "Invalid seconds")?;

    let hemisphere = data[7] as char;
    if hemisphere != 'N' && hemisphere != 'S' && hemisphere != 'W' && hemisphere != 'E' {
        return Err("Invalid hemisphere");
    }

    let sign = if (hemisphere == 'S' || hemisphere == 'W') { -1.0 } else { 1.0 };
    let longitude = sign * (degrees + minutes / 60.0 + seconds / 3600.0);

    Ok(longitude)
}
fn parse_ddmmssh(data: &[u8]) -> Result<f64, &'static str> {
    if data.len() != 7 {
        return Err("Invalid length of data");
    }

    let degrees = String::from_utf8(data[0..2].to_vec())
        .map_err(|_| "Invalid degrees")?
        .parse::<f64>()
        .map_err(|_| "Invalid degrees")?;

    let minutes = String::from_utf8(data[2..4].to_vec())
        .map_err(|_| "Invalid minutes")?
        .parse::<f64>()
        .map_err(|_| "Invalid minutes")?;

    let seconds = String::from_utf8(data[4..6].to_vec())
        .map_err(|_| "Invalid seconds")?
        .parse::<f64>()
        .map_err(|_| "Invalid seconds")?;

    let hemisphere = data[6] as char;
    if hemisphere != 'N' && hemisphere != 'S' && hemisphere != 'W' && hemisphere != 'E' {
        return Err("Invalid hemisphere");
    }

    let sign = if (hemisphere == 'S' || hemisphere == 'W') { -1.0 } else { 1.0 };
    let longitude = sign * (degrees + minutes / 60.0 + seconds / 3600.0);

    Ok(longitude)
}



#[derive(Debug)]
pub struct DT2Region {
    top_left: Coordinate,
    top_right: Coordinate,
    bottom_right: Coordinate,
    bottom_left: Coordinate
}

impl Region for DT2Region {
    fn bottom_left(&self) -> Coordinate {
        self.bottom_left
    }

    fn bottom_right(&self) -> Coordinate {
        self.bottom_right
    }

    fn top_left(&self) -> Coordinate {
        self.top_left
    }

    fn top_right(&self) -> Coordinate {
        self.top_right
    }
}

#[derive(Debug)]
pub struct UserHeaderLabel {
    origin: (f64, f64)
}

impl UserHeaderLabel {
    pub fn from_bytes(buffer: &[u8]) -> Result<UserHeaderLabel, DT2ErrorState> {
        if (buffer.len() != 80) {
            return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidLength(buffer.len())));
        }
        let sentinel = &buffer[0..4];
        if sentinel != [85, 72, 76, 49] { // Assert equivalent to given byte string (UHL1 in ASCII)
            return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidSentinel(<[u8; 4]>::try_from(sentinel.clone()).unwrap())))
        }
        // Sentinel is valid, IS A UHL1!

        // Now parse origin values. LONG, LAT, in DDMMSSH format where H specifies hemisphere.
        let longitude_str= &buffer[4..12];
        let longitude = match parse_dddmmssh(longitude_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode origin longitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(longitude_str.clone().try_into().unwrap())))
            }
        };
        let latitude_str = &buffer[12..20];
        let latitude = match parse_dddmmssh(latitude_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode origin latitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(latitude_str.clone().try_into().unwrap())))
            }
        };
        return Ok(UserHeaderLabel {
            origin: (latitude, longitude) // For now we will only do origin, as is all that is useful.
        })
    }
}

#[derive(Debug)]
pub struct DataSetIdentification {
    sw_corner: Coordinate,
    ne_corner: Coordinate,
    nw_corner: Coordinate,
    se_corner: Coordinate
}

impl DataSetIdentification {
    pub fn from_bytes(buffer: &[u8]) -> Result<DataSetIdentification, DT2ErrorState>{
        if buffer.len() != 648 {
            return Err(DT2ErrorState::DSIError(DSIErrorState::InvalidLength(buffer.len())));
        }

        let sentinel = &buffer[0..4];
        if sentinel != [68, 83, 73, 85] {
            return Err(DT2ErrorState::DSIError(DSIErrorState::InvalidSentinel(sentinel.clone().try_into().unwrap())));
        }
        let sw_lat_str = &buffer[204..211];
        let sw_lat = match parse_ddmmssh(sw_lat_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode sw latitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(sw_lat_str.clone().try_into().unwrap())))
            }
        };
        let sw_long_str = &buffer[211..219];
        let sw_long = match parse_dddmmssh(sw_long_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode sw longitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(sw_long_str.clone().try_into().unwrap())))
            }
        };

        let nw_lat_str = &buffer[219..226];
        let nw_lat = match parse_ddmmssh(nw_lat_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode NW latitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(nw_lat_str.clone().try_into().unwrap())))
            }
        };
        let nw_long_str = &buffer[226..234];
        let nw_long = match parse_dddmmssh(nw_long_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode NW longitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(nw_long_str.clone().try_into().unwrap())))
            }
        };


        let ne_lat_str = &buffer[234..241];
        let ne_lat = match parse_ddmmssh(ne_lat_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode NE latitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(ne_lat_str.clone().try_into().unwrap())))
            }
        };
        let ne_long_str = &buffer[241..249];
        let ne_long = match parse_dddmmssh(ne_long_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode NE longitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(ne_long_str.clone().try_into().unwrap())))
            }
        };

        let se_lat_str = &buffer[249..256];
        let se_lat = match parse_ddmmssh(se_lat_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode SE latitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(se_lat_str.clone().try_into().unwrap())))
            }
        };
        let se_long_str = &buffer[256..264];
        let se_long = match parse_dddmmssh(se_long_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode SE longitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(se_long_str.clone().try_into().unwrap())))
            }
        };



        return Ok(DataSetIdentification {
            sw_corner: (sw_lat, sw_long),
            ne_corner: (ne_lat, ne_long),
            nw_corner: (nw_lat, nw_long),
            se_corner: (se_lat, se_long),
        })
    }
}


pub fn parse_dt2(reader: &mut BufReader<File>) -> Result<Box<DT2Region>, DT2ErrorState> {
    let mut uhl_buf = [0u8; 80];
    let uhl = match reader.read_exact(&mut uhl_buf) {
        Ok(_) => UserHeaderLabel::from_bytes(&uhl_buf)?,
        Err(e) => {
            return Err(DT2ErrorState::UnexpectedFormat(format!("Failed to read bytes: {e:?}").to_string()));
        }
    };
    let mut dsi_buf = [0u8; 648];
    let dsi = match reader.read_exact(&mut dsi_buf) {
        Ok(_) => DataSetIdentification::from_bytes(&dsi_buf)?,
        Err(e) => {
            return Err(DT2ErrorState::UnexpectedFormat(format!("Failed to read bytes: {e:?}").to_string()));
        }
    };

    return Ok(Box::new(DT2Region {
        top_left: dsi.nw_corner,
        top_right: dsi.ne_corner,
        bottom_right: dsi.se_corner,
        bottom_left: dsi.sw_corner,
    }))
}