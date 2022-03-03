use crate::error::ErrorType;

pub type Result<T> = std::result::Result<T, ErrorType>;

pub type IOResult<T> = std::result::Result<T, std::io::Error>;
