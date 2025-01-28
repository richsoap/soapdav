use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScraperError {
    #[error("page not found: {url}")]
    NotFound {
        url: String,
    },

    #[error("network error: {message}")]
    NetWorkError {
        message: String,
    },

    #[error("parse error: {message}")]
    ParseError {
        message: String,
    },

    #[error("cirtical element not found: {name}")]
    ElementNotFound {
        name: String,
    },

    #[error("initialize failed, field={field}, message={message}")]
    InitializeFailed {
        field: String,
        message: String,
    },
}

impl From<ParseIntError> for ScraperError {
    fn from(err: ParseIntError) -> Self {
        ScraperError::ParseError {
            message: err.to_string(),
        }
    }
}

