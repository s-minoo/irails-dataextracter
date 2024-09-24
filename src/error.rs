pub type CleanResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    IO(std::io::Error),
    Json(json::Error),
    Data(String), 
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        ParseError::IO(value)
    }
}

impl From<json::Error> for ParseError {
    fn from(value: json::Error) -> Self {
        ParseError::Json(value)
    }
}
