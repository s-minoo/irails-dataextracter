pub type CleanResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    IOError(std::io::Error),
    JSONError(json::Error),
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        ParseError::IOError(value)
    }
}

impl From<json::Error> for ParseError {
    fn from(value: json::Error) -> Self {
        ParseError::JSONError(value)
    }
}
