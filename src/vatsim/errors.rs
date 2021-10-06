use rusoto_core::{RusotoError};
use rusoto_dynamodb::{PutItemError};
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum VSwimDBError {
    RustoPutItemError(RusotoError<PutItemError>),
    SerdeDynamoError(serde_dynamo::Error),
}

impl fmt::Display for VSwimDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::RustoPutItemError(..) => {
                write!(f, "There was a problem putting item into DB")
            },
            Self::SerdeDynamoError(..) => {
                write!(f, "There was a problem converting struct to dynamo item")
            }
        }
    }
}

impl error::Error for VSwimDBError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::RustoPutItemError(ref error) => Some(error),
            Self::SerdeDynamoError(ref error) => Some(error),
        }
    }
}

impl From<RusotoError<PutItemError>> for VSwimDBError {
    fn from(error: RusotoError<PutItemError>) -> Self {
        Self::RustoPutItemError(error)
    }
}

impl From<serde_dynamo::Error> for VSwimDBError {
    fn from(error: serde_dynamo::Error) -> Self {
        Self::SerdeDynamoError(error)
    }
}

#[derive(Debug)]
pub enum VatsimDataAPIError {
    SerdeJSONError(serde_json::Error),
    ReqwestError(reqwest::Error),
}

impl fmt::Display for VatsimDataAPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::SerdeJSONError(..) => {
                write!(f, "There was a problem parsing JSON data")
            },
            Self::ReqwestError(..) => {
                write!(f, "There was a problem with an API call")
            }
        }
    }
}

impl error::Error for VatsimDataAPIError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::SerdeJSONError(ref error) => Some(error),
            Self::ReqwestError(ref error) => Some(error),
        }
    }
}

impl From<reqwest::Error> for VatsimDataAPIError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error)
    }
}

impl From<serde_json::Error> for VatsimDataAPIError {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJSONError(error)
    }
}