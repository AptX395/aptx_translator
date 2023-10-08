use crate::Api;

#[derive(Debug)]
pub struct Error {
    api: Api,
    code: ErrorCode,
    reason: String,
}

impl Error {
    pub fn new(api: Api, code: ErrorCode, reason: &str) -> Self {
        Self { api, code, reason: String::from(reason) }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}][{}]: `{}`", self.api, self.code, self.reason)
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
#[derive(strum::Display)]
pub enum ErrorCode {
    RequestError,
    DeserializeError,
}
