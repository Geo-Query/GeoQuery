use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum TIFFErrorState {
    HeaderError(HeaderErrorState),
    IFDEntryError(IFDEntryErrorState),
    GeoKeyDirectoryError(GeoKeyDirectoryErrorState),
    UnexpectedFormat(String),
    ProjectionError(String),
    NotEnoughGeoData,
}

impl Display for TIFFErrorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TIFFErrorState::HeaderError(e) => format!("HeaderError: {e}"),
            TIFFErrorState::IFDEntryError(e) => format!("IFDEntryError: {e}"),
            TIFFErrorState::GeoKeyDirectoryError(e) =>format!("GeoKeyDirectoryError: {e}"),
            TIFFErrorState::UnexpectedFormat(e) => format!("Unexpected format error, reason: {e}"),
            TIFFErrorState::ProjectionError(e) => format!("ProjectionError, reason: {e}"),
            TIFFErrorState::NotEnoughGeoData => "NotEnoughGeoData Error, this file does not contain enough data to build bounding box.".to_string()
        })
    }
}

impl Error for TIFFErrorState {}

#[derive(Debug)]
pub enum GeoKeyDirectoryErrorState {
    ProjectionError(String),
    UnexpectedFormat(String),
}

impl Display for GeoKeyDirectoryErrorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GeoKeyDirectoryErrorState::ProjectionError(s) => format!("ProjectionError: {s}"),
                GeoKeyDirectoryErrorState::UnexpectedFormat(s) =>
                    format!("UnexpectedFormatError: {s}"),
            }
        )
    }
}

impl Error for GeoKeyDirectoryErrorState {}

#[derive(Debug)]
pub enum HeaderErrorState {
    UnexpectedByteOrder([u8; 2]),
    UnexpectedMagicNumber([u8; 2]),
    InvalidLength(usize),
}

impl Display for HeaderErrorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HeaderErrorState::UnexpectedByteOrder(e) => format!("UnexpectedByteOrder: {e:?}"),
                HeaderErrorState::UnexpectedMagicNumber(e) =>
                    format!("UnexpectedMagicNumber: {e:?}"),
                HeaderErrorState::InvalidLength(e) => format!("Invalid Header Buffer Length: {e}"),
            }
        )
    }
}

impl Error for HeaderErrorState {}

#[derive(Debug)]
pub enum IFDEntryErrorState {
    UnexpectedEntryType(u16),
    MissingAssociatedValue(u16),
    InvalidLength(usize),
}

impl Display for IFDEntryErrorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IFDEntryErrorState::UnexpectedEntryType(t) =>
                    format!("UnexpectedEntryType, tag_id: {t}"),
                IFDEntryErrorState::MissingAssociatedValue(t) =>
                    format!("MissingAssociatedValue, tag_id: {t}"),
                IFDEntryErrorState::InvalidLength(l) => format!("Invalid IFD buffer length! {l}"),
            }
        )
    }
}

impl Error for IFDEntryErrorState {}
