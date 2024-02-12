use std::error::Error;
use std::fmt::{Display, Formatter, write};

#[derive(Debug, Clone)]
pub enum RootErrorKind {
    InvalidMapDirectory(String),
    UnexpectedPathType
}

impl Display for RootErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RootErrorKind::InvalidMapDirectory(s) => write!(f, "Configured Map Dir invalid! {}", s),
            RootErrorKind::UnexpectedPathType => write!(f, "Unexpected Map Type! Could be symlink?")
        }
    }
}

impl Error for RootErrorKind {}