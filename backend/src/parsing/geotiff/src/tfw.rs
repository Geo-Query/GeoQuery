use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::{Deserialize, Serialize};
use crate::error::TIFFErrorState::NotEnoughGeoData;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TFWData {

}
pub fn parse_tfw(reader: &mut BufReader<File>) -> Result<TFWData, Box<dyn Error>> {
    let lines = reader.lines().take(6);
    let strings: Result<Vec<String>, _> = lines.collect();
    let strings: Vec<String> = strings?;
    eprintln!("Failed to parse tiff file with accompanying sidecar file!");
    eprintln!("This is unstable feature; not currently fully implemented.");
    eprintln!("TFW Parsing Implementation as of current moment incomplete.");
    eprintln!("Ignoring file!");
    return Err(NotEnoughGeoData.into());
}