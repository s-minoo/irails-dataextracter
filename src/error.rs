pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Clone)]
pub enum ParseError {}
