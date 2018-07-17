use std::error::Error as StdError;
use std::fmt;

use reqwest::Error as ReqwestError;
use serde_json;

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
}

#[derive(Debug)]
pub enum ErrorType {
    APIError,
    JsonError,
    NoResults,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.error_type {
            ErrorType::APIError => "Error while calling TradeSatoshi API",
            ErrorType::JsonError => "Error while converting response to Json Value",
            ErrorType::NoResults => "No results found",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_type {
            ErrorType::APIError => write!(f, "{}: {}", self.description(), self.message),
            ErrorType::JsonError => write!(f, "{}: {}", self.description(), self.message),
            ErrorType::NoResults => write!(f, "{} ({})!", self.description(), self.message),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error {
            error_type: ErrorType::JsonError,
            message: error.description().to_string(),
        }
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        let mut err: Option<Error> = None;

        if error.is_http() {
            err = match error.url() {
                Some(url) => Some(Error {
                    error_type: ErrorType::APIError,
                    message: format!("Problem making request to: {}", url),
                }),
                None => Some(Error {
                    error_type: ErrorType::APIError,
                    message: "No Url given".to_string(),
                }),
            }
        }

        if error.is_serialization() {
            err = match error.get_ref() {
                Some(err) => Some(Error {
                    error_type: ErrorType::APIError,
                    message: format!("Problem parsing information {}", err),
                }),
                None => Some(Error {
                    error_type: ErrorType::APIError,
                    message: "Problem parsing information (no info given)".to_string(),
                }),
            }
        }

        if error.is_redirect() {
            err = Some(Error {
                error_type: ErrorType::APIError,
                message: "Server redirecting too many times or making loop".to_string(),
            });
        }

        if err.is_none() {
            err = Some(Error {
                error_type: ErrorType::APIError,
                message: "Error undefined!".to_string(),
            });
        }

        err.expect("Error should exist!")
    }
}
