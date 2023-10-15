use strum::Display;

use crate::Api;

pub const DESERIALIZE_RESPONSE_ERR_MSG: &str = "Cannot deserialize the API response";

#[derive(Debug)]
pub struct Error {
    api: Api,
    code: ErrCode,
    info: String,
}

impl Error {
    pub fn new(api: Api, code: ErrCode, info: &str) -> Self {
        Self {
            api,
            code,
            info: String::from(info),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}][{}]: {}", self.api, self.code, self.info)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Display)]
pub enum ErrCode {
    RequestError,
    DeserializeError,
    ApiError,
}
