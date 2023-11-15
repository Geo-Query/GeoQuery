use std::fs::File;
use std::io::{BufReader, Read};
use crate::dt2::DT2ErrorState::UnexpectedFormat;

#[derive(Debug)]
pub enum DT2ErrorState {
    UnexpectedFormat(String),
    UHLError(UHLErrorState)
}

#[derive(Debug)]
pub enum UHLErrorState {
    InvalidLength(usize),
    InvalidSentinel([u8; 4]),
    InvalidDDMMSSH([u8; 8])
}

fn parse_ddmmssh(data: &[u8]) -> Result<f64, &'static str> {
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



#[derive(Debug)]
pub struct DT2Region {
    top_left: (f64, f64),
    bottom_right: (f64, f64)
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
        if sentinel != [85, 72, 76, 49] {
            return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidSentinel(<[u8; 4]>::try_from(sentinel.clone()).unwrap())))
        }
        // Sentinel is valid, IS A UHL!
        let longitude_str= &buffer[4..12];
        let longitude = match parse_ddmmssh(longitude_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode origin longitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(longitude_str.clone().try_into().unwrap())))
            }
        };
        let latitude_str = &buffer[12..20];
        let latitude = match parse_ddmmssh(latitude_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode origin latitude string.");
                eprintln!("Encountered: {e}");
                return Err(DT2ErrorState::UHLError(UHLErrorState::InvalidDDMMSSH(latitude_str.clone().try_into().unwrap())))
            }
        };
        return Ok(UserHeaderLabel {
            origin: (latitude, longitude)
        })
    }
}


pub fn parse_dt2(reader: &mut BufReader<File>) -> Result<Box<DT2Region>, DT2ErrorState> {
    let mut uhl_buf = [0u8; 80];
    match reader.read_exact(&mut uhl_buf) {
        Ok(_) => match UserHeaderLabel::from_bytes(&uhl_buf) {
            Ok(v) => {
                println!("UHL: {v:?}")

            },
            Err(e) => {
                eprintln!("Failed to read UHL: {e:?}")

            }
        }
        Err(e) => {
            eprintln!("Failed to read bytes: {e:?}")
        }
    }
    return Err(UnexpectedFormat("FOO".to_string()));
}