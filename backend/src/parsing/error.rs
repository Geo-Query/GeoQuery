use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseErrorKind {
    UnparseableExtension,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::UnparseableExtension => write!(f, "Unparseable Extension! For some reason, OsStr cannot be converted into &str! Ignoring file, but this SHOULD NOT BE HAPPENING!")
        }
    }
}

impl Error for ParseErrorKind {}
