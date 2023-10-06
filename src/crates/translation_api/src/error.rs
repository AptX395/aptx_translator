use std::fmt::{Display, Formatter, Result};
use crate::Api;

#[derive(Debug)]
pub struct Error {
    api: Api,
    code: i32,
    reason: String,
}

impl Error {
    pub fn new(api: Api, code: i32, reason: String) -> Self {
        Self { api, code, reason }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{:?}][Error Code {}]: `{}`", self.api, self.code, self.reason)
    }
}

impl std::error::Error for Error {}
