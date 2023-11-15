use std::fs::File;
use std::io::BufReader;

pub enum DT2ErrorState {
    UnexpectedFormat(String)
}

pub struct DT2Region {
    top_left: (f64, f64),
    bottom_right: (f64, f64)
}

pub struct UserHeaderLabel {
    
}

pub fn parse_dt2(reader: &mut BufReader<File>) -> Result<Box<DT2Region>, DT2ErrorState> {
    todo!()
}