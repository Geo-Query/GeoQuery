#[derive(Debug, Clone)]
pub enum TIFFErrorState {
    UnexpectedFormat(String),
    MissingTag,
    FailedToParseTag,
}
